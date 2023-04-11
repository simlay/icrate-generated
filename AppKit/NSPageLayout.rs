//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSPageLayout")]
    pub struct NSPageLayout;

    #[cfg(feature = "AppKit_NSPageLayout")]
    unsafe impl ClassType for NSPageLayout {
        type Super = NSObject;
    }
);

#[cfg(feature = "AppKit_NSPageLayout")]
unsafe impl NSObjectProtocol for NSPageLayout {}

extern_methods!(
    #[cfg(feature = "AppKit_NSPageLayout")]
    unsafe impl NSPageLayout {
        #[method_id(@__retain_semantics Other pageLayout)]
        pub unsafe fn pageLayout() -> Id<NSPageLayout>;

        #[cfg(feature = "AppKit_NSViewController")]
        #[method(addAccessoryController:)]
        pub unsafe fn addAccessoryController(&self, accessory_controller: &NSViewController);

        #[cfg(feature = "AppKit_NSViewController")]
        #[method(removeAccessoryController:)]
        pub unsafe fn removeAccessoryController(&self, accessory_controller: &NSViewController);

        #[cfg(all(feature = "AppKit_NSViewController", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other accessoryControllers)]
        pub unsafe fn accessoryControllers(&self) -> Id<NSArray<NSViewController>>;

        #[cfg(all(feature = "AppKit_NSPrintInfo", feature = "AppKit_NSWindow"))]
        #[method(beginSheetWithPrintInfo:modalForWindow:delegate:didEndSelector:contextInfo:)]
        pub unsafe fn beginSheetWithPrintInfo_modalForWindow_delegate_didEndSelector_contextInfo(
            &self,
            print_info: &NSPrintInfo,
            doc_window: &NSWindow,
            delegate: Option<&Object>,
            did_end_selector: Option<Sel>,
            context_info: *mut c_void,
        );

        #[cfg(feature = "AppKit_NSPrintInfo")]
        #[method(runModalWithPrintInfo:)]
        pub unsafe fn runModalWithPrintInfo(&self, print_info: &NSPrintInfo) -> NSInteger;

        #[method(runModal)]
        pub unsafe fn runModal(&self) -> NSInteger;

        #[cfg(feature = "AppKit_NSPrintInfo")]
        /**
          A simple accessor. Your -beginSheetWithPrintInfo:... delegate can use this so it doesn't have to keep a pointer to the NSPrintInfo elsewhere while waiting for the user to dismiss the print panel.
        */
        #[method_id(@__retain_semantics Other printInfo)]
        pub unsafe fn printInfo(&self) -> Option<Id<NSPrintInfo>>;
    }
);

extern_methods!(
    /// NSDeprecated
    #[cfg(feature = "AppKit_NSPageLayout")]
    unsafe impl NSPageLayout {
        #[cfg(feature = "AppKit_NSView")]
        #[deprecated]
        #[method(setAccessoryView:)]
        pub unsafe fn setAccessoryView(&self, accessory_view: Option<&NSView>);

        #[cfg(feature = "AppKit_NSView")]
        #[deprecated]
        #[method_id(@__retain_semantics Other accessoryView)]
        pub unsafe fn accessoryView(&self) -> Option<Id<NSView>>;

        #[deprecated]
        #[method(readPrintInfo)]
        pub unsafe fn readPrintInfo(&self);

        #[deprecated]
        #[method(writePrintInfo)]
        pub unsafe fn writePrintInfo(&self);
    }
);

extern_methods!(
    /// NSPageLayoutPanel
    #[cfg(feature = "AppKit_NSApplication")]
    unsafe impl NSApplication {
        #[method(runPageLayout:)]
        pub unsafe fn runPageLayout(&self, sender: Option<&Object>);
    }
);
