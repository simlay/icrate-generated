//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::MailKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MailKit_MEAddressAnnotation")]
    /**
      @brief An instance of this class can be used to change the visual style of recipeint email address token when user in composing a mail message.
    */
    pub struct MEAddressAnnotation;

    #[cfg(feature = "MailKit_MEAddressAnnotation")]
    unsafe impl ClassType for MEAddressAnnotation {
        type Super = NSObject;
    }
);

#[cfg(feature = "MailKit_MEAddressAnnotation")]
/**
  @brief An instance of this class can be used to change the visual style of recipeint email address token when user in composing a mail message.
*/
unsafe impl NSCoding for MEAddressAnnotation {}

#[cfg(feature = "MailKit_MEAddressAnnotation")]
/**
  @brief An instance of this class can be used to change the visual style of recipeint email address token when user in composing a mail message.
*/
unsafe impl NSObjectProtocol for MEAddressAnnotation {}

#[cfg(feature = "MailKit_MEAddressAnnotation")]
/**
  @brief An instance of this class can be used to change the visual style of recipeint email address token when user in composing a mail message.
*/
unsafe impl NSSecureCoding for MEAddressAnnotation {}

extern_methods!(
    /**
      @brief An instance of this class can be used to change the visual style of recipeint email address token when user in composing a mail message.
    */
    #[cfg(feature = "MailKit_MEAddressAnnotation")]
    unsafe impl MEAddressAnnotation {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other errorWithLocalizedDescription:)]
        pub unsafe fn errorWithLocalizedDescription(
            localized_description: &NSString,
        ) -> Id<MEAddressAnnotation>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other warningWithLocalizedDescription:)]
        pub unsafe fn warningWithLocalizedDescription(
            localized_description: &NSString,
        ) -> Id<MEAddressAnnotation>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other successWithLocalizedDescription:)]
        pub unsafe fn successWithLocalizedDescription(
            localized_description: &NSString,
        ) -> Id<MEAddressAnnotation>;
    }
);
