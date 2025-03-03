//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSMenuToolbarItem")]
    pub struct NSMenuToolbarItem;

    #[cfg(feature = "AppKit_NSMenuToolbarItem")]
    unsafe impl ClassType for NSMenuToolbarItem {
        #[inherits(NSObject)]
        type Super = NSToolbarItem;
    }
);

#[cfg(feature = "AppKit_NSMenuToolbarItem")]
unsafe impl NSObjectProtocol for NSMenuToolbarItem {}

extern_methods!(
    #[cfg(feature = "AppKit_NSMenuToolbarItem")]
    unsafe impl NSMenuToolbarItem {
        #[cfg(feature = "AppKit_NSMenu")]
        #[method_id(@__retain_semantics Other menu)]
        pub unsafe fn menu(&self) -> Id<NSMenu>;

        #[cfg(feature = "AppKit_NSMenu")]
        #[method(setMenu:)]
        pub unsafe fn setMenu(&self, menu: &NSMenu);

        #[method(showsIndicator)]
        pub unsafe fn showsIndicator(&self) -> bool;

        #[method(setShowsIndicator:)]
        pub unsafe fn setShowsIndicator(&self, shows_indicator: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSToolbarItem`
    #[cfg(feature = "AppKit_NSMenuToolbarItem")]
    unsafe impl NSMenuToolbarItem {
        #[method_id(@__retain_semantics Init initWithItemIdentifier:)]
        pub unsafe fn initWithItemIdentifier(
            this: Option<Allocated<Self>>,
            item_identifier: &NSToolbarItemIdentifier,
        ) -> Id<Self>;
    }
);
