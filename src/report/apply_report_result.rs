use std::collections::HashMap;

use crate::request::WialonRequest;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Default, Debug)]
pub struct ApplyReportResult {
    pub params: ApplyReportResultParams,
}

#[derive(Serialize, Default, Debug)]
pub struct ApplyReportResultParams {}

#[derive(Deserialize, Debug)]
pub struct ApplyReportResultResponse {
    #[serde(flatten)]
    _extra: HashMap<String, Value>
}

impl WialonRequest for ApplyReportResult {
    type Params = ApplyReportResultParams;

    type Response = ApplyReportResultResponse;

    fn service_name(&self) -> &str {
        "report/apply_report_result"
    }

    fn params(&self) -> &Self::Params {
        &self.params
    }
}
