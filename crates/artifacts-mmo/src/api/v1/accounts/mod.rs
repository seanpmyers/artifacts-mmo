// use crate::api::Endpoint;

// #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
// pub struct CreateAccount {
//     pub username: String,
//     pub password: String,
//     pub email: Option<String>,
// }

// #[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
// pub struct CreateAccountResponse {}

// impl Endpoint for CreateAccount {
//     fn http_request_method() -> http::Method {
//         http::Method::POST
//     }

//     fn path() -> &'static str {
//         "/accounts/create"
//     }

//     fn query() -> Option<String> {
//         None
//     }

//     fn request_body(&self) -> Option<CreateAccount> {
//         Some(self.clone())
//     }

//     type Response = CreateAccountResponse;
// }
