use crate::{components::parse_components, paths::parse_paths};

pub mod constants {
    pub const STRUCT_ANNOTATIONS: &'static str =
        r#"#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]"#;
    pub const ENUM_ANNOTATIONS: &'static str = r#"#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]"#;
}
pub mod components;
pub mod file_system;
pub mod paths;

fn main() -> anyhow::Result<()> {
    let file: &'static str = include_str!("../schema/version/v5/openapi.json");
    let spec: oas3::Spec = oas3::from_json(file).unwrap();

    parse_components(&spec)?;
    let _paths: Vec<paths::Path> = parse_paths(&spec)?;

    Ok(())
}

pub fn log_error(error: anyhow::Error) {
    eprintln!("{:#?} {:#?}", error, std::backtrace::Backtrace::capture())
}
