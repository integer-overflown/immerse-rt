use irt_ht_interface as ht;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

#[swift_bridge::bridge]
mod ffi {
    enum ApiError {
        NotAvailable,
        PermissionDenied,
    }

    #[swift_bridge(swift_repr = "struct")]
    struct UnknownError(String);

    enum HtError {
        Api(ApiError),
        Other(UnknownError),
    }

    enum StartResult {
        Success,
        Failure(HtError),
    }

    enum StopResult {
        Success,
        Failure(UnknownError),
    }

    #[swift_bridge(swift_repr = "struct")]
    struct Quaternion {
        w: f64,
        x: f64,
        y: f64,
        z: f64,
    }

    extern "Rust" {
        type MotionDataDestination;

        fn push_quaternion(&mut self, quaternion: Quaternion) -> bool;
    }

    extern "Swift" {
        type CoreMotionHeadTracker;

        #[swift_bridge(init)]
        fn new() -> CoreMotionHeadTracker;

        #[swift_bridge(swift_name = "startMotionUpdates")]
        fn start_motion_updates(&self, dest: MotionDataDestination) -> StartResult;

        #[swift_bridge(swift_name = "stopMotionUpdates")]
        fn stop_motion_updates(&self) -> StopResult;
    }
}

struct MotionDataDestination {
    source: Sender<ht::Orientation>,
}

impl MotionDataDestination {
    fn push_quaternion(&mut self, quaternion: ffi::Quaternion) -> bool {
        self.source.send(quaternion.into()).is_err()
    }
}

pub struct HeadTracker {
    internal: ffi::CoreMotionHeadTracker,
}

impl HeadTracker {
    pub fn new() -> Self {
        let internal = ffi::CoreMotionHeadTracker::new();

        Self { internal }
    }
}

impl Default for HeadTracker {
    fn default() -> Self {
        Self::new()
    }
}

// TODO(max-khm): check that resources are freed correctly between start/stop calls
impl ht::HeadTracker for HeadTracker {
    fn start_motion_updates(&self) -> Result<Receiver<ht::Orientation>, ht::Error> {
        use ffi::StartResult;

        let (tx, rx) = mpsc::channel();

        let dest = MotionDataDestination { source: tx };

        match self.internal.start_motion_updates(dest) {
            StartResult::Success => Ok(rx),
            StartResult::Failure(e) => Err(e.into()),
        }
    }

    fn stop_motion_updates(&self) -> Result<(), ht::UnknownError> {
        use ffi::StopResult;

        let StopResult::Failure(e) = self.internal.stop_motion_updates() else {
            return Ok(());
        };

        Err(e.into())
    }
}

// region FFI->interface conversions
impl From<ffi::Quaternion> for ht::UnitQuaternion {
    fn from(value: ffi::Quaternion) -> Self {
        let q = ht::Quaternion::new(value.w, value.x, value.y, value.z);
        Self::new_normalize(q)
    }
}

impl From<ffi::ApiError> for ht::ApiError {
    fn from(value: ffi::ApiError) -> Self {
        use ffi::ApiError;

        match value {
            ApiError::NotAvailable => Self::NotAvailable,
            ApiError::PermissionDenied => Self::PermissionDenied,
        }
    }
}

impl From<ffi::UnknownError> for ht::UnknownError {
    fn from(value: ffi::UnknownError) -> Self {
        Self::new(value.0)
    }
}

impl From<ffi::HtError> for ht::Error {
    fn from(value: ffi::HtError) -> Self {
        use ffi::HtError;

        match value {
            HtError::Api(e) => Self::Api(e.into()),
            HtError::Other(desc) => Self::Other(ht::UnknownError::new(desc.0)),
        }
    }
}
// endregion
