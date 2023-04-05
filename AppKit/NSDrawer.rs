//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSDrawerState {
        #[deprecated = "Drawers are deprecated; consider using NSSplitViewController"]
        NSDrawerClosedState = 0,
        #[deprecated = "Drawers are deprecated; consider using NSSplitViewController"]
        NSDrawerOpeningState = 1,
        #[deprecated = "Drawers are deprecated; consider using NSSplitViewController"]
        NSDrawerOpenState = 2,
        #[deprecated = "Drawers are deprecated; consider using NSSplitViewController"]
        NSDrawerClosingState = 3,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSDrawer")]
    #[deprecated = "Drawers are deprecated; consider using NSSplitViewController"]
    pub struct NSDrawer;

    #[cfg(feature = "AppKit_NSDrawer")]
    unsafe impl ClassType for NSDrawer {
        #[inherits(NSObject)]
        type Super = NSResponder;
    }
);

#[cfg(feature = "AppKit_NSDrawer")]
unsafe impl NSAccessibility for NSDrawer {}

#[cfg(feature = "AppKit_NSDrawer")]
unsafe impl NSAccessibilityElementProtocol for NSDrawer {}

#[cfg(feature = "AppKit_NSDrawer")]
unsafe impl NSCoding for NSDrawer {}

#[cfg(feature = "AppKit_NSDrawer")]
unsafe impl NSObjectProtocol for NSDrawer {}

extern_methods!(
    #[cfg(feature = "AppKit_NSDrawer")]
    unsafe impl NSDrawer {
        #[method_id(@__retain_semantics Init initWithContentSize:preferredEdge:)]
        pub unsafe fn initWithContentSize_preferredEdge(
            this: Option<Allocated<Self>>,
            content_size: NSSize,
            edge: NSRectEdge,
        ) -> Id<Self>;

        #[cfg(feature = "AppKit_NSWindow")]
        #[method_id(@__retain_semantics Other parentWindow)]
        pub unsafe fn parentWindow(&self) -> Option<Id<NSWindow>>;

        #[cfg(feature = "AppKit_NSWindow")]
        #[method(setParentWindow:)]
        pub unsafe fn setParentWindow(&self, parent_window: Option<&NSWindow>);

        #[cfg(feature = "AppKit_NSView")]
        #[method_id(@__retain_semantics Other contentView)]
        pub unsafe fn contentView(&self) -> Option<Id<NSView>>;

        #[cfg(feature = "AppKit_NSView")]
        #[method(setContentView:)]
        pub unsafe fn setContentView(&self, content_view: Option<&NSView>);

        #[method(preferredEdge)]
        pub unsafe fn preferredEdge(&self) -> NSRectEdge;

        #[method(setPreferredEdge:)]
        pub unsafe fn setPreferredEdge(&self, preferred_edge: NSRectEdge);

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<ProtocolObject<dyn NSDrawerDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&ProtocolObject<dyn NSDrawerDelegate>>);

        #[method(open)]
        pub unsafe fn open(&self);

        #[method(openOnEdge:)]
        pub unsafe fn openOnEdge(&self, edge: NSRectEdge);

        #[method(close)]
        pub unsafe fn close(&self);

        #[method(open:)]
        pub unsafe fn open_(&self, sender: Option<&Object>);

        #[method(close:)]
        pub unsafe fn close_(&self, sender: Option<&Object>);

        #[method(toggle:)]
        pub unsafe fn toggle(&self, sender: Option<&Object>);

        #[method(state)]
        pub unsafe fn state(&self) -> NSInteger;

        #[method(edge)]
        pub unsafe fn edge(&self) -> NSRectEdge;

        #[method(contentSize)]
        pub unsafe fn contentSize(&self) -> NSSize;

        #[method(setContentSize:)]
        pub unsafe fn setContentSize(&self, content_size: NSSize);

        #[method(minContentSize)]
        pub unsafe fn minContentSize(&self) -> NSSize;

        #[method(setMinContentSize:)]
        pub unsafe fn setMinContentSize(&self, min_content_size: NSSize);

        #[method(maxContentSize)]
        pub unsafe fn maxContentSize(&self) -> NSSize;

        #[method(setMaxContentSize:)]
        pub unsafe fn setMaxContentSize(&self, max_content_size: NSSize);

        #[method(leadingOffset)]
        pub unsafe fn leadingOffset(&self) -> CGFloat;

        #[method(setLeadingOffset:)]
        pub unsafe fn setLeadingOffset(&self, leading_offset: CGFloat);

        #[method(trailingOffset)]
        pub unsafe fn trailingOffset(&self) -> CGFloat;

        #[method(setTrailingOffset:)]
        pub unsafe fn setTrailingOffset(&self, trailing_offset: CGFloat);
    }
);

