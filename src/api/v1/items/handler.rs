use log::info;
use url::Url;

use crate::{
    constants::api::ARTIFACTS_MMO_HOST,
    http::{get_headers, make_api_call},
};

use super::{GetItemRequest, GetItemResponse, GET_ITEM};

pub fn call_get_item(
    http_client: &mut ureq::Agent,
    api_token: &String,
    body: GetItemRequest,
) -> Option<GetItemResponse> {
    let url: Url = Url::parse(&format!(
        "{}{}{}",
        "https://",
        ARTIFACTS_MMO_HOST,
        GetItemRequest::get_path(body.code)
    ))
    .unwrap();

    let api_request_result: Option<ureq::Response> = make_api_call::<GetItemRequest>(
        http_client,
        get_headers(api_token),
        GET_ITEM.http_request_method,
        url,
        None,
    );
    if let Some(response) = api_request_result {
        info!("{:?}", response);

        let response_data: GetItemResponse = response.into_json::<GetItemResponse>().unwrap();
        info!("{}", serde_json::to_string_pretty(&response_data).unwrap());
        return Some(response_data);
    }
    None
}
