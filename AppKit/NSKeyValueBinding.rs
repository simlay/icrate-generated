//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

typed_extensible_enum!(
    pub type NSBindingName = NSString;
);

typed_enum!(
    pub type NSBindingOption = NSString;
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSBindingSelectionMarker")]
    #[cfg(not(any(target_os = "ios")))]
    pub struct NSBindingSelectionMarker;

    #[cfg(feature = "AppKit_NSBindingSelectionMarker")]
    unsafe impl ClassType for NSBindingSelectionMarker {
        type Super = NSObject;
    }
);

#[cfg(feature = "AppKit_NSBindingSelectionMarker")]
unsafe impl NSObjectProtocol for NSBindingSelectionMarker {}

extern_methods!(
    #[cfg(feature = "AppKit_NSBindingSelectionMarker")]
    unsafe impl NSBindingSelectionMarker {
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other multipleValuesSelectionMarker)]
        pub unsafe fn multipleValuesSelectionMarker() -> Id<NSBindingSelectionMarker>;

        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other noSelectionMarker)]
        pub unsafe fn noSelectionMarker() -> Id<NSBindingSelectionMarker>;

        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other notApplicableSelectionMarker)]
        pub unsafe fn notApplicableSelectionMarker() -> Id<NSBindingSelectionMarker>;

        #[cfg(not(any(target_os = "ios")))]
        #[method(setDefaultPlaceholder:forMarker:onClass:withBinding:)]
        pub unsafe fn setDefaultPlaceholder_forMarker_onClass_withBinding(
            placeholder: Option<&Object>,
            marker: Option<&NSBindingSelectionMarker>,
            object_class: &Class,
            binding: &NSBindingName,
        );

        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other defaultPlaceholderForMarker:onClass:withBinding:)]
        pub unsafe fn defaultPlaceholderForMarker_onClass_withBinding(
            marker: Option<&NSBindingSelectionMarker>,
            object_class: &Class,
            binding: &NSBindingName,
        ) -> Option<Id<Object>>;
    }
);

