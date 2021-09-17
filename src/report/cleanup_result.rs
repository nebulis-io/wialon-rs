use crate::request::WialonRequest;
use serde::{Deserialize, Serialize};

pub struct CleanupResult {
    pub params: CleanupResultParams,
}

#[derive(Serialize, Default)]
pub struct CleanupResultParams {}

#[derive(Deserialize, Debug)]
pub struct CleanupResultResponse {
    pub error: i32,
}

impl WialonRequest for CleanupResult {
    type Params = CleanupResultParams;

    type Response = CleanupResultResponse;

    fn service_name(&self) -> &str {
        "report/cleanup_result"
    }

    fn params(&self) -> &Self::Params {
        &self.params
    }
}
