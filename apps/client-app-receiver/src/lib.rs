use tracing_subscriber::EnvFilter;

mod client;
mod interop;
mod stream;
mod utils;

pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    gst::init()?;
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    Ok(())
}