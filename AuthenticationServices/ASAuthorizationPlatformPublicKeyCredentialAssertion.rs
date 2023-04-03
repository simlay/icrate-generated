//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AuthenticationServices_ASAuthorizationPlatformPublicKeyCredentialAssertion")]
    #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
    pub struct ASAuthorizationPlatformPublicKeyCredentialAssertion;

    #[cfg(feature = "AuthenticationServices_ASAuthorizationPlatformPublicKeyCredentialAssertion")]
    unsafe impl ClassType for ASAuthorizationPlatformPublicKeyCredentialAssertion {
        type Super = NSObject;
    }
);

#[cfg(feature = "AuthenticationServices_ASAuthorizationPlatformPublicKeyCredentialAssertion")]
unsafe impl ASAuthorizationCredential for ASAuthorizationPlatformPublicKeyCredentialAssertion {}

#[cfg(feature = "AuthenticationServices_ASAuthorizationPlatformPublicKeyCredentialAssertion")]
unsafe impl ASAuthorizationPublicKeyCredentialAssertion
    for ASAuthorizationPlatformPublicKeyCredentialAssertion
{
}

#[cfg(feature = "AuthenticationServices_ASAuthorizationPlatformPublicKeyCredentialAssertion")]
unsafe impl ASPublicKeyCredential for ASAuthorizationPlatformPublicKeyCredentialAssertion {}

#[cfg(feature = "AuthenticationServices_ASAuthorizationPlatformPublicKeyCredentialAssertion")]
unsafe impl NSCoding for ASAuthorizationPlatformPublicKeyCredentialAssertion {}

#[cfg(feature = "AuthenticationServices_ASAuthorizationPlatformPublicKeyCredentialAssertion")]
unsafe impl NSObjectProtocol for ASAuthorizationPlatformPublicKeyCredentialAssertion {}

#[cfg(feature = "AuthenticationServices_ASAuthorizationPlatformPublicKeyCredentialAssertion")]
unsafe impl NSSecureCoding for ASAuthorizationPlatformPublicKeyCredentialAssertion {}

extern_methods!(
    #[cfg(feature = "AuthenticationServices_ASAuthorizationPlatformPublicKeyCredentialAssertion")]
    unsafe impl ASAuthorizationPlatformPublicKeyCredentialAssertion {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);
