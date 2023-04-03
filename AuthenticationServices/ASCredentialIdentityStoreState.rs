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
    }
);

#[cfg(feature = "AuthenticationServices_ASCredentialIdentityStoreState")]
unsafe impl NSObjectProtocol for ASCredentialIdentityStoreState {}

extern_methods!(
    #[cfg(feature = "AuthenticationServices_ASCredentialIdentityStoreState")]
    unsafe impl ASCredentialIdentityStoreState {
        #[method(isEnabled)]
        pub unsafe fn isEnabled(&self) -> bool;

        #[method(supportsIncrementalUpdates)]
        pub unsafe fn supportsIncrementalUpdates(&self) -> bool;
    }
);
