//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::IdentityLookup::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "IdentityLookup_ILCommunication")]
    #[cfg(not(any(target_os = "macos")))]
    pub struct ILCommunication;

    #[cfg(feature = "IdentityLookup_ILCommunication")]
    unsafe impl ClassType for ILCommunication {
        type Super = NSObject;
    }
);

#[cfg(feature = "IdentityLookup_ILCommunication")]
unsafe impl NSCoding for ILCommunication {}

#[cfg(feature = "IdentityLookup_ILCommunication")]
unsafe impl NSObjectProtocol for ILCommunication {}

#[cfg(feature = "IdentityLookup_ILCommunication")]
unsafe impl NSSecureCoding for ILCommunication {}

extern_methods!(
    #[cfg(feature = "IdentityLookup_ILCommunication")]
    unsafe impl ILCommunication {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other sender)]
        pub unsafe fn sender(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other dateReceived)]
        pub unsafe fn dateReceived(&self) -> Id<NSDate>;

        #[method(isEqualToCommunication:)]
        pub unsafe fn isEqualToCommunication(&self, communication: &ILCommunication) -> bool;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);
