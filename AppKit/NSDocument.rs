//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSDocumentChangeType {
        NSChangeDone = 0,
        NSChangeUndone = 1,
        NSChangeRedone = 5,
        NSChangeCleared = 2,
        NSChangeReadOtherContents = 3,
        NSChangeAutosaved = 4,
        NSChangeDiscardable = 256,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSSaveOperationType {
        NSSaveOperation = 0,
        NSSaveAsOperation = 1,
        NSSaveToOperation = 2,
        NSAutosaveInPlaceOperation = 4,
        NSAutosaveElsewhereOperation = 3,
        NSAutosaveAsOperation = 5,
        #[deprecated = "Use NSAutosaveElsewhereOperation instead"]
        NSAutosaveOperation = 3,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSDocument")]
    pub struct NSDocument;

    #[cfg(feature = "AppKit_NSDocument")]
    unsafe impl ClassType for NSDocument {
        type Super = NSObject;
    }
);

#[cfg(feature = "AppKit_NSDocument")]
unsafe impl NSEditorRegistration for NSDocument {}

#[cfg(feature = "AppKit_NSDocument")]
unsafe impl NSFilePresenter for NSDocument {}

#[cfg(feature = "AppKit_NSDocument")]
unsafe impl NSMenuItemValidation for NSDocument {}

#[cfg(feature = "AppKit_NSDocument")]
unsafe impl NSObjectProtocol for NSDocument {}

#[cfg(feature = "AppKit_NSDocument")]
unsafe impl NSUserInterfaceValidations for NSDocument {}

