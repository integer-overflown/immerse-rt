use std::time::Duration;

use url::Url;

use app_protocol::token::{TokenRequest, TokenResponse};

pub struct Client {
    client: reqwest::Client,
    server_url: Url,
}

impl Client {
    pub fn new(server_url: &str) -> anyhow::Result<Self> {
        let client = reqwest::ClientBuilder::new()
            .connect_timeout(Duration::from_secs(10))
            .build()?;
        let server_url = Url::parse(server_url)?;

        Ok(Self { client, server_url })
    }

    pub async fn request_token(&self, token_request: TokenRequest) -> reqwest::Result<String> {
        let request = self
            .client
            .get(self.endpoint("request-token"))
            .json(&token_request)
            .build()?;

        self.client
            .execute(request)
            .await?
            .json::<TokenResponse>()
            .await
            .map(|response| response.token)
    }

    fn endpoint(&self, path: &str) -> Url {
        let mut url = self.server_url.clone();
        url.set_path(path);
        url
    }
}
