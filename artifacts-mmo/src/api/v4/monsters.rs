use crate::api::{Endpoint, NoBody, PageInput, PageOutput};

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct Monster {
    pub name: String,
    pub code: String,
    pub level: u32,
    pub hp: i32,
    pub attack_fire: i32,
    pub attack_earth: i32,
    pub attack_water: i32,
    pub attack_air: i32,
    pub res_air: i32,
    pub res_earth: i32,
    pub res_water: i32,
    pub res_fire: i32,
    pub critical_strike: i32,
    pub effects: Vec<SimpleEffect>,
    pub min_gold: i32,
    pub max_gold: i32,
    pub drops: Vec<DropRate>,
}

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct SimpleEffect {
    pub code: String,
    pub value: i32,
}

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct DropRate {
    pub code: String,
    pub max_quantity: u32,
    pub min_quantity: u32,
    pub rate: u32,
}

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetAllMonstersRequest {
    pub drop: Option<String>,
    pub max_level: Option<u32>,
    pub min_level: Option<u32>,
    #[serde(flatten)]
    pub page_input: PageInput,
}

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetAllMonstersResponse {
    pub data: Vec<Monster>,
    #[serde(flatten)]
    pub page_output: PageOutput,
}

#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetMonsterRequest {
    pub code: String,
}
#[derive(Default, Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct GetMonsterResponse {
    pub data: Monster,
}

impl Endpoint for GetAllMonstersRequest {
    type Response = GetAllMonstersResponse;
    type RequestBody = NoBody;

    fn http_request_method() -> http::Method {
        http::Method::GET
    }

    fn path(&self) -> String {
        format!("/monsters")
    }

    fn query_parameters(&self) -> Vec<(String, String)> {
        let mut parameters = self.page_input.to_tuples();

        if let Some(drop) = &self.drop {
            parameters.push((String::from("drop"), drop.clone()));
        }
        if let Some(max_level) = &self.max_level {
            parameters.push((String::from("max_level"), max_level.to_string()));
        }
        if let Some(min_level) = &self.min_level {
            parameters.push((String::from("min_level"), min_level.to_string()));
        }

        parameters
    }
}

impl Endpoint for GetMonsterRequest {
    type Response = GetMonsterResponse;
    type RequestBody = NoBody;

    fn http_request_method() -> http::Method {
        http::Method::GET
    }

    fn path(&self) -> String {
        format!("/monsters/{}", self.code)
    }
}
