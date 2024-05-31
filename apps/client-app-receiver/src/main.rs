use std::env;

use gst::glib;
use tracing_subscriber::EnvFilter;

use app_protocol::token::{PeerRole, TokenRequest};
use client::Client;

mod client;
mod stream;
mod utils;

fn usage() -> ! {
    println!("usage: ./app [server_url]");
    std::process::exit(1)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut args = env::args().skip(1);

    let Some(server_url) = args.next() else {
        usage();
    };

    gst::init().expect("Cannot initialize gstreamer");
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let client = Client::new(&server_url)?;
    let token = client
        .request_token(TokenRequest {
            identity: "app-receiver".to_owned(),
            role: PeerRole::Subscriber,
            room_id: "room#1".to_owned(),
            name: None,
        })
        .await?;

    let main_loop = glib::MainLoop::new(None, false);
    let stream = stream::connect(&token);

    stream.play()?;

    main_loop.run();

    Ok(())
}
