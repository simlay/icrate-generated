//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::MailKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MailKit_MEAddressAnnotation")]
    #[cfg(not(any(target_os = "ios", target_os = "tvos", target_os = "watchos")))]
    pub struct MEAddressAnnotation;

    #[cfg(feature = "MailKit_MEAddressAnnotation")]
    unsafe impl ClassType for MEAddressAnnotation {
        type Super = NSObject;
    }
);

#[cfg(feature = "MailKit_MEAddressAnnotation")]
unsafe impl NSCoding for MEAddressAnnotation {}

#[cfg(feature = "MailKit_MEAddressAnnotation")]
unsafe impl NSObjectProtocol for MEAddressAnnotation {}

#[cfg(feature = "MailKit_MEAddressAnnotation")]
unsafe impl NSSecureCoding for MEAddressAnnotation {}

extern_methods!(
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
