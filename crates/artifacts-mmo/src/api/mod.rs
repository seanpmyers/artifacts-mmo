use std::str::FromStr;

pub mod v1;
pub mod v4;

pub const ARTIFACTS_MMO_API_HOST: &str = "api.artifactsmmo.com";
pub const ARTIFACTS_MMO_HOST: &str = "artifactsmmo.com";
pub const HTTP_ACCEPT_HEADER: &str = "accept";
pub const HTTP_AUTHORIZATION_HEADER: &str = "authorization";
pub const HTTP_APPLICATION_JSON: &str = "application/json";

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

    fn default_page_size() -> i32 {
        50i32
    }

    fn get_default_host() -> &'static str {
        ARTIFACTS_MMO_API_HOST
    }
    fn http_request_method() -> http::Method;
    fn pageable() -> bool;

    fn path(&self) -> String;

    fn query(&self) -> Option<String>;

    fn request_body(&self) -> Option<Self>
    where
        Self: Sized;
}

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
pub struct ArtifactsMMOClient {
    pub api_token: String,
}

impl ArtifactsMMOClient {
    pub fn update_api_token(&mut self, api_token: String) {
        self.api_token = api_token;
    }
    pub fn get_headers(&self) -> Vec<(String, String)> {
        vec![
            (
                HTTP_ACCEPT_HEADER.to_string(),
                HTTP_APPLICATION_JSON.to_string(),
            ),
            (
                HTTP_AUTHORIZATION_HEADER.to_string(),
                format!("Bearer {}", self.api_token),
            ),
        ]
    }

    pub fn make_api_call<T: serde::Serialize + for<'de> serde::Deserialize<'de> + Endpoint>(
        &self,
        http_client: &mut ureq::Agent,
        headers: Vec<(String, String)>,
        request: T,
    ) -> Result<ureq::http::Response<ureq::Body>, ureq::Error> {
        let mut request_builder: ureq::http::request::Builder = ureq::http::Request::builder()
            .uri(request.build_uri(None)?)
            .method(T::http_request_method());

        for (key, value) in Self::get_headers(self) {
            request_builder = request_builder.header(&key, &value);
        }

        for (key, value) in headers {
            request_builder = request_builder.header(&key, &value);
        }

        match request.request_body() {
            Some(body) => {
                Ok(http_client.run(request_builder.body(ureq::SendBody::from_json(&body)?)?)?)
            }
            None => Ok(http_client.run(request_builder.body(())?)?),
        }
    }
}
