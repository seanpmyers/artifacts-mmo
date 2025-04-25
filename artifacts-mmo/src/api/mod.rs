use std::str::FromStr;

pub mod v1;
pub mod v4;

pub const ARTIFACTS_MMO_API_HOST: &str = "api.artifactsmmo.com";
pub const ARTIFACTS_MMO_HOST: &str = "artifactsmmo.com";
pub const HTTP_ACCEPT_HEADER: &str = "accept";
pub const HTTP_AUTHORIZATION_HEADER: &str = "authorization";
pub const HTTP_APPLICATION_JSON: &str = "application/json";
pub const PAGE_SIZE_MAX: u32 = 100u32;
pub const PAGE_SIZE_MIN: u32 = 1u32;
pub const PAGE_SIZE_DEFAULT: u32 = 50u32;

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
pub enum EndpointResponse<T> {
    #[default]
    Error,
    Success(T),
}

pub trait Endpoint: serde::Serialize + for<'de> serde::Deserialize<'de> {
    type Response: serde::de::DeserializeOwned;

    fn build_uri(&self, host: Option<&'static str>) -> Result<http::Uri, http::Error> {
        let authority: http::uri::Authority = match host {
            None => http::uri::Authority::from_static(&ARTIFACTS_MMO_API_HOST),
            Some(host) => http::uri::Authority::from_static(&host),
        };
        let path_and_query: http::uri::PathAndQuery = match Self::query(self) {
            None => http::uri::PathAndQuery::from_str(&Self::path(self))?,
            Some(query) => {
                http::uri::PathAndQuery::from_str(&format!("{}/?{}", Self::path(self), query))?
            }
        };
        http::Uri::builder()
            .scheme(http::uri::Scheme::HTTPS)
            .authority(authority)
            .path_and_query(path_and_query)
            .build()
    }

    fn call(
        &self,
        http_client: &mut ureq::Agent,
        bearer_token: String,
    ) -> EndpointResponse<Self::Response>
    where
        <Self as Endpoint>::Response: serde::Serialize,
    {
        match self.make_api_call::<Self::Response>(http_client, bearer_token) {
            Ok(bytes) => match Self::to_data(bytes) {
                Ok(data) => {
                    return EndpointResponse::Success(data);
                }
                Err(error) => log::error!("{}", error),
            },
            Err(error) => log::error!("{}", error),
        };
        EndpointResponse::Error
    }

    fn default_headers(&self, bearer_token: String) -> Vec<(String, String)> {
        vec![
            (
                HTTP_ACCEPT_HEADER.to_string(),
                HTTP_APPLICATION_JSON.to_string(),
            ),
            (
                HTTP_AUTHORIZATION_HEADER.to_string(),
                format!("Bearer {}", bearer_token),
            ),
        ]
    }

    fn get_default_host() -> &'static str {
        ARTIFACTS_MMO_API_HOST
    }

    fn get_headers(&self, bearer_token: String) -> Vec<(String, String)> {
        Self::default_headers(&self, bearer_token)
    }

    fn http_request_method() -> http::Method;

    fn make_api_call<T: serde::Serialize + for<'de> serde::Deserialize<'de>>(
        &self,
        http_client: &mut ureq::Agent,
        bearer_token: String,
    ) -> Result<Vec<u8>, ureq::Error> {
        let mut request_builder: ureq::http::request::Builder = ureq::http::Request::builder()
            .uri(Self::build_uri(self, None)?)
            .method(Self::http_request_method());

        for (key, value) in Self::get_headers(self, bearer_token) {
            request_builder = request_builder.header(&key, &value);
        }

        log::info!("Making HTTP requet: {:?}", request_builder);

        match Self::request_body(self) {
            Some(body) => Ok(http_client
                .run(request_builder.body(ureq::SendBody::from_json(&body)?)?)?
                .body_mut()
                .read_to_vec()?),
            None => Ok(http_client
                .run(request_builder.body(())?)?
                .body_mut()
                .read_to_vec()?),
        }
    }

    fn page_size() -> u32 {
        PAGE_SIZE_DEFAULT
    }

    fn pageable() -> bool {
        false
    }

    fn path(&self) -> String;

    fn query(&self) -> Option<String> {
        None
    }

    fn request_body(&self) -> Option<Self>
    where
        Self: Sized,
    {
        None
    }

    fn to_data(bytes: Vec<u8>) -> Result<Self::Response, serde_json::Error> {
        Ok(serde_json::de::from_slice::<Self::Response>(&bytes)?)
    }
}
