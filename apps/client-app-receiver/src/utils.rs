use anyhow::anyhow;
use tracing::debug;

#[macro_export]
macro_rules! element {
    ($name:literal) => {
        gst::ElementFactory::make($name).build().unwrap()
    };
}

pub(crate) fn preload_gst_element(element_name: &str) -> anyhow::Result<()> {
    debug!("Preloading {element_name}");

    gst::ElementFactory::make(element_name)
        .build()
        .map(|_| {})
        .map_err(|_| anyhow!("Failed to preload {element_name}"))
}
