//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_static!(NSVariableStatusItemLength: CGFloat = -1.0);

extern_static!(NSSquareStatusItemLength: CGFloat = -2.0);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSStatusBar")]
    pub struct NSStatusBar;

    #[cfg(feature = "AppKit_NSStatusBar")]
    unsafe impl ClassType for NSStatusBar {
        type Super = NSObject;
    }
);

#[cfg(feature = "AppKit_NSStatusBar")]
unsafe impl NSObjectProtocol for NSStatusBar {}

extern_methods!(
    #[cfg(feature = "AppKit_NSStatusBar")]
    unsafe impl NSStatusBar {
        #[method_id(@__retain_semantics Other systemStatusBar)]
        pub unsafe fn systemStatusBar() -> Id<NSStatusBar>;

        #[cfg(feature = "AppKit_NSStatusItem")]
        #[method_id(@__retain_semantics Other statusItemWithLength:)]
        pub unsafe fn statusItemWithLength(&self, length: CGFloat) -> Id<NSStatusItem>;

        #[cfg(feature = "AppKit_NSStatusItem")]
        #[method(removeStatusItem:)]
        pub unsafe fn removeStatusItem(&self, item: &NSStatusItem);

        #[method(isVertical)]
        pub unsafe fn isVertical(&self) -> bool;

        #[method(thickness)]
        pub unsafe fn thickness(&self) -> CGFloat;
    }
);
