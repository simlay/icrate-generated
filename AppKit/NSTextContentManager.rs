//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSTextContentManagerEnumerationOptions {
        NSTextContentManagerEnumerationOptionsNone = 0,
        NSTextContentManagerEnumerationOptionsReverse = 1 << 0,
    }
);

extern_protocol!(
    pub unsafe trait NSTextElementProvider: NSObjectProtocol {
        #[cfg(feature = "AppKit_NSTextRange")]
        #[method_id(@__retain_semantics Other documentRange)]
        unsafe fn documentRange(&self) -> Id<NSTextRange>;

        #[cfg(feature = "AppKit_NSTextElement")]
        #[method_id(@__retain_semantics Other enumerateTextElementsFromLocation:options:usingBlock:)]
        unsafe fn enumerateTextElementsFromLocation_options_usingBlock(
            &self,
            text_location: Option<&ProtocolObject<dyn NSTextLocation>>,
            options: NSTextContentManagerEnumerationOptions,
            block: &Block<(NonNull<NSTextElement>,), Bool>,
        ) -> Option<Id<ProtocolObject<dyn NSTextLocation>>>;

        #[cfg(all(
            feature = "AppKit_NSTextElement",
            feature = "AppKit_NSTextRange",
            feature = "Foundation_NSArray"
        ))]
        #[method(replaceContentsInRange:withTextElements:)]
        unsafe fn replaceContentsInRange_withTextElements(
            &self,
            range: &NSTextRange,
            text_elements: Option<&NSArray<NSTextElement>>,
        );

        #[cfg(feature = "Foundation_NSError")]
        #[method(synchronizeToBackingStore:)]
        unsafe fn synchronizeToBackingStore(
            &self,
            completion_handler: Option<&Block<(*mut NSError,), ()>>,
        );

        #[optional]
        #[method_id(@__retain_semantics Other locationFromLocation:withOffset:)]
        unsafe fn locationFromLocation_withOffset(
            &self,
            location: &ProtocolObject<dyn NSTextLocation>,
            offset: NSInteger,
        ) -> Option<Id<ProtocolObject<dyn NSTextLocation>>>;

        #[optional]
        #[method(offsetFromLocation:toLocation:)]
        unsafe fn offsetFromLocation_toLocation(
            &self,
            from: &ProtocolObject<dyn NSTextLocation>,
            to: &ProtocolObject<dyn NSTextLocation>,
        ) -> NSInteger;

        #[cfg(feature = "AppKit_NSTextRange")]
        #[optional]
        #[method_id(@__retain_semantics Other adjustedRangeFromRange:forEditingTextSelection:)]
        unsafe fn adjustedRangeFromRange_forEditingTextSelection(
            &self,
            text_range: &NSTextRange,
            for_editing_text_selection: bool,
        ) -> Option<Id<NSTextRange>>;
    }

    unsafe impl ProtocolType for dyn NSTextElementProvider {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSTextContentManager")]
    pub struct NSTextContentManager;

    #[cfg(feature = "AppKit_NSTextContentManager")]
    unsafe impl ClassType for NSTextContentManager {
        type Super = NSObject;
    }
);

#[cfg(feature = "AppKit_NSTextContentManager")]
unsafe impl NSCoding for NSTextContentManager {}

#[cfg(feature = "AppKit_NSTextContentManager")]
unsafe impl NSObjectProtocol for NSTextContentManager {}

#[cfg(feature = "AppKit_NSTextContentManager")]
unsafe impl NSSecureCoding for NSTextContentManager {}

#[cfg(feature = "AppKit_NSTextContentManager")]
unsafe impl NSTextElementProvider for NSTextContentManager {}

