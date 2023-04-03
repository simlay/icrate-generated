//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::IdentityLookup::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "IdentityLookup_ILMessageClassificationRequest")]
    #[cfg(not(any(target_os = "macos", target_os = "tvos", target_os = "watchos")))]
    pub struct ILMessageClassificationRequest;

    #[cfg(feature = "IdentityLookup_ILMessageClassificationRequest")]
    unsafe impl ClassType for ILMessageClassificationRequest {
        #[inherits(NSObject)]
        type Super = ILClassificationRequest;
    }
);

#[cfg(feature = "IdentityLookup_ILMessageClassificationRequest")]
unsafe impl NSCoding for ILMessageClassificationRequest {}

#[cfg(feature = "IdentityLookup_ILMessageClassificationRequest")]
unsafe impl NSObjectProtocol for ILMessageClassificationRequest {}

#[cfg(feature = "IdentityLookup_ILMessageClassificationRequest")]
unsafe impl NSSecureCoding for ILMessageClassificationRequest {}

extern_methods!(
    #[cfg(feature = "IdentityLookup_ILMessageClassificationRequest")]
    unsafe impl ILMessageClassificationRequest {
        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "IdentityLookup_ILMessageCommunication"
        ))]
        #[method_id(@__retain_semantics Other messageCommunications)]
        pub unsafe fn messageCommunications(&self) -> Id<NSArray<ILMessageCommunication>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);
