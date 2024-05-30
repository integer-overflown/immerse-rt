use std::env;

#[derive(Clone)]
pub struct AppState {
    api_key: String,
    api_secret: String,
}

impl AppState {
    pub fn new_from_env() -> Self {
        let api_key = env::var("LIVEKIT_API_KEY").expect("LIVEKIT_API_KEY is not set");
        let api_secret = env::var("LIVEKIT_API_SECRET").expect("LIVEKIT_API_SECRET is not set");

        Self {
            api_key,
            api_secret,
        }
    }

    pub fn api_key(&self) -> &str {
        &self.api_key
    }

    pub fn api_secret(&self) -> &str {
        &self.api_secret
    }
}
