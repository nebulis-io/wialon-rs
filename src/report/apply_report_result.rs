use crate::request::WialonRequest;
use serde::{Deserialize, Serialize};

pub struct ApplyReportResult {
    pub params: ApplyReportResultParams,
}

#[derive(Serialize, Default)]
pub struct ApplyReportResultParams {}

#[derive(Deserialize, Debug)]
pub struct ApplyReportResultResponse {}

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
