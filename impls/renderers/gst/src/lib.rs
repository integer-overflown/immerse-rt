use std::io::Read;
use std::path::Path;
use std::{fs, io};

use gst::prelude::*;

use irt_spatial::{Renderer, Scene};

#[derive(Debug)]
pub struct HrtfRenderer {
    element: gst::Element,
}

#[derive(thiserror::Error, Debug)]
pub enum SetupError {
    #[error("HRTF renderer element is missing: please make sure you have audiofx Rust plugins installed and available in the plugin search path"
    )]
    MissingPlugin,
    #[error("cannot open HRIR file")]
    CannotReadHRIRFile(
        #[from]
        #[source]
        io::Error,
    ),
}

trait ToValueArray {
    fn to_value_array(&self) -> gst::Array;
}

impl HrtfRenderer {
    pub fn new_with_file(path: impl AsRef<Path>) -> Result<Self, SetupError> {
        let mut file = fs::File::open(path)?;
        let mut bytes = Vec::new();

        file.read_to_end(&mut bytes)?;

        Self::new_with_raw_bytes(&bytes)
    }

    pub fn new_with_raw_bytes(hrir_bytes: &[u8]) -> Result<Self, SetupError> {
        let element = gst::ElementFactory::make("hrtfrender")
            .property("hrir-raw", gst::glib::Bytes::from(&hrir_bytes))
            .build()
            .map_err(|_| SetupError::MissingPlugin)?;

        Ok(Self { element })
    }

    pub fn element(&self) -> gst::Element {
        self.element.clone()
    }
}

impl ToValueArray for Scene {
    fn to_value_array(&self) -> gst::Array {
        gst::Array::from_iter(self.sources().iter().map(|source| {
            let location = source.location();

            gst::Structure::builder("application/spatial-object")
                .field("x", location.x)
                .field("y", location.y)
                .field("z", location.z)
                .field("distance-gain", source.distance_gain())
                .build()
                .to_send_value()
        }))
    }
}

impl Renderer for HrtfRenderer {
    fn render_scene(&mut self, scene: &Scene) {
        self.element
            .set_property("spatial-objects", scene.to_value_array());
    }
}
