import AppKit
import CoreMotion
import os

let logger = Logger(subsystem: "com.github.immerse_rt", category: "ht_core_motion")

class CoreMotionHeadTracker: NSObject, CMHeadphoneMotionManagerDelegate {
    let motionService = CMHeadphoneMotionManager()

    override init() {
        super.init()
        motionService.delegate = self
    }

    func startMotionUpdates(dest: MotionDataDestination) -> StartResult {
        guard ensureServiceAvailability() else {
            return StartResult.Failure(HtError.Api(ApiError.NotAvailable))
        }

        guard ensurePermissions() else {
            return StartResult.Failure(HtError.Api(ApiError.PermissionDenied))
        }

        logger.info("Starting motion updates")
        logger.debug("isDeviceMotionAvailable: \(self.motionService.isDeviceMotionAvailable)")
        logger.debug("isDeviceMotionActive: \(self.motionService.isDeviceMotionActive)")

        motionService.startDeviceMotionUpdates(to: OperationQueue(), withHandler: { [weak self] motion, error in
            guard let self else {
                return
            }

            guard error == nil else {
                logger.error("Error received during receiving motion updates: \(error)")
                let _ = stopMotionUpdates()
                return // TODO(max-khm): propagate error on the higher level
            }

            guard let motion else {
                logger.debug("No updated motion info")
                return
            }

            onMotionUpdate(motion, destination: dest)
        })

        return StartResult.Success
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

    func onMotionUpdate(_ motion: CMDeviceMotion, destination: MotionDataDestination) {
        let q = motion.attitude.quaternion
        logger.debug("Orientation: w: \(q.w), x: \(q.x), y: \(q.y), z: \(q.z)")

        if !destination.push_quaternion(Quaternion(w: q.w, x: q.x, y: q.y, z: q.z)) {
            logger.debug("Caller is no longer interested in updates - stopping")
            doStopMotionUpdates()
        }
    }
}