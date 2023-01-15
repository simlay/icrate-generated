//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSDrawerState {
        NSDrawerClosedState = 0,
        NSDrawerOpeningState = 1,
        NSDrawerOpenState = 2,
        NSDrawerClosingState = 3,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSDrawer")]
    pub struct NSDrawer;

    #[cfg(feature = "AppKit_NSDrawer")]
    unsafe impl ClassType for NSDrawer {
        #[inherits(NSObject)]
        type Super = NSResponder;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSDrawer")]
    unsafe impl NSDrawer {
        #[method_id(@__retain_semantics Init initWithContentSize:preferredEdge:)]
        pub unsafe fn initWithContentSize_preferredEdge(
            this: Option<Allocated<Self>>,
            contentSize: NSSize,
            edge: NSRectEdge,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "AppKit_NSWindow")]
        #[method_id(@__retain_semantics Other parentWindow)]
        pub unsafe fn parentWindow(&self) -> Option<Id<NSWindow, Shared>>;

        #[cfg(feature = "AppKit_NSWindow")]
        #[method(setParentWindow:)]
        pub unsafe fn setParentWindow(&self, parentWindow: Option<&NSWindow>);

        #[cfg(feature = "AppKit_NSView")]
        #[method_id(@__retain_semantics Other contentView)]
        pub unsafe fn contentView(&self) -> Option<Id<NSView, Shared>>;

        #[cfg(feature = "AppKit_NSView")]
        #[method(setContentView:)]
        pub unsafe fn setContentView(&self, contentView: Option<&NSView>);

        #[method(preferredEdge)]
        pub unsafe fn preferredEdge(&self) -> NSRectEdge;

        #[method(setPreferredEdge:)]
        pub unsafe fn setPreferredEdge(&self, preferredEdge: NSRectEdge);

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSDrawerDelegate, Shared>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSDrawerDelegate>);

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
        pub unsafe fn setContentSize(&self, contentSize: NSSize);

        #[method(minContentSize)]
        pub unsafe fn minContentSize(&self) -> NSSize;

        #[method(setMinContentSize:)]
        pub unsafe fn setMinContentSize(&self, minContentSize: NSSize);

        #[method(maxContentSize)]
        pub unsafe fn maxContentSize(&self) -> NSSize;

        #[method(setMaxContentSize:)]
        pub unsafe fn setMaxContentSize(&self, maxContentSize: NSSize);

        #[method(leadingOffset)]
        pub unsafe fn leadingOffset(&self) -> CGFloat;

        #[method(setLeadingOffset:)]
        pub unsafe fn setLeadingOffset(&self, leadingOffset: CGFloat);

        #[method(trailingOffset)]
        pub unsafe fn trailingOffset(&self) -> CGFloat;

        #[method(setTrailingOffset:)]
        pub unsafe fn setTrailingOffset(&self, trailingOffset: CGFloat);
    }
);

extern_methods!(
    /// NSDrawers
    #[cfg(feature = "AppKit_NSWindow")]
    unsafe impl NSWindow {
        #[cfg(all(feature = "AppKit_NSDrawer", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other drawers)]
        pub unsafe fn drawers(&self) -> Option<Id<NSArray<NSDrawer>, Shared>>;
    }
);

extern_protocol!(
    pub struct NSDrawerDelegate;

    unsafe impl ProtocolType for NSDrawerDelegate {
        #[cfg(feature = "AppKit_NSDrawer")]
        #[optional]
        #[method(drawerShouldOpen:)]
        pub unsafe fn drawerShouldOpen(&self, sender: &NSDrawer) -> bool;

        #[cfg(feature = "AppKit_NSDrawer")]
        #[optional]
        #[method(drawerShouldClose:)]
        pub unsafe fn drawerShouldClose(&self, sender: &NSDrawer) -> bool;

        #[cfg(feature = "AppKit_NSDrawer")]
        #[optional]
        #[method(drawerWillResizeContents:toSize:)]
        pub unsafe fn drawerWillResizeContents_toSize(
            &self,
            sender: &NSDrawer,
            contentSize: NSSize,
        ) -> NSSize;

        #[cfg(feature = "Foundation_NSNotification")]
        #[optional]
        #[method(drawerWillOpen:)]
        pub unsafe fn drawerWillOpen(&self, notification: &NSNotification);

        #[cfg(feature = "Foundation_NSNotification")]
        #[optional]
        #[method(drawerDidOpen:)]
        pub unsafe fn drawerDidOpen(&self, notification: &NSNotification);

        #[cfg(feature = "Foundation_NSNotification")]
        #[optional]
        #[method(drawerWillClose:)]
        pub unsafe fn drawerWillClose(&self, notification: &NSNotification);

        #[cfg(feature = "Foundation_NSNotification")]
        #[optional]
        #[method(drawerDidClose:)]
        pub unsafe fn drawerDidClose(&self, notification: &NSNotification);
    }
);

extern_static!(NSDrawerWillOpenNotification: &'static NSNotificationName);

extern_static!(NSDrawerDidOpenNotification: &'static NSNotificationName);

extern_static!(NSDrawerWillCloseNotification: &'static NSNotificationName);

extern_static!(NSDrawerDidCloseNotification: &'static NSNotificationName);
