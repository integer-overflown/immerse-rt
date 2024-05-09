import AppKit
import CoreMotion
import os

enum AppError: Error {
    case ServiceNotAvailable
    case PermissionDenied
}

let logger = Logger(subsystem: "com.github.immerse_rt", category: "ht_core_motion")

class CoreMotionApiImpl: NSObject, CMHeadphoneMotionManagerDelegate {
    let motionService = CMHeadphoneMotionManager()

    override init() {
        super.init()
        motionService.delegate = self
    }

    public func checkRequirements() throws {
        try ensureServiceAvailability()
        try ensurePermissions()
    }

    public func startMotionUpdates() {
        logger.info("Starting motion updates")

        motionService.startDeviceMotionUpdates(to: OperationQueue()) { [weak self] motion, error in
            guard let self else {
                return
            }

            guard error == nil else {
                logger.error("Error received during receiving motion updates: \(error)")
                stopMotionUpdates()
                // TODO(max-khm): propagate error on the higher level
                return
            }

            guard let motion else {
                return
            }

            let attitude = motion.attitude
            logger.debug("Pitch: \(attitude.pitch), yaw: \(attitude.yaw), roll: \(attitude.roll)")
        }

        logger.debug("Motion manager status: \(self.motionService.isDeviceMotionActive)")
    }

    func stopMotionUpdates() {
        logger.info("Stopping motion updates")
        motionService.stopDeviceMotionUpdates()
    }

    func headphoneMotionManagerDidConnect(_: CMHeadphoneMotionManager) {
        logger.info("Headphone manager has connected")
    }

    func headphoneMotionManagerDidDisconnect(_: CMHeadphoneMotionManager) {
        logger.info("Headphone manager has disconnected")
    }

    func ensureServiceAvailability() throws {
        logger.debug("isDeviceMotionAvailable: \(self.motionService.isDeviceMotionAvailable)")

        guard motionService.isDeviceMotionAvailable else {
            logger.error("Device motion service is not available")
            throw AppError.ServiceNotAvailable
        }
    }

    func ensurePermissions() throws {
        let authStatus = CMHeadphoneMotionManager.authorizationStatus()

        switch authStatus {
        case CMAuthorizationStatus.denied:
            logger.error("Motion data access has been denied")
            throw AppError.PermissionDenied
        case CMAuthorizationStatus.restricted:
            logger.error("Motion data access is restricted")
            throw AppError.PermissionDenied
        default:
            logger.info("Motion data access permission is \(authStatus.rawValue): continuing")
        }
    }
}

@main
class App {
    static func main() throws {
        let app = NSApplication.shared
        let api = CoreMotionApiImpl()

        try api.checkRequirements()
        api.startMotionUpdates()

        app.run()
    }
}
