//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AuthenticationServices_ASCredentialIdentityStoreState")]
    #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
    pub struct ASCredentialIdentityStoreState;

    #[cfg(feature = "AuthenticationServices_ASCredentialIdentityStoreState")]
    unsafe impl ClassType for ASCredentialIdentityStoreState {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AuthenticationServices_ASCredentialIdentityStoreState")]
#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
unsafe impl NSObjectProtocol for ASCredentialIdentityStoreState {}

#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
extern_methods!(
    #[cfg(feature = "AuthenticationServices_ASCredentialIdentityStoreState")]
    unsafe impl ASCredentialIdentityStoreState {
        #[method(isEnabled)]
        pub unsafe fn isEnabled(&self) -> bool;

        #[method(supportsIncrementalUpdates)]
        pub unsafe fn supportsIncrementalUpdates(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AuthenticationServices_ASCredentialIdentityStoreState")]
    unsafe impl ASCredentialIdentityStoreState {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
