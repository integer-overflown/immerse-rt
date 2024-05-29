use std::env::args;
use std::time::Duration;

use anyhow::anyhow;
use gst::glib;
use gst::prelude::*;
use irt_gst_renderer::HrtfRenderer;

use irt_spatial::{na, Orientation, Scene, Soundscape, Source};
use irt_spatial::na::Vector3;

fn create_pipeline(file_path: &str, scene: Scene) -> anyhow::Result<(gst::Pipeline, Soundscape<HrtfRenderer>)> {
    let pipeline = gst::Pipeline::new();
    let source = gst::ElementFactory::make("audiotestsrc")
        .property("is-live", true)
        .property_from_str("wave", "sine")
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
    let soundscape = Soundscape::new(
        scene,
        Orientation::from_axis_angle(&na::Vector3::z_axis(), 0.0).into(),
        HrtfRenderer::new_with_file(file_path)?,
    );
    let sink = gst::ElementFactory::make("autoaudiosink").build().unwrap();

    let elements = [
        &source,
        &queue,
        &caps_filter,
        &soundscape.renderer().element(),
        &sink,
    ];

    pipeline.add_many(elements)?;
    gst::Element::link_many(elements)?;

    Ok((pipeline, soundscape))
}

fn rotations() -> Vec<Orientation> {
    let angle = std::f32::consts::FRAC_PI_2;

    let steps = 12;

    let right_angles: Vec<f32> = (1..=steps).map(|i| -angle * i as f32 / steps as f32).collect();
    let left_angles: Vec<f32> = (1..=steps).map(|i| angle * i as f32 / steps as f32).collect();

    let mut angles = Vec::with_capacity(steps * 4 + 1);

    angles.extend_from_slice(&right_angles);
    angles.extend(right_angles.iter().cloned().rev().skip(1));
    angles.extend(&left_angles);
    angles.extend(left_angles.iter().cloned().rev().skip(1));
    angles.push(0f32);

    angles.into_iter().map(|angle| Orientation::from_axis_angle(&Vector3::y_axis(), angle)).collect()
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
    let (pipeline, mut soundscape) = create_pipeline(&hrir_file, scene)?;

    pipeline.set_state(gst::State::Playing)?;

    // Wait for state change to finish
    let _ = pipeline.state(None);

    let rotations = rotations();
    let mut i = 1usize;

    glib::timeout_add(Duration::from_millis(100), move || {
        soundscape.set_listener(rotations[i].into());
        i = i.wrapping_add(1) % rotations.len();
        glib::ControlFlow::Continue
    });

    main_loop.run();
    Ok(())
}
