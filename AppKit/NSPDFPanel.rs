//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_options!(
    #[underlying(NSInteger)]
    pub enum NSPDFPanelOptions {
        NSPDFPanelShowsPaperSize = 1 << 2,
        NSPDFPanelShowsOrientation = 1 << 3,
        NSPDFPanelRequestsParentDirectory = 1 << 24,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSPDFPanel")]
    pub struct NSPDFPanel;

    #[cfg(feature = "AppKit_NSPDFPanel")]
    unsafe impl ClassType for NSPDFPanel {
        type Super = NSObject;
    }
);

#[cfg(feature = "AppKit_NSPDFPanel")]
unsafe impl NSObjectProtocol for NSPDFPanel {}

extern_methods!(
    #[cfg(feature = "AppKit_NSPDFPanel")]
    unsafe impl NSPDFPanel {
        #[method_id(@__retain_semantics Other panel)]
        pub unsafe fn panel() -> Id<NSPDFPanel>;

        #[cfg(feature = "AppKit_NSViewController")]
        /**
          Controller for the accessory views that will be presented in the panel by the methods below. When the panel is presented to the user the controller is automatically sent a -setRepresentedObject: message with an NSPDFInfo equivalent to those one passed to -beginSheetWithPDFInfo:modalForWindow:completionHandler:.
        */
        #[method_id(@__retain_semantics Other accessoryController)]
        pub unsafe fn accessoryController(&self) -> Option<Id<NSViewController>>;

        #[cfg(feature = "AppKit_NSViewController")]
        /**
          Controller for the accessory views that will be presented in the panel by the methods below. When the panel is presented to the user the controller is automatically sent a -setRepresentedObject: message with an NSPDFInfo equivalent to those one passed to -beginSheetWithPDFInfo:modalForWindow:completionHandler:.
        */
        #[method(setAccessoryController:)]
        pub unsafe fn setAccessoryController(
            &self,
            accessory_controller: Option<&NSViewController>,
        );

        /**
          The options described above. The default value is 0. To allow your application to take advantage of controls that may be added by default in future versions OS X, get the options from the panel you've just created, turn on and off the flags you care about, and then set the options.
        */
        #[method(options)]
        pub unsafe fn options(&self) -> NSPDFPanelOptions;

        /**
          The options described above. The default value is 0. To allow your application to take advantage of controls that may be added by default in future versions OS X, get the options from the panel you've just created, turn on and off the flags you care about, and then set the options.
        */
        #[method(setOptions:)]
        pub unsafe fn setOptions(&self, options: NSPDFPanelOptions);

        #[cfg(feature = "Foundation_NSString")]
        /**
           The user-editable file name shown in the name field. The default value is the equivalent of 'Untitled' for the current locale. Note that this string should never include a file extension.
        */
        #[method_id(@__retain_semantics Other defaultFileName)]
        pub unsafe fn defaultFileName(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        /**
           The user-editable file name shown in the name field. The default value is the equivalent of 'Untitled' for the current locale. Note that this string should never include a file extension.
        */
        #[method(setDefaultFileName:)]
        pub unsafe fn setDefaultFileName(&self, default_file_name: &NSString);

        #[cfg(all(feature = "AppKit_NSPDFInfo", feature = "AppKit_NSWindow"))]
        #[method(beginSheetWithPDFInfo:modalForWindow:completionHandler:)]
        pub unsafe fn beginSheetWithPDFInfo_modalForWindow_completionHandler(
            &self,
            pdf_info: &NSPDFInfo,
            doc_window: Option<&NSWindow>,
            completion_handler: &Block<(NSInteger,), ()>,
        );
    }
);
