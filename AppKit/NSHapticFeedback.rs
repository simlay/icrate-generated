//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSHapticFeedbackPattern {
        NSHapticFeedbackPatternGeneric = 0,
        NSHapticFeedbackPatternAlignment = 1,
        NSHapticFeedbackPatternLevelChange = 2,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSHapticFeedbackPerformanceTime {
        NSHapticFeedbackPerformanceTimeDefault = 0,
        NSHapticFeedbackPerformanceTimeNow = 1,
        NSHapticFeedbackPerformanceTimeDrawCompleted = 2,
    }
);

extern_protocol!(
    pub unsafe trait NSHapticFeedbackPerformer: NSObjectProtocol {
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
        /**
          The most appropriate feedback performer for the current input device, accessibility settings and user preferences. Note: This device may change during the life of your application. Always request the defaultPerformer when you need perform feedback.
        */
        #[method_id(@__retain_semantics Other defaultPerformer)]
        pub unsafe fn defaultPerformer() -> Id<ProtocolObject<dyn NSHapticFeedbackPerformer>>;
    }
);
