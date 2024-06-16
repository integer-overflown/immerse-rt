import AppKit
import CoreMotion
import os

let logger = Logger(subsystem: "com.github.immerse_rt", category: "ht_core_motion")

extension Quaternion {
    init(_ q: CMQuaternion) {
        self.init(w: q.w, x: q.x, y: q.y, z: q.z)
    }
}

class CoreMotionHeadTracker: NSObject, CMHeadphoneMotionManagerDelegate {
    let motionService = CMHeadphoneMotionManager()

    override init() {
        super.init()
        motionService.delegate = self
    }

    deinit {
        logger.debug("Destroying head-tracker instance")
        doStopMotionUpdates()
    }

    func startMotionUpdates() -> StartResult {
        guard ensureServiceAvailability() else {
            return StartResult.Failure(HtError.Api(ApiError.NotAvailable))
        }

        guard ensurePermissions() else {
            return StartResult.Failure(HtError.Api(ApiError.PermissionDenied))
        }

        logger.info("Starting motion updates")
        logger.debug("isDeviceMotionAvailable: \(self.motionService.isDeviceMotionAvailable)")
        logger.debug("isDeviceMotionActive: \(self.motionService.isDeviceMotionActive)")

        motionService.startDeviceMotionUpdates();

        return StartResult.Success
    }

    func pullOrientation() -> Orientation {
        let q = motionService.deviceMotion?.attitude.quaternion;

        guard let q else {
            return Orientation.None;
        }

        return Orientation.Some(Quaternion(q))
    }

    func stopMotionUpdates() -> StopResult {
        doStopMotionUpdates()
        return StopResult.Success
    }

    func doStopMotionUpdates() {
        logger.info("Stopping motion updates")
        motionService.stopDeviceMotionUpdates()
    }

    func headphoneMotionManagerDidConnect(_: CMHeadphoneMotionManager) {
        logger.info("Headphone manager has connected")
        logger.debug("isDeviceMotionActive: \(self.motionService.isDeviceMotionActive)")
    }

    func headphoneMotionManagerDidDisconnect(_: CMHeadphoneMotionManager) {
        logger.info("Headphone manager has disconnected")
        logger.debug("isDeviceMotionActive: \(self.motionService.isDeviceMotionActive)")
    }

    func ensureServiceAvailability() -> Bool {
        logger.debug("isDeviceMotionAvailable: \(self.motionService.isDeviceMotionAvailable)")
        return motionService.isDeviceMotionAvailable
    }

    func ensurePermissions() -> Bool {
        let authStatus = CMHeadphoneMotionManager.authorizationStatus()

        switch authStatus {
        case CMAuthorizationStatus.denied:
            logger.error("Motion data access has been denied")
            return false
        case CMAuthorizationStatus.restricted:
            logger.error("Motion data access is restricted")
            return false
        default:
            logger.info("Motion data access permission is \(authStatus.rawValue): continuing")
            return true
        }
    }
}
