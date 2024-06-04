use std::convert::Infallible;
use std::ffi::{self, CString};
use std::ops::FromResidual;
use std::str::Utf8Error;

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

#[repr(C)]
#[derive(Copy, Clone)]
enum ResultErrorCode {
    InvalidUtf8 = -1,
    RequestFailed = -2,
}

#[repr(C)]
union ResultPayload {
    token: *mut ffi::c_char,
    error: ResultErrorCode,
}

#[repr(C)]
struct RequestResult {
    success: bool,
    payload: ResultPayload,
}

impl RequestResult {
    fn new_with_payload(payload: *mut ffi::c_char) -> Self {
        Self {
            success: true,
            payload: ResultPayload { token: payload },
        }
    }

    fn new_with_error(error: ResultErrorCode) -> Self {
        Self {
            success: false,
            payload: ResultPayload { error },
        }
    }
}

impl From<CString> for RequestResult {
    fn from(value: CString) -> Self {
        Self::new_with_payload(value.into_raw())
    }
}

#[repr(C)]
struct RoomOptions {
    room_id: *const ffi::c_char,
    identity: *const ffi::c_char,
    name: *const ffi::c_char,
}

impl<E: Into<ResultErrorCode>> FromResidual<Result<Infallible, E>> for RequestResult {
    fn from_residual(residual: Result<Infallible, E>) -> Self {
        RequestResult::new_with_error(residual.err().unwrap().into())
    }
}

macro_rules! define_error_code {
    ($source_err:ty, $err_code_type:ty, $err_code:expr) => {
        impl From<$source_err> for $err_code_type {
            fn from(e: $source_err) -> Self {
                tracing::error!("error occurred: {e}");
                $err_code
            }
        }
    };
}

macro_rules! try_convert {
    ($str:expr) => {
        unsafe { ffi::CStr::from_ptr($str) }.to_str()?
    };
}

define_error_code!(Utf8Error, ResultErrorCode, ResultErrorCode::InvalidUtf8);
define_error_code!(
    crate::RequestError,
    ResultErrorCode,
    ResultErrorCode::RequestFailed
);

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
        let _ = unsafe { CString::from_raw(request_result.payload.token) };
    }
}