extern_methods!(
    /// NSDrawers
    #[cfg(feature = "AppKit_NSWindow")]
    unsafe impl NSWindow {
        #[cfg(all(feature = "AppKit_NSDrawer", feature = "Foundation_NSArray"))]
        #[deprecated = "Drawers are deprecated; consider using NSSplitViewController"]
        #[method_id(@__retain_semantics Other drawers)]
        pub unsafe fn drawers(&self) -> Option<Id<NSArray<NSDrawer>>>;
    }
);

extern_protocol!(
    pub unsafe trait NSDrawerDelegate: NSObjectProtocol {
        #[cfg(feature = "AppKit_NSDrawer")]
        #[deprecated = "Drawers are deprecated; consider using NSSplitViewController"]
        #[optional]
        #[method(drawerShouldOpen:)]
        unsafe fn drawerShouldOpen(&self, sender: &NSDrawer) -> bool;

        #[cfg(feature = "AppKit_NSDrawer")]
        #[deprecated = "Drawers are deprecated; consider using NSSplitViewController"]
        #[optional]
        #[method(drawerShouldClose:)]
        unsafe fn drawerShouldClose(&self, sender: &NSDrawer) -> bool;

        #[cfg(feature = "AppKit_NSDrawer")]
        #[deprecated = "Drawers are deprecated; consider using NSSplitViewController"]
        #[optional]
        #[method(drawerWillResizeContents:toSize:)]
        unsafe fn drawerWillResizeContents_toSize(
            &self,
            sender: &NSDrawer,
            content_size: NSSize,
        ) -> NSSize;

        #[cfg(feature = "Foundation_NSNotification")]
        #[deprecated = "Drawers are deprecated; consider using NSSplitViewController"]
        #[optional]
        #[method(drawerWillOpen:)]
        unsafe fn drawerWillOpen(&self, notification: &NSNotification);

        #[cfg(feature = "Foundation_NSNotification")]
        #[deprecated = "Drawers are deprecated; consider using NSSplitViewController"]
        #[optional]
        #[method(drawerDidOpen:)]
        unsafe fn drawerDidOpen(&self, notification: &NSNotification);

        #[cfg(feature = "Foundation_NSNotification")]
        #[deprecated = "Drawers are deprecated; consider using NSSplitViewController"]
        #[optional]
        #[method(drawerWillClose:)]
        unsafe fn drawerWillClose(&self, notification: &NSNotification);

        #[cfg(feature = "Foundation_NSNotification")]
        #[deprecated = "Drawers are deprecated; consider using NSSplitViewController"]
        #[optional]
        #[method(drawerDidClose:)]
        unsafe fn drawerDidClose(&self, notification: &NSNotification);
    }

    unsafe impl ProtocolType for dyn NSDrawerDelegate {}
);

#[deprecated = "Drawers are deprecated; consider using NSSplitViewController"]
extern_static!(NSDrawerWillOpenNotification: &'static NSNotificationName);

#[deprecated = "Drawers are deprecated; consider using NSSplitViewController"]
extern_static!(NSDrawerDidOpenNotification: &'static NSNotificationName);

#[deprecated = "Drawers are deprecated; consider using NSSplitViewController"]
extern_static!(NSDrawerWillCloseNotification: &'static NSNotificationName);

#[deprecated = "Drawers are deprecated; consider using NSSplitViewController"]
extern_static!(NSDrawerDidCloseNotification: &'static NSNotificationName);
