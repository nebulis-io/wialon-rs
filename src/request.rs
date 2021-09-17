use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct WialonFormData {
    #[serde(skip_serializing_if = "Option::is_none")]
    sid: Option<String>,
    svc: String,
    params: String,
}

#[async_trait]
pub trait WialonRequest: Sync {
    type Params: Serialize + Send + Sync;
    type Response: for<'de> Deserialize<'de>;

    fn service_name(&self) -> &str;

    fn params(&self) -> &Self::Params;

    fn method(&self) -> reqwest::Method {
        reqwest::Method::POST
    }

    async fn send(
        &self,
        client: &reqwest::Client,
        base_url: &str,
        sid: Option<&String>,
        token: &str,
    ) -> Result<Self::Response, reqwest::Error> {
        let params = self.params();

        let wialon_form_data = WialonFormData {
            sid: sid.map(String::to_string),
            svc: self.service_name().to_string(),
            params: serde_json::to_string(params).expect("Can't serialize params"),
        };

        let response = if self.method() == reqwest::Method::POST {
            client
                .request(self.method(), format!("{}/wialon/proxy", base_url))
                .header("Content-Type", "application/x-www-form-urlencoded")
                .bearer_auth(&token)
                .form(&wialon_form_data)
                .send()
                .await?
        } else {
            let form_data = serde_urlencoded::to_string(&wialon_form_data).unwrap();

            client
                .request(
                    self.method(),
                    format!("{}/wialon/proxy?{}", base_url, form_data),
                )
                .bearer_auth(&token)
                .send()
                .await?
        };
        Ok(response.json::<Self::Response>().await?)
    }
}
