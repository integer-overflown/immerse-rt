use std::ffi::{self, CStr};

#[repr(C)]
enum PluginLoadingStatus {
    Success = 0,
    InvalidName = -1,
    FailedToFindPlugin = -2,
    FailedToLoadPlugin = -3,
}

#[no_mangle]
extern "C" fn preload_gst_plugin(name: *const ffi::c_char) -> PluginLoadingStatus {
    use PluginLoadingStatus::*;

    let registry = gst::Registry::get();
    let name = unsafe { CStr::from_ptr(name) };
    let plugin_name = match name.to_str() {
        Ok(str) => str,
        Err(_) => return InvalidName,
    };

    let plugin = registry.find_plugin(plugin_name);

    let Some(plugin) = plugin else {
        return FailedToFindPlugin;
    };

    if plugin.load().is_err() {
        return FailedToLoadPlugin;
    }

    Success
}

#[no_mangle]
extern "C" fn init() -> ffi::c_int {
    crate::init().is_ok().into()
}
