use std::mem::ManuallyDrop;
use std::str::Utf8Error;
use std::{ffi, slice};

use crate::stream::subscriber::{self, StreamController};
use crate::{define_error_code, try_convert};

#[repr(C)]
#[derive(Copy, Clone)]
enum CreateSubscriberErrorCode {
    InvalidUtf8 = -1,
}

type CreateSubscriberResult =
    crate::interop::FfiResult<*mut StreamController, CreateSubscriberErrorCode>;

define_error_code!(
    Utf8Error,
    CreateSubscriberErrorCode,
    CreateSubscriberErrorCode::InvalidUtf8
);

#[repr(C)]
struct MemoryBuffer {
    data: *const u8,
    len: usize,
}

#[no_mangle]
extern "C" fn create_subscriber_stream(
    token: *const ffi::c_char,
    widget: *mut ffi::c_void,
    hrir_bytes: MemoryBuffer,
) -> CreateSubscriberResult {
    let controller = Box::new(subscriber::create(try_convert!(token), widget, unsafe {
        slice::from_raw_parts(hrir_bytes.data, hrir_bytes.len)
    }));

    CreateSubscriberResult::new_with_payload(Box::into_raw(controller))
}

#[no_mangle]
extern "C" fn free_subscriber_stream_result(result: CreateSubscriberResult) {
    if let Some(p) = result.value() {
        free_subscriber_stream(p);
    }
}

#[no_mangle]
extern "C" fn free_subscriber_stream(stream: *mut StreamController) {
    let _ = unsafe { Box::from_raw(stream) };
}

#[no_mangle]
#[must_use]
extern "C" fn start_subscriber_stream(stream: *mut StreamController) -> bool {
    let stream = ManuallyDrop::new(unsafe { Box::from_raw(stream) });

    stream.play().is_ok()
}

#[no_mangle]
#[must_use]
extern "C" fn setup_subscriber_stream(stream: *mut StreamController) -> bool {
    let stream = ManuallyDrop::new(unsafe { Box::from_raw(stream) });

    stream.setup().is_ok()
}
