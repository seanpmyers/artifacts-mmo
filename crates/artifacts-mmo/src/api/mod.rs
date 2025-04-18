use std::str::FromStr;

pub mod v1;
pub mod v4;

pub const ARTIFACTS_MMO_HOST: &str = "api.artifactsmmo.com";

pub trait Endpoint: serde::Serialize + for<'de> serde::Deserialize<'de> {
    type Response: serde::de::DeserializeOwned;

    fn build_uri(&self, host: Option<&'static str>) -> Result<http::Uri, http::Error> {
        let authority: http::uri::Authority = match host {
            None => http::uri::Authority::from_static(&ARTIFACTS_MMO_HOST),
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
        ARTIFACTS_MMO_HOST
    }
    fn http_request_method() -> http::Method;
    fn pageable() -> bool;

    fn path(&self) -> String;

    fn query(&self) -> Option<String>;

    fn request_body(&self) -> Option<Self>
    where
        Self: Sized;
}
