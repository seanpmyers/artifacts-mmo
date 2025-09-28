use std::{io::Write, str::from_utf8};

use oas3::{
    spec::{ObjectOrReference, ObjectSchema, SchemaType, SchemaTypeSet},
    Spec,
};

pub const STRUCT_ANNOTATIONS: &'static str =
    r#"#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]"#;
pub const ENUM_ANNOTATIONS: &'static str = r#"#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]"#;

fn main() -> anyhow::Result<()> {
    let file: &'static str = include_str!("../schema/version/v5/openapi.json");
    let spec: oas3::Spec = oas3::from_json(file).unwrap();
    let Some(components) = &spec.components else {
        panic!("No components");
    };
    let mut schema_file_modules: Vec<String> = vec![];
    for (schema_name, schema) in components.schemas.iter() {
        let is_reference = matches!(&schema, ObjectOrReference::Ref { .. });
        let object_schema: ObjectSchema = schema.resolve(&spec)?;
        match &object_schema.schema_type {
            Some(typeset) => match &typeset {
                SchemaTypeSet::Single(single_type) => match single_type {
                    SchemaType::Boolean => {
                        json_to_file("output/test.json", serde_json::to_value(object_schema)?)?;
                        continue;
                    }
                    SchemaType::Integer => {
                        json_to_file("output/test.json", serde_json::to_value(object_schema)?)?;
                        continue;
                    }
                    SchemaType::Number => {
                        json_to_file("output/test.json", serde_json::to_value(object_schema)?)?;
                        continue;
                    }
                    SchemaType::String => {
                        match object_schema.enum_values.is_empty() {
                            true => {
                                json_to_file(
                                    "output/test.json",
                                    serde_json::to_value(object_schema)?,
                                )?;
                                break;
                                // continue;
                            }
                            false => {
                                let rust_string: String = schema_string_enum_to_rust_string(
                                    schema_name,
                                    &object_schema,
                                    &spec,
                                )?;
                                let camel_case_schema_name: String = to_camel_case(schema_name);
                                rust_to_file(
                                    rust_string.as_bytes(),
                                    &format!("output/{}.rs", camel_case_schema_name),
                                )?;
                                schema_file_modules.push(camel_case_schema_name);
                            }
                        }
                    }
                    SchemaType::Array => {
                        json_to_file("output/test.json", serde_json::to_value(object_schema)?)?;
                        continue;
                    }
                    SchemaType::Object => {
                        let rust_string: String = schema_object_to_string(
                            schema_name,
                            &object_schema,
                            &spec,
                            is_reference,
                        )?;
                        let camel_case_schema_name: String = to_camel_case(schema_name);
                        rust_to_file(
                            rust_string.as_bytes(),
                            &format!("output/{}.rs", camel_case_schema_name),
                        )?;
                        schema_file_modules.push(camel_case_schema_name);
                    }
                    SchemaType::Null => {
                        json_to_file("output/test.json", serde_json::to_value(object_schema)?)?;
                        continue;
                    }
                },
                SchemaTypeSet::Multiple(_items) => {
                    json_to_file("output/test.json", serde_json::to_value(object_schema)?)?;
                    continue;
                }
            },
            None => {
                json_to_file("output/test.json", serde_json::to_value(object_schema)?)?;
                continue;
            }
        }
        // println!("{}, {:?}", schema_name, object_schema);
    }

    rust_to_file(
        create_modules(&schema_file_modules).as_bytes(),
        "output/mod.rs",
    )?;

    println!("schema count: {}", components.schemas.len());

    Ok(())
}

pub fn json_to_file(file_path_string: &str, data: serde_json::Value) -> anyhow::Result<()> {
    let file_path: std::path::PathBuf = std::path::Path::new(&file_path_string).to_path_buf();

    let file: std::fs::File = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_path)?;

    serde_json::to_writer_pretty(file, &data)?;

    Ok(())
}

pub fn remove_invalid_chars(input: &str) -> String {
    input
        .chars()
        .map(|x| match x.is_ascii_alphanumeric() {
            true => x,
            false => '_',
        })
        .collect()
}

