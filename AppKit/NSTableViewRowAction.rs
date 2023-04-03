//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    #[cfg(not(any(target_os = "ios")))]
    pub enum NSTableViewRowActionStyle {
        #[cfg(not(any(target_os = "ios")))]
        NSTableViewRowActionStyleRegular = 0,
        #[cfg(not(any(target_os = "ios")))]
        NSTableViewRowActionStyleDestructive = 1,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSTableViewRowAction")]
    #[cfg(not(any(target_os = "ios")))]
    pub struct NSTableViewRowAction;

    #[cfg(feature = "AppKit_NSTableViewRowAction")]
    unsafe impl ClassType for NSTableViewRowAction {
        type Super = NSObject;
    }
);

#[cfg(feature = "AppKit_NSTableViewRowAction")]
unsafe impl NSObjectProtocol for NSTableViewRowAction {}

extern_methods!(
    #[cfg(feature = "AppKit_NSTableViewRowAction")]
    unsafe impl NSTableViewRowAction {
        #[cfg(feature = "Foundation_NSString")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other rowActionWithStyle:title:handler:)]
        pub unsafe fn rowActionWithStyle_title_handler(
            style: NSTableViewRowActionStyle,
            title: &NSString,
            handler: &Block<(NonNull<NSTableViewRowAction>, NSInteger), ()>,
        ) -> Id<Self>;

        #[cfg(not(any(target_os = "ios")))]
        #[method(style)]
        pub unsafe fn style(&self) -> NSTableViewRowActionStyle;

        #[cfg(feature = "Foundation_NSString")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[cfg(not(any(target_os = "ios")))]
        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: &NSString);

        #[cfg(feature = "AppKit_NSColor")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other backgroundColor)]
        pub unsafe fn backgroundColor(&self) -> Id<NSColor>;

        #[cfg(feature = "AppKit_NSColor")]
        #[cfg(not(any(target_os = "ios")))]
        #[method(setBackgroundColor:)]
        pub unsafe fn setBackgroundColor(&self, background_color: Option<&NSColor>);

        #[cfg(feature = "AppKit_NSImage")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other image)]
        pub unsafe fn image(&self) -> Option<Id<NSImage>>;

        #[cfg(feature = "AppKit_NSImage")]
        #[cfg(not(any(target_os = "ios")))]
        #[method(setImage:)]
        pub unsafe fn setImage(&self, image: Option<&NSImage>);
    }
);
