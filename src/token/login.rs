use crate::request::WialonRequest;
use serde::{Deserialize, Serialize};

pub struct TokenLogin {
    pub params: TokenLoginParams,
}

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TokenLoginParams {
    pub token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operate_as: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fl: Option<u32>,
}

#[derive(Deserialize, Debug)]
pub struct UserInfo {
    pub bact: u32,
}

#[derive(Deserialize, Debug)]
pub struct TokenLoginResponse {
    pub eid: String,
    pub user: UserInfo,
}

impl WialonRequest for TokenLogin {
    type Params = TokenLoginParams;

    type Response = TokenLoginResponse;

    fn service_name(&self) -> &str {
        "token/login"
    }

    fn params(&self) -> &Self::Params {
        &self.params
    }
}
