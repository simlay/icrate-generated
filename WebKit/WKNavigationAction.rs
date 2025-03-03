//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum WKNavigationType {
        WKNavigationTypeLinkActivated = 0,
        WKNavigationTypeFormSubmitted = 1,
        WKNavigationTypeBackForward = 2,
        WKNavigationTypeReload = 3,
        WKNavigationTypeFormResubmitted = 4,
        WKNavigationTypeOther = -1,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_WKNavigationAction")]
    pub struct WKNavigationAction;

    #[cfg(feature = "WebKit_WKNavigationAction")]
    unsafe impl ClassType for WKNavigationAction {
        type Super = NSObject;
    }
);

#[cfg(feature = "WebKit_WKNavigationAction")]
unsafe impl NSObjectProtocol for WKNavigationAction {}

extern_methods!(
    #[cfg(feature = "WebKit_WKNavigationAction")]
    unsafe impl WKNavigationAction {
        #[cfg(feature = "WebKit_WKFrameInfo")]
        #[method_id(@__retain_semantics Other targetFrame)]
        pub unsafe fn targetFrame(&self) -> Option<Id<WKFrameInfo>>;

        #[method(navigationType)]
        pub unsafe fn navigationType(&self) -> WKNavigationType;

        #[cfg(feature = "Foundation_NSURLRequest")]
        #[method_id(@__retain_semantics Other request)]
        pub unsafe fn request(&self) -> Id<NSURLRequest>;

        #[method(shouldPerformDownload)]
        pub unsafe fn shouldPerformDownload(&self) -> bool;

        #[method(modifierFlags)]
        pub unsafe fn modifierFlags(&self) -> NSEventModifierFlags;

        #[method(buttonNumber)]
        pub unsafe fn buttonNumber(&self) -> NSInteger;
    }
);
