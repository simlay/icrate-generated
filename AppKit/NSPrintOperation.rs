//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    /**
      Values for -setPageOrder:/-pageOrder.
    */
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

        /**
          The current print operation for this thread. If this is nil, there is no current operation for the current thread.
        */
        #[method_id(@__retain_semantics Other currentOperation)]
        pub unsafe fn currentOperation() -> Option<Id<NSPrintOperation>>;

        /**
          The current print operation for this thread. If this is nil, there is no current operation for the current thread.
        */
        #[method(setCurrentOperation:)]
        pub unsafe fn setCurrentOperation(current_operation: Option<&NSPrintOperation>);

        /**
          Return YES if the operation for copying to PDF or EPS, NO if it's for printing.
        */
        #[method(isCopyingOperation)]
        pub unsafe fn isCopyingOperation(&self) -> bool;

        /**
          If the print sheet is unresponsive or sluggish due to the time is takes you to fully render a page, you can check this method in drawRect and other printing methods such as beginDocument and knowsPageRage: to determine if the print operation prefers speed over fidelity. Please see the comments for NSPrintRenderingQuality. Most applications render each page fast enough and do not need to call this method.
        */
        #[method(preferredRenderingQuality)]
        pub unsafe fn preferredRenderingQuality(&self) -> NSPrintRenderingQuality;

        #[cfg(feature = "Foundation_NSString")]
        /**
          The title of the print job. If a job title is set it overrides anything that might be gotten by sending the printed view an [NSView(NSPrinting) printJobTitle] message.
        */
        #[method_id(@__retain_semantics Other jobTitle)]
        pub unsafe fn jobTitle(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        /**
          The title of the print job. If a job title is set it overrides anything that might be gotten by sending the printed view an [NSView(NSPrinting) printJobTitle] message.
        */
        #[method(setJobTitle:)]
        pub unsafe fn setJobTitle(&self, job_title: Option<&NSString>);

        /**
          Whether the print and progress panels are shown during the operation.
        */
        #[method(showsPrintPanel)]
        pub unsafe fn showsPrintPanel(&self) -> bool;

        /**
          Whether the print and progress panels are shown during the operation.
        */
        #[method(setShowsPrintPanel:)]
        pub unsafe fn setShowsPrintPanel(&self, shows_print_panel: bool);

        #[method(showsProgressPanel)]
        pub unsafe fn showsProgressPanel(&self) -> bool;

        #[method(setShowsProgressPanel:)]
        pub unsafe fn setShowsProgressPanel(&self, shows_progress_panel: bool);

        #[cfg(feature = "AppKit_NSPrintPanel")]
        /**
          The print panel to be presented by the operation when it is run, if showsProgressPanel is YES and isCopyingOperation is NO. -printPanel will create a new NSPrintPanel if one hasn't been set yet.
        */
        #[method_id(@__retain_semantics Other printPanel)]
        pub unsafe fn printPanel(&self) -> Id<NSPrintPanel>;

        #[cfg(feature = "AppKit_NSPrintPanel")]
        /**
          The print panel to be presented by the operation when it is run, if showsProgressPanel is YES and isCopyingOperation is NO. -printPanel will create a new NSPrintPanel if one hasn't been set yet.
        */
        #[method(setPrintPanel:)]
        pub unsafe fn setPrintPanel(&self, print_panel: &NSPrintPanel);

        #[cfg(feature = "AppKit_NSPDFPanel")]
        /**
          The panel to be presented by the operation when it is run, if [[self printInfo] jobDisposition] is NSPrintSaveJob and [[[self printInfo] dictionary] objectForKey:NSPrintJobSavingURL] is nil. -PDFPanel will create a new NSPDFPanel if one hasn't been set yet. NSPrintOperation will throw an exception if panel.options contains NSPDFPanelRequestsParentDirectory when it attempts to display the panel.
        */
        #[method_id(@__retain_semantics Other PDFPanel)]
        pub unsafe fn PDFPanel(&self) -> Id<NSPDFPanel>;

        #[cfg(feature = "AppKit_NSPDFPanel")]
        /**
          The panel to be presented by the operation when it is run, if [[self printInfo] jobDisposition] is NSPrintSaveJob and [[[self printInfo] dictionary] objectForKey:NSPrintJobSavingURL] is nil. -PDFPanel will create a new NSPDFPanel if one hasn't been set yet. NSPrintOperation will throw an exception if panel.options contains NSPDFPanelRequestsParentDirectory when it attempts to display the panel.
        */
        #[method(setPDFPanel:)]
        pub unsafe fn setPDFPanel(&self, pdf_panel: &NSPDFPanel);

        /**
          Whether the print operation should spawn a separate thread in which to run itself.
        */
        #[method(canSpawnSeparateThread)]
        pub unsafe fn canSpawnSeparateThread(&self) -> bool;

        /**
          Whether the print operation should spawn a separate thread in which to run itself.
        */
        #[method(setCanSpawnSeparateThread:)]
        pub unsafe fn setCanSpawnSeparateThread(&self, can_spawn_separate_thread: bool);

        /**
          The page order that will be used to generate the pages in this job. This is the physical page order of the pages. It depends on the stacking order of the printer, the capability of the app to reverse page order, etc.
        */
        #[method(pageOrder)]
        pub unsafe fn pageOrder(&self) -> NSPrintingPageOrder;

        /**
          The page order that will be used to generate the pages in this job. This is the physical page order of the pages. It depends on the stacking order of the printer, the capability of the app to reverse page order, etc.
        */
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
        /**
          The view being printed.
        */
        #[method_id(@__retain_semantics Other view)]
        pub unsafe fn view(&self) -> Option<Id<NSView>>;

        #[cfg(feature = "AppKit_NSPrintInfo")]
        /**
          The print info of the operation. -printInfo always returns a copy of the NSPrintInfo passed into the factory method used to create the print operation, unless -setPrintInfo: has been invoked, in which case it returns the exact same object passed into -setPrintInfo:. So, the factory methods listed above copy the passed-in NSPrintInfo, but -setPrintInfo: doesn't.
        */
        #[method_id(@__retain_semantics Other printInfo)]
        pub unsafe fn printInfo(&self) -> Id<NSPrintInfo>;

        #[cfg(feature = "AppKit_NSPrintInfo")]
        /**
          The print info of the operation. -printInfo always returns a copy of the NSPrintInfo passed into the factory method used to create the print operation, unless -setPrintInfo: has been invoked, in which case it returns the exact same object passed into -setPrintInfo:. So, the factory methods listed above copy the passed-in NSPrintInfo, but -setPrintInfo: doesn't.
        */
        #[method(setPrintInfo:)]
        pub unsafe fn setPrintInfo(&self, print_info: &NSPrintInfo);

        #[cfg(feature = "AppKit_NSGraphicsContext")]
        /**
          The context for the output of this operation.
        */
        #[method_id(@__retain_semantics Other context)]
        pub unsafe fn context(&self) -> Option<Id<NSGraphicsContext>>;

        /**
          The first through last one-based page numbers of the operation as it's being previewed or printed. The first page number might not be 1, and the page count might be NSIntegerMax to indicate that the number of pages is not known, depending on what the printed view returned when sent an [NSView(NSPrinting) knowsPageRange:] message.
        */
        #[method(pageRange)]
        pub unsafe fn pageRange(&self) -> NSRange;

        /**
          The current one-based page number of the operation as it's being previewed or printed.
        */
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
