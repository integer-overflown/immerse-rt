use url::Url;

use app_protocol::token::{TokenRequest, TokenResponse};

pub struct Client {
    server_url: Url,
}

impl Client {
    pub fn new(server_url: &str) -> anyhow::Result<Self> {
        let server_url = Url::parse(server_url)?;

        Ok(Self { server_url })
    }

    pub fn request_token(&self, token_request: TokenRequest) -> anyhow::Result<String> {
        let endpoint = self.endpoint("request-token");
        let response = ureq::request_url("GET", &endpoint).send_json(token_request)?;
        let result: TokenResponse = response.into_json()?;

        Ok(result.token)
    }

    fn endpoint(&self, path: &str) -> Url {
        let mut url = self.server_url.clone();
        url.set_path(path);
        url
    }
}
