//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSAccessibilityCustomRotorSearchDirection {
        NSAccessibilityCustomRotorSearchDirectionPrevious = 0,
        NSAccessibilityCustomRotorSearchDirectionNext = 1,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSAccessibilityCustomRotorType {
        NSAccessibilityCustomRotorTypeCustom = 0,
        NSAccessibilityCustomRotorTypeAny = 1,
        NSAccessibilityCustomRotorTypeAnnotation = 2,
        NSAccessibilityCustomRotorTypeBoldText = 3,
        NSAccessibilityCustomRotorTypeHeading = 4,
        NSAccessibilityCustomRotorTypeHeadingLevel1 = 5,
        NSAccessibilityCustomRotorTypeHeadingLevel2 = 6,
        NSAccessibilityCustomRotorTypeHeadingLevel3 = 7,
        NSAccessibilityCustomRotorTypeHeadingLevel4 = 8,
        NSAccessibilityCustomRotorTypeHeadingLevel5 = 9,
        NSAccessibilityCustomRotorTypeHeadingLevel6 = 10,
        NSAccessibilityCustomRotorTypeImage = 11,
        NSAccessibilityCustomRotorTypeItalicText = 12,
        NSAccessibilityCustomRotorTypeLandmark = 13,
        NSAccessibilityCustomRotorTypeLink = 14,
        NSAccessibilityCustomRotorTypeList = 15,
        NSAccessibilityCustomRotorTypeMisspelledWord = 16,
        NSAccessibilityCustomRotorTypeTable = 17,
        NSAccessibilityCustomRotorTypeTextField = 18,
        NSAccessibilityCustomRotorTypeUnderlinedText = 19,
        NSAccessibilityCustomRotorTypeVisitedLink = 20,
        NSAccessibilityCustomRotorTypeAudiograph = 21,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSAccessibilityCustomRotor")]
    pub struct NSAccessibilityCustomRotor;

    #[cfg(feature = "AppKit_NSAccessibilityCustomRotor")]
    unsafe impl ClassType for NSAccessibilityCustomRotor {
        type Super = NSObject;
    }
);

#[cfg(feature = "AppKit_NSAccessibilityCustomRotor")]
unsafe impl NSObjectProtocol for NSAccessibilityCustomRotor {}

extern_methods!(
    #[cfg(feature = "AppKit_NSAccessibilityCustomRotor")]
    unsafe impl NSAccessibilityCustomRotor {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithLabel:itemSearchDelegate:)]
        pub unsafe fn initWithLabel_itemSearchDelegate(
            this: Option<Allocated<Self>>,
            label: &NSString,
            item_search_delegate: &ProtocolObject<dyn NSAccessibilityCustomRotorItemSearchDelegate>,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithRotorType:itemSearchDelegate:)]
        pub unsafe fn initWithRotorType_itemSearchDelegate(
            this: Option<Allocated<Self>>,
            rotor_type: NSAccessibilityCustomRotorType,
            item_search_delegate: &ProtocolObject<dyn NSAccessibilityCustomRotorItemSearchDelegate>,
        ) -> Id<Self>;

        #[method(type)]
        pub unsafe fn r#type(&self) -> NSAccessibilityCustomRotorType;

        #[method(setType:)]
        pub unsafe fn setType(&self, r#type: NSAccessibilityCustomRotorType);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other label)]
        pub unsafe fn label(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setLabel:)]
        pub unsafe fn setLabel(&self, label: &NSString);

        #[method_id(@__retain_semantics Other itemSearchDelegate)]
        pub unsafe fn itemSearchDelegate(
            &self,
        ) -> Option<Id<ProtocolObject<dyn NSAccessibilityCustomRotorItemSearchDelegate>>>;

        #[method(setItemSearchDelegate:)]
        pub unsafe fn setItemSearchDelegate(
            &self,
            item_search_delegate: Option<
                &ProtocolObject<dyn NSAccessibilityCustomRotorItemSearchDelegate>,
            >,
        );

        #[method_id(@__retain_semantics Other itemLoadingDelegate)]
        pub unsafe fn itemLoadingDelegate(
            &self,
        ) -> Option<Id<ProtocolObject<dyn NSAccessibilityElementLoading>>>;

        #[method(setItemLoadingDelegate:)]
        pub unsafe fn setItemLoadingDelegate(
            &self,
            item_loading_delegate: Option<&ProtocolObject<dyn NSAccessibilityElementLoading>>,
        );
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSAccessibilityCustomRotorSearchParameters")]
    pub struct NSAccessibilityCustomRotorSearchParameters;

    #[cfg(feature = "AppKit_NSAccessibilityCustomRotorSearchParameters")]
    unsafe impl ClassType for NSAccessibilityCustomRotorSearchParameters {
        type Super = NSObject;
    }
);

