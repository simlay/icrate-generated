//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::IdentityLookup::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "IdentityLookup_ILClassificationRequest")]
    #[cfg(not(any(target_os = "macos", target_os = "tvos", target_os = "watchos")))]
    pub struct ILClassificationRequest;

    #[cfg(feature = "IdentityLookup_ILClassificationRequest")]
    unsafe impl ClassType for ILClassificationRequest {
        type Super = NSObject;
    }
);

#[cfg(feature = "IdentityLookup_ILClassificationRequest")]
unsafe impl NSCoding for ILClassificationRequest {}

#[cfg(feature = "IdentityLookup_ILClassificationRequest")]
unsafe impl NSObjectProtocol for ILClassificationRequest {}

#[cfg(feature = "IdentityLookup_ILClassificationRequest")]
unsafe impl NSSecureCoding for ILClassificationRequest {}

extern_methods!(
    #[cfg(feature = "IdentityLookup_ILClassificationRequest")]
    unsafe impl ILClassificationRequest {}
);
