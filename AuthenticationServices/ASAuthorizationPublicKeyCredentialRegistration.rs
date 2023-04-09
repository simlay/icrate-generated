//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
extern_protocol!(
    pub unsafe trait ASAuthorizationPublicKeyCredentialRegistration:
        ASPublicKeyCredential
    {
        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other rawAttestationObject)]
        unsafe fn rawAttestationObject(&self) -> Option<Id<NSData>>;
    }

    unsafe impl ProtocolType for dyn ASAuthorizationPublicKeyCredentialRegistration {}
);
