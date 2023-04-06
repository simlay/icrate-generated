//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMHTMLFontElement")]
    #[deprecated]
    pub struct DOMHTMLFontElement;

    #[deprecated]
    #[cfg(feature = "WebKit_DOMHTMLFontElement")]
    unsafe impl ClassType for DOMHTMLFontElement {
        #[inherits(DOMElement, DOMNode, DOMObject, WebScriptObject, NSObject)]
        type Super = DOMHTMLElement;
    }
);

#[cfg(feature = "WebKit_DOMHTMLFontElement")]
unsafe impl DOMEventTarget for DOMHTMLFontElement {}

#[cfg(feature = "WebKit_DOMHTMLFontElement")]
unsafe impl NSObjectProtocol for DOMHTMLFontElement {}

extern_methods!(
    #[cfg(feature = "WebKit_DOMHTMLFontElement")]
    unsafe impl DOMHTMLFontElement {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other color)]
        pub unsafe fn color(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setColor:)]
        pub unsafe fn setColor(&self, color: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other face)]
        pub unsafe fn face(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setFace:)]
        pub unsafe fn setFace(&self, face: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other size)]
        pub unsafe fn size(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setSize:)]
        pub unsafe fn setSize(&self, size: Option<&NSString>);
    }
);
