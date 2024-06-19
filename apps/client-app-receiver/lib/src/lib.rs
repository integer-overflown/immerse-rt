#![feature(try_trait_v2)]

use tracing_subscriber::EnvFilter;

use app_protocol::token::{PeerRole, TokenRequest};

use crate::client::Client;
use crate::utils::preload_gst_element;

mod client;
mod interop;
mod stream;
mod utils;

pub(crate) fn init() -> Result<(), Box<dyn std::error::Error>> {
    gst::init()?;
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    preload_gst_element("qml6glsink")?;

    Ok(())
}

pub(crate) struct RoomOptions {
    room_id: String,
    identity: String,
    name: Option<String>,
    role: PeerRole,
}

#[derive(thiserror::Error, Debug)]
pub(crate) enum RequestError {
    #[error("invalid server url")]
    InvalidUrl(#[from] url::ParseError),
    #[error("request failed")]
    RequestFailed(#[from] client::RequestError),
}

pub(crate) fn request_token(
    server_url: &str,
    room_options: RoomOptions,
) -> Result<String, RequestError> {
    let client = Client::new(server_url)?;

    let token = client.request_token(TokenRequest {
        name: room_options.name,
        identity: room_options.identity,
        room_id: room_options.room_id,
        role: room_options.role,
    })?;

    Ok(token)
}