extern_methods!(
    #[cfg(feature = "AppKit_NSTextContentManager")]
    unsafe impl NSTextContentManager {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self>>;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Id<ProtocolObject<dyn NSTextContentManagerDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn NSTextContentManagerDelegate>>,
        );

        #[cfg(all(feature = "AppKit_NSTextLayoutManager", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other textLayoutManagers)]
        pub unsafe fn textLayoutManagers(&self) -> Id<NSArray<NSTextLayoutManager>>;

        #[cfg(feature = "AppKit_NSTextLayoutManager")]
        #[method(addTextLayoutManager:)]
        pub unsafe fn addTextLayoutManager(&self, text_layout_manager: &NSTextLayoutManager);

        #[cfg(feature = "AppKit_NSTextLayoutManager")]
        #[method(removeTextLayoutManager:)]
        pub unsafe fn removeTextLayoutManager(&self, text_layout_manager: &NSTextLayoutManager);

        #[cfg(feature = "AppKit_NSTextLayoutManager")]
        #[method_id(@__retain_semantics Other primaryTextLayoutManager)]
        pub unsafe fn primaryTextLayoutManager(&self) -> Option<Id<NSTextLayoutManager>>;

        #[cfg(feature = "AppKit_NSTextLayoutManager")]
        #[method(setPrimaryTextLayoutManager:)]
        pub unsafe fn setPrimaryTextLayoutManager(
            &self,
            primary_text_layout_manager: Option<&NSTextLayoutManager>,
        );

        #[cfg(feature = "Foundation_NSError")]
        #[method(synchronizeTextLayoutManagers:)]
        pub unsafe fn synchronizeTextLayoutManagers(
            &self,
            completion_handler: Option<&Block<(*mut NSError,), ()>>,
        );

        #[cfg(all(
            feature = "AppKit_NSTextElement",
            feature = "AppKit_NSTextRange",
            feature = "Foundation_NSArray"
        ))]
        #[method_id(@__retain_semantics Other textElementsForRange:)]
        pub unsafe fn textElementsForRange(
            &self,
            range: &NSTextRange,
        ) -> Id<NSArray<NSTextElement>>;

        #[method(hasEditingTransaction)]
        pub unsafe fn hasEditingTransaction(&self) -> bool;

        #[method(performEditingTransactionUsingBlock:)]
        pub unsafe fn performEditingTransactionUsingBlock(&self, transaction: &Block<(), ()>);

        #[cfg(feature = "AppKit_NSTextRange")]
        #[method(recordEditActionInRange:newTextRange:)]
        pub unsafe fn recordEditActionInRange_newTextRange(
            &self,
            original_text_range: &NSTextRange,
            new_text_range: &NSTextRange,
        );

        #[method(automaticallySynchronizesTextLayoutManagers)]
        pub unsafe fn automaticallySynchronizesTextLayoutManagers(&self) -> bool;

        #[method(setAutomaticallySynchronizesTextLayoutManagers:)]
        pub unsafe fn setAutomaticallySynchronizesTextLayoutManagers(
            &self,
            automatically_synchronizes_text_layout_managers: bool,
        );

        #[method(automaticallySynchronizesToBackingStore)]
        pub unsafe fn automaticallySynchronizesToBackingStore(&self) -> bool;

        #[method(setAutomaticallySynchronizesToBackingStore:)]
        pub unsafe fn setAutomaticallySynchronizesToBackingStore(
            &self,
            automatically_synchronizes_to_backing_store: bool,
        );
    }
);

extern_protocol!(
    pub unsafe trait NSTextContentManagerDelegate: NSObjectProtocol {
        #[cfg(all(
            feature = "AppKit_NSTextContentManager",
            feature = "AppKit_NSTextElement"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other textContentManager:textElementAtLocation:)]
        unsafe fn textContentManager_textElementAtLocation(
            &self,
            text_content_manager: &NSTextContentManager,
            location: &ProtocolObject<dyn NSTextLocation>,
        ) -> Option<Id<NSTextElement>>;

        #[cfg(all(
            feature = "AppKit_NSTextContentManager",
            feature = "AppKit_NSTextElement"
        ))]
        #[optional]
        #[method(textContentManager:shouldEnumerateTextElement:options:)]
        unsafe fn textContentManager_shouldEnumerateTextElement_options(
            &self,
            text_content_manager: &NSTextContentManager,
            text_element: &NSTextElement,
            options: NSTextContentManagerEnumerationOptions,
        ) -> bool;
    }

    unsafe impl ProtocolType for dyn NSTextContentManagerDelegate {}
);

