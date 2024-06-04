use std::io;

use url::Url;

use app_protocol::token::{TokenRequest, TokenResponse};

pub struct Client {
    server_url: Url,
}

#[derive(thiserror::Error, Debug)]
pub enum RequestError {
    #[error("Failed to send the request")]
    SendFailed(#[from] Box<ureq::Error>),
    #[error("failed to parse JSON in the response")]
    InvalidJson(#[from] io::Error),
}

impl Client {
    pub fn new(server_url: &str) -> Result<Client, url::ParseError> {
        let server_url = Url::parse(server_url)?;

        Ok(Self { server_url })
    }

    pub fn request_token(&self, token_request: TokenRequest) -> Result<String, RequestError> {
        let endpoint = self.endpoint("request-token");
        let response = ureq::request_url("GET", &endpoint)
            .send_json(token_request)
            .map_err(Box::new)?;
        let result: TokenResponse = response.into_json()?;

        Ok(result.token)
    }

    fn endpoint(&self, path: &str) -> Url {
        let mut url = self.server_url.clone();
        url.set_path(path);
        url
    }
}
