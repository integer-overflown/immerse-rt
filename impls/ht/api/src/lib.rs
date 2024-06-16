#![feature(cfg_match)]

use tracing::info;

use irt_ht_interface as ht;

pub type PlatformHeadTracker = Box<dyn ht::HeadTracker + Send + Sync>;

type PlatformHtImpl = Option<PlatformHeadTracker>;

cfg_match! {
    cfg(target_os = "macos") => {
        fn create_ht_instance() -> PlatformHtImpl
        {
            use irt_ht_core_motion::HeadTracker;
            info!("Instantiating CoreMotion implementation");
            Some(Box::new(HeadTracker::new()))
        }
    }
    _ => {
        fn create_ht_instance() -> PlatformHtImpl
        {
            info!("No implementation is available on this platform");
            None
        }
    }
}

pub fn platform_impl() -> PlatformHtImpl {
    create_ht_instance()
}
