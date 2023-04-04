//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSDocumentController")]
    pub struct NSDocumentController;

    #[cfg(feature = "AppKit_NSDocumentController")]
    unsafe impl ClassType for NSDocumentController {
        type Super = NSObject;
    }
);

#[cfg(feature = "AppKit_NSDocumentController")]
unsafe impl NSCoding for NSDocumentController {}

#[cfg(feature = "AppKit_NSDocumentController")]
unsafe impl NSMenuItemValidation for NSDocumentController {}

#[cfg(feature = "AppKit_NSDocumentController")]
unsafe impl NSObjectProtocol for NSDocumentController {}

#[cfg(feature = "AppKit_NSDocumentController")]
unsafe impl NSUserInterfaceValidations for NSDocumentController {}

extern_methods!(
    #[cfg(feature = "AppKit_NSDocumentController")]
    unsafe impl NSDocumentController {
        #[method_id(@__retain_semantics Other sharedDocumentController)]
        pub unsafe fn sharedDocumentController() -> Id<NSDocumentController>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self>>;

        #[cfg(all(feature = "AppKit_NSDocument", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other documents)]
        pub unsafe fn documents(&self) -> Id<NSArray<NSDocument>>;

        #[cfg(feature = "AppKit_NSDocument")]
        #[method_id(@__retain_semantics Other currentDocument)]
        pub unsafe fn currentDocument(&self) -> Option<Id<NSDocument>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other currentDirectory)]
        pub unsafe fn currentDirectory(&self) -> Option<Id<NSString>>;

        #[cfg(all(feature = "AppKit_NSDocument", feature = "Foundation_NSURL"))]
        #[method_id(@__retain_semantics Other documentForURL:)]
        pub unsafe fn documentForURL(&self, url: &NSURL) -> Option<Id<NSDocument>>;

        #[cfg(all(feature = "AppKit_NSDocument", feature = "AppKit_NSWindow"))]
        #[method_id(@__retain_semantics Other documentForWindow:)]
        pub unsafe fn documentForWindow(&self, window: &NSWindow) -> Option<Id<NSDocument>>;

        #[cfg(feature = "AppKit_NSDocument")]
        #[method(addDocument:)]
        pub unsafe fn addDocument(&self, document: &NSDocument);

        #[cfg(feature = "AppKit_NSDocument")]
        #[method(removeDocument:)]
        pub unsafe fn removeDocument(&self, document: &NSDocument);

        #[method(newDocument:)]
        pub unsafe fn newDocument(&self, sender: Option<&Object>);

        #[cfg(all(feature = "AppKit_NSDocument", feature = "Foundation_NSError"))]
        #[method_id(@__retain_semantics Other openUntitledDocumentAndDisplay:error:_)]
        pub unsafe fn openUntitledDocumentAndDisplay_error(
            &self,
            display_document: bool,
        ) -> Result<Id<NSDocument>, Id<NSError>>;

        #[cfg(all(
            feature = "AppKit_NSDocument",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other makeUntitledDocumentOfType:error:_)]
        pub unsafe fn makeUntitledDocumentOfType_error(
            &self,
            type_name: &NSString,
        ) -> Result<Id<NSDocument>, Id<NSError>>;

        #[method(openDocument:)]
        pub unsafe fn openDocument(&self, sender: Option<&Object>);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSURL"))]
        #[method_id(@__retain_semantics Other URLsFromRunningOpenPanel)]
        pub unsafe fn URLsFromRunningOpenPanel(&self) -> Option<Id<NSArray<NSURL>>>;

        #[cfg(all(
            feature = "AppKit_NSOpenPanel",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSString"
        ))]
        #[method(runModalOpenPanel:forTypes:)]
        pub unsafe fn runModalOpenPanel_forTypes(
            &self,
            open_panel: &NSOpenPanel,
            types: Option<&NSArray<NSString>>,
        ) -> NSInteger;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSURL"))]
        #[method(beginOpenPanelWithCompletionHandler:)]
        pub unsafe fn beginOpenPanelWithCompletionHandler(
            &self,
            completion_handler: &Block<(*mut NSArray<NSURL>,), ()>,
        );

        #[cfg(all(
            feature = "AppKit_NSOpenPanel",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSString"
        ))]
        #[method(beginOpenPanel:forTypes:completionHandler:)]
        pub unsafe fn beginOpenPanel_forTypes_completionHandler(
            &self,
            open_panel: &NSOpenPanel,
            in_types: Option<&NSArray<NSString>>,
            completion_handler: &Block<(NSInteger,), ()>,
        );

        #[cfg(all(
            feature = "AppKit_NSDocument",
            feature = "Foundation_NSError",
            feature = "Foundation_NSURL"
        ))]
        #[method(openDocumentWithContentsOfURL:display:completionHandler:)]
        pub unsafe fn openDocumentWithContentsOfURL_display_completionHandler(
            &self,
            url: &NSURL,
            display_document: bool,
            completion_handler: &Block<(*mut NSDocument, Bool, *mut NSError), ()>,
        );

        #[cfg(all(
            feature = "AppKit_NSDocument",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURL"
        ))]
        #[method_id(@__retain_semantics Other makeDocumentWithContentsOfURL:ofType:error:_)]
        pub unsafe fn makeDocumentWithContentsOfURL_ofType_error(
            &self,
            url: &NSURL,
            type_name: &NSString,
        ) -> Result<Id<NSDocument>, Id<NSError>>;

        #[cfg(all(
            feature = "AppKit_NSDocument",
            feature = "Foundation_NSError",
            feature = "Foundation_NSURL"
        ))]
        #[method(reopenDocumentForURL:withContentsOfURL:display:completionHandler:)]
        pub unsafe fn reopenDocumentForURL_withContentsOfURL_display_completionHandler(
            &self,
            url_or_nil: Option<&NSURL>,
            contents_url: &NSURL,
            display_document: bool,
            completion_handler: &Block<(*mut NSDocument, Bool, *mut NSError), ()>,
        );

        #[cfg(all(
            feature = "AppKit_NSDocument",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURL"
        ))]
        #[method_id(@__retain_semantics Other makeDocumentForURL:withContentsOfURL:ofType:error:_)]
        pub unsafe fn makeDocumentForURL_withContentsOfURL_ofType_error(
            &self,
            url_or_nil: Option<&NSURL>,
            contents_url: &NSURL,
            type_name: &NSString,
        ) -> Result<Id<NSDocument>, Id<NSError>>;

        #[method(autosavingDelay)]
        pub unsafe fn autosavingDelay(&self) -> NSTimeInterval;

        #[method(setAutosavingDelay:)]
        pub unsafe fn setAutosavingDelay(&self, autosaving_delay: NSTimeInterval);

        #[method(saveAllDocuments:)]
        pub unsafe fn saveAllDocuments(&self, sender: Option<&Object>);

        #[method(hasEditedDocuments)]
        pub unsafe fn hasEditedDocuments(&self) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method(reviewUnsavedDocumentsWithAlertTitle:cancellable:delegate:didReviewAllSelector:contextInfo:)]
        pub unsafe fn reviewUnsavedDocumentsWithAlertTitle_cancellable_delegate_didReviewAllSelector_contextInfo(
            &self,
            title: Option<&NSString>,
            cancellable: bool,
            delegate: Option<&Object>,
            did_review_all_selector: Option<Sel>,
            context_info: *mut c_void,
        );

        #[method(closeAllDocumentsWithDelegate:didCloseAllSelector:contextInfo:)]
        pub unsafe fn closeAllDocumentsWithDelegate_didCloseAllSelector_contextInfo(
            &self,
            delegate: Option<&Object>,
            did_close_all_selector: Option<Sel>,
            context_info: *mut c_void,
        );

        #[cfg(all(
            feature = "AppKit_NSDocument",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURL"
        ))]
        #[method_id(@__retain_semantics Other duplicateDocumentWithContentsOfURL:copying:displayName:error:_)]
        pub unsafe fn duplicateDocumentWithContentsOfURL_copying_displayName_error(
            &self,
            url: &NSURL,
            duplicate_by_copying: bool,
            display_name_or_nil: Option<&NSString>,
        ) -> Result<Id<NSDocument>, Id<NSError>>;

        #[method(allowsAutomaticShareMenu)]
        pub unsafe fn allowsAutomaticShareMenu(&self) -> bool;

        #[cfg(feature = "AppKit_NSMenuItem")]
        #[method_id(@__retain_semantics Other standardShareMenuItem)]
        pub unsafe fn standardShareMenuItem(&self) -> Id<NSMenuItem>;

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

        #[method(maximumRecentDocumentCount)]
        pub unsafe fn maximumRecentDocumentCount(&self) -> NSUInteger;

        #[method(clearRecentDocuments:)]
        pub unsafe fn clearRecentDocuments(&self, sender: Option<&Object>);

        #[cfg(feature = "AppKit_NSDocument")]
        #[method(noteNewRecentDocument:)]
        pub unsafe fn noteNewRecentDocument(&self, document: &NSDocument);

        #[cfg(feature = "Foundation_NSURL")]
        #[method(noteNewRecentDocumentURL:)]
        pub unsafe fn noteNewRecentDocumentURL(&self, url: &NSURL);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSURL"))]
        #[method_id(@__retain_semantics Other recentDocumentURLs)]
        pub unsafe fn recentDocumentURLs(&self) -> Id<NSArray<NSURL>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other defaultType)]
        pub unsafe fn defaultType(&self) -> Option<Id<NSString>>;

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURL"
        ))]
        #[method_id(@__retain_semantics Other typeForContentsOfURL:error:_)]
        pub unsafe fn typeForContentsOfURL_error(
            &self,
            url: &NSURL,
        ) -> Result<Id<NSString>, Id<NSError>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other documentClassNames)]
        pub unsafe fn documentClassNames(&self) -> Id<NSArray<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(documentClassForType:)]
        pub unsafe fn documentClassForType(&self, type_name: &NSString) -> Option<&'static Class>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other displayNameForType:)]
        pub unsafe fn displayNameForType(&self, type_name: &NSString) -> Option<Id<NSString>>;

        #[method(validateUserInterfaceItem:)]
        pub unsafe fn validateUserInterfaceItem(
            &self,
            item: &ProtocolObject<dyn NSValidatedUserInterfaceItem>,
        ) -> bool;
    }
);

