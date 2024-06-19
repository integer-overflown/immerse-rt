use std::ffi;
use std::mem::ManuallyDrop;
use std::str::Utf8Error;

use crate::stream::publisher::{self, SetupError, StreamController};
use crate::{define_error_code, try_convert};

#[repr(C)]
#[derive(Copy, Clone)]
enum CreatePublisherErrorCode {
    InvalidUtf8 = -1,
    SetupFailed = -2,
}

type CreatePublisherResult =
    crate::interop::FfiResult<*mut StreamController, CreatePublisherErrorCode>;

define_error_code!(
    SetupError,
    CreatePublisherErrorCode,
    CreatePublisherErrorCode::SetupFailed
);

define_error_code!(
    Utf8Error,
    CreatePublisherErrorCode,
    CreatePublisherErrorCode::InvalidUtf8
);

#[no_mangle]
extern "C" fn create_publisher_stream(
    token: *const ffi::c_char,
    file_path: *const ffi::c_char,
) -> CreatePublisherResult {
    let token = try_convert!(token);
    let file_path = try_convert!(file_path);
    let controller = Box::new(publisher::create(token, file_path)?);

    CreatePublisherResult::new_with_payload(Box::into_raw(controller))
}

#[no_mangle]
extern "C" fn start_publisher_stream(stream: *mut StreamController) -> bool {
    let stream = ManuallyDrop::new(unsafe { Box::from_raw(stream) });

    stream.play().is_ok()
}

#[no_mangle]
extern "C" fn free_publisher_stream(stream: *mut StreamController) {
    let _ = unsafe { Box::from_raw(stream) };
}
