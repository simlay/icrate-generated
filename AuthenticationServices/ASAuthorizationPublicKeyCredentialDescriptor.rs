//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
extern_protocol!(
    pub unsafe trait ASAuthorizationPublicKeyCredentialDescriptor:
        NSObjectProtocol + NSSecureCoding
    {
        #[cfg(feature = "Foundation_NSData")]
        #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
        #[method_id(@__retain_semantics Other credentialID)]
        unsafe fn credentialID(&self) -> Id<NSData>;

        #[cfg(feature = "Foundation_NSData")]
        #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
        #[method(setCredentialID:)]
        unsafe fn setCredentialID(&self, credential_id: &NSData);
    }

    unsafe impl ProtocolType for dyn ASAuthorizationPublicKeyCredentialDescriptor {}
);
