use std::error::Error;

use gst::glib;
use gst::prelude::*;
use tracing::{debug, warn};

use crate::element;

fn handle_webrtc_pad(
    src_pad: &gst::Pad,
    pipeline: &gst::Pipeline,
    video_sink: &gst::Element,
) -> Option<glib::Value> {
    debug!("Have new pad: {}", src_pad.name());

    let mut elements = Vec::with_capacity(3);
    elements.push(element!("queue"));

    let pad_name = src_pad.name();

    match &pad_name[..5] {
        "video" => {
            let pad = video_sink.static_pad("sink").unwrap();

            if pad.is_linked() {
                warn!("Video sink os already linked - ignoring this pad");
                return None;
            }

            let filter_caps = gst::Caps::builder("video/x-raw")
                .field("format", "YV12")
                .build();
            let caps_filter = gst::ElementFactory::make("capsfilter")
                .property("caps", filter_caps)
                .build()
                .unwrap();

            elements.push(element!("videoconvert"));
            elements.push(caps_filter);
            elements.push(element!("glupload"));
            elements.push(video_sink.clone());
        }
        "audio" => {
            elements.push(element!("audioconvert"));
            elements.push(element!("autoaudiosink"));
        }
        _ => {
            warn!("Cannot handle pad: '{pad_name}'");
            return None;
        }
    }

    pipeline
        .add_many(&elements)
        .expect("Could not add elements");
    gst::Element::link_many(&elements).expect("Could not link elements");

    let chain_head = &elements[0];
    let chain_sink = chain_head.static_pad("sink").unwrap();

    src_pad
        .link(&chain_sink)
        .expect("Could not link WebRTC pad");

    elements
        .iter()
        .for_each(|element| element.sync_state_with_parent().unwrap());

    None
}

pub struct StreamController {
    pipeline: gst::Pipeline,
}

pub fn create(token: &str, widget: glib::ffi::gpointer) -> StreamController {
    let pipeline = gst::Pipeline::new();
    let src = gst::ElementFactory::make("livekitwebrtcsrc")
        .build()
        .expect("Should have gst-webrtc Rust plugins installed");

    let signaller: glib::Object = src.property("signaller");
    signaller.set_property("auth-token", token);

    let video_sink = gst::ElementFactory::make("qml6glsink")
        .property("widget", widget)
        .build()
        .expect("Should have Qt6 plugin installed");

    {
        let pipeline = pipeline.clone();

        src.connect("pad-added", false, move |args| {
            let pad: gst::Pad = args[1].get().unwrap();
            handle_webrtc_pad(&pad, &pipeline, &video_sink);
            None
        });
    }
    pipeline.add(&src).expect("Could not add elements");

    StreamController { pipeline }
}

impl StreamController {
    pub fn play(&self) -> Result<(), Box<dyn Error>> {
        self.pipeline.set_state(gst::State::Playing)?;
        Ok(())
    }
}