extern_methods!(
    #[cfg(feature = "AppKit_NSDocument")]
    unsafe impl NSDocument {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Init initWithType:error:_)]
        pub unsafe fn initWithType_error(
            this: Option<Allocated<Self>>,
            type_name: &NSString,
        ) -> Result<Id<Self>, Id<NSError>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(canConcurrentlyReadDocumentsOfType:)]
        pub unsafe fn canConcurrentlyReadDocumentsOfType(type_name: &NSString) -> bool;

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURL"
        ))]
        #[method_id(@__retain_semantics Init initWithContentsOfURL:ofType:error:_)]
        pub unsafe fn initWithContentsOfURL_ofType_error(
            this: Option<Allocated<Self>>,
            url: &NSURL,
            type_name: &NSString,
        ) -> Result<Id<Self>, Id<NSError>>;

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURL"
        ))]
        #[method_id(@__retain_semantics Init initForURL:withContentsOfURL:ofType:error:_)]
        pub unsafe fn initForURL_withContentsOfURL_ofType_error(
            this: Option<Allocated<Self>>,
            url_or_nil: Option<&NSURL>,
            contents_url: &NSURL,
            type_name: &NSString,
        ) -> Result<Id<Self>, Id<NSError>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other fileType)]
        pub unsafe fn fileType(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setFileType:)]
        pub unsafe fn setFileType(&self, file_type: Option<&NSString>);

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other fileURL)]
        pub unsafe fn fileURL(&self) -> Option<Id<NSURL>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method(setFileURL:)]
        pub unsafe fn setFileURL(&self, file_url: Option<&NSURL>);

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other fileModificationDate)]
        pub unsafe fn fileModificationDate(&self) -> Option<Id<NSDate>>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method(setFileModificationDate:)]
        pub unsafe fn setFileModificationDate(&self, file_modification_date: Option<&NSDate>);

        #[method(isDraft)]
        pub unsafe fn isDraft(&self) -> bool;

        #[method(setDraft:)]
        pub unsafe fn setDraft(&self, draft: bool);

        #[method(performActivityWithSynchronousWaiting:usingBlock:)]
        pub unsafe fn performActivityWithSynchronousWaiting_usingBlock(
            &self,
            wait_synchronously: bool,
            block: &Block<(NonNull<Block<(), ()>>,), ()>,
        );

        #[method(continueActivityUsingBlock:)]
        pub unsafe fn continueActivityUsingBlock(&self, block: &Block<(), ()>);

        #[method(continueAsynchronousWorkOnMainThreadUsingBlock:)]
        pub unsafe fn continueAsynchronousWorkOnMainThreadUsingBlock(&self, block: &Block<(), ()>);

        #[method(performSynchronousFileAccessUsingBlock:)]
        pub unsafe fn performSynchronousFileAccessUsingBlock(&self, block: &Block<(), ()>);

        #[method(performAsynchronousFileAccessUsingBlock:)]
        pub unsafe fn performAsynchronousFileAccessUsingBlock(
            &self,
            block: &Block<(NonNull<Block<(), ()>>,), ()>,
        );

        #[method(revertDocumentToSaved:)]
        pub unsafe fn revertDocumentToSaved(&self, sender: Option<&Object>);

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURL"
        ))]
        #[method(revertToContentsOfURL:ofType:error:_)]
        pub unsafe fn revertToContentsOfURL_ofType_error(
            &self,
            url: &NSURL,
            type_name: &NSString,
        ) -> Result<(), Id<NSError>>;

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURL"
        ))]
        #[method(readFromURL:ofType:error:_)]
        pub unsafe fn readFromURL_ofType_error(
            &self,
            url: &NSURL,
            type_name: &NSString,
        ) -> Result<(), Id<NSError>>;

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSFileWrapper",
            feature = "Foundation_NSString"
        ))]
        #[method(readFromFileWrapper:ofType:error:_)]
        pub unsafe fn readFromFileWrapper_ofType_error(
            &self,
            file_wrapper: &NSFileWrapper,
            type_name: &NSString,
        ) -> Result<(), Id<NSError>>;

        #[cfg(all(
            feature = "Foundation_NSData",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString"
        ))]
        #[method(readFromData:ofType:error:_)]
        pub unsafe fn readFromData_ofType_error(
            &self,
            data: &NSData,
            type_name: &NSString,
        ) -> Result<(), Id<NSError>>;

        #[method(isEntireFileLoaded)]
        pub unsafe fn isEntireFileLoaded(&self) -> bool;

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURL"
        ))]
        #[method(writeToURL:ofType:error:_)]
        pub unsafe fn writeToURL_ofType_error(
            &self,
            url: &NSURL,
            type_name: &NSString,
        ) -> Result<(), Id<NSError>>;

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSFileWrapper",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other fileWrapperOfType:error:_)]
        pub unsafe fn fileWrapperOfType_error(
            &self,
            type_name: &NSString,
        ) -> Result<Id<NSFileWrapper>, Id<NSError>>;

        #[cfg(all(
            feature = "Foundation_NSData",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other dataOfType:error:_)]
        pub unsafe fn dataOfType_error(
            &self,
            type_name: &NSString,
        ) -> Result<Id<NSData>, Id<NSError>>;

        #[method(unblockUserInteraction)]
        pub unsafe fn unblockUserInteraction(&self);

        #[method(autosavingIsImplicitlyCancellable)]
        pub unsafe fn autosavingIsImplicitlyCancellable(&self) -> bool;

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURL"
        ))]
        #[method(writeSafelyToURL:ofType:forSaveOperation:error:_)]
        pub unsafe fn writeSafelyToURL_ofType_forSaveOperation_error(
            &self,
            url: &NSURL,
            type_name: &NSString,
            save_operation: NSSaveOperationType,
        ) -> Result<(), Id<NSError>>;

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURL"
        ))]
        #[method(writeToURL:ofType:forSaveOperation:originalContentsURL:error:_)]
        pub unsafe fn writeToURL_ofType_forSaveOperation_originalContentsURL_error(
            &self,
            url: &NSURL,
            type_name: &NSString,
            save_operation: NSSaveOperationType,
            absolute_original_contents_url: Option<&NSURL>,
        ) -> Result<(), Id<NSError>>;

        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURL"
        ))]
        #[method_id(@__retain_semantics Other fileAttributesToWriteToURL:ofType:forSaveOperation:originalContentsURL:error:_)]
        pub unsafe fn fileAttributesToWriteToURL_ofType_forSaveOperation_originalContentsURL_error(
            &self,
            url: &NSURL,
            type_name: &NSString,
            save_operation: NSSaveOperationType,
            absolute_original_contents_url: Option<&NSURL>,
        ) -> Result<Id<NSDictionary<NSString, Object>>, Id<NSError>>;

        #[method(keepBackupFile)]
        pub unsafe fn keepBackupFile(&self) -> bool;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other backupFileURL)]
        pub unsafe fn backupFileURL(&self) -> Option<Id<NSURL>>;

        #[method(saveDocument:)]
        pub unsafe fn saveDocument(&self, sender: Option<&Object>);

        #[method(saveDocumentAs:)]
        pub unsafe fn saveDocumentAs(&self, sender: Option<&Object>);

        #[method(saveDocumentTo:)]
        pub unsafe fn saveDocumentTo(&self, sender: Option<&Object>);

        #[method(saveDocumentWithDelegate:didSaveSelector:contextInfo:)]
        pub unsafe fn saveDocumentWithDelegate_didSaveSelector_contextInfo(
            &self,
            delegate: Option<&Object>,
            did_save_selector: Option<Sel>,
            context_info: *mut c_void,
        );

        #[method(runModalSavePanelForSaveOperation:delegate:didSaveSelector:contextInfo:)]
        pub unsafe fn runModalSavePanelForSaveOperation_delegate_didSaveSelector_contextInfo(
            &self,
            save_operation: NSSaveOperationType,
            delegate: Option<&Object>,
            did_save_selector: Option<Sel>,
            context_info: *mut c_void,
        );

        #[method(shouldRunSavePanelWithAccessoryView)]
        pub unsafe fn shouldRunSavePanelWithAccessoryView(&self) -> bool;

        #[cfg(feature = "AppKit_NSSavePanel")]
        #[method(prepareSavePanel:)]
        pub unsafe fn prepareSavePanel(&self, save_panel: &NSSavePanel) -> bool;

        #[method(fileNameExtensionWasHiddenInLastRunSavePanel)]
        pub unsafe fn fileNameExtensionWasHiddenInLastRunSavePanel(&self) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other fileTypeFromLastRunSavePanel)]
        pub unsafe fn fileTypeFromLastRunSavePanel(&self) -> Option<Id<NSString>>;

        #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSURL"))]
        #[method(saveToURL:ofType:forSaveOperation:delegate:didSaveSelector:contextInfo:)]
        pub unsafe fn saveToURL_ofType_forSaveOperation_delegate_didSaveSelector_contextInfo(
            &self,
            url: &NSURL,
            type_name: &NSString,
            save_operation: NSSaveOperationType,
            delegate: Option<&Object>,
            did_save_selector: Option<Sel>,
            context_info: *mut c_void,
        );

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURL"
        ))]
        #[method(saveToURL:ofType:forSaveOperation:completionHandler:)]
        pub unsafe fn saveToURL_ofType_forSaveOperation_completionHandler(
            &self,
            url: &NSURL,
            type_name: &NSString,
            save_operation: NSSaveOperationType,
            completion_handler: &Block<(*mut NSError,), ()>,
        );

        #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSURL"))]
        #[method(canAsynchronouslyWriteToURL:ofType:forSaveOperation:)]
        pub unsafe fn canAsynchronouslyWriteToURL_ofType_forSaveOperation(
            &self,
            url: &NSURL,
            type_name: &NSString,
            save_operation: NSSaveOperationType,
        ) -> bool;

        #[cfg(feature = "Foundation_NSError")]
        #[method(checkAutosavingSafetyAndReturnError:_)]
        pub unsafe fn checkAutosavingSafetyAndReturnError(&self) -> Result<(), Id<NSError>>;

        #[method(scheduleAutosaving)]
        pub unsafe fn scheduleAutosaving(&self);

        #[method(hasUnautosavedChanges)]
        pub unsafe fn hasUnautosavedChanges(&self) -> bool;

        #[method(autosaveDocumentWithDelegate:didAutosaveSelector:contextInfo:)]
        pub unsafe fn autosaveDocumentWithDelegate_didAutosaveSelector_contextInfo(
            &self,
            delegate: Option<&Object>,
            did_autosave_selector: Option<Sel>,
            context_info: *mut c_void,
        );

        #[cfg(feature = "Foundation_NSError")]
        #[method(autosaveWithImplicitCancellability:completionHandler:)]
        pub unsafe fn autosaveWithImplicitCancellability_completionHandler(
            &self,
            autosaving_is_implicitly_cancellable: bool,
            completion_handler: &Block<(*mut NSError,), ()>,
        );

        #[method(autosavesInPlace)]
        pub unsafe fn autosavesInPlace() -> bool;

        #[method(preservesVersions)]
        pub unsafe fn preservesVersions() -> bool;

        #[method(browseDocumentVersions:)]
        pub unsafe fn browseDocumentVersions(&self, sender: Option<&Object>);

        #[method(isBrowsingVersions)]
        pub unsafe fn isBrowsingVersions(&self) -> bool;

        #[method(stopBrowsingVersionsWithCompletionHandler:)]
        pub unsafe fn stopBrowsingVersionsWithCompletionHandler(
            &self,
            completion_handler: Option<&Block<(), ()>>,
        );

        #[method(autosavesDrafts)]
        pub unsafe fn autosavesDrafts() -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other autosavingFileType)]
        pub unsafe fn autosavingFileType(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other autosavedContentsFileURL)]
        pub unsafe fn autosavedContentsFileURL(&self) -> Option<Id<NSURL>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method(setAutosavedContentsFileURL:)]
        pub unsafe fn setAutosavedContentsFileURL(
            &self,
            autosaved_contents_file_url: Option<&NSURL>,
        );

        #[method(canCloseDocumentWithDelegate:shouldCloseSelector:contextInfo:)]
        pub unsafe fn canCloseDocumentWithDelegate_shouldCloseSelector_contextInfo(
            &self,
            delegate: &Object,
            should_close_selector: Option<Sel>,
            context_info: *mut c_void,
        );

        #[method(close)]
        pub unsafe fn close(&self);

        #[method(duplicateDocument:)]
        pub unsafe fn duplicateDocument(&self, sender: Option<&Object>);

        #[method(duplicateDocumentWithDelegate:didDuplicateSelector:contextInfo:)]
        pub unsafe fn duplicateDocumentWithDelegate_didDuplicateSelector_contextInfo(
            &self,
            delegate: Option<&Object>,
            did_duplicate_selector: Option<Sel>,
            context_info: *mut c_void,
        );

        #[cfg(feature = "Foundation_NSError")]
        #[method_id(@__retain_semantics Other duplicateAndReturnError:_)]
        pub unsafe fn duplicateAndReturnError(&self) -> Result<Id<NSDocument>, Id<NSError>>;

        #[method(renameDocument:)]
        pub unsafe fn renameDocument(&self, sender: Option<&Object>);

        #[method(moveDocumentToUbiquityContainer:)]
        pub unsafe fn moveDocumentToUbiquityContainer(&self, sender: Option<&Object>);

        #[method(moveDocument:)]
        pub unsafe fn moveDocument(&self, sender: Option<&Object>);

        #[method(moveDocumentWithCompletionHandler:)]
        pub unsafe fn moveDocumentWithCompletionHandler(
            &self,
            completion_handler: Option<&Block<(Bool,), ()>>,
        );

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
        #[method(moveToURL:completionHandler:)]
        pub unsafe fn moveToURL_completionHandler(
            &self,
            url: &NSURL,
            completion_handler: Option<&Block<(*mut NSError,), ()>>,
        );

        #[method(lockDocument:)]
        pub unsafe fn lockDocument(&self, sender: Option<&Object>);

        #[method(unlockDocument:)]
        pub unsafe fn unlockDocument(&self, sender: Option<&Object>);

        #[method(lockDocumentWithCompletionHandler:)]
        pub unsafe fn lockDocumentWithCompletionHandler(
            &self,
            completion_handler: Option<&Block<(Bool,), ()>>,
        );

        #[cfg(feature = "Foundation_NSError")]
        #[method(lockWithCompletionHandler:)]
        pub unsafe fn lockWithCompletionHandler(
            &self,
            completion_handler: Option<&Block<(*mut NSError,), ()>>,
        );

        #[method(unlockDocumentWithCompletionHandler:)]
        pub unsafe fn unlockDocumentWithCompletionHandler(
            &self,
            completion_handler: Option<&Block<(Bool,), ()>>,
        );

        #[cfg(feature = "Foundation_NSError")]
        #[method(unlockWithCompletionHandler:)]
        pub unsafe fn unlockWithCompletionHandler(
            &self,
            completion_handler: Option<&Block<(*mut NSError,), ()>>,
        );

        #[method(isLocked)]
        pub unsafe fn isLocked(&self) -> bool;

        #[method(runPageLayout:)]
        pub unsafe fn runPageLayout(&self, sender: Option<&Object>);

        #[cfg(feature = "AppKit_NSPrintInfo")]
        #[method(runModalPageLayoutWithPrintInfo:delegate:didRunSelector:contextInfo:)]
        pub unsafe fn runModalPageLayoutWithPrintInfo_delegate_didRunSelector_contextInfo(
            &self,
            print_info: &NSPrintInfo,
            delegate: Option<&Object>,
            did_run_selector: Option<Sel>,
            context_info: *mut c_void,
        );

        #[cfg(feature = "AppKit_NSPageLayout")]
        #[method(preparePageLayout:)]
        pub unsafe fn preparePageLayout(&self, page_layout: &NSPageLayout) -> bool;

        #[cfg(feature = "AppKit_NSPrintInfo")]
        #[method(shouldChangePrintInfo:)]
        pub unsafe fn shouldChangePrintInfo(&self, new_print_info: &NSPrintInfo) -> bool;

        #[cfg(feature = "AppKit_NSPrintInfo")]
        #[method_id(@__retain_semantics Other printInfo)]
        pub unsafe fn printInfo(&self) -> Id<NSPrintInfo>;

        #[cfg(feature = "AppKit_NSPrintInfo")]
        #[method(setPrintInfo:)]
        pub unsafe fn setPrintInfo(&self, print_info: &NSPrintInfo);

        #[method(printDocument:)]
        pub unsafe fn printDocument(&self, sender: Option<&Object>);

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method(printDocumentWithSettings:showPrintPanel:delegate:didPrintSelector:contextInfo:)]
        pub unsafe fn printDocumentWithSettings_showPrintPanel_delegate_didPrintSelector_contextInfo(
            &self,
            print_settings: &NSDictionary<NSPrintInfoAttributeKey, Object>,
            show_print_panel: bool,
            delegate: Option<&Object>,
            did_print_selector: Option<Sel>,
            context_info: *mut c_void,
        );

        #[cfg(all(
            feature = "AppKit_NSPrintOperation",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSError"
        ))]
        #[method_id(@__retain_semantics Other printOperationWithSettings:error:_)]
        pub unsafe fn printOperationWithSettings_error(
            &self,
            print_settings: &NSDictionary<NSPrintInfoAttributeKey, Object>,
        ) -> Result<Id<NSPrintOperation>, Id<NSError>>;

        #[cfg(feature = "AppKit_NSPrintOperation")]
        #[method(runModalPrintOperation:delegate:didRunSelector:contextInfo:)]
        pub unsafe fn runModalPrintOperation_delegate_didRunSelector_contextInfo(
            &self,
            print_operation: &NSPrintOperation,
            delegate: Option<&Object>,
            did_run_selector: Option<Sel>,
            context_info: *mut c_void,
        );

        #[method(saveDocumentToPDF:)]
        pub unsafe fn saveDocumentToPDF(&self, sender: Option<&Object>);

        #[cfg(feature = "AppKit_NSPrintOperation")]
        #[method_id(@__retain_semantics Other PDFPrintOperation)]
        pub unsafe fn PDFPrintOperation(&self) -> Id<NSPrintOperation>;

        #[method(allowsDocumentSharing)]
        pub unsafe fn allowsDocumentSharing(&self) -> bool;

        #[cfg(feature = "AppKit_NSSharingService")]
        #[method(shareDocumentWithSharingService:completionHandler:)]
        pub unsafe fn shareDocumentWithSharingService_completionHandler(
            &self,
            sharing_service: &NSSharingService,
            completion_handler: Option<&Block<(Bool,), ()>>,
        );

        #[cfg(feature = "AppKit_NSSharingServicePicker")]
        #[method(prepareSharingServicePicker:)]
        pub unsafe fn prepareSharingServicePicker(
            &self,
            sharing_service_picker: &NSSharingServicePicker,
        );

        #[method(isDocumentEdited)]
        pub unsafe fn isDocumentEdited(&self) -> bool;

        #[method(isInViewingMode)]
        pub unsafe fn isInViewingMode(&self) -> bool;

        #[method(updateChangeCount:)]
        pub unsafe fn updateChangeCount(&self, change: NSDocumentChangeType);

        #[method_id(@__retain_semantics Other changeCountTokenForSaveOperation:)]
        pub unsafe fn changeCountTokenForSaveOperation(
            &self,
            save_operation: NSSaveOperationType,
        ) -> Id<Object>;

        #[method(updateChangeCountWithToken:forSaveOperation:)]
        pub unsafe fn updateChangeCountWithToken_forSaveOperation(
            &self,
            change_count_token: &Object,
            save_operation: NSSaveOperationType,
        );

        #[cfg(feature = "Foundation_NSUndoManager")]
        #[method_id(@__retain_semantics Other undoManager)]
        pub unsafe fn undoManager(&self) -> Option<Id<NSUndoManager>>;

        #[cfg(feature = "Foundation_NSUndoManager")]
        #[method(setUndoManager:)]
        pub unsafe fn setUndoManager(&self, undo_manager: Option<&NSUndoManager>);

        #[method(hasUndoManager)]
        pub unsafe fn hasUndoManager(&self) -> bool;

        #[method(setHasUndoManager:)]
        pub unsafe fn setHasUndoManager(&self, has_undo_manager: bool);

        #[cfg(all(feature = "AppKit_NSWindow", feature = "Foundation_NSError"))]
        #[method(presentError:modalForWindow:delegate:didPresentSelector:contextInfo:)]
        pub unsafe fn presentError_modalForWindow_delegate_didPresentSelector_contextInfo(
            &self,
            error: &NSError,
            window: &NSWindow,
            delegate: Option<&Object>,
            did_present_selector: Option<Sel>,
            context_info: *mut c_void,
        );

        #[cfg(feature = "Foundation_NSError")]
        #[method(presentError:)]
        pub unsafe fn presentError(&self, error: &NSError) -> bool;

        #[cfg(feature = "Foundation_NSError")]
        #[method_id(@__retain_semantics Other willPresentError:)]
        pub unsafe fn willPresentError(&self, error: &NSError) -> Id<NSError>;

        #[cfg(feature = "Foundation_NSError")]
        #[method(willNotPresentError:)]
        pub unsafe fn willNotPresentError(&self, error: &NSError);

        #[method(makeWindowControllers)]
        pub unsafe fn makeWindowControllers(&self);

        #[method_id(@__retain_semantics Other windowNibName)]
        pub unsafe fn windowNibName(&self) -> Option<Id<NSNibName>>;

        #[cfg(feature = "AppKit_NSWindowController")]
        #[method(windowControllerWillLoadNib:)]
        pub unsafe fn windowControllerWillLoadNib(&self, window_controller: &NSWindowController);

        #[cfg(feature = "AppKit_NSWindowController")]
        #[method(windowControllerDidLoadNib:)]
        pub unsafe fn windowControllerDidLoadNib(&self, window_controller: &NSWindowController);

        #[cfg(feature = "AppKit_NSWindow")]
        #[method(setWindow:)]
        pub unsafe fn setWindow(&self, window: Option<&NSWindow>);

        #[cfg(feature = "AppKit_NSWindowController")]
        #[method(addWindowController:)]
        pub unsafe fn addWindowController(&self, window_controller: &NSWindowController);

        #[cfg(feature = "AppKit_NSWindowController")]
        #[method(removeWindowController:)]
        pub unsafe fn removeWindowController(&self, window_controller: &NSWindowController);

        #[method(showWindows)]
        pub unsafe fn showWindows(&self);

        #[cfg(all(feature = "AppKit_NSWindowController", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other windowControllers)]
        pub unsafe fn windowControllers(&self) -> Id<NSArray<NSWindowController>>;

        #[cfg(feature = "AppKit_NSWindowController")]
        #[method(shouldCloseWindowController:delegate:shouldCloseSelector:contextInfo:)]
        pub unsafe fn shouldCloseWindowController_delegate_shouldCloseSelector_contextInfo(
            &self,
            window_controller: &NSWindowController,
            delegate: Option<&Object>,
            should_close_selector: Option<Sel>,
            context_info: *mut c_void,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other displayName)]
        pub unsafe fn displayName(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other defaultDraftName)]
        pub unsafe fn defaultDraftName(&self) -> Id<NSString>;

        #[cfg(feature = "AppKit_NSWindow")]
        #[method_id(@__retain_semantics Other windowForSheet)]
        pub unsafe fn windowForSheet(&self) -> Option<Id<NSWindow>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other readableTypes)]
        pub unsafe fn readableTypes() -> Id<NSArray<NSString>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other writableTypes)]
        pub unsafe fn writableTypes() -> Id<NSArray<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(isNativeType:)]
        pub unsafe fn isNativeType(r#type: &NSString) -> bool;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other writableTypesForSaveOperation:)]
        pub unsafe fn writableTypesForSaveOperation(
            &self,
            save_operation: NSSaveOperationType,
        ) -> Id<NSArray<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other fileNameExtensionForType:saveOperation:)]
        pub unsafe fn fileNameExtensionForType_saveOperation(
            &self,
            type_name: &NSString,
            save_operation: NSSaveOperationType,
        ) -> Option<Id<NSString>>;

        #[method(validateUserInterfaceItem:)]
        pub unsafe fn validateUserInterfaceItem(
            &self,
            item: &ProtocolObject<dyn NSValidatedUserInterfaceItem>,
        ) -> bool;

        #[method(usesUbiquitousStorage)]
        pub unsafe fn usesUbiquitousStorage() -> bool;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other presentedItemURL)]
        pub unsafe fn presentedItemURL(&self) -> Option<Id<NSURL>>;

        #[cfg(feature = "Foundation_NSSet")]
        #[method_id(@__retain_semantics Other observedPresentedItemUbiquityAttributes)]
        pub unsafe fn observedPresentedItemUbiquityAttributes(&self)
            -> Id<NSSet<NSURLResourceKey>>;

        #[method(relinquishPresentedItemToReader:)]
        pub unsafe fn relinquishPresentedItemToReader(
            &self,
            reader: &Block<(*mut Block<(), ()>,), ()>,
        );

        #[method(relinquishPresentedItemToWriter:)]
        pub unsafe fn relinquishPresentedItemToWriter(
            &self,
            writer: &Block<(*mut Block<(), ()>,), ()>,
        );

        #[cfg(feature = "Foundation_NSError")]
        #[method(savePresentedItemChangesWithCompletionHandler:)]
        pub unsafe fn savePresentedItemChangesWithCompletionHandler(
            &self,
            completion_handler: &Block<(*mut NSError,), ()>,
        );

        #[cfg(feature = "Foundation_NSError")]
        #[method(accommodatePresentedItemDeletionWithCompletionHandler:)]
        pub unsafe fn accommodatePresentedItemDeletionWithCompletionHandler(
            &self,
            completion_handler: &Block<(*mut NSError,), ()>,
        );

        #[cfg(feature = "Foundation_NSURL")]
        #[method(presentedItemDidMoveToURL:)]
        pub unsafe fn presentedItemDidMoveToURL(&self, new_url: &NSURL);

        #[method(presentedItemDidChange)]
        pub unsafe fn presentedItemDidChange(&self);

        #[cfg(feature = "Foundation_NSSet")]
        #[method(presentedItemDidChangeUbiquityAttributes:)]
        pub unsafe fn presentedItemDidChangeUbiquityAttributes(
            &self,
            attributes: &NSSet<NSURLResourceKey>,
        );

        #[cfg(feature = "Foundation_NSFileVersion")]
        #[method(presentedItemDidGainVersion:)]
        pub unsafe fn presentedItemDidGainVersion(&self, version: &NSFileVersion);

        #[cfg(feature = "Foundation_NSFileVersion")]
        #[method(presentedItemDidLoseVersion:)]
        pub unsafe fn presentedItemDidLoseVersion(&self, version: &NSFileVersion);

        #[cfg(feature = "Foundation_NSFileVersion")]
        #[method(presentedItemDidResolveConflictVersion:)]
        pub unsafe fn presentedItemDidResolveConflictVersion(&self, version: &NSFileVersion);
    }
);

