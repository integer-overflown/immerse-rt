import CoreMotion
import os

enum AppError: Error {
    case ServiceNotAvailable
    case PermissionDenied
}

@main
struct App {
    static func main() throws {
        let greeter = Greeter("Swift")
        greeter.sayHello()

        let logger = Logger(subsystem: "com.github.immerse_rt", category: "ht_core_motion")
        let motionService = CMHeadphoneMotionManager()

        guard motionService.isDeviceMotionAvailable else {
            logger.error("Device motion service is not available")
            throw AppError.ServiceNotAvailable
        }

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
        };
    }
}
