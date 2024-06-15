use std::error::Error;
use std::sync::mpsc::{self, Sender};
use std::thread::{self, JoinHandle};

use gst::prelude::*;
use gst::{glib, BusSyncReply, MessageView};
use tracing::{debug, error, info, warn};

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

#[derive(Debug)]
enum StateChangeMessage {
    StartedPlaying,
    ShuttingDown,
}

struct HtThreadConfig {
    handle: Option<JoinHandle<()>>,
    sender: Sender<StateChangeMessage>,
}

pub struct StreamController {
    pipeline: gst::Pipeline,
    ht_thread: Option<HtThreadConfig>,
}

fn create_ht_thread() -> Option<HtThreadConfig> {
    let Some(_ht_api_impl) = api::platform_impl() else {
        info!("No platform head-tracking implementation: dynamic spatial audio is disabled");
        return None;
    };

    info!("Have platform head-tracking implementation: dynamic spatial audio is enabled");

    let (tx, rx) = mpsc::channel();

    let ht_thread_handle = thread::Builder::new()
        .name("irt-ht-thread".to_owned())
        .spawn(move || {
            use StateChangeMessage::*;
            debug!("Head-tracking thread has started");

            loop {
                let Ok(message) = rx.recv() else {
                    debug!("Sending counterpart has dropped - quitting");
                    return;
                };

                if matches!(message, StartedPlaying | ShuttingDown) {
                    debug!("Unblocking, reason: {:?}", message);
                    break;
                }
            }
        })
        .unwrap();

    Some(HtThreadConfig {
        sender: tx,
        handle: Some(ht_thread_handle),
    })
}

struct MessageHandlerData<'a>(&'a Sender<StateChangeMessage>, &'a gst::Pipeline);

fn on_bus_message(
    message: &gst::Message,
    MessageHandlerData(tx, pipeline): MessageHandlerData,
) -> BusSyncReply {
    match message.view() {
        MessageView::StateChanged(state_changed) => 'b: {
            if message.src() != Some(pipeline.upcast_ref::<gst::Object>()) {
                break 'b;
            }

            debug!(
                "Pipeline state changed: {:?} -> {:?} (pending {:?})",
                state_changed.old(),
                state_changed.current(),
                state_changed.pending()
            );

            if state_changed.current() != gst::State::Playing {
                break 'b;
            }

            if let Err(e) = tx.send(StateChangeMessage::StartedPlaying) {
                warn!("Receiver has already hang up: {e}");
            }
        }
        MessageView::Error(error) => {
            error!(
                "Pipeline error: {}; debug info: {:?}",
                error.error().message(),
                error.debug()
            )
        }
        _ => {}
    }

    BusSyncReply::Pass
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

    let ht_thread = create_ht_thread();

    if let Some(config) = ht_thread.as_ref() {
        let tx = config.sender.clone();
        let pipeline = pipeline.clone();

        pipeline
            .bus()
            .unwrap()
            .set_sync_handler(move |_bus, message| {
                on_bus_message(message, MessageHandlerData(&tx, &pipeline))
            });
    }

    StreamController {
        pipeline,
        ht_thread,
    }
}

impl StreamController {
    pub fn setup(&self) -> Result<(), Box<dyn Error>> {
        self.pipeline.set_state(gst::State::Ready)?;
        Ok(())
    }

    pub fn play(&self) -> Result<(), Box<dyn Error>> {
        self.pipeline.set_state(gst::State::Playing)?;
        Ok(())
    }
}

impl Drop for StreamController {
    fn drop(&mut self) {
        debug!("Shutting down");

        let Some(HtThreadConfig { sender, handle }) = self.ht_thread.as_mut() else {
            return;
        };

        let _ = sender.send(StateChangeMessage::ShuttingDown);

        let Err(_) = handle.take().map_or(Ok(()), JoinHandle::<()>::join) else {
            debug!("Head-tracking thread is finished");
            return;
        };

        warn!("Head-tracking thread panicked on shutdown");
    }
}
