//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::ClassKit::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum CLSBinaryValueType {
        CLSBinaryValueTypeTrueFalse = 0,
        CLSBinaryValueTypePassFail = 1,
        CLSBinaryValueTypeYesNo = 2,
        CLSBinaryValueTypeCorrectIncorrect = 3,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "ClassKit_CLSBinaryItem")]
    /**
     @abstract      CLSBinaryItem represents user generated information that is true or false, pass or fail, yes or no.
    */
    pub struct CLSBinaryItem;

    #[cfg(feature = "ClassKit_CLSBinaryItem")]
    unsafe impl ClassType for CLSBinaryItem {
        #[inherits(CLSObject, NSObject)]
        type Super = CLSActivityItem;
    }
);

#[cfg(feature = "ClassKit_CLSBinaryItem")]
/**
 @abstract      CLSBinaryItem represents user generated information that is true or false, pass or fail, yes or no.
*/
unsafe impl NSCoding for CLSBinaryItem {}

#[cfg(feature = "ClassKit_CLSBinaryItem")]
/**
 @abstract      CLSBinaryItem represents user generated information that is true or false, pass or fail, yes or no.
*/
unsafe impl NSObjectProtocol for CLSBinaryItem {}

#[cfg(feature = "ClassKit_CLSBinaryItem")]
/**
 @abstract      CLSBinaryItem represents user generated information that is true or false, pass or fail, yes or no.
*/
unsafe impl NSSecureCoding for CLSBinaryItem {}

extern_methods!(
    /**
     @abstract      CLSBinaryItem represents user generated information that is true or false, pass or fail, yes or no.
    */
    #[cfg(feature = "ClassKit_CLSBinaryItem")]
    unsafe impl CLSBinaryItem {
        /**
         @abstract      True or false value.
        */
        #[method(value)]
        pub unsafe fn value(&self) -> bool;

        /**
         @abstract      True or false value.
        */
        #[method(setValue:)]
        pub unsafe fn setValue(&self, value: bool);

        /**
         @abstract      Value type of this CLSBinaryItem.
        @discussion    The type that best describes this CLSBinaryItem value.
        */
        #[method(valueType)]
        pub unsafe fn valueType(&self) -> CLSBinaryValueType;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithIdentifier:title:type:)]
        pub unsafe fn initWithIdentifier_title_type(
            this: Option<Allocated<Self>>,
            identifier: &NSString,
            title: &NSString,
            value_type: CLSBinaryValueType,
        ) -> Id<Self>;
    }
);
