//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMHTMLOListElement")]
    #[deprecated]
    pub struct DOMHTMLOListElement;

    #[deprecated]
    #[cfg(feature = "WebKit_DOMHTMLOListElement")]
    unsafe impl ClassType for DOMHTMLOListElement {
        #[inherits(DOMElement, DOMNode, DOMObject, WebScriptObject, NSObject)]
        type Super = DOMHTMLElement;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "WebKit_DOMHTMLOListElement")]
unsafe impl DOMEventTarget for DOMHTMLOListElement {}

#[cfg(feature = "WebKit_DOMHTMLOListElement")]
unsafe impl NSCopying for DOMHTMLOListElement {}

#[cfg(feature = "WebKit_DOMHTMLOListElement")]
unsafe impl NSObjectProtocol for DOMHTMLOListElement {}

extern_methods!(
    #[cfg(feature = "WebKit_DOMHTMLOListElement")]
    unsafe impl DOMHTMLOListElement {
        #[method(compact)]
        pub unsafe fn compact(&self) -> bool;

        #[method(setCompact:)]
        pub unsafe fn setCompact(&self, compact: bool);

        #[method(start)]
        pub unsafe fn start(&self) -> c_int;

        #[method(setStart:)]
        pub unsafe fn setStart(&self, start: c_int);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other type)]
        pub unsafe fn r#type(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setType:)]
        pub unsafe fn setType(&self, r#type: Option<&NSString>);
    }
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(feature = "WebKit_DOMHTMLOListElement")]
    unsafe impl DOMHTMLOListElement {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "WebKit_DOMHTMLOListElement")]
    unsafe impl DOMHTMLOListElement {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
