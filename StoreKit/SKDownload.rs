//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::StoreKit::*;

ns_closed_enum!(
    #[underlying(NSInteger)]
    #[deprecated = "Hosted content is no longer supported"]
    pub enum SKDownloadState {
        #[deprecated = "Hosted content is no longer supported"]
        SKDownloadStateWaiting = 0,
        #[deprecated = "Hosted content is no longer supported"]
        SKDownloadStateActive = 1,
        #[deprecated = "Hosted content is no longer supported"]
        SKDownloadStatePaused = 2,
        #[deprecated = "Hosted content is no longer supported"]
        SKDownloadStateFinished = 3,
        #[deprecated = "Hosted content is no longer supported"]
        SKDownloadStateFailed = 4,
        #[deprecated = "Hosted content is no longer supported"]
        SKDownloadStateCancelled = 5,
    }
);

extern_static!(SKDownloadTimeRemainingUnknown: NSTimeInterval);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "StoreKit_SKDownload")]
    #[deprecated = "Hosted content is no longer supported"]
    pub struct SKDownload;

    #[cfg(feature = "StoreKit_SKDownload")]
    unsafe impl ClassType for SKDownload {
        type Super = NSObject;
    }
);

#[cfg(feature = "StoreKit_SKDownload")]
unsafe impl NSObjectProtocol for SKDownload {}

extern_methods!(
    #[cfg(feature = "StoreKit_SKDownload")]
    unsafe impl SKDownload {
        #[deprecated = "Hosted content is no longer supported"]
        #[method(state)]
        pub unsafe fn state(&self) -> SKDownloadState;

        #[deprecated]
        #[cfg(not(any(target_os = "macos", target_os = "watchos")))]
        #[method(downloadState)]
        pub unsafe fn downloadState(&self) -> SKDownloadState;

        #[cfg(feature = "Foundation_NSNumber")]
        #[deprecated]
        #[cfg(not(any(target_os = "ios", target_os = "tvos", target_os = "watchos")))]
        #[method_id(@__retain_semantics Other contentLength)]
        pub unsafe fn contentLength(&self) -> Id<NSNumber>;

        #[deprecated = "Hosted content is no longer supported"]
        #[method(expectedContentLength)]
        pub unsafe fn expectedContentLength(&self) -> c_longlong;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Hosted content is no longer supported"]
        #[method_id(@__retain_semantics Other contentIdentifier)]
        pub unsafe fn contentIdentifier(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSURL")]
        #[deprecated = "Hosted content is no longer supported"]
        #[method_id(@__retain_semantics Other contentURL)]
        pub unsafe fn contentURL(&self) -> Option<Id<NSURL>>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Hosted content is no longer supported"]
        #[method_id(@__retain_semantics Other contentVersion)]
        pub unsafe fn contentVersion(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSError")]
        #[deprecated = "Hosted content is no longer supported"]
        #[method_id(@__retain_semantics Other error)]
        pub unsafe fn error(&self) -> Option<Id<NSError>>;

        #[deprecated = "Hosted content is no longer supported"]
        #[method(progress)]
        pub unsafe fn progress(&self) -> c_float;

        #[deprecated = "Hosted content is no longer supported"]
        #[method(timeRemaining)]
        pub unsafe fn timeRemaining(&self) -> NSTimeInterval;

        #[cfg(feature = "StoreKit_SKPaymentTransaction")]
        #[deprecated = "Hosted content is no longer supported"]
        #[method_id(@__retain_semantics Other transaction)]
        pub unsafe fn transaction(&self) -> Id<SKPaymentTransaction>;

        #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSURL"))]
        #[deprecated = "Hosted content is no longer supported"]
        #[cfg(not(any(target_os = "ios", target_os = "tvos", target_os = "watchos")))]
        #[method_id(@__retain_semantics Other contentURLForProductID:)]
        pub unsafe fn contentURLForProductID(product_id: &NSString) -> Option<Id<NSURL>>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Hosted content is no longer supported"]
        #[cfg(not(any(target_os = "ios", target_os = "tvos", target_os = "watchos")))]
        #[method(deleteContentForProductID:)]
        pub unsafe fn deleteContentForProductID(product_id: &NSString);
    }
);
