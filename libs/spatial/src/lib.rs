pub use irt_lin_alg::{na, Orientation, Point3};

#[derive(Debug, PartialEq)]
pub struct Scene {
    sources: Vec<Source>,
}

#[derive(Debug, PartialEq)]
struct DistanceGain(f32);

impl Default for DistanceGain {
    fn default() -> Self {
        Self(1.0)
    }
}

#[derive(Debug, PartialEq)]
pub struct Source {
    position: Point3,
    distance_gain: DistanceGain,
}

#[derive(Debug)]
pub struct Listener {
    location: Point3,
    orientation: Orientation,
}

pub trait Renderer {
    fn render_scene(&mut self, scene: &Scene);
}

pub struct Soundscape<T: Renderer> {
    listener: Listener,
    scene: Scene,
    renderer: T,
}

pub struct SourceBuilder {
    position: Point3,
    distance_gain: DistanceGain,
}

impl SourceBuilder {
    pub fn distance_gain(mut self, value: f32) -> Self {
        self.distance_gain = DistanceGain(value);
        self
    }

    pub fn build(self) -> Source {
        Source {
            position: self.position,
            distance_gain: self.distance_gain,
        }
    }
}

impl Source {
    pub fn new(position: impl Into<Point3>) -> Self {
        Self {
            position: position.into(),
            distance_gain: Default::default(),
        }
    }

    pub fn with_location(position: impl Into<Point3>) -> SourceBuilder {
        SourceBuilder {
            position: position.into(),
            distance_gain: Default::default(),
        }
    }

    pub fn perceived_from(&self, orientation: &Orientation) -> Self {
        let position = orientation.transform_point(&self.position);
        Self::new(position)
    }

    pub fn location(&self) -> Point3 {
        self.position
    }

    pub fn distance_gain(&self) -> f32 {
        self.distance_gain.0
    }
}

impl Listener {
    pub fn new(orientation: Orientation) -> Self {
        Self {
            location: Point3::origin(),
            orientation,
        }
    }

    pub fn new_with_location(location: Point3, orientation: Orientation) -> Self {
        Self {
            location,
            orientation,
        }
    }

    pub fn perceived_scene(&self, scene: &Scene) -> Scene {
        // TODO(max-khm): This could be potentially optimized with SIMD calculations
        let sources = scene
            .sources
            .iter()
            .map(|source| source.position - self.location)
            .map(Point3::from)
            .map(Source::new)
            .collect();
        Scene::new(sources).relative_to(self.orientation.inverse())
    }

    pub fn location(&self) -> Point3 {
        self.location
    }
}

impl Scene {
    pub fn new(sources: Vec<Source>) -> Self {
        Self { sources }
    }

    pub fn relative_to(&self, orientation: Orientation) -> Self {
        let sources = self
            .sources
            .iter()
            .map(|source| source.perceived_from(&orientation))
            .collect();

        Self { sources }
    }

    pub fn sources(&self) -> &[Source] {
        &self.sources
    }
}

impl FromIterator<Source> for Scene {
    fn from_iter<T: IntoIterator<Item = Source>>(iter: T) -> Self {
        Self::new(iter.into_iter().collect())
    }
}

impl From<Orientation> for Listener {
    fn from(value: Orientation) -> Self {
        Self::new(value)
    }
}

impl From<(Point3, Orientation)> for Listener {
    fn from((location, orientation): (Point3, Orientation)) -> Self {
        Self::new_with_location(location, orientation)
    }
}

impl<T: Renderer> Soundscape<T> {
    pub fn new(initial_scene: Scene, initial_listener: Listener, renderer: T) -> Soundscape<T> {
        let mut instance = Self {
            listener: initial_listener,
            scene: initial_scene,
            renderer,
        };

        instance.update_scene();

        instance
    }

    pub fn set_listener(&mut self, listener: Listener) {
        self.listener = listener;
        self.update_scene();
    }

    fn update_scene(&mut self) {
        let new_scene = self.listener.perceived_scene(&self.scene);
        self.renderer.render_scene(&new_scene);
    }

    pub fn renderer(&self) -> &T {
        &self.renderer
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::*;

    use nalgebra::{point, Vector3};

    use super::*;

    #[test]
    fn test_listener_perceived_scene_rotation() {
        let point = point![-2.0, -1.0, 4.0];
        let orientation = Orientation::from_axis_angle(&Vector3::y_axis(), FRAC_PI_2);

        let expected = orientation.inverse() * point;

        let scene = Scene::new(vec![Source::new(point)]);
        let listener = Listener::new(orientation);

        let perceived = listener.perceived_scene(&scene);

        assert_eq!(perceived.sources().first(), Some(&Source::new(expected)));
    }

    #[test]
    fn test_listener_with_no_location_defaults_to_origin() {
        let listener = Listener::new(Orientation::from_axis_angle(&Vector3::y_axis(), 0.0));
        assert_eq!(listener.location(), Point3::origin());
    }
}
