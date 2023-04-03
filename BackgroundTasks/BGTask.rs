//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::BackgroundTasks::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "BackgroundTasks_BGTask")]
    #[cfg(not(any(target_os = "macos", target_os = "tvos", target_os = "watchos")))]
    pub struct BGTask;

    #[cfg(feature = "BackgroundTasks_BGTask")]
    unsafe impl ClassType for BGTask {
        type Super = NSObject;
    }
);

#[cfg(feature = "BackgroundTasks_BGTask")]
unsafe impl NSObjectProtocol for BGTask {}

extern_methods!(
    #[cfg(feature = "BackgroundTasks_BGTask")]
    unsafe impl BGTask {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Id<NSString>;

        #[method(expirationHandler)]
        pub unsafe fn expirationHandler(&self) -> *mut Block<(), ()>;

        #[method(setExpirationHandler:)]
        pub unsafe fn setExpirationHandler(&self, expiration_handler: Option<&Block<(), ()>>);

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new(&self) -> Id<Self>;

        #[method(setTaskCompletedWithSuccess:)]
        pub unsafe fn setTaskCompletedWithSuccess(&self, success: bool);
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "BackgroundTasks_BGProcessingTask")]
    #[cfg(not(any(target_os = "macos", target_os = "tvos", target_os = "watchos")))]
    pub struct BGProcessingTask;

    #[cfg(feature = "BackgroundTasks_BGProcessingTask")]
    unsafe impl ClassType for BGProcessingTask {
        #[inherits(NSObject)]
        type Super = BGTask;
    }
);

#[cfg(feature = "BackgroundTasks_BGProcessingTask")]
unsafe impl NSObjectProtocol for BGProcessingTask {}

extern_methods!(
    #[cfg(feature = "BackgroundTasks_BGProcessingTask")]
    unsafe impl BGProcessingTask {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "BackgroundTasks_BGAppRefreshTask")]
    #[cfg(not(any(target_os = "macos", target_os = "tvos", target_os = "watchos")))]
    pub struct BGAppRefreshTask;

    #[cfg(feature = "BackgroundTasks_BGAppRefreshTask")]
    unsafe impl ClassType for BGAppRefreshTask {
        #[inherits(NSObject)]
        type Super = BGTask;
    }
);

#[cfg(feature = "BackgroundTasks_BGAppRefreshTask")]
unsafe impl NSObjectProtocol for BGAppRefreshTask {}

extern_methods!(
    #[cfg(feature = "BackgroundTasks_BGAppRefreshTask")]
    unsafe impl BGAppRefreshTask {}
);
