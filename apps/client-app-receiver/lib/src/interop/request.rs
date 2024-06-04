use std::ffi;
use std::ffi::CString;
use std::str::Utf8Error;

use crate::{define_error_code, try_convert};

#[repr(C)]
#[derive(Copy, Clone)]
enum RequestErrorCode {
    InvalidUtf8 = -1,
    RequestFailed = -2,
}

type RequestResult = crate::interop::FfiResult<*mut ffi::c_char, RequestErrorCode>;

define_error_code!(Utf8Error, RequestErrorCode, RequestErrorCode::InvalidUtf8);
define_error_code!(
    crate::RequestError,
    RequestErrorCode,
    RequestErrorCode::RequestFailed
);

#[repr(C)]
struct RoomOptions {
    room_id: *const ffi::c_char,
    identity: *const ffi::c_char,
    name: *const ffi::c_char,
}

impl TryFrom<RoomOptions> for crate::RoomOptions {
    type Error = Utf8Error;

    fn try_from(value: RoomOptions) -> Result<Self, Self::Error> {
        Ok(Self {
            room_id: try_convert!(value.room_id).to_owned(),
            identity: try_convert!(value.identity).to_owned(),
            name: match unsafe { value.name.as_ref() } {
                Some(v) => Some(try_convert!(v).to_owned()),
                None => None,
            },
        })
    }
}

#[must_use]
#[no_mangle]
extern "C" fn request_token(
    server_url: *const ffi::c_char,
    room_options: RoomOptions, 
) -> RequestResult {
    let options = room_options.try_into()?;
    let token = crate::request_token(try_convert!(server_url), options)?;

    RequestResult::new_with_payload(CString::new(token).unwrap().into_raw())
}

#[no_mangle]
extern "C" fn free_result(request_result: RequestResult) {
    if request_result.success {
        let _ = unsafe { CString::from_raw(request_result.payload.value) };
    }
}
