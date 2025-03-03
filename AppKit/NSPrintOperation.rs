//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSPrintingPageOrder {
        NSDescendingPageOrder = -1,
        NSSpecialPageOrder = 0,
        NSAscendingPageOrder = 1,
        NSUnknownPageOrder = 2,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSPrintRenderingQuality {
        NSPrintRenderingQualityBest = 0,
        NSPrintRenderingQualityResponsive = 1,
    }
);

extern_static!(NSPrintOperationExistsException: &'static NSExceptionName);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSPrintOperation")]
    pub struct NSPrintOperation;

    #[cfg(feature = "AppKit_NSPrintOperation")]
    unsafe impl ClassType for NSPrintOperation {
        type Super = NSObject;
    }
);

#[cfg(feature = "AppKit_NSPrintOperation")]
unsafe impl NSObjectProtocol for NSPrintOperation {}

extern_methods!(
    #[cfg(feature = "AppKit_NSPrintOperation")]
    unsafe impl NSPrintOperation {
        #[cfg(all(feature = "AppKit_NSPrintInfo", feature = "AppKit_NSView"))]
        #[method_id(@__retain_semantics Other printOperationWithView:printInfo:)]
        pub unsafe fn printOperationWithView_printInfo(
            view: &NSView,
            print_info: &NSPrintInfo,
        ) -> Id<NSPrintOperation>;

        #[cfg(all(
            feature = "AppKit_NSPrintInfo",
            feature = "AppKit_NSView",
            feature = "Foundation_NSMutableData"
        ))]
        #[method_id(@__retain_semantics Other PDFOperationWithView:insideRect:toData:printInfo:)]
        pub unsafe fn PDFOperationWithView_insideRect_toData_printInfo(
            view: &NSView,
            rect: NSRect,
            data: &NSMutableData,
            print_info: &NSPrintInfo,
        ) -> Id<NSPrintOperation>;

        #[cfg(all(
            feature = "AppKit_NSPrintInfo",
            feature = "AppKit_NSView",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other PDFOperationWithView:insideRect:toPath:printInfo:)]
        pub unsafe fn PDFOperationWithView_insideRect_toPath_printInfo(
            view: &NSView,
            rect: NSRect,
            path: &NSString,
            print_info: &NSPrintInfo,
        ) -> Id<NSPrintOperation>;

        #[cfg(all(
            feature = "AppKit_NSPrintInfo",
            feature = "AppKit_NSView",
            feature = "Foundation_NSMutableData"
        ))]
        #[method_id(@__retain_semantics Other EPSOperationWithView:insideRect:toData:printInfo:)]
        pub unsafe fn EPSOperationWithView_insideRect_toData_printInfo(
            view: &NSView,
            rect: NSRect,
            data: &NSMutableData,
            print_info: &NSPrintInfo,
        ) -> Id<NSPrintOperation>;

        #[cfg(all(
            feature = "AppKit_NSPrintInfo",
            feature = "AppKit_NSView",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other EPSOperationWithView:insideRect:toPath:printInfo:)]
        pub unsafe fn EPSOperationWithView_insideRect_toPath_printInfo(
            view: &NSView,
            rect: NSRect,
            path: &NSString,
            print_info: &NSPrintInfo,
        ) -> Id<NSPrintOperation>;

        #[cfg(feature = "AppKit_NSView")]
        #[method_id(@__retain_semantics Other printOperationWithView:)]
        pub unsafe fn printOperationWithView(view: &NSView) -> Id<NSPrintOperation>;

        #[cfg(all(feature = "AppKit_NSView", feature = "Foundation_NSMutableData"))]
        #[method_id(@__retain_semantics Other PDFOperationWithView:insideRect:toData:)]
        pub unsafe fn PDFOperationWithView_insideRect_toData(
            view: &NSView,
            rect: NSRect,
            data: &NSMutableData,
        ) -> Id<NSPrintOperation>;

        #[cfg(all(feature = "AppKit_NSView", feature = "Foundation_NSMutableData"))]
        #[method_id(@__retain_semantics Other EPSOperationWithView:insideRect:toData:)]
        pub unsafe fn EPSOperationWithView_insideRect_toData(
            view: &NSView,
            rect: NSRect,
            data: Option<&NSMutableData>,
        ) -> Id<NSPrintOperation>;

        #[method_id(@__retain_semantics Other currentOperation)]
        pub unsafe fn currentOperation() -> Option<Id<NSPrintOperation>>;

        #[method(setCurrentOperation:)]
        pub unsafe fn setCurrentOperation(current_operation: Option<&NSPrintOperation>);

        #[method(isCopyingOperation)]
        pub unsafe fn isCopyingOperation(&self) -> bool;

        #[method(preferredRenderingQuality)]
        pub unsafe fn preferredRenderingQuality(&self) -> NSPrintRenderingQuality;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other jobTitle)]
        pub unsafe fn jobTitle(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setJobTitle:)]
        pub unsafe fn setJobTitle(&self, job_title: Option<&NSString>);

        #[method(showsPrintPanel)]
        pub unsafe fn showsPrintPanel(&self) -> bool;

        #[method(setShowsPrintPanel:)]
        pub unsafe fn setShowsPrintPanel(&self, shows_print_panel: bool);

        #[method(showsProgressPanel)]
        pub unsafe fn showsProgressPanel(&self) -> bool;

        #[method(setShowsProgressPanel:)]
        pub unsafe fn setShowsProgressPanel(&self, shows_progress_panel: bool);

        #[cfg(feature = "AppKit_NSPrintPanel")]
        #[method_id(@__retain_semantics Other printPanel)]
        pub unsafe fn printPanel(&self) -> Id<NSPrintPanel>;

        #[cfg(feature = "AppKit_NSPrintPanel")]
        #[method(setPrintPanel:)]
        pub unsafe fn setPrintPanel(&self, print_panel: &NSPrintPanel);

        #[cfg(feature = "AppKit_NSPDFPanel")]
        #[method_id(@__retain_semantics Other PDFPanel)]
        pub unsafe fn PDFPanel(&self) -> Id<NSPDFPanel>;

        #[cfg(feature = "AppKit_NSPDFPanel")]
        #[method(setPDFPanel:)]
        pub unsafe fn setPDFPanel(&self, pdf_panel: &NSPDFPanel);

        #[method(canSpawnSeparateThread)]
        pub unsafe fn canSpawnSeparateThread(&self) -> bool;

        #[method(setCanSpawnSeparateThread:)]
        pub unsafe fn setCanSpawnSeparateThread(&self, can_spawn_separate_thread: bool);

        #[method(pageOrder)]
        pub unsafe fn pageOrder(&self) -> NSPrintingPageOrder;

        #[method(setPageOrder:)]
        pub unsafe fn setPageOrder(&self, page_order: NSPrintingPageOrder);

        #[cfg(feature = "AppKit_NSWindow")]
        #[method(runOperationModalForWindow:delegate:didRunSelector:contextInfo:)]
        pub unsafe fn runOperationModalForWindow_delegate_didRunSelector_contextInfo(
            &self,
            doc_window: &NSWindow,
            delegate: Option<&Object>,
            did_run_selector: Option<Sel>,
            context_info: *mut c_void,
        );

        #[method(runOperation)]
        pub unsafe fn runOperation(&self) -> bool;

        #[cfg(feature = "AppKit_NSView")]
        #[method_id(@__retain_semantics Other view)]
        pub unsafe fn view(&self) -> Option<Id<NSView>>;

        #[cfg(feature = "AppKit_NSPrintInfo")]
        #[method_id(@__retain_semantics Other printInfo)]
        pub unsafe fn printInfo(&self) -> Id<NSPrintInfo>;

        #[cfg(feature = "AppKit_NSPrintInfo")]
        #[method(setPrintInfo:)]
        pub unsafe fn setPrintInfo(&self, print_info: &NSPrintInfo);

        #[cfg(feature = "AppKit_NSGraphicsContext")]
        #[method_id(@__retain_semantics Other context)]
        pub unsafe fn context(&self) -> Option<Id<NSGraphicsContext>>;

        #[method(pageRange)]
        pub unsafe fn pageRange(&self) -> NSRange;

        #[method(currentPage)]
        pub unsafe fn currentPage(&self) -> NSInteger;

        #[cfg(feature = "AppKit_NSGraphicsContext")]
        #[method_id(@__retain_semantics Other createContext)]
        pub unsafe fn createContext(&self) -> Option<Id<NSGraphicsContext>>;

        #[method(destroyContext)]
        pub unsafe fn destroyContext(&self);

        #[method(deliverResult)]
        pub unsafe fn deliverResult(&self) -> bool;

        #[method(cleanUpOperation)]
        pub unsafe fn cleanUpOperation(&self);
    }
);