extern_static!(NSMultipleValuesMarker: &'static Object);

extern_static!(NSNoSelectionMarker: &'static Object);

extern_static!(NSNotApplicableMarker: &'static Object);

extern_fn!(
    #[cfg(not(any(target_os = "ios")))]
    pub unsafe fn NSIsControllerMarker(object: Option<&Object>) -> Bool;
);

typed_enum!(
    pub type NSBindingInfoKey = NSString;
);

extern_static!(NSObservedObjectKey: &'static NSBindingInfoKey);

extern_static!(NSObservedKeyPathKey: &'static NSBindingInfoKey);

extern_static!(NSOptionsKey: &'static NSBindingInfoKey);

extern_protocol!(
    #[cfg(not(any(target_os = "ios")))]
    pub unsafe trait NSEditor: NSObjectProtocol {
        #[cfg(not(any(target_os = "ios")))]
        #[method(discardEditing)]
        unsafe fn discardEditing(&self);

        #[cfg(not(any(target_os = "ios")))]
        #[method(commitEditing)]
        unsafe fn commitEditing(&self) -> bool;

        #[cfg(not(any(target_os = "ios")))]
        #[method(commitEditingWithDelegate:didCommitSelector:contextInfo:)]
        unsafe fn commitEditingWithDelegate_didCommitSelector_contextInfo(
            &self,
            delegate: Option<&Object>,
            did_commit_selector: Option<Sel>,
            context_info: *mut c_void,
        );

        #[cfg(feature = "Foundation_NSError")]
        #[cfg(not(any(target_os = "ios")))]
        #[method(commitEditingAndReturnError:_)]
        unsafe fn commitEditingAndReturnError(&self) -> Result<(), Id<NSError>>;
    }

    unsafe impl ProtocolType for dyn NSEditor {}
);

extern_protocol!(
    #[cfg(not(any(target_os = "ios")))]
    pub unsafe trait NSEditorRegistration: NSObjectProtocol {
        #[cfg(not(any(target_os = "ios")))]
        #[optional]
        #[method(objectDidBeginEditing:)]
        unsafe fn objectDidBeginEditing(&self, editor: &ProtocolObject<dyn NSEditor>);

        #[cfg(not(any(target_os = "ios")))]
        #[optional]
        #[method(objectDidEndEditing:)]
        unsafe fn objectDidEndEditing(&self, editor: &ProtocolObject<dyn NSEditor>);
    }

    unsafe impl ProtocolType for dyn NSEditorRegistration {}
);

extern_static!(NSAlignmentBinding: &'static NSBindingName);

extern_static!(NSAlternateImageBinding: &'static NSBindingName);

extern_static!(NSAlternateTitleBinding: &'static NSBindingName);

extern_static!(NSAnimateBinding: &'static NSBindingName);

extern_static!(NSAnimationDelayBinding: &'static NSBindingName);

extern_static!(NSArgumentBinding: &'static NSBindingName);

extern_static!(NSAttributedStringBinding: &'static NSBindingName);

extern_static!(NSContentArrayBinding: &'static NSBindingName);

extern_static!(NSContentArrayForMultipleSelectionBinding: &'static NSBindingName);

extern_static!(NSContentBinding: &'static NSBindingName);

extern_static!(NSContentDictionaryBinding: &'static NSBindingName);

extern_static!(NSContentHeightBinding: &'static NSBindingName);

extern_static!(NSContentObjectBinding: &'static NSBindingName);

extern_static!(NSContentObjectsBinding: &'static NSBindingName);

extern_static!(NSContentSetBinding: &'static NSBindingName);

extern_static!(NSContentValuesBinding: &'static NSBindingName);

extern_static!(NSContentWidthBinding: &'static NSBindingName);

extern_static!(NSCriticalValueBinding: &'static NSBindingName);

extern_static!(NSDataBinding: &'static NSBindingName);

extern_static!(NSDisplayPatternTitleBinding: &'static NSBindingName);

extern_static!(NSDisplayPatternValueBinding: &'static NSBindingName);

extern_static!(NSDocumentEditedBinding: &'static NSBindingName);

extern_static!(NSDoubleClickArgumentBinding: &'static NSBindingName);

extern_static!(NSDoubleClickTargetBinding: &'static NSBindingName);

extern_static!(NSEditableBinding: &'static NSBindingName);

extern_static!(NSEnabledBinding: &'static NSBindingName);

extern_static!(NSExcludedKeysBinding: &'static NSBindingName);

extern_static!(NSFilterPredicateBinding: &'static NSBindingName);

extern_static!(NSFontBinding: &'static NSBindingName);

extern_static!(NSFontBoldBinding: &'static NSBindingName);

extern_static!(NSFontFamilyNameBinding: &'static NSBindingName);

extern_static!(NSFontItalicBinding: &'static NSBindingName);

extern_static!(NSFontNameBinding: &'static NSBindingName);

extern_static!(NSFontSizeBinding: &'static NSBindingName);

extern_static!(NSHeaderTitleBinding: &'static NSBindingName);

extern_static!(NSHiddenBinding: &'static NSBindingName);

extern_static!(NSImageBinding: &'static NSBindingName);

extern_static!(NSIncludedKeysBinding: &'static NSBindingName);

extern_static!(NSInitialKeyBinding: &'static NSBindingName);

extern_static!(NSInitialValueBinding: &'static NSBindingName);

extern_static!(NSIsIndeterminateBinding: &'static NSBindingName);

extern_static!(NSLabelBinding: &'static NSBindingName);

extern_static!(NSLocalizedKeyDictionaryBinding: &'static NSBindingName);

extern_static!(NSManagedObjectContextBinding: &'static NSBindingName);

extern_static!(NSMaximumRecentsBinding: &'static NSBindingName);

extern_static!(NSMaxValueBinding: &'static NSBindingName);

extern_static!(NSMaxWidthBinding: &'static NSBindingName);

extern_static!(NSMinValueBinding: &'static NSBindingName);

extern_static!(NSMinWidthBinding: &'static NSBindingName);

extern_static!(NSMixedStateImageBinding: &'static NSBindingName);

extern_static!(NSOffStateImageBinding: &'static NSBindingName);

extern_static!(NSOnStateImageBinding: &'static NSBindingName);

extern_static!(NSPositioningRectBinding: &'static NSBindingName);

extern_static!(NSPredicateBinding: &'static NSBindingName);

extern_static!(NSRecentSearchesBinding: &'static NSBindingName);

extern_static!(NSRepresentedFilenameBinding: &'static NSBindingName);

extern_static!(NSRowHeightBinding: &'static NSBindingName);

extern_static!(NSSelectedIdentifierBinding: &'static NSBindingName);

extern_static!(NSSelectedIndexBinding: &'static NSBindingName);

extern_static!(NSSelectedLabelBinding: &'static NSBindingName);

extern_static!(NSSelectedObjectBinding: &'static NSBindingName);

extern_static!(NSSelectedObjectsBinding: &'static NSBindingName);

extern_static!(NSSelectedTagBinding: &'static NSBindingName);

extern_static!(NSSelectedValueBinding: &'static NSBindingName);

extern_static!(NSSelectedValuesBinding: &'static NSBindingName);

extern_static!(NSSelectionIndexesBinding: &'static NSBindingName);

extern_static!(NSSelectionIndexPathsBinding: &'static NSBindingName);

extern_static!(NSSortDescriptorsBinding: &'static NSBindingName);

extern_static!(NSTargetBinding: &'static NSBindingName);

extern_static!(NSTextColorBinding: &'static NSBindingName);

extern_static!(NSTitleBinding: &'static NSBindingName);

extern_static!(NSToolTipBinding: &'static NSBindingName);

extern_static!(NSTransparentBinding: &'static NSBindingName);

extern_static!(NSValueBinding: &'static NSBindingName);

extern_static!(NSValuePathBinding: &'static NSBindingName);

extern_static!(NSValueURLBinding: &'static NSBindingName);

extern_static!(NSVisibleBinding: &'static NSBindingName);

extern_static!(NSWarningValueBinding: &'static NSBindingName);

extern_static!(NSWidthBinding: &'static NSBindingName);

extern_static!(NSAllowsEditingMultipleValuesSelectionBindingOption: &'static NSBindingOption);

extern_static!(NSAllowsNullArgumentBindingOption: &'static NSBindingOption);

extern_static!(NSAlwaysPresentsApplicationModalAlertsBindingOption: &'static NSBindingOption);

extern_static!(NSConditionallySetsEditableBindingOption: &'static NSBindingOption);

extern_static!(NSConditionallySetsEnabledBindingOption: &'static NSBindingOption);

extern_static!(NSConditionallySetsHiddenBindingOption: &'static NSBindingOption);

extern_static!(NSContinuouslyUpdatesValueBindingOption: &'static NSBindingOption);

extern_static!(NSCreatesSortDescriptorBindingOption: &'static NSBindingOption);

extern_static!(NSDeletesObjectsOnRemoveBindingsOption: &'static NSBindingOption);

extern_static!(NSDisplayNameBindingOption: &'static NSBindingOption);

extern_static!(NSDisplayPatternBindingOption: &'static NSBindingOption);

extern_static!(NSContentPlacementTagBindingOption: &'static NSBindingOption);

extern_static!(NSHandlesContentAsCompoundValueBindingOption: &'static NSBindingOption);

extern_static!(NSInsertsNullPlaceholderBindingOption: &'static NSBindingOption);

extern_static!(NSInvokesSeparatelyWithArrayObjectsBindingOption: &'static NSBindingOption);

extern_static!(NSMultipleValuesPlaceholderBindingOption: &'static NSBindingOption);

extern_static!(NSNoSelectionPlaceholderBindingOption: &'static NSBindingOption);

extern_static!(NSNotApplicablePlaceholderBindingOption: &'static NSBindingOption);

extern_static!(NSNullPlaceholderBindingOption: &'static NSBindingOption);

extern_static!(NSRaisesForNotApplicableKeysBindingOption: &'static NSBindingOption);

extern_static!(NSPredicateFormatBindingOption: &'static NSBindingOption);

extern_static!(NSSelectorNameBindingOption: &'static NSBindingOption);

extern_static!(NSSelectsAllWhenSettingContentBindingOption: &'static NSBindingOption);

extern_static!(NSValidatesImmediatelyBindingOption: &'static NSBindingOption);

extern_static!(NSValueTransformerNameBindingOption: &'static NSBindingOption);

extern_static!(NSValueTransformerBindingOption: &'static NSBindingOption);

extern_methods!(
    /// NSEditorAndEditorRegistrationConformance
    #[cfg(feature = "CoreData_NSManagedObjectContext")]
    unsafe impl NSManagedObjectContext {}
);

#[cfg(feature = "CoreData_NSManagedObjectContext")]
unsafe impl NSEditor for NSManagedObjectContext {}

#[cfg(feature = "CoreData_NSManagedObjectContext")]
unsafe impl NSEditorRegistration for NSManagedObjectContext {}
