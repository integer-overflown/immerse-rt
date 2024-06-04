use tracing::debug;

#[macro_export]
macro_rules! element {
    ($name:literal) => {
        gst::ElementFactory::make($name).build().unwrap()
    };
}

#[derive(thiserror::Error, Debug)]
#[error("failed to preload {element_name}")]
pub struct PreloadError {
    element_name: String,
}

pub(crate) fn preload_gst_element(element_name: &str) -> Result<(), PreloadError> {
    debug!("Preloading {element_name}");

    gst::ElementFactory::make(element_name)
        .build()
        .map(|_| {})
        .map_err(|_| PreloadError {
            element_name: element_name.into(),
        })
}
