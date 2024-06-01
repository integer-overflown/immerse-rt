use std::ffi;

use tracing::{debug, warn};

fn preload_gst_element(element_name: &str) -> bool {
    debug!("Preloading {element_name}");

    let res = gst::ElementFactory::make(element_name).build();

    if res.is_err() {
        warn!("Failed to load {element_name}");
    }

    res.is_ok()
}

#[no_mangle]
extern "C" fn init() -> ffi::c_int {
    let res = crate::init().is_ok() && preload_gst_element("qml6glsink");
    res.into()
}
