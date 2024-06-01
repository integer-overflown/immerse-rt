#![feature(cfg_match)]

use irt_ht_interface as ht;

type HtImpl = Box<dyn ht::HeadTracker>;
type PlatformHtImpl = Option<HtImpl>;

cfg_match! {
    cfg(target_os = "macos") => {
        fn create_ht_instance() -> PlatformHtImpl
        {
            use irt_ht_core_motion::HeadTracker;
            Some(Box::new(HeadTracker::new()))
        }
    }
    _ => {
        fn create_ht_instance() -> PlatformHtImpl
        {
            None
        }
    }
}

pub fn platform_impl() -> PlatformHtImpl {
    create_ht_instance()
}
