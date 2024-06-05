use std::convert::Infallible;
use std::ffi;
use std::ops::FromResidual;
use std::ptr::NonNull;

use tracing::warn;

mod request;
mod stream;

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
union FfiResultPayload<V: Copy, E: Copy> {
    value: V,
    error: E,
}

#[repr(C)]
struct FfiResult<V: Copy, E: Copy> {
    success: bool,
    payload: FfiResultPayload<V, E>,
}

impl<V: Copy, E: Copy> FfiResultPayload<V, E> {
    fn value(value: V) -> Self {
        Self { value }
    }

    fn error(error: E) -> Self {
        Self { error }
    }
}

impl<V: Copy, E: Copy> FfiResult<V, E> {
    fn new_with_payload(payload: V) -> Self {
        Self {
            success: true,
            payload: FfiResultPayload::value(payload),
        }
    }

    fn new_with_error(error: E) -> Self {
        Self {
            success: false,
            payload: FfiResultPayload::error(error),
        }
    }

    fn value(&self) -> Option<V> {
        if self.success {
            Some(unsafe { self.payload.value })
        } else {
            None
        }
    }
}

impl<V, E, RE> FromResidual<Result<Infallible, RE>> for FfiResult<V, E>
where
    V: Copy,
    E: Copy,
    RE: Into<E>,
{
    fn from_residual(residual: Result<Infallible, RE>) -> Self {
        FfiResult::new_with_error(residual.err().unwrap().into())
    }
}

#[macro_export]
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

#[macro_export]
macro_rules! try_convert {
    ($str:expr) => {
        unsafe { ffi::CStr::from_ptr($str) }.to_str()?
    };
}
