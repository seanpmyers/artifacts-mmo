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
                                rust_to_file(
                                    rust_string.as_bytes(),
                                    &format!("output/{}.rs", schema_name),
                                )?;
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
                        rust_to_file(
                            rust_string.as_bytes(),
                            &format!("output/{}.rs", schema_name),
                        )?;
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

const RESERVED_FIELD_NAMES: [&str; 1] = ["type"];

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
    result.push_str(&format!("pub struct {} {{\n", schema_name));
    for property in schema.properties.iter() {
        let is_reference: bool = matches!(property.1, ObjectOrReference::Ref { .. });
        let mut property_name: String = property.0.clone();
        let property_schema: ObjectSchema = property.1.resolve(&spec)?;

        let property_type: &str = match property_schema.schema_type {
            Some(typeset) => match typeset {
                SchemaTypeSet::Single(single_type) => match single_type {
                    SchemaType::Boolean => "bool",
                    SchemaType::Integer => "i32",
                    SchemaType::Number => "f32",
                    SchemaType::String => match property_schema.format {
                        Some(format) => match format.as_str() {
                            "date-time" => "chrono::DateTime<chrono::Utc>",
                            _ => &format!("String__TODO__{}", format),
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
                let mut terrible_disease = String::new();
                if !property_schema.all_of.is_empty() {
                    for all_of_schema in property_schema.all_of.iter() {
                        let is_ref: bool = matches!(all_of_schema, ObjectOrReference::Ref { .. });
                        let resolved_schema: ObjectSchema = all_of_schema.resolve(&spec)?;
                        match is_ref {
                            true => terrible_disease.push_str(&format!(
                                "#[serde(flatten)]\n\t{}\n",
                                resolved_schema
                                    .title
                                    .unwrap_or("TODO_MISSING_TITLE_ALL_OF".to_string())
                            )),
                            false => terrible_disease.push_str("TODO_NOT_REF_ALL_OF\n"),
                        }
                    }
                }
                if !property_schema.any_of.is_empty() {
                    let mut is_option: bool = false;
                    let is_actually_just_optional: bool =
                        is_actually_just_optional(&property_schema.any_of, spec);
                    let possible_types: Vec<&str> =
                        Vec::with_capacity(property_schema.any_of.len());
                    for value in property_schema.any_of.iter() {
                        let value_schema = value.resolve(spec)?;
                        match &value_schema.schema_type {
                            Some(value_type_set) => match value_type_set {
                                SchemaTypeSet::Single(value_type) => match value_type {
                                    SchemaType::Boolean => println!("TODO: bool"),
                                    SchemaType::Integer => println!("TODO: integer"),
                                    SchemaType::Number => println!("TODO: number"),
                                    SchemaType::String => match value_schema.format {
                                        Some(format) => match format.as_str() {
                                            "date-time" => {
                                                "chrono::DateTime<chrono::Utc>";
                                            }
                                            _ => println!("TODO: string with no format"),
                                        },
                                        None => {
                                            "String";
                                        }
                                    },
                                    SchemaType::Array => println!("TODO: array"),
                                    SchemaType::Object => println!("TODO: object"),
                                    SchemaType::Null => is_option = true,
                                },
                                SchemaTypeSet::Multiple(items) => println!("TODO: MULTIPLE"),
                            },
                            None => println!("TOOD: None for value type set"),
                        }
                    }
                }
                terrible_disease
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
    result.push_str(&format!("pub enum {} {{\n", schema_name));
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
                *first_letter -= (b'a' - b'A');
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

pub fn is_actually_just_optional(
    any_of: &Vec<ObjectOrReference<ObjectSchema>>,
    spec: &Spec,
) -> bool {
    any_of.iter().any(|item| {
        item.resolve(spec).is_ok_and(|ok| {
            ok.schema_type
                .eq(&Some(SchemaTypeSet::Single(SchemaType::Null)))
        })
    })
}

pub fn get_type_from_optional_any_of(
    any_of: &Vec<ObjectOrReference<ObjectSchema>>,
    spec: &Spec,
) -> &'static str {
    // let any_of = any_of.iter().filter(|item| {
    //     item.resolve(spec).is_ok_and(|ok| {
    //         !ok.schema_type
    //             .eq(&Some(SchemaTypeSet::Single(SchemaType::Null)))
    //     })
    // }).map(|item| item.resolve(spec).is_ok_and(|ok| match ok.schema_type {
    //     Some(type_thing) =>  match type_thing {
    //         SchemaTypeSet::Single(single_type) => match single_type{
    //             SchemaType::Boolean => todo!(),
    //             SchemaType::Integer => todo!(),
    //             SchemaType::Number => todo!(),
    //             SchemaType::String => {
    //                 "TODO"
    //             },
    //             SchemaType::Array => todo!(),
    //             SchemaType::Object => todo!(),
    //             SchemaType::Null => todo!(),
    //         },
    //         SchemaTypeSet::Multiple(items) => todo!(),
    //     },
    //     None => panic!("uh oh i messed up"),
    // })).last().unwrap()
    ""
}
