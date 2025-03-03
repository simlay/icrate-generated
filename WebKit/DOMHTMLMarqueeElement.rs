//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMHTMLMarqueeElement")]
    #[deprecated]
    pub struct DOMHTMLMarqueeElement;

    #[cfg(feature = "WebKit_DOMHTMLMarqueeElement")]
    unsafe impl ClassType for DOMHTMLMarqueeElement {
        #[inherits(DOMElement, DOMNode, DOMObject, WebScriptObject, NSObject)]
        type Super = DOMHTMLElement;
    }
);

#[cfg(feature = "WebKit_DOMHTMLMarqueeElement")]
unsafe impl DOMEventTarget for DOMHTMLMarqueeElement {}

#[cfg(feature = "WebKit_DOMHTMLMarqueeElement")]
unsafe impl NSObjectProtocol for DOMHTMLMarqueeElement {}

extern_methods!(
    #[cfg(feature = "WebKit_DOMHTMLMarqueeElement")]
    unsafe impl DOMHTMLMarqueeElement {
        #[method(start)]
        pub unsafe fn start(&self);

        #[method(stop)]
        pub unsafe fn stop(&self);
    }
);
