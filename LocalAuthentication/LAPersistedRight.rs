//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::LocalAuthentication::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "LocalAuthentication_LAPersistedRight")]
    #[cfg(not(any(target_os = "watchos")))]
    pub struct LAPersistedRight;

    #[cfg(not(any(target_os = "watchos")))]
    #[cfg(feature = "LocalAuthentication_LAPersistedRight")]
    unsafe impl ClassType for LAPersistedRight {
        #[inherits(NSObject)]
        type Super = LARight;
    }
);

#[cfg(feature = "LocalAuthentication_LAPersistedRight")]
#[cfg(not(any(target_os = "watchos")))]
unsafe impl NSObjectProtocol for LAPersistedRight {}

#[cfg(not(any(target_os = "watchos")))]
extern_methods!(
    #[cfg(feature = "LocalAuthentication_LAPersistedRight")]
    unsafe impl LAPersistedRight {
        #[cfg(feature = "LocalAuthentication_LAPrivateKey")]
        #[method_id(@__retain_semantics Other key)]
        pub unsafe fn key(&self) -> Id<LAPrivateKey>;

        #[cfg(feature = "LocalAuthentication_LASecret")]
        #[method_id(@__retain_semantics Other secret)]
        pub unsafe fn secret(&self) -> Id<LASecret>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

#[cfg(not(any(target_os = "watchos")))]
extern_methods!(
    /// Methods declared on superclass `LARight`
    #[cfg(feature = "LocalAuthentication_LAPersistedRight")]
    unsafe impl LAPersistedRight {
        #[cfg(feature = "LocalAuthentication_LAAuthenticationRequirement")]
        #[method_id(@__retain_semantics Init initWithRequirement:)]
        pub unsafe fn initWithRequirement(
            this: Option<Allocated<Self>>,
            requirement: &LAAuthenticationRequirement,
        ) -> Id<Self>;
    }
);
