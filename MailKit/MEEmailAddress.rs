//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::MailKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MailKit_MEEmailAddress")]
    pub struct MEEmailAddress;

    #[cfg(feature = "MailKit_MEEmailAddress")]
    unsafe impl ClassType for MEEmailAddress {
        type Super = NSObject;
    }
);

#[cfg(feature = "MailKit_MEEmailAddress")]
unsafe impl NSCoding for MEEmailAddress {}

#[cfg(feature = "MailKit_MEEmailAddress")]
unsafe impl NSObjectProtocol for MEEmailAddress {}

#[cfg(feature = "MailKit_MEEmailAddress")]
unsafe impl NSSecureCoding for MEEmailAddress {}

extern_methods!(
    #[cfg(feature = "MailKit_MEEmailAddress")]
    unsafe impl MEEmailAddress {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other rawString)]
        pub unsafe fn rawString(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other addressString)]
        pub unsafe fn addressString(&self) -> Option<Id<NSString>>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithRawString:)]
        pub unsafe fn initWithRawString(
            this: Option<Allocated<Self>>,
            raw_string: &NSString,
        ) -> Id<Self>;
    }
);
