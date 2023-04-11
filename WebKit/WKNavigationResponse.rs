//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_WKNavigationResponse")]
    /**
      Contains information about a navigation response, used for making policy decisions.
    */
    pub struct WKNavigationResponse;

    #[cfg(feature = "WebKit_WKNavigationResponse")]
    unsafe impl ClassType for WKNavigationResponse {
        type Super = NSObject;
    }
);

#[cfg(feature = "WebKit_WKNavigationResponse")]
/**
  Contains information about a navigation response, used for making policy decisions.
*/
unsafe impl NSObjectProtocol for WKNavigationResponse {}

extern_methods!(
    /**
      Contains information about a navigation response, used for making policy decisions.
    */
    #[cfg(feature = "WebKit_WKNavigationResponse")]
    unsafe impl WKNavigationResponse {
        /**
          @abstract A Boolean value indicating whether the frame being navigated is the main frame.
        */
        #[method(isForMainFrame)]
        pub unsafe fn isForMainFrame(&self) -> bool;

        #[cfg(feature = "Foundation_NSURLResponse")]
        /**
          @abstract The frame's response.
        */
        #[method_id(@__retain_semantics Other response)]
        pub unsafe fn response(&self) -> Id<NSURLResponse>;

        /**
          @abstract A Boolean value indicating whether WebKit can display the response's MIME type natively.
        @discussion Allowing a navigation response with a MIME type that can't be shown will cause the navigation to fail.
        */
        #[method(canShowMIMEType)]
        pub unsafe fn canShowMIMEType(&self) -> bool;
    }
);
