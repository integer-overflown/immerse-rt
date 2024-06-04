use std::ffi;
use std::str::Utf8Error;

use crate::stream::StreamController;
use crate::{define_error_code, try_convert};

#[repr(C)]
#[derive(Copy, Clone)]
enum CreateStreamErrorCode {
    InvalidUtf8 = -1,
}

type CreateStreamResult = crate::interop::FfiResult<*mut StreamController, CreateStreamErrorCode>;

define_error_code!(
    Utf8Error,
    CreateStreamErrorCode,
    CreateStreamErrorCode::InvalidUtf8
);

#[no_mangle]
extern "C" fn create_stream(
    token: *const ffi::c_char,
    widget: *mut ffi::c_void,
) -> CreateStreamResult {
    let controller = Box::new(crate::stream::create(try_convert!(token), widget));

    CreateStreamResult::new_with_payload(Box::into_raw(controller))
}

#[no_mangle]
extern "C" fn free_stream_result(result: CreateStreamResult) {
    if result.success {
        let _ = unsafe { Box::from_raw(result.payload.value) };
    }
}