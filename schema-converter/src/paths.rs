use std::{collections::BTreeMap, str::FromStr};

use oas3::spec::{ObjectOrReference, Operation, PathItem};

use crate::log_error;

pub struct Request {}

impl Request {
    pub fn path() -> &'static str {
        ""
    }

    pub fn tags() -> [&'static str; 1] {
        [""]
    }

    pub fn summary() -> &'static str {
        ""
    }
}

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

pub struct HttpRequest {
    pub description: String,
    pub http_method: HttpMethod,
    pub response: Vec<HttpResponse>,
    pub summary: String,
    pub operation_id: String,
    pub body: Option<HttpRequestBody>,
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
        }
    }
}

pub fn parse_paths(spec: &oas3::Spec) -> anyhow::Result<Vec<Path>> {
    let mut parsed_paths: Vec<Path> = Vec::new();
    let Some(spec_paths) = &spec.paths else {
        return Ok(parsed_paths);
    };

    for (path, item) in spec_paths.iter() {
        parsed_paths.push(Path {
            path: path.clone(),
            requests: parse_path_item(item, spec),
        });
    }

    Ok(parsed_paths)
}

pub fn parse_path_item(item: &PathItem, spec: &oas3::Spec) -> Vec<HttpRequest> {
    let mut requests: Vec<HttpRequest> = Vec::new();

    item.get.as_ref().map(|get: &Operation| {
        match parse_operation(item, get, spec, HttpMethod::GET) {
            Ok(request) => requests.push(request),
            Err(error) => log_error(error),
        }
    });

    item.post.as_ref().map(|get: &Operation| {
        match parse_operation(item, get, spec, HttpMethod::POST) {
            Ok(request) => requests.push(request),
            Err(error) => log_error(error),
        }
    });

    item.put.as_ref().map(|get: &Operation| {
        match parse_operation(item, get, spec, HttpMethod::PUT) {
            Ok(request) => requests.push(request),
            Err(error) => log_error(error),
        }
    });

    item.delete.as_ref().map(|get: &Operation| {
        match parse_operation(item, get, spec, HttpMethod::DELETE) {
            Ok(request) => requests.push(request),
            Err(error) => log_error(error),
        }
    });

    item.patch.as_ref().map(|get: &Operation| {
        match parse_operation(item, get, spec, HttpMethod::PATCH) {
            Ok(request) => requests.push(request),
            Err(error) => log_error(error),
        }
    });

    item.head.as_ref().map(|get: &Operation| {
        match parse_operation(item, get, spec, HttpMethod::HEAD) {
            Ok(request) => requests.push(request),
            Err(error) => log_error(error),
        }
    });

    item.options.as_ref().map(|get: &Operation| {
        match parse_operation(item, get, spec, HttpMethod::OPTIONS) {
            Ok(request) => requests.push(request),
            Err(error) => log_error(error),
        }
    });

    item.trace.as_ref().map(|get: &Operation| {
        match parse_operation(item, get, spec, HttpMethod::TRACE) {
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
) -> anyhow::Result<HttpRequest> {
    Ok(HttpRequest {
        description: item
            .description
            .clone()
            .unwrap_or(operation.description.clone().unwrap_or_default()),
        http_method,
        response: parse_respones(item, operation, spec),
        summary: item
            .summary
            .clone()
            .unwrap_or(operation.summary.clone().unwrap_or_default()),
        operation_id: operation.operation_id.clone().unwrap_or_default(),
        body: operation
            .request_body
            .as_ref()
            .map(|body| parse_request_body(&body, item, operation, spec))
            .transpose()?,
        tags: operation.tags.clone(),
    })
}

pub fn parse_respones(
    item: &PathItem,
    operation: &Operation,
    spec: &oas3::Spec,
) -> Vec<HttpResponse> {
    let mut result: Vec<HttpResponse> = Vec::new();

    result
}

pub fn parse_request_body(
    body: &ObjectOrReference<oas3::spec::RequestBody>,
    _item: &PathItem,
    operation: &Operation,
    spec: &oas3::Spec,
) -> anyhow::Result<HttpRequestBody> {
    let body = body.resolve(spec)?;

    Ok(HttpRequestBody {
        required: body.required.unwrap_or_default(),
        description: body.description,
        content: parse_content_types(&body.content, operation, spec)?,
    })
}

pub fn parse_content_types(
    content: &BTreeMap<String, oas3::spec::MediaType>,
    operation: &Operation,
    spec: &oas3::Spec,
) -> anyhow::Result<HttpContent> {
    Ok(HttpContent {
        content_type: todo!(),
        schema_name: todo!(),
    })
}