extern_methods!(
    /// NSDeprecated
    #[cfg(feature = "AppKit_NSPrintOperation")]
    unsafe impl NSPrintOperation {
        #[cfg(feature = "AppKit_NSView")]
        #[deprecated = "Use -[NSPrintPanel addAccessoryController:] and -[NSPrintPanel removeAccessoryController:] instead"]
        #[method(setAccessoryView:)]
        pub unsafe fn setAccessoryView(&self, view: Option<&NSView>);

        #[cfg(feature = "AppKit_NSView")]
        #[deprecated = "Use -[NSPrintPanel accessoryControllers] instead"]
        #[method_id(@__retain_semantics Other accessoryView)]
        pub unsafe fn accessoryView(&self) -> Option<Id<NSView>>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(setJobStyleHint:)]
        pub unsafe fn setJobStyleHint(&self, hint: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other jobStyleHint)]
        pub unsafe fn jobStyleHint(&self) -> Option<Id<NSString>>;

        #[deprecated = "Use -setShowsPrintPanel: and -setShowsProgressPanel: instead"]
        #[method(setShowPanels:)]
        pub unsafe fn setShowPanels(&self, flag: bool);

        #[deprecated = "Use -showsPrintPanel and -showsProgressPanel instead"]
        #[method(showPanels)]
        pub unsafe fn showPanels(&self) -> bool;
    }
);
