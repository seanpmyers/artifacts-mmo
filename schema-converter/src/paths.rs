use std::{
    collections::{BTreeMap, HashMap},
    str::FromStr,
};

use oas3::spec::{ObjectOrReference, ObjectSchema, Operation, Parameter, PathItem};

use crate::{constants::STRUCT_ANNOTATIONS, log_error};

pub enum HttpMethod {
    CONNECT,
    DELETE,
    GET,
    HEAD,
    OPTIONS,
    PATCH,
    POST,
    PUT,
    TRACE,
}

impl std::fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                HttpMethod::CONNECT => "CONNECT",
                HttpMethod::DELETE => "DELETE",
                HttpMethod::GET => "GET",
                HttpMethod::HEAD => "HEAD",
                HttpMethod::OPTIONS => "OPTIONS",
                HttpMethod::PATCH => "PATCH",
                HttpMethod::POST => "POST",
                HttpMethod::PUT => "PUT",
                HttpMethod::TRACE => "TRACE",
            }
        )
    }
}

impl FromStr for HttpMethod {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "get" => Ok(HttpMethod::GET),
            "head" => Ok(HttpMethod::HEAD),
            "post" => Ok(HttpMethod::POST),
            "put" => Ok(HttpMethod::PUT),
            "delete" => Ok(HttpMethod::DELETE),
            "connect" => Ok(HttpMethod::CONNECT),
            "options" => Ok(HttpMethod::OPTIONS),
            "trace" => Ok(HttpMethod::TRACE),
            "patch" => Ok(HttpMethod::PATCH),
            _ => Err(anyhow::anyhow!("Invalid Http Request: {}", s)),
        }
    }
}

#[derive(Default)]
pub struct Path {
    pub path: String,
    pub requests: Vec<HttpRequest>,
}

#[derive(Default)]
pub struct HttpResponse {
    pub status_code: u16,
    pub description: String,
    pub body: HttpResponseBody,
}

#[derive(Default)]
pub struct HttpRequestBody {
    pub content: HttpContent,
    pub description: Option<String>,
    pub required: bool,
}

#[derive(Default)]
pub struct HttpResponseBody {
    pub content: HttpContent,
}

#[derive(Default)]
pub struct HttpContent {
    pub content_type: String,
    pub schema_name: String,
}

#[derive(Default)]
pub struct HttpRequestParameter {
    pub name: String,
    pub r#in: String,
    pub required: bool,
    pub description: String,
    pub schema: HttpParameterSchema,
}

#[derive(Default)]
pub struct HttpParameterSchema {
    pub r#type: String,
    pub pattern: Option<String>,
    pub title: String,
    pub description: String,
}

pub struct HttpRequest {
    pub body: Option<HttpRequestBody>,
    pub description: String,
    pub http_method: HttpMethod,
    pub operation_id: String,
    pub parameters: Vec<HttpRequestParameter>,
    pub response: Vec<HttpResponse>,
    pub summary: String,
    pub tags: Vec<String>,
}

impl HttpRequest {
    pub fn new(http_method: HttpMethod) -> Self {
        Self {
            description: Default::default(),
            http_method,
            response: Default::default(),
            summary: Default::default(),
            operation_id: Default::default(),
            body: Default::default(),
            tags: Default::default(),
            parameters: Default::default(),
        }
    }

    pub fn operation_id_to_struct_name(&self) -> String {
        let mut result: String = String::new();

        for x in self.operation_id.split("_") {
            if x.is_empty() {
                continue;
            }
            let mut chars = x.chars();
            let first = chars.next().unwrap().to_ascii_uppercase();
            result.push(first);
            result.push_str(chars.as_str());
        }

        result
    }

    pub fn operation_id_to_output_function(&self) -> String {
        format!(
            "\tpub fn operation_id() -> &'static str {{\n\t\t\"{}\"\n\t}}\n\n",
            self.operation_id
        )
    }

    pub fn description_to_output_function(&self) -> String {
        format!(
            "\tpub fn description() -> &'static str {{\n\t\t\"{}\"\n\t}}\n\n",
            self.description
        )
    }

    pub fn summary_to_output_function(&self) -> String {
        format!(
            "\tpub fn summary() -> &'static str {{\n\t\t\"{}\"\n\t}}\n\n",
            self.summary
        )
    }

    pub fn tags_to_output_function(&self) -> String {
        let mut result: String = format!(
            "\tpub fn tags() -> [&'static str; {}] {{\n\t\t[\n",
            self.tags.len()
        );

        for tag in self.tags.iter() {
            result.push_str(&format!("\t\t\t\"{}\",\n", tag));
        }

        result.push_str(&format!("\t\t]\n\t}}\n\n"));

        result
    }

    pub fn http_method_to_output_function(&self) -> String {
        format!(
            "\tpub fn http_method() -> http::Method {{\n\t\thttp::Method::{}\n\t}}\n\n",
            self.http_method.to_string()
        )
    }

