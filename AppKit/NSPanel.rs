//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSPanel")]
    pub struct NSPanel;

    #[cfg(feature = "AppKit_NSPanel")]
    unsafe impl ClassType for NSPanel {
        #[inherits(NSResponder, NSObject)]
        type Super = NSWindow;
    }
);

#[cfg(feature = "AppKit_NSPanel")]
unsafe impl NSAccessibility for NSPanel {}

#[cfg(feature = "AppKit_NSPanel")]
unsafe impl NSAccessibilityElementProtocol for NSPanel {}

#[cfg(feature = "AppKit_NSPanel")]
unsafe impl NSAnimatablePropertyContainer for NSPanel {}

#[cfg(feature = "AppKit_NSPanel")]
unsafe impl NSAppearanceCustomization for NSPanel {}

#[cfg(feature = "AppKit_NSPanel")]
unsafe impl NSCoding for NSPanel {}

#[cfg(feature = "AppKit_NSPanel")]
unsafe impl NSMenuItemValidation for NSPanel {}

#[cfg(feature = "AppKit_NSPanel")]
unsafe impl NSObjectProtocol for NSPanel {}

#[cfg(feature = "AppKit_NSPanel")]
unsafe impl NSUserInterfaceItemIdentification for NSPanel {}

#[cfg(feature = "AppKit_NSPanel")]
unsafe impl NSUserInterfaceValidations for NSPanel {}

extern_methods!(
    #[cfg(feature = "AppKit_NSPanel")]
    unsafe impl NSPanel {
        #[method(isFloatingPanel)]
        pub unsafe fn isFloatingPanel(&self) -> bool;

        #[method(setFloatingPanel:)]
        pub unsafe fn setFloatingPanel(&self, floating_panel: bool);

        #[method(becomesKeyOnlyIfNeeded)]
        pub unsafe fn becomesKeyOnlyIfNeeded(&self) -> bool;

        #[method(setBecomesKeyOnlyIfNeeded:)]
        pub unsafe fn setBecomesKeyOnlyIfNeeded(&self, becomes_key_only_if_needed: bool);

        #[method(worksWhenModal)]
        pub unsafe fn worksWhenModal(&self) -> bool;

        #[method(setWorksWhenModal:)]
        pub unsafe fn setWorksWhenModal(&self, works_when_modal: bool);
    }
);

extern_fn!(
    #[deprecated = "Use NSAlert instead"]
    pub unsafe fn NSReleaseAlertPanel(panel: Option<&Object>);
);

extern_enum!(
    #[underlying(c_int)]
    /**
     These constants are used by deprecated NSRunAlertPanel() and NSGetAlertPanel() functions.
    Modern NSAlert API uses NSAlertFirstButtonReturn, etc.
    */
    pub enum __anonymous__ {
        #[deprecated = "Use NSAlertFirstButtonReturn with an NSAlert presentation instead"]
        NSAlertDefaultReturn = 1,
        #[deprecated = "Use NSAlertFirstButtonReturn and other NSModalResponses with an NSAlert presentation instead"]
        NSAlertAlternateReturn = 0,
        #[deprecated = "Use NSAlertFirstButtonReturn and other NSModalResponses with an NSAlert presentation instead"]
        NSAlertOtherReturn = -1,
        #[deprecated = "Use NSAlertFirstButtonReturn and other NSModalResponses with an NSAlert presentation instead"]
        NSAlertErrorReturn = -2,
    }
);

extern_methods!(
    /// Methods declared on superclass `NSWindow`
    #[cfg(feature = "AppKit_NSPanel")]
    unsafe impl NSPanel {
        #[method_id(@__retain_semantics Init initWithContentRect:styleMask:backing:defer:)]
        pub unsafe fn initWithContentRect_styleMask_backing_defer(
            this: Option<Allocated<Self>>,
            content_rect: NSRect,
            style: NSWindowStyleMask,
            backing_store_type: NSBackingStoreType,
            flag: bool,
        ) -> Id<Self>;

        #[cfg(feature = "AppKit_NSScreen")]
        #[method_id(@__retain_semantics Init initWithContentRect:styleMask:backing:defer:screen:)]
        pub unsafe fn initWithContentRect_styleMask_backing_defer_screen(
            this: Option<Allocated<Self>>,
            content_rect: NSRect,
            style: NSWindowStyleMask,
            backing_store_type: NSBackingStoreType,
            flag: bool,
            screen: Option<&NSScreen>,
        ) -> Id<Self>;

        #[cfg(feature = "AppKit_NSViewController")]
        #[method_id(@__retain_semantics Other windowWithContentViewController:)]
        pub unsafe fn windowWithContentViewController(
            content_view_controller: &NSViewController,
        ) -> Id<Self>;
    }
);
