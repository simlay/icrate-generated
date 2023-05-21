//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMHTMLOptionsCollection")]
    #[deprecated]
    pub struct DOMHTMLOptionsCollection;

    #[deprecated]
    #[cfg(feature = "WebKit_DOMHTMLOptionsCollection")]
    unsafe impl ClassType for DOMHTMLOptionsCollection {
        #[inherits(WebScriptObject, NSObject)]
        type Super = DOMObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "WebKit_DOMHTMLOptionsCollection")]
unsafe impl NSCopying for DOMHTMLOptionsCollection {}

#[cfg(feature = "WebKit_DOMHTMLOptionsCollection")]
unsafe impl NSObjectProtocol for DOMHTMLOptionsCollection {}

extern_methods!(
    #[cfg(feature = "WebKit_DOMHTMLOptionsCollection")]
    unsafe impl DOMHTMLOptionsCollection {
        #[method(selectedIndex)]
        pub unsafe fn selectedIndex(&self) -> c_int;

        #[method(setSelectedIndex:)]
        pub unsafe fn setSelectedIndex(&self, selected_index: c_int);

        #[method(length)]
        pub unsafe fn length(&self) -> c_uint;

        #[method(setLength:)]
        pub unsafe fn setLength(&self, length: c_uint);

        #[cfg(all(feature = "Foundation_NSString", feature = "WebKit_DOMNode"))]
        #[method_id(@__retain_semantics Other namedItem:)]
        pub unsafe fn namedItem(&self, name: Option<&NSString>) -> Option<Id<DOMNode>>;

        #[cfg(feature = "WebKit_DOMHTMLOptionElement")]
        #[method(add:index:)]
        pub unsafe fn add_index(&self, option: Option<&DOMHTMLOptionElement>, index: c_uint);

        #[method(remove:)]
        pub unsafe fn remove(&self, index: c_uint);

        #[cfg(feature = "WebKit_DOMNode")]
        #[method_id(@__retain_semantics Other item:)]
        pub unsafe fn item(&self, index: c_uint) -> Option<Id<DOMNode>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `DOMObject`
    #[cfg(feature = "WebKit_DOMHTMLOptionsCollection")]
    unsafe impl DOMHTMLOptionsCollection {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "WebKit_DOMHTMLOptionsCollection")]
    unsafe impl DOMHTMLOptionsCollection {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