    pub fn path_to_output_function(path: &str) -> String {
        format!(
            "\tpub fn path() -> &'static str {{\n\t\t\"{}\"\n\t}}\n\n",
            path
        )
    }

    pub fn to_output_struct(&self, path: &str) -> String {
        let struct_name = self.operation_id_to_struct_name();
        let mut result: String = format!(
            "\n{}\npub struct {} {{}}\n\n",
            STRUCT_ANNOTATIONS, &struct_name
        );
        result.push_str(&format!("impl {} {{\n", &struct_name));

        result.push_str(&Self::path_to_output_function(path));
        result.push_str(&self.description_to_output_function());
        result.push_str(&self.summary_to_output_function());
        result.push_str(&self.operation_id_to_output_function());
        result.push_str(&self.tags_to_output_function());
        result.push_str(&self.http_method_to_output_function());

        result.push_str("\n}\n");
        let response_type = &String::from("String");
        let request_body_type = match &self.body {
            Some(body) => &body.content.schema_name,
            None => &String::from("crate::api::NoBody"),
        };

        result.push_str(&self.implement_endpoint(&struct_name, response_type, request_body_type));

        result
    }

    pub fn implement_endpoint(
        &self,
        struct_name: &str,
        response_type: &str,
        request_body_type: &str,
    ) -> String {
        let mut result: String = format!("\nimpl Endpoint for {} {{\n", struct_name);

        result.push_str(&format!("\t\ttype Response = {};\n", response_type));
        result.push_str(&format!(
            "\t\ttype RequestBody = {};\n\n",
            request_body_type
        ));

        result.push_str(&format!(
            "\t\tfn http_request_method() -> http::Method {{\n"
        ));

        result.push_str(&format!("\t\t\t{}::http_method()\n", &struct_name));

        result.push_str("\t\t}\n\n");

        // fn http_request_method() -> http::Method {
        //     http::Method::POST
        // }
        result.push_str("\n}\n\n");

        result
    }
}

pub fn parse_paths(
    spec: &oas3::Spec,
    reference_map: &HashMap<String, String>,
) -> anyhow::Result<Vec<Path>> {
    let mut parsed_paths: Vec<Path> = Vec::new();
    let Some(spec_paths) = &spec.paths else {
        return Ok(parsed_paths);
    };

    for (path, item) in spec_paths.iter() {
        parsed_paths.push(Path {
            path: path.clone(),
            requests: parse_path_item(item, spec, reference_map),
        });
    }

    Ok(parsed_paths)
}

pub fn parse_path_item(
    item: &PathItem,
    spec: &oas3::Spec,
    reference_map: &HashMap<String, String>,
) -> Vec<HttpRequest> {
    let mut requests: Vec<HttpRequest> = Vec::new();

    item.get.as_ref().map(|get: &Operation| {
        match parse_operation(item, get, spec, HttpMethod::GET, reference_map) {
            Ok(request) => requests.push(request),
            Err(error) => log_error(error),
        }
    });

    item.post.as_ref().map(|get: &Operation| {
        match parse_operation(item, get, spec, HttpMethod::POST, reference_map) {
            Ok(request) => requests.push(request),
            Err(error) => log_error(error),
        }
    });

    item.put.as_ref().map(|get: &Operation| {
        match parse_operation(item, get, spec, HttpMethod::PUT, reference_map) {
            Ok(request) => requests.push(request),
            Err(error) => log_error(error),
        }
    });

    item.delete.as_ref().map(|get: &Operation| {
        match parse_operation(item, get, spec, HttpMethod::DELETE, reference_map) {
            Ok(request) => requests.push(request),
            Err(error) => log_error(error),
        }
    });

    item.patch.as_ref().map(|get: &Operation| {
        match parse_operation(item, get, spec, HttpMethod::PATCH, reference_map) {
            Ok(request) => requests.push(request),
            Err(error) => log_error(error),
        }
    });

    item.head.as_ref().map(|get: &Operation| {
        match parse_operation(item, get, spec, HttpMethod::HEAD, reference_map) {
            Ok(request) => requests.push(request),
            Err(error) => log_error(error),
        }
    });

    item.options.as_ref().map(|get: &Operation| {
        match parse_operation(item, get, spec, HttpMethod::OPTIONS, reference_map) {
            Ok(request) => requests.push(request),
            Err(error) => log_error(error),
        }
    });

    item.trace.as_ref().map(|get: &Operation| {
        match parse_operation(item, get, spec, HttpMethod::TRACE, reference_map) {
            Ok(request) => requests.push(request),
            Err(error) => log_error(error),
        }
    });

    requests
}

