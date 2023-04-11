//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_WKNavigation")]
    /**
      A WKNavigation object can be used for tracking the loading progress of a webpage.
    @discussion A navigation is returned from the web view load methods, and is
    also passed to the navigation delegate methods, to uniquely identify a webpage
    load from start to finish.
    */
    pub struct WKNavigation;

    #[cfg(feature = "WebKit_WKNavigation")]
    unsafe impl ClassType for WKNavigation {
        type Super = NSObject;
    }
);

#[cfg(feature = "WebKit_WKNavigation")]
/**
  A WKNavigation object can be used for tracking the loading progress of a webpage.
@discussion A navigation is returned from the web view load methods, and is
also passed to the navigation delegate methods, to uniquely identify a webpage
load from start to finish.
*/
unsafe impl NSObjectProtocol for WKNavigation {}

extern_methods!(
    /**
      A WKNavigation object can be used for tracking the loading progress of a webpage.
    @discussion A navigation is returned from the web view load methods, and is
    also passed to the navigation delegate methods, to uniquely identify a webpage
    load from start to finish.
    */
    #[cfg(feature = "WebKit_WKNavigation")]
    unsafe impl WKNavigation {
        /**
          The content mode used when loading this webpage.
        @discussion The value is either WKContentModeMobile or WKContentModeDesktop.
        */
        #[method(effectiveContentMode)]
        pub unsafe fn effectiveContentMode(&self) -> WKContentMode;
    }
);
