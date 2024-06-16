use irt_ht_interface as ht;

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

    enum Orientation {
        Some(Quaternion),
        None,
    }

    extern "Swift" {
        type CoreMotionHeadTracker;

        #[swift_bridge(init)]
        fn new() -> CoreMotionHeadTracker;

        #[swift_bridge(swift_name = "startMotionUpdates")]
        fn start_motion_updates(&self) -> StartResult;

        #[swift_bridge(swift_name = "pullOrientation")]
        fn pull_orientation(&self) -> Orientation;

        #[swift_bridge(swift_name = "stopMotionUpdates")]
        fn stop_motion_updates(&self) -> StopResult;
    }
}

pub struct HeadTracker {
    internal: ffi::CoreMotionHeadTracker,
}

unsafe impl Send for HeadTracker {}

unsafe impl Sync for HeadTracker {}

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
    fn start_motion_updates(&self) -> Result<(), ht::Error> {
        use ffi::StartResult;

        match self.internal.start_motion_updates() {
            StartResult::Success => Ok(()),
            StartResult::Failure(e) => Err(e.into()),
        }
    }

    fn pull_orientation(&self) -> Option<ht::UnitQuaternion> {
        use ffi::Orientation;

        match self.internal.pull_orientation() {
            Orientation::Some(q) => Some(q.into()),
            Orientation::None => None,
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
        // This essentially does two conversions at the same time:
        // 1. ffi::Quaternion -> ht::Quaternion
        // 2. RH Cartesian (CoreMotion) -> LH Cartesian (irt_ht_interface)
        let q = ht::Quaternion::new(
            value.w as f32,
            value.x as f32,
            value.z as f32, // yz swapped on purpose, see (2) above
            value.y as f32,
        );

        // TODO(max-khm): this could use unchecked API is CoreMotion already outputs unit quaternions
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
