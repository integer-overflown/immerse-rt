pub use irt_lin_alg::{Orientation, Point3};

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
    fn set_scene(&self, scene: &Scene);
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
        Scene::new(sources).relative_to(self.orientation)
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
