use std::error::Error;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Condvar, Mutex,
};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use gst::prelude::*;
use gst::{glib, BusSyncReply, EventView, MessageView, PadProbeReturn, PadProbeType};
use tracing::{debug, error, info, warn};

use irt_gst_renderer::HrtfRenderer;
use irt_ht_api as api;
use irt_ht_api::PlatformHeadTracker;

use crate::element;

fn handle_webrtc_pad(
    src_pad: &gst::Pad,
    pipeline: &gst::Pipeline,
    video_sink: &gst::Element,
    hrtf_renderer: &gst::Element,
) -> Option<glib::Value> {
    debug!("Have new pad: {}", src_pad.name());

    let mut elements = Vec::with_capacity(8);
    elements.push(element!("queue"));

    let pad_name = src_pad.name();

    match pad_name.get(..5) {
        Some("video") => {
            let pad = video_sink.static_pad("sink").unwrap();

            if pad.is_linked() {
                warn!("Duplicate video branch");
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
            let pad = hrtf_renderer.static_pad("sink").unwrap();

            if pad.is_linked() {
                warn!("Duplicate audio branch");
                return None;
            }

            elements.push(element!("audioconvert"));
            elements.push(element!("audioresample"));
            elements.push(hrtf_renderer.clone());
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

struct HtThreadConfig {
    handle: JoinHandle<()>,
    start_condition: Arc<(Mutex<bool>, Condvar)>,
    running: Arc<AtomicBool>,
}

pub struct StreamController {
    pipeline: gst::Pipeline,
    ht_thread: Option<HtThreadConfig>,
}

const SAMPLING_RESOLUTION: Duration = Duration::from_millis(100);

fn ht_thread_fn(
    start_condition: &Arc<(Mutex<bool>, Condvar)>,
    running: &Arc<AtomicBool>,
    head_tracker: &PlatformHeadTracker,
) {
    debug!("Head-tracking thread has started");

    let (lock, condvar) = &**start_condition;

    let mut started = lock.lock().unwrap();
    while !*started {
        started = condvar.wait(started).unwrap();
    }

    debug!("Unblocked");

    if !running.load(Ordering::Acquire) {
        debug!("Exiting early");
        return;
    }

    if let Err(e) = head_tracker.start_motion_updates() {
        error!("Failed to start receiving motion updates: {e}");
        return;
    }

    while running.load(Ordering::Acquire) {
        match head_tracker.pull_orientation() {
            Some(q) => {
                debug!("orientation: q: {q}");
            }
            None => {
                debug!("orientation: none");
            }
        }

        thread::sleep(SAMPLING_RESOLUTION);
    }

    debug!("Exiting");
}

fn create_ht_thread() -> Option<HtThreadConfig> {
    let Some(head_tracker) = api::platform_impl() else {
        info!("No platform head-tracking implementation: dynamic spatial audio is disabled");
        return None;
    };

    info!("Have platform head-tracking implementation: dynamic spatial audio is enabled");

    let start_condition = Arc::new((Mutex::new(false), Condvar::new()));
    let running = Arc::new(AtomicBool::new(true));

    let handle = thread::Builder::new()
        .name("irt-ht-thread".to_owned())
        .spawn({
            let start_condition = start_condition.clone();
            let running = running.clone();

            move || ht_thread_fn(&start_condition, &running, &head_tracker)
        })
        .unwrap();

    Some(HtThreadConfig {
        running,
        start_condition,
        handle,
    })
}

fn on_bus_message(
    message: &gst::Message,
    start_condition: &Arc<(Mutex<bool>, Condvar)>,
    pipeline: &gst::Pipeline,
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

            let (lock, condvar) = &**start_condition;
            let mut started = lock.lock().unwrap();

            *started = true;

            condvar.notify_one();
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

fn on_renderer_src_event(renderer: &gst::Element, event: &gst::Event) -> PadProbeReturn {
    let EventView::Caps(_) = event.view() else {
        return PadProbeReturn::Ok;
    };

    debug!("Have caps event on src pad");

    let scene = irt_gst_renderer::current_scene(renderer);
    debug!("Initial scene: {scene:?}");

    PadProbeReturn::Remove
}

pub fn create(token: &str, widget: glib::ffi::gpointer, hrir_bytes: &[u8]) -> StreamController {
    let pipeline = gst::Pipeline::new();
    let src = gst::ElementFactory::make("livekitwebrtcsrc")
        .build()
        .expect("Should have gst-webrtc Rust plugins installed");

    let signaller: glib::Object = src.property("signaller");
    signaller.set_property("auth-token", token);

    let renderer = HrtfRenderer::new_with_raw_bytes(hrir_bytes).unwrap();

    let video_sink = gst::ElementFactory::make("qml6glsink")
        .property("widget", widget)
        .build()
        .expect("Should have Qt6 plugin installed");

    {
        let pipeline = pipeline.clone();
        let video_sink = video_sink.clone();
        let hrtf_render = renderer.element();

        src.connect("pad-added", false, move |args| {
            let pad: gst::Pad = args[1].get().unwrap();
            handle_webrtc_pad(&pad, &pipeline, &video_sink, &hrtf_render);
            None
        });
    }

    {
        let element = renderer.element();
        let src_pad = element.static_pad("src").unwrap();

        src_pad.add_probe(PadProbeType::EVENT_DOWNSTREAM, move |_pad, info| {
            on_renderer_src_event(&element, info.event().unwrap())
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
        let start_condition = config.start_condition.clone();
        let pipeline = pipeline.clone();

        pipeline
            .bus()
            .unwrap()
            .set_sync_handler(move |_bus, message| {
                on_bus_message(message, &start_condition, &pipeline)
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

        let Some(HtThreadConfig {
            running,
            start_condition,
            handle,
        }) = self.ht_thread.take()
        else {
            return;
        };

        let (lock, condvar) = &*start_condition;

        running.store(false, Ordering::Release);

        let mut started = lock.lock().unwrap();
        *started = true;
        condvar.notify_one();

        match handle.join() {
            Ok(_) => {
                info!("Head-tracking thread has finished")
            }
            Err(_) => {
                warn!("Head-tracking thread panicked on shutdown");
            }
        }
    }
}
