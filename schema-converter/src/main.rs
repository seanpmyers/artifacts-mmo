use std::collections::HashMap;

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
    let file: &'static str = include_str!("../schema/version/v6/openapi.json");
    file_system::output_folder()?;
    let spec: oas3::Spec = oas3::from_json(file).unwrap();

    let map: HashMap<String, String> = parse_components(&spec)?;
    println!("{:?}", map);
    let paths: Vec<paths::Path> = parse_paths(&spec, &map)?;
    request_to_rust(paths)?;

    Ok(())
}

pub fn log_error(error: anyhow::Error) {
    eprintln!("{:#?} {:#?}", error, std::backtrace::Backtrace::capture())
}

pub fn request_to_rust(paths: Vec<paths::Path>) -> anyhow::Result<()> {
    let mut result: String = String::from("use crate::api::Endpoint;\n");

    for path in paths.into_iter() {
        for request in path.requests.into_iter() {
            result.push_str(&request.to_output_struct(&path.path));
        }
    }

    file_system::rust_to_file(result.as_bytes(), "output/request/mod.rs")?;

    Ok(())
}