extern_methods!(
    /// NSDeprecated
    #[cfg(feature = "AppKit_NSDocument")]
    unsafe impl NSDocument {
        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURL"
        ))]
        #[deprecated = "Use -saveToURL:ofType:forSaveOperation:completionHandler: instead"]
        #[method(saveToURL:ofType:forSaveOperation:error:_)]
        pub unsafe fn saveToURL_ofType_forSaveOperation_error(
            &self,
            url: &NSURL,
            type_name: &NSString,
            save_operation: NSSaveOperationType,
        ) -> Result<(), Id<NSError>>;

        #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSString"))]
        #[deprecated]
        #[method_id(@__retain_semantics Other dataRepresentationOfType:)]
        pub unsafe fn dataRepresentationOfType(&self, r#type: &NSString) -> Option<Id<NSData>>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[deprecated]
        #[method_id(@__retain_semantics Other fileAttributesToWriteToFile:ofType:saveOperation:)]
        pub unsafe fn fileAttributesToWriteToFile_ofType_saveOperation(
            &self,
            full_document_path: &NSString,
            document_type_name: &NSString,
            save_operation_type: NSSaveOperationType,
        ) -> Option<Id<NSDictionary>>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other fileName)]
        pub unsafe fn fileName(&self) -> Option<Id<NSString>>;

        #[cfg(all(feature = "Foundation_NSFileWrapper", feature = "Foundation_NSString"))]
        #[deprecated]
        #[method_id(@__retain_semantics Other fileWrapperRepresentationOfType:)]
        pub unsafe fn fileWrapperRepresentationOfType(
            &self,
            r#type: &NSString,
        ) -> Option<Id<NSFileWrapper>>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Init initWithContentsOfFile:ofType:)]
        pub unsafe fn initWithContentsOfFile_ofType(
            this: Option<Allocated<Self>>,
            absolute_path: &NSString,
            type_name: &NSString,
        ) -> Option<Id<Self>>;

        #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSURL"))]
        #[deprecated]
        #[method_id(@__retain_semantics Init initWithContentsOfURL:ofType:)]
        pub unsafe fn initWithContentsOfURL_ofType(
            this: Option<Allocated<Self>>,
            url: &NSURL,
            type_name: &NSString,
        ) -> Option<Id<Self>>;

        #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSString"))]
        #[deprecated]
        #[method(loadDataRepresentation:ofType:)]
        pub unsafe fn loadDataRepresentation_ofType(
            &self,
            data: &NSData,
            r#type: &NSString,
        ) -> bool;

        #[cfg(all(feature = "Foundation_NSFileWrapper", feature = "Foundation_NSString"))]
        #[deprecated]
        #[method(loadFileWrapperRepresentation:ofType:)]
        pub unsafe fn loadFileWrapperRepresentation_ofType(
            &self,
            wrapper: &NSFileWrapper,
            r#type: &NSString,
        ) -> bool;

        #[deprecated]
        #[method(printShowingPrintPanel:)]
        pub unsafe fn printShowingPrintPanel(&self, flag: bool);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(readFromFile:ofType:)]
        pub unsafe fn readFromFile_ofType(&self, file_name: &NSString, r#type: &NSString) -> bool;

        #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSURL"))]
        #[deprecated]
        #[method(readFromURL:ofType:)]
        pub unsafe fn readFromURL_ofType(&self, url: &NSURL, r#type: &NSString) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(revertToSavedFromFile:ofType:)]
        pub unsafe fn revertToSavedFromFile_ofType(
            &self,
            file_name: &NSString,
            r#type: &NSString,
        ) -> bool;

        #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSURL"))]
        #[deprecated]
        #[method(revertToSavedFromURL:ofType:)]
        pub unsafe fn revertToSavedFromURL_ofType(&self, url: &NSURL, r#type: &NSString) -> bool;

        #[cfg(feature = "AppKit_NSPrintInfo")]
        #[deprecated]
        #[method(runModalPageLayoutWithPrintInfo:)]
        pub unsafe fn runModalPageLayoutWithPrintInfo(&self, print_info: &NSPrintInfo)
            -> NSInteger;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(saveToFile:saveOperation:delegate:didSaveSelector:contextInfo:)]
        pub unsafe fn saveToFile_saveOperation_delegate_didSaveSelector_contextInfo(
            &self,
            file_name: &NSString,
            save_operation: NSSaveOperationType,
            delegate: Option<&Object>,
            did_save_selector: Option<Sel>,
            context_info: *mut c_void,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(setFileName:)]
        pub unsafe fn setFileName(&self, file_name: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(writeToFile:ofType:)]
        pub unsafe fn writeToFile_ofType(&self, file_name: &NSString, r#type: &NSString) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(writeToFile:ofType:originalFile:saveOperation:)]
        pub unsafe fn writeToFile_ofType_originalFile_saveOperation(
            &self,
            full_document_path: &NSString,
            document_type_name: &NSString,
            full_original_document_path: Option<&NSString>,
            save_operation_type: NSSaveOperationType,
        ) -> bool;

        #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSURL"))]
        #[deprecated]
        #[method(writeToURL:ofType:)]
        pub unsafe fn writeToURL_ofType(&self, url: &NSURL, r#type: &NSString) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(writeWithBackupToFile:ofType:saveOperation:)]
        pub unsafe fn writeWithBackupToFile_ofType_saveOperation(
            &self,
            full_document_path: &NSString,
            document_type_name: &NSString,
            save_operation_type: NSSaveOperationType,
        ) -> bool;
    }
);