pub fn parse_operation(
    item: &PathItem,
    operation: &Operation,
    spec: &oas3::Spec,
    http_method: HttpMethod,
    reference_map: &HashMap<String, String>,
) -> anyhow::Result<HttpRequest> {
    Ok(HttpRequest {
        description: item
            .description
            .clone()
            .unwrap_or(operation.description.clone().unwrap_or_default()),
        http_method,
        response: parse_respones(item, operation, spec, reference_map),
        summary: item
            .summary
            .clone()
            .unwrap_or(operation.summary.clone().unwrap_or_default()),
        operation_id: operation.operation_id.clone().unwrap_or_default(),
        body: operation
            .request_body
            .as_ref()
            .map(|body| parse_request_body(&body, item, operation, spec, reference_map))
            .transpose()?,
        parameters: parse_request_paramters(&operation.parameters, spec, reference_map)?,
        tags: operation.tags.clone(),
    })
}

pub fn parse_request_paramters(
    parameters: &Vec<ObjectOrReference<Parameter>>,
    spec: &oas3::Spec,
    reference_map: &HashMap<String, String>,
) -> anyhow::Result<Vec<HttpRequestParameter>> {
    let mut result: Vec<HttpRequestParameter> = Vec::new();

    for parameter in parameters.iter() {
        let parameter = parameter.resolve(spec)?;
        match parameter.location {
            oas3::spec::ParameterIn::Path
            | oas3::spec::ParameterIn::Query
            | oas3::spec::ParameterIn::Header
            | oas3::spec::ParameterIn::Cookie => {}
        };

        if let Some(style) = &parameter.style {
            match style {
                oas3::spec::ParameterStyle::Matrix => todo!(),
                oas3::spec::ParameterStyle::Label => todo!(),
                oas3::spec::ParameterStyle::Form => todo!(),
                oas3::spec::ParameterStyle::Simple => todo!(),
                oas3::spec::ParameterStyle::SpaceDelimited => todo!(),
                oas3::spec::ParameterStyle::PipeDelimited => todo!(),
                oas3::spec::ParameterStyle::DeepObject => todo!(),
            }
        };

        result.push(HttpRequestParameter {
            name: parameter.name,
            r#in: "".to_string(),
            required: parameter.required.unwrap_or_default(),
            description: parameter.description.unwrap_or_default(),
            schema: parse_parameter_schema(&parameter.schema, spec, reference_map)?,
        });
    }

    Ok(result)
}

pub fn parse_parameter_schema(
    schema: &Option<ObjectOrReference<ObjectSchema>>,
    spec: &oas3::Spec,
    reference_map: &HashMap<String, String>,
) -> anyhow::Result<HttpParameterSchema> {
    match schema {
        Some(schema) => {
            let schema = schema.resolve(spec)?;
            Ok(HttpParameterSchema {
                r#type: "TODO_TYPE".to_string(),
                pattern: None,
                title: schema.title.unwrap_or_default(),
                description: schema.description.unwrap_or_default(),
            })
        }
        None => Ok(HttpParameterSchema {
            r#type: "TODO_TYPE".to_string(),
            pattern: None,
            title: "".to_string(),
            description: "".to_string(),
        }),
    }
}

pub fn parse_respones(
    item: &PathItem,
    operation: &Operation,
    spec: &oas3::Spec,
    reference_map: &HashMap<String, String>,
) -> Vec<HttpResponse> {
    let mut result: Vec<HttpResponse> = Vec::new();

    result
}

pub fn parse_request_body(
    body: &ObjectOrReference<oas3::spec::RequestBody>,
    _item: &PathItem,
    operation: &Operation,
    spec: &oas3::Spec,
    reference_map: &HashMap<String, String>,
) -> anyhow::Result<HttpRequestBody> {
    let body = body.resolve(spec)?;

    Ok(HttpRequestBody {
        required: body.required.unwrap_or_default(),
        description: body.description,
        content: parse_content_types(&body.content, operation, spec, reference_map)?,
    })
}

pub fn parse_content_types(
    content: &BTreeMap<String, oas3::spec::MediaType>,
    operation: &Operation,
    spec: &oas3::Spec,
    reference_map: &HashMap<String, String>,
) -> anyhow::Result<HttpContent> {
    for item in content
        .iter()
        .filter(|item| item.0.to_lowercase().contains("application/json"))
    {
        match &item.1.schema {
            Some(schema) => {
                return Ok(HttpContent {
                    content_type: item.0.clone(),
                    schema_name: object_schema_to_type(schema, spec, reference_map)?,
                });
            }
            None => {
                return Ok(HttpContent {
                    content_type: item.0.clone(),
                    schema_name: "TODO_no_schema_serde_json::Value".to_string(),
                });
            }
        }
    }
    panic!("No json content");
}

pub fn object_schema_to_type(
    object_schema: &ObjectOrReference<ObjectSchema>,
    spec: &oas3::Spec,
    reference_map: &HashMap<String, String>,
) -> anyhow::Result<String> {
    let schema = object_schema.resolve(&spec)?;
    let Some(title) = schema.title else {
        return Ok("serde_json::Value".to_string());
    };

    let Some(camel_case) = reference_map.get(&title) else {
        return Ok("serde_json::Value".to_string());
    };

    Ok(format!("super::{}::{}", camel_case, title))
}
