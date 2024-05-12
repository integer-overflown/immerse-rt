#[swift_bridge::bridge]
mod ffi {
    enum ApiError {
        NotAvailable,
        PermissionDenied,
    }

    enum SwiftResult {
        Success,
        Failure(ApiError),
    }

    extern "Swift" {
        type CoreMotionHeadTracker;

        #[swift_bridge(init)]
        fn new() -> CoreMotionHeadTracker;

        #[swift_bridge(swift_name = "startMotionUpdates")]
        fn start_motion_updates(&self) -> SwiftResult;

        #[swift_bridge(swift_name = "stopMotionUpdates")]
        fn stop_motion_updates(&self) -> SwiftResult;
    }
}
