//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMHTMLHRElement")]
    #[deprecated]
    pub struct DOMHTMLHRElement;

    #[cfg(feature = "WebKit_DOMHTMLHRElement")]
    unsafe impl ClassType for DOMHTMLHRElement {
        #[inherits(DOMElement, DOMNode, DOMObject, WebScriptObject, NSObject)]
        type Super = DOMHTMLElement;
    }
);

#[cfg(feature = "WebKit_DOMHTMLHRElement")]
unsafe impl DOMEventTarget for DOMHTMLHRElement {}

#[cfg(feature = "WebKit_DOMHTMLHRElement")]
unsafe impl NSObjectProtocol for DOMHTMLHRElement {}

extern_methods!(
    #[cfg(feature = "WebKit_DOMHTMLHRElement")]
    unsafe impl DOMHTMLHRElement {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other align)]
        pub unsafe fn align(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setAlign:)]
        pub unsafe fn setAlign(&self, align: Option<&NSString>);

        #[method(noShade)]
        pub unsafe fn noShade(&self) -> bool;

        #[method(setNoShade:)]
        pub unsafe fn setNoShade(&self, no_shade: bool);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other size)]
        pub unsafe fn size(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setSize:)]
        pub unsafe fn setSize(&self, size: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other width)]
        pub unsafe fn width(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setWidth:)]
        pub unsafe fn setWidth(&self, width: Option<&NSString>);
    }
);
