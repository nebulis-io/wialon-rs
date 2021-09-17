use crate::request::WialonRequest;
use serde::{Deserialize, Serialize};

pub struct SearchItems {
    pub params: SearchItemsParams,
}

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SearchItemsParamsSpec {
    pub items_type: String,
    pub prop_name: String,
    pub prop_value_mask: String,
    pub sort_type: String,
}

#[derive(Serialize, Default)]
pub struct SearchItemsParams {
    pub spec: SearchItemsParamsSpec,
    pub force: u32,
    pub flags: u32,
    pub from: u64,
    pub to: u64,
}

#[derive(Deserialize, Debug)]
pub struct SearchItemsResponseItem {
    pub nm: String,
    pub id: u64,
}

#[derive(Deserialize, Debug)]
pub struct SearchItemsResponse {
    pub items: Vec<SearchItemsResponseItem>,
}

impl WialonRequest for SearchItems {
    type Params = SearchItemsParams;

    type Response = SearchItemsResponse;

    fn service_name(&self) -> &str {
        "core/search_items"
    }

    fn params(&self) -> &Self::Params {
        &self.params
    }
}
