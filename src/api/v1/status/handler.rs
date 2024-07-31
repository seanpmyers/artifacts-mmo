use log::{error, info};

use crate::constants::api::ARTIFACTS_MMO_HOST;

use super::{StatusResponse, STATUS};

pub fn call_get_status(http_client: &mut ureq::Agent) -> Option<StatusResponse> {
    let status_url: String = format!("{}{}{}", "https://", ARTIFACTS_MMO_HOST, STATUS.path);
    let get_status_result: Result<ureq::Response, ureq::Error> = http_client
        .get(&status_url)
        .set("accept", "application/json")
        .call();
    match get_status_result {
        Ok(response) => {
            let status: StatusResponse = response.into_json::<StatusResponse>().unwrap();
            info!("{:?}", status);
            info!("{}", serde_json::to_string_pretty(&status).unwrap());
            return Some(status);
        }
        Err(err) => error!("{:?}", err),
    }
    None
}
