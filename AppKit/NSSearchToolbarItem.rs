//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSSearchToolbarItem")]
    #[cfg(not(any(target_os = "ios")))]
    pub struct NSSearchToolbarItem;

    #[cfg(feature = "AppKit_NSSearchToolbarItem")]
    unsafe impl ClassType for NSSearchToolbarItem {
        #[inherits(NSObject)]
        type Super = NSToolbarItem;
    }
);

#[cfg(feature = "AppKit_NSSearchToolbarItem")]
unsafe impl NSObjectProtocol for NSSearchToolbarItem {}

extern_methods!(
    #[cfg(feature = "AppKit_NSSearchToolbarItem")]
    unsafe impl NSSearchToolbarItem {
        #[cfg(feature = "AppKit_NSSearchField")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other searchField)]
        pub unsafe fn searchField(&self) -> Id<NSSearchField>;

        #[cfg(feature = "AppKit_NSSearchField")]
        #[cfg(not(any(target_os = "ios")))]
        #[method(setSearchField:)]
        pub unsafe fn setSearchField(&self, search_field: &NSSearchField);

        #[cfg(feature = "AppKit_NSView")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other view)]
        pub unsafe fn view(&self) -> Option<Id<NSView>>;

        #[cfg(feature = "AppKit_NSView")]
        #[cfg(not(any(target_os = "ios")))]
        #[method(setView:)]
        pub unsafe fn setView(&self, view: Option<&NSView>);

        #[cfg(not(any(target_os = "ios")))]
        #[method(resignsFirstResponderWithCancel)]
        pub unsafe fn resignsFirstResponderWithCancel(&self) -> bool;

        #[cfg(not(any(target_os = "ios")))]
        #[method(setResignsFirstResponderWithCancel:)]
        pub unsafe fn setResignsFirstResponderWithCancel(
            &self,
            resigns_first_responder_with_cancel: bool,
        );

        #[cfg(not(any(target_os = "ios")))]
        #[method(preferredWidthForSearchField)]
        pub unsafe fn preferredWidthForSearchField(&self) -> CGFloat;

        #[cfg(not(any(target_os = "ios")))]
        #[method(setPreferredWidthForSearchField:)]
        pub unsafe fn setPreferredWidthForSearchField(
            &self,
            preferred_width_for_search_field: CGFloat,
        );

        #[cfg(not(any(target_os = "ios")))]
        #[method(beginSearchInteraction)]
        pub unsafe fn beginSearchInteraction(&self);

        #[cfg(not(any(target_os = "ios")))]
        #[method(endSearchInteraction)]
        pub unsafe fn endSearchInteraction(&self);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSToolbarItem`
    #[cfg(feature = "AppKit_NSSearchToolbarItem")]
    unsafe impl NSSearchToolbarItem {
        #[method_id(@__retain_semantics Init initWithItemIdentifier:)]
        pub unsafe fn initWithItemIdentifier(
            this: Option<Allocated<Self>>,
            item_identifier: &NSToolbarItemIdentifier,
        ) -> Id<Self>;
    }
);
