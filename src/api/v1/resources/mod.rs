use crate::{
    constants::api::ARTIFACTS_MMO_HOST,
    http::{ApiEndpoint, HttpRequestMethod},
};

pub const GET_ALL_RESOURCES: ApiEndpoint = ApiEndpoint {
    host: ARTIFACTS_MMO_HOST,
    path: "/resources/",
    http_request_method: HttpRequestMethod::GET,
};
pub const GET_RESOURCES: ApiEndpoint = ApiEndpoint {
    host: ARTIFACTS_MMO_HOST,
    path: "/resources/{code}",
    http_request_method: HttpRequestMethod::GET,
};

pub const ARTIFACTS_MMO_IMAGE_BASE_URL: &str = "https://artifactsmmo.com/images";

#[derive(Debug)]
pub enum ImageResourceType {
    Characters,
    Items,
    Monsters,
    Maps,
    Resources,
    Effects,
}

impl std::fmt::Display for ImageResourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Characters => write!(f, "characters"),
            Self::Items => write!(f, "items"),
            Self::Monsters => write!(f, "monsters"),
            Self::Maps => write!(f, "maps"),
            Self::Resources => write!(f, "resources"),
            Self::Effects => write!(f, "effects"),
        }
    }
}

pub fn get_image_url(code: String, image_resource_type: ImageResourceType) -> String {
    format!(
        "{}/{}/{}.png",
        ARTIFACTS_MMO_IMAGE_BASE_URL, image_resource_type, code
    )
}
