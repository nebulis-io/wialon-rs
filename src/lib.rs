use self::request::WialonRequest;

pub mod core;
pub mod report;
pub mod request;
pub mod token;

pub struct WialonApi {
    pub base_url: String,
    pub client: reqwest::Client,
    pub sid: Option<String>,
}

impl WialonApi {
    pub async fn make_request<T: WialonRequest>(
        &self,
        endpoint: T,
    ) -> Result<T::Response, reqwest::Error> {
        endpoint
            .send(&self.client, &self.base_url, self.sid.as_ref())
            .await
    }

    pub fn set_sid(&mut self, sid: String) {
        self.sid = Some(sid)
    }
}