#[cfg(feature = "AppKit_NSAccessibilityCustomRotorSearchParameters")]
unsafe impl NSObjectProtocol for NSAccessibilityCustomRotorSearchParameters {}

extern_methods!(
    #[cfg(feature = "AppKit_NSAccessibilityCustomRotorSearchParameters")]
    unsafe impl NSAccessibilityCustomRotorSearchParameters {
        #[cfg(feature = "AppKit_NSAccessibilityCustomRotorItemResult")]
        #[method_id(@__retain_semantics Other currentItem)]
        pub unsafe fn currentItem(&self) -> Option<Id<NSAccessibilityCustomRotorItemResult>>;

        #[cfg(feature = "AppKit_NSAccessibilityCustomRotorItemResult")]
        #[method(setCurrentItem:)]
        pub unsafe fn setCurrentItem(
            &self,
            current_item: Option<&NSAccessibilityCustomRotorItemResult>,
        );

        #[method(searchDirection)]
        pub unsafe fn searchDirection(&self) -> NSAccessibilityCustomRotorSearchDirection;

        #[method(setSearchDirection:)]
        pub unsafe fn setSearchDirection(
            &self,
            search_direction: NSAccessibilityCustomRotorSearchDirection,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other filterString)]
        pub unsafe fn filterString(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setFilterString:)]
        pub unsafe fn setFilterString(&self, filter_string: &NSString);
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSAccessibilityCustomRotorItemResult")]
    pub struct NSAccessibilityCustomRotorItemResult;

    #[cfg(feature = "AppKit_NSAccessibilityCustomRotorItemResult")]
    unsafe impl ClassType for NSAccessibilityCustomRotorItemResult {
        type Super = NSObject;
    }
);

#[cfg(feature = "AppKit_NSAccessibilityCustomRotorItemResult")]
unsafe impl NSObjectProtocol for NSAccessibilityCustomRotorItemResult {}

extern_methods!(
    #[cfg(feature = "AppKit_NSAccessibilityCustomRotorItemResult")]
    unsafe impl NSAccessibilityCustomRotorItemResult {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithTargetElement:)]
        pub unsafe fn initWithTargetElement(
            this: Option<Allocated<Self>>,
            target_element: &ProtocolObject<dyn NSAccessibilityElementProtocol>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithItemLoadingToken:customLabel:)]
        pub unsafe fn initWithItemLoadingToken_customLabel(
            this: Option<Allocated<Self>>,
            item_loading_token: &NSAccessibilityLoadingToken,
            custom_label: &NSString,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Other targetElement)]
        pub unsafe fn targetElement(
            &self,
        ) -> Option<Id<ProtocolObject<dyn NSAccessibilityElementProtocol>>>;

        #[method_id(@__retain_semantics Other itemLoadingToken)]
        pub unsafe fn itemLoadingToken(&self) -> Option<Id<NSAccessibilityLoadingToken>>;

        #[method(targetRange)]
        pub unsafe fn targetRange(&self) -> NSRange;

        #[method(setTargetRange:)]
        pub unsafe fn setTargetRange(&self, target_range: NSRange);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other customLabel)]
        pub unsafe fn customLabel(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setCustomLabel:)]
        pub unsafe fn setCustomLabel(&self, custom_label: Option<&NSString>);
    }
);

extern_protocol!(
    pub unsafe trait NSAccessibilityCustomRotorItemSearchDelegate: NSObjectProtocol {
        #[cfg(all(
            feature = "AppKit_NSAccessibilityCustomRotor",
            feature = "AppKit_NSAccessibilityCustomRotorItemResult",
            feature = "AppKit_NSAccessibilityCustomRotorSearchParameters"
        ))]
        #[method_id(@__retain_semantics Other rotor:resultForSearchParameters:)]
        unsafe fn rotor_resultForSearchParameters(
            &self,
            rotor: &NSAccessibilityCustomRotor,
            search_parameters: &NSAccessibilityCustomRotorSearchParameters,
        ) -> Option<Id<NSAccessibilityCustomRotorItemResult>>;
    }

    unsafe impl ProtocolType for dyn NSAccessibilityCustomRotorItemSearchDelegate {}
);
