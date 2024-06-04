use std::ffi::{self, c_char, CString};
use std::str::Utf8Error;

use tracing::warn;

use app_protocol::token::{PeerRole, TokenRequest};

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
    None = 0,
    InvalidUTF8 = -1,
    InvalidUrl = -2,
    RequestFailed = -3,
}

#[repr(C)]
union ResultPayload {
    token: *const ffi::c_char,
    error: ResultErrorCode,
}

#[repr(C)]
struct RequestResult {
    success: bool,
    payload: ResultPayload,
}

impl From<ResultErrorCode> for RequestResult {
    fn from(value: ResultErrorCode) -> Self {
        Self {
            success: false,
            payload: ResultPayload { error: value },
        }
    }
}

impl From<CString> for RequestResult {
    fn from(value: CString) -> Self {
        Self {
            success: true,
            payload: ResultPayload {
                token: value.into_raw(),
            },
        }
    }
}

#[repr(C)]
struct RoomOptions {
    room_id: *const ffi::c_char,
    identity: *const ffi::c_char,
    name: *const ffi::c_char,
}

impl From<Utf8Error> for RequestResult {
    fn from(_: Utf8Error) -> Self {
        RequestResult {
            success: false,
            payload: ResultPayload {
                error: ResultErrorCode::InvalidUTF8,
            },
        }
    }
}

impl From<crate::RequestError> for RequestResult {
    fn from(_: crate::RequestError) -> Self {
        RequestResult {
            success: false,
            payload: ResultPayload {
                error: ResultErrorCode::RequestFailed,
            },
        }
    }
}

macro_rules! to_rust_str {
    ($str:expr) => {
        unsafe { ffi::CStr::from_ptr($str) }.to_str()
    };
}

macro_rules! try_convert {
    ($str:expr) => {
        match unsafe { ffi::CStr::from_ptr($str) }.to_str() {
            Ok(v) => v,
            Err(e) => {
                warn!("Invalid UTF-8 ({}): {}", std::stringify!($str), e);
                return e.into();
            }
        }
    };
}

impl TryFrom<RoomOptions> for crate::RoomOptions {
    type Error = Utf8Error;

    fn try_from(value: RoomOptions) -> Result<Self, Self::Error> {
        Ok(Self {
            room_id: to_rust_str!(value.room_id)?.to_owned(),
            identity: to_rust_str!(value.identity)?.to_owned(),
            name: match unsafe { value.name.as_ref() } {
                Some(v) => Some(to_rust_str!(v)?.to_owned()),
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
    let options = match room_options.try_into() {
        Ok(v) => v,
        Err(e) => return RequestResult::from(e),
    };

    match crate::request_token(try_convert!(server_url), options) {
        Ok(v) => CString::new(v).unwrap().into(),
        Err(e) => e.into(),
    }
}
