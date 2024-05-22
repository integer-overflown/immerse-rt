use std::f64::consts::*;

use approx::assert_relative_eq;
use na::{point, Vector3};
use nalgebra as na;

use irt_space::Orientation;
use irt_spatializer::{Listener, Scene, Source};

#[test]
fn test_point_rotation() {
    let point = point![-1.0, 2.0, 0.0];
    let source = Source::new(point);
    let orientation = Orientation::from_axis_angle(&Vector3::z_axis(), -FRAC_PI_2);

    let perceived_source = source.perceived_from(&orientation);

    assert_relative_eq!(*perceived_source.location(), orientation * point);
}

#[test]
fn test_scene_rotation() {
    let orientation = Orientation::from_axis_angle(&Vector3::z_axis(), -FRAC_PI_2);
    let scene_points = [
        point![-1.0, 2.0, 0.0],
        point![1.0, 3.0, 0.0],
        point![0.0, 3.0, 2.0],
    ];

    let rotated_points = scene_points.map(|p| orientation * p);

    let initial_scene = Scene::new(scene_points.map(Source::new).into());
    let rotated_scene = Scene::new(rotated_points.map(Source::new).into());

    assert_eq!(initial_scene.relative_to(orientation), rotated_scene);
}

#[test]
fn test_listener_perceived_scene() {
    let orientation = Orientation::from_axis_angle(&Vector3::z_axis(), -FRAC_PI_2);
    let listener = Listener::new(orientation);

    let scene = Scene::new(vec![
        Source::new(point![-1.0, 2.0, 0.0]),
        Source::new(point![1.0, 3.0, 0.0]),
        Source::new(point![0.0, 3.0, 2.0]),
    ]);

    assert_eq!(
        listener.perceived_scene(&scene),
        scene.relative_to(orientation)
    );
}