extern_methods!(
    /// NSDeprecated
    #[cfg(feature = "AppKit_NSDocumentController")]
    unsafe impl NSDocumentController {
        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
        #[deprecated = "Use -openDocumentWithContentsOfURL:display:completionHandler: instead"]
        #[method_id(@__retain_semantics Other openDocumentWithContentsOfURL:display:error:_)]
        pub unsafe fn openDocumentWithContentsOfURL_display_error(
            &self,
            url: &NSURL,
            display_document: bool,
        ) -> Result<Id<Object>, Id<NSError>>;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
        #[deprecated = "Use -reopenDocumentForURL:withContentsOfURL:display:completionHandler: instead"]
        #[method(reopenDocumentForURL:withContentsOfURL:error:_)]
        pub unsafe fn reopenDocumentForURL_withContentsOfURL_error(
            &self,
            url: Option<&NSURL>,
            contents_url: &NSURL,
        ) -> Result<(), Id<NSError>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[deprecated]
        #[method_id(@__retain_semantics Other fileExtensionsFromType:)]
        pub unsafe fn fileExtensionsFromType(&self, type_name: &NSString) -> Option<Id<NSArray>>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other typeFromFileExtension:)]
        pub unsafe fn typeFromFileExtension(
            &self,
            file_name_extension_or_hfs_file_type: &NSString,
        ) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other documentForFileName:)]
        pub unsafe fn documentForFileName(&self, file_name: &NSString) -> Option<Id<Object>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[deprecated]
        #[method_id(@__retain_semantics Other fileNamesFromRunningOpenPanel)]
        pub unsafe fn fileNamesFromRunningOpenPanel(&self) -> Option<Id<NSArray>>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other makeDocumentWithContentsOfFile:ofType:)]
        pub unsafe fn makeDocumentWithContentsOfFile_ofType(
            &self,
            file_name: &NSString,
            r#type: &NSString,
        ) -> Option<Id<Object>>;

        #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSURL"))]
        #[deprecated]
        #[method_id(@__retain_semantics Other makeDocumentWithContentsOfURL:ofType:)]
        pub unsafe fn makeDocumentWithContentsOfURL_ofType(
            &self,
            url: &NSURL,
            r#type: Option<&NSString>,
        ) -> Option<Id<Object>>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other makeUntitledDocumentOfType:)]
        pub unsafe fn makeUntitledDocumentOfType(&self, r#type: &NSString) -> Option<Id<Object>>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other openDocumentWithContentsOfFile:display:)]
        pub unsafe fn openDocumentWithContentsOfFile_display(
            &self,
            file_name: &NSString,
            display: bool,
        ) -> Option<Id<Object>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[deprecated]
        #[method_id(@__retain_semantics Other openDocumentWithContentsOfURL:display:)]
        pub unsafe fn openDocumentWithContentsOfURL_display(
            &self,
            url: &NSURL,
            display: bool,
        ) -> Option<Id<Object>>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other openUntitledDocumentOfType:display:)]
        pub unsafe fn openUntitledDocumentOfType_display(
            &self,
            r#type: &NSString,
            display: bool,
        ) -> Option<Id<Object>>;

        #[deprecated]
        #[method(setShouldCreateUI:)]
        pub unsafe fn setShouldCreateUI(&self, flag: bool);

        #[deprecated]
        #[method(shouldCreateUI)]
        pub unsafe fn shouldCreateUI(&self) -> bool;
    }
);
