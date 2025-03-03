//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMHTMLBaseElement")]
    #[deprecated]
    pub struct DOMHTMLBaseElement;

    #[cfg(feature = "WebKit_DOMHTMLBaseElement")]
    unsafe impl ClassType for DOMHTMLBaseElement {
        #[inherits(DOMElement, DOMNode, DOMObject, WebScriptObject, NSObject)]
        type Super = DOMHTMLElement;
    }
);

#[cfg(feature = "WebKit_DOMHTMLBaseElement")]
unsafe impl DOMEventTarget for DOMHTMLBaseElement {}

#[cfg(feature = "WebKit_DOMHTMLBaseElement")]
unsafe impl NSObjectProtocol for DOMHTMLBaseElement {}

extern_methods!(
    #[cfg(feature = "WebKit_DOMHTMLBaseElement")]
    unsafe impl DOMHTMLBaseElement {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other href)]
        pub unsafe fn href(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setHref:)]
        pub unsafe fn setHref(&self, href: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other target)]
        pub unsafe fn target(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setTarget:)]
        pub unsafe fn setTarget(&self, target: Option<&NSString>);
    }
);
