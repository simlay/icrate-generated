//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::StoreKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "StoreKit_SKStoreReviewController")]
    pub struct SKStoreReviewController;

    #[cfg(feature = "StoreKit_SKStoreReviewController")]
    unsafe impl ClassType for SKStoreReviewController {
        type Super = NSObject;
    }
);

#[cfg(feature = "StoreKit_SKStoreReviewController")]
unsafe impl NSObjectProtocol for SKStoreReviewController {}

extern_methods!(
    #[cfg(feature = "StoreKit_SKStoreReviewController")]
    unsafe impl SKStoreReviewController {
        #[deprecated]
        #[method(requestReview)]
        pub unsafe fn requestReview();
    }
);
