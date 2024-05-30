use std::net::Ipv4Addr;

use axum::{debug_handler, extract::State, http, Json};
use livekit_api::access_token;
use tracing::{error, info};

use app::AppState;
use app_protocol::token;

mod app;

#[tokio::main]
async fn main() {
    use axum::routing::get;
    tracing_subscriber::fmt::init();

    let app = axum::Router::new()
        .route("/request-token", get(create_token))
        .with_state(AppState::new_from_env());

    let addr = (Ipv4Addr::UNSPECIFIED, 3000);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    info!("Listening on {addr:?}");
    axum::serve(listener, app).await.unwrap();
}

#[debug_handler]
async fn create_token(
    State(state): State<AppState>,
    Json(req): Json<token::TokenRequest>,
) -> Result<Json<token::TokenResponse>, http::StatusCode> {
    let token = access_token::AccessToken::with_api_key(state.api_key(), state.api_secret())
        .with_identity(&req.identity)
        .with_name(req.name.as_ref().unwrap_or(&req.identity))
        .with_grants(access_token::VideoGrants {
            room_join: true,
            room: req.room_id,
            can_publish: req.role.can_publish(),
            can_subscribe: req.role.can_subscribe(),
            ..Default::default()
        })
        .to_jwt()
        .map_err(|e| {
            error!("Failed to create access token: {e}");
            http::StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(token::TokenResponse { token }))
}
