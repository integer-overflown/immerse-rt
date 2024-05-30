use gst::glib;
use gst::prelude::*;
use tracing::{debug, warn};
use tracing_subscriber::EnvFilter;

macro_rules! element {
    ($name:literal) => {
        gst::ElementFactory::make($name).build().unwrap()
    };
}

fn handle_webrtc_pad(pad: &gst::Pad, pipeline: &gst::Pipeline) -> Option<glib::Value> {
    debug!("Have new pad: {}", pad.name());

    let Some(caps) = pad.current_caps() else {
        warn!("Pad has no current caps");
        return None;
    };

    let Some(format) = caps.structure(0) else {
        warn!("Received invalid empty caps");
        return None;
    };

    let mut elements = Vec::with_capacity(3);
    elements.push(element!("queue"));

    match format.name().as_str() {
        "video/x-raw" => {
            elements.push(element!("videoconvert"));
            elements.push(element!("autovideosink"));
        }
        "audio/x-raw" => {
            elements.push(element!("audioconvert"));
            elements.push(element!("autoaudiosink"));
        }
        unknown => {
            warn!("Unknown format name: '{unknown}' - skipping");
            return None;
        }
    }

    pipeline
        .add_many(&elements)
        .expect("Could not add elements");
    gst::Element::link_many(&elements).expect("Could not link elements");

    None
}

fn create_pipeline() -> gst::Pipeline {
    let pipeline = gst::Pipeline::new();
    let src = gst::ElementFactory::make("livekitwebrtcsrc")
        .build()
        .expect("Should have gst-webrtc Rust plugins installed");

    {
        let pipeline = pipeline.clone();

        src.connect("pad-added", false, move |args| {
            let pad: gst::Pad = args[1].get().unwrap();
            handle_webrtc_pad(&pad, &pipeline);
            None
        });
    }

    pipeline.add(&src).expect("Could not add elements");

    pipeline
}

fn main() -> anyhow::Result<()> {
    gst::init().expect("Cannot initialize gstreamer");
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let main_loop = glib::MainLoop::new(None, false);
    let pipeline = create_pipeline();

    pipeline.set_state(gst::State::Playing)?;

    main_loop.run();

    Ok(())
}
