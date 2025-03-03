//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AuthenticationServices_ASAuthorizationAppleIDRequest")]
    pub struct ASAuthorizationAppleIDRequest;

    #[cfg(feature = "AuthenticationServices_ASAuthorizationAppleIDRequest")]
    unsafe impl ClassType for ASAuthorizationAppleIDRequest {
        #[inherits(ASAuthorizationRequest, NSObject)]
        type Super = ASAuthorizationOpenIDRequest;
    }
);

#[cfg(feature = "AuthenticationServices_ASAuthorizationAppleIDRequest")]
unsafe impl NSCoding for ASAuthorizationAppleIDRequest {}

#[cfg(feature = "AuthenticationServices_ASAuthorizationAppleIDRequest")]
unsafe impl NSObjectProtocol for ASAuthorizationAppleIDRequest {}

#[cfg(feature = "AuthenticationServices_ASAuthorizationAppleIDRequest")]
unsafe impl NSSecureCoding for ASAuthorizationAppleIDRequest {}

extern_methods!(
    #[cfg(feature = "AuthenticationServices_ASAuthorizationAppleIDRequest")]
    unsafe impl ASAuthorizationAppleIDRequest {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other user)]
        pub unsafe fn user(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setUser:)]
        pub unsafe fn setUser(&self, user: Option<&NSString>);
    }
);
