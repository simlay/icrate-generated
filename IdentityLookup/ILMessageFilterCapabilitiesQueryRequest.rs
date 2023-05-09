//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::IdentityLookup::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "IdentityLookup_ILMessageFilterCapabilitiesQueryRequest")]
    #[cfg(not(any(target_os = "macos")))]
    pub struct ILMessageFilterCapabilitiesQueryRequest;

    #[cfg(feature = "IdentityLookup_ILMessageFilterCapabilitiesQueryRequest")]
    unsafe impl ClassType for ILMessageFilterCapabilitiesQueryRequest {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "IdentityLookup_ILMessageFilterCapabilitiesQueryRequest")]
#[cfg(not(any(target_os = "macos")))]
unsafe impl NSCoding for ILMessageFilterCapabilitiesQueryRequest {}

#[cfg(feature = "IdentityLookup_ILMessageFilterCapabilitiesQueryRequest")]
#[cfg(not(any(target_os = "macos")))]
unsafe impl NSObjectProtocol for ILMessageFilterCapabilitiesQueryRequest {}

#[cfg(feature = "IdentityLookup_ILMessageFilterCapabilitiesQueryRequest")]
#[cfg(not(any(target_os = "macos")))]
unsafe impl NSSecureCoding for ILMessageFilterCapabilitiesQueryRequest {}

#[cfg(not(any(target_os = "macos")))]
extern_methods!(
    #[cfg(feature = "IdentityLookup_ILMessageFilterCapabilitiesQueryRequest")]
    unsafe impl ILMessageFilterCapabilitiesQueryRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "IdentityLookup_ILMessageFilterCapabilitiesQueryRequest")]
    unsafe impl ILMessageFilterCapabilitiesQueryRequest {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
