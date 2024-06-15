use std::error::Error;

use gst::glib;
use gst::prelude::*;
use tracing::{debug, info, warn};

use irt_ht_api as api;

use crate::element;

fn handle_webrtc_pad(
    src_pad: &gst::Pad,
    pipeline: &gst::Pipeline,
    video_sink: &gst::Element,
) -> Option<glib::Value> {
    debug!("Have new pad: {}", src_pad.name());

    let mut elements = Vec::with_capacity(8);
    elements.push(element!("queue"));

    let pad_name = src_pad.name();

    match pad_name.get(..5) {
        Some("video") => {
            let pad = video_sink.static_pad("sink").unwrap();

            if pad.is_linked() {
                warn!("Video sink is already linked - ignoring this pad");
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

            pipeline
                .add_many(&elements[..elements.len() - 1])
                .expect("Could not add elements");
        }
        Some("audio") => {
            elements.push(element!("audioconvert"));
            elements.push(element!("autoaudiosink"));

            pipeline
                .add_many(&elements)
                .expect("Could not add elements");
        }
        _ => {
            warn!("Cannot handle pad: '{pad_name}'");
            return None;
        }
    }

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
    head_tracker: Option<api::PlatformHeadTracker>,
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
        let video_sink = video_sink.clone();

        src.connect("pad-added", false, move |args| {
            let pad: gst::Pad = args[1].get().unwrap();
            handle_webrtc_pad(&pad, &pipeline, &video_sink);
            None
        });
    }

    pipeline
        // It's important to add video_sink to the initial construction of the pipeline
        // to guarantee correct setup for the rendering during the actual stream.
        // The setup method is supposed to be called from the rendering thread, which
        // should bring the elements, including the sink, to the READY state and therefore
        // acquire an OpenGL context.
        // Otherwise, if the NULL=>READY transition happens in handle_webrtc_pad, the rendering
        // will most likely fail.
        // FIXME(max-khm): this will cause the pipeline to hang for the audio-only streams
        .add_many([src, video_sink])
        .expect("Could not add elements");

    StreamController {
        pipeline,
        head_tracker: api::platform_impl(),
    }
}

impl StreamController {
    pub fn setup(&self) -> Result<(), Box<dyn Error>> {
        self.pipeline.set_state(gst::State::Ready)?;
        Ok(())
    }

    pub fn play(&self) -> Result<(), Box<dyn Error>> {
        self.pipeline.set_state(gst::State::Playing)?;

        info!(
            "Starting stream, head tracking is {}",
            if self.head_tracker.is_some() {
                "on"
            } else {
                "off"
            }
        );

        Ok(())
    }
}
