use crate::request::WialonRequest;
use async_trait::async_trait;
use reqwest::Response;
use serde::{Deserialize, Serialize};
use serde_repr::*;

pub struct ExportResult {
    pub params: ExportResultParams,
}

#[derive(Serialize_repr)]
#[repr(u8)]
pub enum ReportFormat {
    HTML = 1,
    PDF = 2,
    XLS = 4,
    XLSX = 8,
    XML = 16,
    CSV = 32,
}

impl Default for ReportFormat {
    fn default() -> Self {
        Self::HTML
    }
}

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ExportResultParams {
    pub format: ReportFormat,
    pub compress: Option<String>,
    pub page_orientation: Option<String>,
    pub page_size: Option<String>,
    pub page_width: Option<String>,
    pub coding: Option<String>,
    pub delimiter: Option<String>,
    pub headings: Option<String>,
    pub attach_map: Option<String>,
    pub extend_bounds: Option<String>,
    pub hide_map_basis: Option<String>,
    pub output_file_name: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExportResultResponse {
    pub location: String,
}

#[async_trait]
impl WialonRequest for ExportResult {
    type Params = ExportResultParams;

    type Response = Vec<u8>;

    fn service_name(&self) -> &str {
        "report/export_result"
    }

    fn method(&self) -> reqwest::Method {
        reqwest::Method::GET
    }

    fn params(&self) -> &Self::Params {
        &self.params
    }

    async fn get_output(&self, response: Response) -> Result<Self::Response, reqwest::Error> {
        Ok(response.bytes().await?.to_vec())
    }
}
