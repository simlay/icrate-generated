//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    #[cfg(not(any(target_os = "ios")))]
    pub enum NSHapticFeedbackPattern {
        #[cfg(not(any(target_os = "ios")))]
        NSHapticFeedbackPatternGeneric = 0,
        #[cfg(not(any(target_os = "ios")))]
        NSHapticFeedbackPatternAlignment = 1,
        #[cfg(not(any(target_os = "ios")))]
        NSHapticFeedbackPatternLevelChange = 2,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    #[cfg(not(any(target_os = "ios")))]
    pub enum NSHapticFeedbackPerformanceTime {
        #[cfg(not(any(target_os = "ios")))]
        NSHapticFeedbackPerformanceTimeDefault = 0,
        #[cfg(not(any(target_os = "ios")))]
        NSHapticFeedbackPerformanceTimeNow = 1,
        #[cfg(not(any(target_os = "ios")))]
        NSHapticFeedbackPerformanceTimeDrawCompleted = 2,
    }
);

extern_protocol!(
    #[cfg(not(any(target_os = "ios")))]
    pub unsafe trait NSHapticFeedbackPerformer: NSObjectProtocol {
        #[cfg(not(any(target_os = "ios")))]
        #[method(performFeedbackPattern:performanceTime:)]
        unsafe fn performFeedbackPattern_performanceTime(
            &self,
            pattern: NSHapticFeedbackPattern,
            performance_time: NSHapticFeedbackPerformanceTime,
        );
    }

    unsafe impl ProtocolType for dyn NSHapticFeedbackPerformer {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSHapticFeedbackManager")]
    #[cfg(not(any(target_os = "ios")))]
    pub struct NSHapticFeedbackManager;

    #[cfg(feature = "AppKit_NSHapticFeedbackManager")]
    unsafe impl ClassType for NSHapticFeedbackManager {
        type Super = NSObject;
    }
);

#[cfg(feature = "AppKit_NSHapticFeedbackManager")]
unsafe impl NSObjectProtocol for NSHapticFeedbackManager {}

extern_methods!(
    #[cfg(feature = "AppKit_NSHapticFeedbackManager")]
    unsafe impl NSHapticFeedbackManager {
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other defaultPerformer)]
        pub unsafe fn defaultPerformer() -> Id<ProtocolObject<dyn NSHapticFeedbackPerformer>>;
    }
);
