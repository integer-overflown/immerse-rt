use irt_lin_alg::{Orientation, Point3};

#[derive(Debug, PartialEq)]
pub struct Scene {
    sources: Vec<Source>,
}

#[derive(Debug, PartialEq)]
pub struct Source {
    position: Point3,
}

#[derive(Debug)]
pub struct Listener {
    location: Point3,
    orientation: Orientation,
}

impl Source {
    pub fn new(position: Point3) -> Self {
        Self { position }
    }

    pub fn perceived_from(&self, orientation: &Orientation) -> Self {
        let position = orientation.transform_point(&self.position);
        Self::new(position)
    }

    pub fn location(&self) -> &Point3 {
        &self.position
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
