use async_trait::async_trait;
use reqwest::Response;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
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

    async fn get_output(&self, response: Response) -> Result<Self::Response, reqwest::Error> {
        response.json::<Self::Response>().await
    }

    async fn send(
        &self,
        client: &reqwest::Client,
        base_url: &str,
        sid: Option<&String>,
    ) -> Result<Self::Response, reqwest::Error> {
        let params = self.params();

        let wialon_form_data = WialonFormData {
            sid: sid.map(String::to_string),
            svc: self.service_name().to_string(),
            params: serde_json::to_string(params).expect("Can't serialize params"),
        };

        let response = if self.method() == reqwest::Method::POST {
            client
                .request(self.method(), base_url.to_string())
                .header("Content-Type", "application/x-www-form-urlencoded")
                .form(&wialon_form_data)
                .send()
                .await?
        } else {
            let form_data = serde_urlencoded::to_string(&wialon_form_data).unwrap();

            client
                .request(self.method(), format!("{}?{}", base_url, form_data))
                .send()
                .await?
        };
        self.get_output(response).await
    }
}
