use std::ffi::{self, CString};
use std::str::Utf8Error;

use tracing::warn;

use app_protocol::token::{PeerRole, TokenRequest};

use crate::client::Client;

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

#[must_use]
#[no_mangle]
extern "C" fn request_token(
    server_url: *const ffi::c_char,
    room_options: RoomOptions,
) -> RequestResult {
    let server_url = try_convert!(server_url);
    let room_id = try_convert!(room_options.room_id).to_owned();
    let identity = try_convert!(room_options.identity).to_owned();
    let name = match unsafe { room_options.name.as_ref() } {
        Some(v) => Some(try_convert!(v).to_string()),
        None => None,
    };

    let client = match Client::new(server_url) {
        Ok(v) => v,
        Err(e) => {
            warn!("Invalid server url: {server_url}");
            return ResultErrorCode::InvalidUrl.into();
        }
    };

    match client.request_token(TokenRequest {
        name,
        identity,
        room_id,
        role: PeerRole::Subscriber,
    }) {
        Ok(v) => CString::new(v).unwrap().into(),
        Err(e) => ResultErrorCode::RequestFailed.into(),
    }
}
