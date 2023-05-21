//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMHTMLFormElement")]
    #[deprecated]
    pub struct DOMHTMLFormElement;

    #[deprecated]
    #[cfg(feature = "WebKit_DOMHTMLFormElement")]
    unsafe impl ClassType for DOMHTMLFormElement {
        #[inherits(DOMElement, DOMNode, DOMObject, WebScriptObject, NSObject)]
        type Super = DOMHTMLElement;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "WebKit_DOMHTMLFormElement")]
unsafe impl DOMEventTarget for DOMHTMLFormElement {}

#[cfg(feature = "WebKit_DOMHTMLFormElement")]
unsafe impl NSCopying for DOMHTMLFormElement {}

#[cfg(feature = "WebKit_DOMHTMLFormElement")]
unsafe impl NSObjectProtocol for DOMHTMLFormElement {}

extern_methods!(
    #[cfg(feature = "WebKit_DOMHTMLFormElement")]
    unsafe impl DOMHTMLFormElement {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other acceptCharset)]
        pub unsafe fn acceptCharset(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setAcceptCharset:)]
        pub unsafe fn setAcceptCharset(&self, accept_charset: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other action)]
        pub unsafe fn action(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setAction:)]
        pub unsafe fn setAction(&self, action: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other enctype)]
        pub unsafe fn enctype(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setEnctype:)]
        pub unsafe fn setEnctype(&self, enctype: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other encoding)]
        pub unsafe fn encoding(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setEncoding:)]
        pub unsafe fn setEncoding(&self, encoding: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other method)]
        pub unsafe fn method(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setMethod:)]
        pub unsafe fn setMethod(&self, method: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setName:)]
        pub unsafe fn setName(&self, name: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other target)]
        pub unsafe fn target(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setTarget:)]
        pub unsafe fn setTarget(&self, target: Option<&NSString>);

        #[cfg(feature = "WebKit_DOMHTMLCollection")]
        #[method_id(@__retain_semantics Other elements)]
        pub unsafe fn elements(&self) -> Option<Id<DOMHTMLCollection>>;

        #[method(length)]
        pub unsafe fn length(&self) -> c_int;

        #[method(submit)]
        pub unsafe fn submit(&self);

        #[method(reset)]
        pub unsafe fn reset(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(feature = "WebKit_DOMHTMLFormElement")]
    unsafe impl DOMHTMLFormElement {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "WebKit_DOMHTMLFormElement")]
    unsafe impl DOMHTMLFormElement {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
