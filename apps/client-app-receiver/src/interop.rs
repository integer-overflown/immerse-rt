use std::ffi;

use tracing::warn;

#[no_mangle]
#[must_use]
extern "C" fn init() -> ffi::c_int {
    let res = match crate::init() {
        Ok(_) => true,
        Err(e) => {
            warn!("Failed to initialize: {e}");
            false
        }
    };

    res.into()
}
