use irt_space::{Orientation, Point3};

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
        Self { orientation }
    }

    pub fn perceived_scene(&self, scene: &Scene) -> Scene {
        scene.relative_to(self.orientation)
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
}
