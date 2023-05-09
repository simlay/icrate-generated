//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMHTMLModElement")]
    #[deprecated]
    pub struct DOMHTMLModElement;

    #[cfg(feature = "WebKit_DOMHTMLModElement")]
    unsafe impl ClassType for DOMHTMLModElement {
        #[inherits(DOMElement, DOMNode, DOMObject, WebScriptObject, NSObject)]
        type Super = DOMHTMLElement;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "WebKit_DOMHTMLModElement")]
unsafe impl DOMEventTarget for DOMHTMLModElement {}

#[cfg(feature = "WebKit_DOMHTMLModElement")]
unsafe impl NSCopying for DOMHTMLModElement {}

#[cfg(feature = "WebKit_DOMHTMLModElement")]
unsafe impl NSObjectProtocol for DOMHTMLModElement {}

extern_methods!(
    #[cfg(feature = "WebKit_DOMHTMLModElement")]
    unsafe impl DOMHTMLModElement {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other cite)]
        pub unsafe fn cite(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setCite:)]
        pub unsafe fn setCite(&self, cite: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other dateTime)]
        pub unsafe fn dateTime(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setDateTime:)]
        pub unsafe fn setDateTime(&self, date_time: Option<&NSString>);
    }
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(feature = "WebKit_DOMHTMLModElement")]
    unsafe impl DOMHTMLModElement {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "WebKit_DOMHTMLModElement")]
    unsafe impl DOMHTMLModElement {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
