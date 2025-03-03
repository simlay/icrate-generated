//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_WKDownload")]
    pub struct WKDownload;

    #[cfg(feature = "WebKit_WKDownload")]
    unsafe impl ClassType for WKDownload {
        type Super = NSObject;
    }
);

#[cfg(feature = "WebKit_WKDownload")]
unsafe impl NSObjectProtocol for WKDownload {}

#[cfg(feature = "WebKit_WKDownload")]
unsafe impl NSProgressReporting for WKDownload {}

extern_methods!(
    #[cfg(feature = "WebKit_WKDownload")]
    unsafe impl WKDownload {
        #[cfg(feature = "Foundation_NSURLRequest")]
        #[method_id(@__retain_semantics Other originalRequest)]
        pub unsafe fn originalRequest(&self) -> Option<Id<NSURLRequest>>;

        #[cfg(feature = "WebKit_WKWebView")]
        #[method_id(@__retain_semantics Other webView)]
        pub unsafe fn webView(&self) -> Option<Id<WKWebView>>;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<ProtocolObject<dyn WKDownloadDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&ProtocolObject<dyn WKDownloadDelegate>>);

        #[cfg(feature = "Foundation_NSData")]
        #[method(cancel:)]
        pub unsafe fn cancel(&self, completion_handler: Option<&Block<(*mut NSData,), ()>>);
    }
);
