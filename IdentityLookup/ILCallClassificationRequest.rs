//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::IdentityLookup::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "IdentityLookup_ILCallClassificationRequest")]
    #[cfg(not(any(target_os = "macos")))]
    pub struct ILCallClassificationRequest;

    #[cfg(feature = "IdentityLookup_ILCallClassificationRequest")]
    unsafe impl ClassType for ILCallClassificationRequest {
        #[inherits(NSObject)]
        type Super = ILClassificationRequest;
    }
);

#[cfg(feature = "IdentityLookup_ILCallClassificationRequest")]
unsafe impl NSCoding for ILCallClassificationRequest {}

#[cfg(feature = "IdentityLookup_ILCallClassificationRequest")]
unsafe impl NSObjectProtocol for ILCallClassificationRequest {}

#[cfg(feature = "IdentityLookup_ILCallClassificationRequest")]
unsafe impl NSSecureCoding for ILCallClassificationRequest {}

extern_methods!(
    #[cfg(feature = "IdentityLookup_ILCallClassificationRequest")]
    unsafe impl ILCallClassificationRequest {
        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "IdentityLookup_ILCallCommunication"
        ))]
        #[method_id(@__retain_semantics Other callCommunications)]
        pub unsafe fn callCommunications(&self) -> Id<NSArray<ILCallCommunication>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);
