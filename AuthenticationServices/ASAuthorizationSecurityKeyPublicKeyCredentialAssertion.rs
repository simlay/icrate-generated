//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(
        feature = "AuthenticationServices_ASAuthorizationSecurityKeyPublicKeyCredentialAssertion"
    )]
    #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
    pub struct ASAuthorizationSecurityKeyPublicKeyCredentialAssertion;

    #[cfg(
        feature = "AuthenticationServices_ASAuthorizationSecurityKeyPublicKeyCredentialAssertion"
    )]
    unsafe impl ClassType for ASAuthorizationSecurityKeyPublicKeyCredentialAssertion {
        type Super = NSObject;
    }
);

#[cfg(feature = "AuthenticationServices_ASAuthorizationSecurityKeyPublicKeyCredentialAssertion")]
unsafe impl ASAuthorizationCredential for ASAuthorizationSecurityKeyPublicKeyCredentialAssertion {}

#[cfg(feature = "AuthenticationServices_ASAuthorizationSecurityKeyPublicKeyCredentialAssertion")]
unsafe impl ASAuthorizationPublicKeyCredentialAssertion
    for ASAuthorizationSecurityKeyPublicKeyCredentialAssertion
{
}

#[cfg(feature = "AuthenticationServices_ASAuthorizationSecurityKeyPublicKeyCredentialAssertion")]
unsafe impl ASPublicKeyCredential for ASAuthorizationSecurityKeyPublicKeyCredentialAssertion {}

#[cfg(feature = "AuthenticationServices_ASAuthorizationSecurityKeyPublicKeyCredentialAssertion")]
unsafe impl NSCoding for ASAuthorizationSecurityKeyPublicKeyCredentialAssertion {}

#[cfg(feature = "AuthenticationServices_ASAuthorizationSecurityKeyPublicKeyCredentialAssertion")]
unsafe impl NSObjectProtocol for ASAuthorizationSecurityKeyPublicKeyCredentialAssertion {}

#[cfg(feature = "AuthenticationServices_ASAuthorizationSecurityKeyPublicKeyCredentialAssertion")]
unsafe impl NSSecureCoding for ASAuthorizationSecurityKeyPublicKeyCredentialAssertion {}

extern_methods!(
    #[cfg(
        feature = "AuthenticationServices_ASAuthorizationSecurityKeyPublicKeyCredentialAssertion"
    )]
    unsafe impl ASAuthorizationSecurityKeyPublicKeyCredentialAssertion {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);
