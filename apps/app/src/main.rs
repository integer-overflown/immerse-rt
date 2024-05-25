use std::env::args;

use anyhow::anyhow;
use gst::glib;
use gst::prelude::*;

use irt_gst_renderer::HrtfRenderer;
use irt_spatial::{Renderer, Scene, Source};

fn create_pipeline(file_path: &str, scene: &Scene) -> anyhow::Result<gst::Pipeline> {
    let pipeline = gst::Pipeline::new();
    let source = gst::ElementFactory::make("audiotestsrc")
        .property("is-live", true)
        .build()
        .unwrap();
    let caps_filter = gst::ElementFactory::make("capsfilter")
        .property(
            "caps",
            gst::Caps::builder("audio/x-raw")
                .field("channels", scene.sources().len() as i32)
                .field("channel-mask", gst::Bitmask(0u64))
                .build(),
        )
        .build()
        .unwrap();
    let queue = gst::ElementFactory::make("queue").build().unwrap();
    let renderer = HrtfRenderer::new_with_file(file_path)?;
    let sink = gst::ElementFactory::make("autoaudiosink").build().unwrap();

    renderer.set_scene(scene);

    let elements = [&source, &queue, &caps_filter, &renderer.element(), &sink];

    pipeline.add_many(elements)?;
    gst::Element::link_many(elements)?;

    Ok(pipeline)
}

fn main() -> anyhow::Result<()> {
    gst::init()?;

    let mut args = args().skip(1);

    let Some(hrir_file) = args.next() else {
        return Err(anyhow!("First argument must be a path to HRIR"));
    };

    let main_loop = glib::MainLoop::new(None, false);
    let sources = vec![Source::with_location([-5.0, 0.0, 1.0])
        .distance_gain(0.2)
        .build()];

    let scene = Scene::new(sources);
    let pipeline = create_pipeline(&hrir_file, &scene)?;

    pipeline.set_state(gst::State::Playing)?;

    main_loop.run();
    Ok(())
}