pub fn to_camel_case(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    let mut buffer = String::new();
    for (_, b) in text.chars().rev().enumerate() {
        let mut b = b;
        if !b.is_alphanumeric() {
            b = '_';
        }
        match b.is_ascii_uppercase() {
            true => {
                buffer.push(b.to_ascii_lowercase());
            }
            false => {
                if buffer.len() > 0 {
                    result.push_str(&buffer);
                    result.push('_');
                    buffer.clear();
                }
                result.push(b);
            }
        }
    }

    if !buffer.is_empty() {
        if buffer.len() > 1 {
            if buffer.ends_with("cpn") {
                let x = buffer.split_off(1);
                result.push_str(&buffer);
                result.push('_');
                result.push_str(&x);
            } else {
                result.push_str(&buffer);
            }
        } else {
            result.push_str(&buffer);
        }
    }

    result.chars().rev().collect()
}

pub fn create_modules(module_names: &Vec<String>) -> String {
    let mut result: String = String::new();

    for module in module_names.iter() {
        result.push_str(&format!("pub mod {};\n", remove_invalid_chars(module)));
    }

    result
}

pub fn rust_to_file(bytes: &[u8], file_path_string: &str) -> anyhow::Result<()> {
    let file_path: std::path::PathBuf = std::path::Path::new(&file_path_string).to_path_buf();

    let mut file: std::fs::File = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_path)?;

    file.write_all(bytes)?;

    Ok(())
}

const RESERVED_FIELD_NAMES: [&str; 2] = ["type", "enum"];

pub fn schema_object_to_string(
    schema_name: &str,
    schema: &ObjectSchema,
    spec: &Spec,
    is_reference: bool,
) -> anyhow::Result<String> {
    let invalid_field_names: std::collections::HashSet<String> = RESERVED_FIELD_NAMES
        .into_iter()
        .map(|x| x.to_string())
        .collect();

    let mut result: String = String::new();

    result.push_str(&format!("{STRUCT_ANNOTATIONS}\n"));
    result.push_str(&format!(
        "pub struct {} {{\n",
        remove_invalid_chars(schema_name)
    ));
    for property in schema.properties.iter() {
        let is_reference: bool = matches!(property.1, ObjectOrReference::Ref { .. });
        let mut property_name: String = property.0.clone();
        let property_schema: ObjectSchema = property.1.resolve(&spec)?;

        let mut property_annotations: Vec<String> = vec![];
        let mut property_comments: String = String::new();
        let property_type: &str = match property_schema.schema_type {
            Some(typeset) => match typeset {
                SchemaTypeSet::Single(single_type) => match single_type {
                    SchemaType::Boolean => "bool",
                    SchemaType::Integer => "i32",
                    SchemaType::Number => "f32",
                    SchemaType::String => match property_schema.format {
                        Some(format) => match format.as_str() {
                            "date-time" => "chrono::DateTime<chrono::Utc>",
                            _ => {
                                property_comments.push_str(&format!("TODO_format_{}", format));
                                &format!("String")
                            }
                        },
                        None => "String",
                    },
                    SchemaType::Array => match property_schema.items {
                        Some(items) => {
                            let schema: &oas3::spec::Schema = items.as_ref();
                            let t = match schema {
                                oas3::spec::Schema::Boolean(_boolean_schema) => "bool",
                                oas3::spec::Schema::Object(object_or_reference) => {
                                    let t_object = object_or_reference.resolve(&spec)?;
                                    &match t_object.title {
                                        Some(title) => title,
                                        None => "serde::Value".to_string(),
                                    }
                                }
                            };
                            &format!("Vec<{}>", t)
                        }
                        _ => "Vec<serde::Value>",
                    },
                    SchemaType::Object => match is_reference {
                        true => &property
                            .1
                            .resolve(&spec)?
                            .title
                            .map_or("TODO_MISSING_TITLE".to_string(), |x| x),
                        false => "TODO_OBJECT_NOT_REFERENCE",
                    },
                    SchemaType::Null => "TODO__NULL",
                },
                SchemaTypeSet::Multiple(_items) => "TODO__MULTIPLE ",
            },
            None => &{
                let mut none_type_result = String::new();
                if !property_schema.all_of.is_empty() {
                    for all_of_schema in property_schema.all_of.iter() {
                        let is_ref: bool = matches!(all_of_schema, ObjectOrReference::Ref { .. });
                        let resolved_schema: ObjectSchema = all_of_schema.resolve(&spec)?;
                        match is_ref {
                            true => {
                                property_annotations.push("#[serde(flatten)]".to_string());
                                none_type_result.push_str(&format!(
                                    "{}",
                                    resolved_schema
                                        .title
                                        .unwrap_or("TODO_MISSING_TITLE_ALL_OF".to_string())
                                ))
                            }
                            false => none_type_result.push_str("TODO_NOT_REF_ALL_OF\n"),
                        }
                    }
                }
                if !property_schema.any_of.is_empty() {
                    let any_of_result: AnyOfResult = any_of_to_type(&property_schema.any_of, &spec);
                    if any_of_result.enum_type.is_some() {
                        todo!();
                    }

                    none_type_result = any_of_result.field_type;
                }
                none_type_result
            },
        };

        let mut field_comment: String = String::from("\t///");
        match (&property_schema.title, &property_schema.description) {
            (None, None) => {}
            (None, Some(description)) => {
                field_comment.push_str(&format!(" {}", description));
            }
            (Some(title), None) => {
                field_comment.push_str(&format!(" {}", title));
            }
            (Some(title), Some(description)) => {
                field_comment.push_str(&format!(" {}: {}", title, description));
            }
        };
        field_comment.push_str("\n");

        if property_schema.title.is_some() || property_schema.description.is_some() {
            result.push_str(&field_comment);
        }

        for annotation in property_annotations.into_iter() {
            result.push_str(&format!("\t{}\n", annotation));
        }

        if invalid_field_names.contains(&property_name) {
            property_name = format!("{}_{}", schema_name, property_name);
        };
        result.push_str(&format!("\t{}: {},\n", property_name, property_type));
    }
    result.push_str(&format!("\n}}\n"));
    Ok(result)
}

