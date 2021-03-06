use std::collections::HashMap;

use crate::request::WialonRequest;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub struct ExecuteReport {
    pub params: ExecuteReportParams,
}

#[derive(Serialize, Default,Debug)]
pub struct ExecuteReportInterval {
    pub from: u32,
    pub to: u32,
    pub flags: u32,
}

#[derive(Serialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteReportParams {
    pub report_resource_id: i64,
    pub report_template_id: i64,
    pub report_object_id: i64,
    pub report_object_sec_id: i64,
    pub report_object_id_list: Vec<u32>,
    pub interval: ExecuteReportInterval,
    pub remote_exec: Option<u32>,
    pub report_template: Option<Value>,
}

#[derive(Deserialize, Debug)]
pub struct ExecuteReportReportResult {
    #[serde(flatten)]
    _extra: HashMap<String, Value>
}

#[derive(Deserialize, Debug)]
pub struct ExecuteReportReportLayer {
    pub name: String,
    pub bounds: [f64; 4],
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteReportResult {
    pub report_result: ExecuteReportReportResult,
    pub report_layer: ExecuteReportReportLayer,
    pub layer_count: u32,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
#[serde(rename_all = "camelCase")]
pub enum ExecuteReportResponse {
    FullResult(ExecuteReportResult),
    RemotelyExecuted {
        #[serde(flatten)]
        _extra: HashMap<String, Value>
    },
}

impl WialonRequest for ExecuteReport {
    type Params = ExecuteReportParams;

    type Response = ExecuteReportResponse;

    fn service_name(&self) -> &str {
        "report/exec_report"
    }

    fn params(&self) -> &Self::Params {
        &self.params
    }
}