extern_protocol!(
    pub unsafe trait NSTextContentStorageDelegate: NSTextContentManagerDelegate {
        #[cfg(all(
            feature = "AppKit_NSTextContentStorage",
            feature = "AppKit_NSTextParagraph"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other textContentStorage:textParagraphWithRange:)]
        unsafe fn textContentStorage_textParagraphWithRange(
            &self,
            text_content_storage: &NSTextContentStorage,
            range: NSRange,
        ) -> Option<Id<NSTextParagraph>>;
    }

    unsafe impl ProtocolType for dyn NSTextContentStorageDelegate {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSTextContentStorage")]
    pub struct NSTextContentStorage;

    #[cfg(feature = "AppKit_NSTextContentStorage")]
    unsafe impl ClassType for NSTextContentStorage {
        #[inherits(NSObject)]
        type Super = NSTextContentManager;
    }
);

#[cfg(feature = "AppKit_NSTextContentStorage")]
unsafe impl NSCoding for NSTextContentStorage {}

#[cfg(feature = "AppKit_NSTextContentStorage")]
unsafe impl NSObjectProtocol for NSTextContentStorage {}

#[cfg(feature = "AppKit_NSTextContentStorage")]
unsafe impl NSSecureCoding for NSTextContentStorage {}

#[cfg(feature = "AppKit_NSTextContentStorage")]
unsafe impl NSTextElementProvider for NSTextContentStorage {}

#[cfg(feature = "AppKit_NSTextContentStorage")]
unsafe impl NSTextStorageObserving for NSTextContentStorage {}

extern_methods!(
    #[cfg(feature = "AppKit_NSTextContentStorage")]
    unsafe impl NSTextContentStorage {
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Id<ProtocolObject<dyn NSTextContentStorageDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn NSTextContentStorageDelegate>>,
        );

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method_id(@__retain_semantics Other attributedString)]
        pub unsafe fn attributedString(&self) -> Option<Id<NSAttributedString>>;

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method(setAttributedString:)]
        pub unsafe fn setAttributedString(&self, attributed_string: Option<&NSAttributedString>);

        #[cfg(all(
            feature = "AppKit_NSTextElement",
            feature = "Foundation_NSAttributedString"
        ))]
        #[method_id(@__retain_semantics Other attributedStringForTextElement:)]
        pub unsafe fn attributedStringForTextElement(
            &self,
            text_element: &NSTextElement,
        ) -> Option<Id<NSAttributedString>>;

        #[cfg(all(
            feature = "AppKit_NSTextElement",
            feature = "Foundation_NSAttributedString"
        ))]
        #[method_id(@__retain_semantics Other textElementForAttributedString:)]
        pub unsafe fn textElementForAttributedString(
            &self,
            attributed_string: &NSAttributedString,
        ) -> Option<Id<NSTextElement>>;

        #[method_id(@__retain_semantics Other locationFromLocation:withOffset:)]
        pub unsafe fn locationFromLocation_withOffset(
            &self,
            location: &ProtocolObject<dyn NSTextLocation>,
            offset: NSInteger,
        ) -> Option<Id<ProtocolObject<dyn NSTextLocation>>>;

        #[method(offsetFromLocation:toLocation:)]
        pub unsafe fn offsetFromLocation_toLocation(
            &self,
            from: &ProtocolObject<dyn NSTextLocation>,
            to: &ProtocolObject<dyn NSTextLocation>,
        ) -> NSInteger;

        #[cfg(feature = "AppKit_NSTextRange")]
        #[method_id(@__retain_semantics Other adjustedRangeFromRange:forEditingTextSelection:)]
        pub unsafe fn adjustedRangeFromRange_forEditingTextSelection(
            &self,
            text_range: &NSTextRange,
            for_editing_text_selection: bool,
        ) -> Option<Id<NSTextRange>>;
    }
);

extern_static!(
    NSTextContentStorageUnsupportedAttributeAddedNotification: &'static NSNotificationName
);