pub fn schema_string_enum_to_rust_string(
    schema_name: &str,
    schema: &ObjectSchema,
    spec: &Spec,
) -> anyhow::Result<String> {
    let mut result: String = String::new();

    result.push_str(&format!("{}\n", ENUM_ANNOTATIONS));
    result.push_str(&format!(
        "pub enum {} {{\n",
        remove_invalid_chars(schema_name)
    ));
    result.push_str(&format!("\t#[default]\n"));
    for x in schema.enum_values.iter() {
        let mut enum_variation_name = x.as_str().unwrap_or_default();
        let mut y: Vec<Vec<u8>> = enum_variation_name
            .split('_')
            .map(|part| part.as_bytes().to_vec())
            .collect();
        let mut reformatted_enum_name: String = String::new();

        for part in y.iter_mut() {
            if let Some(first_letter) = part.get_mut(0) {
                *first_letter -= b'a' - b'A';
                reformatted_enum_name.push_str(from_utf8(part)?);
            }
        }

        if !reformatted_enum_name.is_empty() {
            enum_variation_name = &reformatted_enum_name;
        }

        result.push_str(&format!("\t{},\n", enum_variation_name));
    }
    result.push_str(&format!("}}\n"));

    Ok(result)
}

pub struct AnyOfResult {
    pub field_type: String,
    pub enum_type: Option<String>,
}

pub fn any_of_to_type(any_of: &Vec<ObjectOrReference<ObjectSchema>>, spec: &Spec) -> AnyOfResult {
    let mut result: AnyOfResult = AnyOfResult {
        field_type: String::new(),
        enum_type: None,
    };

    if any_of.len().eq(&2)
        && any_of.iter().any(|x| {
            x.resolve(&spec).is_ok_and(|obj| {
                obj.schema_type
                    .eq(&Some(SchemaTypeSet::Single(SchemaType::Null)))
            })
        })
    {
        let field_type = any_of
            .iter()
            .filter(|x| {
                x.resolve(&spec).is_ok_and(|y| {
                    !y.schema_type
                        .eq(&Some(SchemaTypeSet::Single(SchemaType::Null)))
                })
            })
            .last()
            .unwrap()
            .resolve(&spec)
            .unwrap();
        let type_str = match field_type.schema_type {
            Some(typeset) => match typeset {
                SchemaTypeSet::Single(typeset) => match typeset {
                    SchemaType::Boolean => format!("Option<bool>"),
                    SchemaType::Integer => format!("Option<f64>"),
                    SchemaType::Number => format!("Option<i64>"),
                    SchemaType::String => format!("Option<String>"),
                    SchemaType::Array => format!("Opttion<Vec<TODO>>"),
                    SchemaType::Object => format!("{}", field_type.title.unwrap()),
                    SchemaType::Null => panic!("Null should be filtered out"),
                },
                SchemaTypeSet::Multiple(_items) => todo!(),
            },
            None => todo!(),
        };

        result.field_type.push_str(&type_str);
        return result;
    }

    result.field_type.push_str("TODO_ENUM_TYPES");

    result
}
