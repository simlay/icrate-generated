//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AuthenticationServices_ASAuthorizationSecurityKeyPublicKeyCredentialProvider")]
    #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
    pub struct ASAuthorizationSecurityKeyPublicKeyCredentialProvider;

    #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
    #[cfg(feature = "AuthenticationServices_ASAuthorizationSecurityKeyPublicKeyCredentialProvider")]
    unsafe impl ClassType for ASAuthorizationSecurityKeyPublicKeyCredentialProvider {
        type Super = NSObject;
    }
);

#[cfg(feature = "AuthenticationServices_ASAuthorizationSecurityKeyPublicKeyCredentialProvider")]
#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
unsafe impl ASAuthorizationProvider for ASAuthorizationSecurityKeyPublicKeyCredentialProvider {}

#[cfg(feature = "AuthenticationServices_ASAuthorizationSecurityKeyPublicKeyCredentialProvider")]
#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
unsafe impl NSObjectProtocol for ASAuthorizationSecurityKeyPublicKeyCredentialProvider {}

extern_methods!(
    #[cfg(feature = "AuthenticationServices_ASAuthorizationSecurityKeyPublicKeyCredentialProvider")]
    #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
    unsafe impl ASAuthorizationSecurityKeyPublicKeyCredentialProvider {
        #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithRelyingPartyIdentifier:)]
        pub unsafe fn initWithRelyingPartyIdentifier(
            this: Option<Allocated<Self>>,
            relying_party_identifier: &NSString,
        ) -> Id<Self>;

        #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
        #[cfg(all(
            feature = "AuthenticationServices_ASAuthorizationSecurityKeyPublicKeyCredentialRegistrationRequest",
            feature = "Foundation_NSData",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other createCredentialRegistrationRequestWithChallenge:displayName:name:userID:)]
        pub unsafe fn createCredentialRegistrationRequestWithChallenge_displayName_name_userID(
            &self,
            challenge: &NSData,
            display_name: &NSString,
            name: &NSString,
            user_id: &NSData,
        ) -> Id<ASAuthorizationSecurityKeyPublicKeyCredentialRegistrationRequest>;

        #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
        #[cfg(all(
            feature = "AuthenticationServices_ASAuthorizationSecurityKeyPublicKeyCredentialAssertionRequest",
            feature = "Foundation_NSData"
        ))]
        #[method_id(@__retain_semantics Other createCredentialAssertionRequestWithChallenge:)]
        pub unsafe fn createCredentialAssertionRequestWithChallenge(
            &self,
            challenge: &NSData,
        ) -> Id<ASAuthorizationSecurityKeyPublicKeyCredentialAssertionRequest>;

        #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other relyingPartyIdentifier)]
        pub unsafe fn relyingPartyIdentifier(&self) -> Id<NSString>;

        #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
