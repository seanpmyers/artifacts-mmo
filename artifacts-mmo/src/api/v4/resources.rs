use crate::api::ARTIFACTS_MMO_HOST;

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ImageResourceType {
    #[default]
    Characters,
    Effects,
    Items,
    Maps,
    Monsters,
    Resources,
}

impl std::fmt::Display for ImageResourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Characters => "characters",
                Self::Items => "items",
                Self::Monsters => "monsters",
                Self::Maps => "maps",
                Self::Resources => "resources",
                Self::Effects => "effects",
            }
        )
    }
}

impl ImageResourceType {
    pub fn to_uri_string(&self, code: &str) -> String {
        format!("{}{}.png", ARTIFACTS_MMO_HOST, Self::path(&self, code))
    }

    pub fn path(&self, code: &str) -> String {
        format!("/images/{}/{}.png", self.to_string(), code)
    }

    pub fn to_uri(&self, code: &str) -> Result<http::Uri, http::Error> {
        http::Uri::builder()
            .scheme(http::uri::Scheme::HTTPS)
            .authority(ARTIFACTS_MMO_HOST)
            .path_and_query(&Self::path(self, code))
            .build()
    }
}
