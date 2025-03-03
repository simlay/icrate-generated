//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AuthenticationServices_ASAuthorizationSingleSignOnRequest")]
    pub struct ASAuthorizationSingleSignOnRequest;

    #[cfg(feature = "AuthenticationServices_ASAuthorizationSingleSignOnRequest")]
    unsafe impl ClassType for ASAuthorizationSingleSignOnRequest {
        #[inherits(ASAuthorizationRequest, NSObject)]
        type Super = ASAuthorizationOpenIDRequest;
    }
);

#[cfg(feature = "AuthenticationServices_ASAuthorizationSingleSignOnRequest")]
unsafe impl NSCoding for ASAuthorizationSingleSignOnRequest {}

#[cfg(feature = "AuthenticationServices_ASAuthorizationSingleSignOnRequest")]
unsafe impl NSObjectProtocol for ASAuthorizationSingleSignOnRequest {}

#[cfg(feature = "AuthenticationServices_ASAuthorizationSingleSignOnRequest")]
unsafe impl NSSecureCoding for ASAuthorizationSingleSignOnRequest {}

extern_methods!(
    #[cfg(feature = "AuthenticationServices_ASAuthorizationSingleSignOnRequest")]
    unsafe impl ASAuthorizationSingleSignOnRequest {
        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSURLQueryItem"))]
        #[method_id(@__retain_semantics Other authorizationOptions)]
        pub unsafe fn authorizationOptions(&self) -> Id<NSArray<NSURLQueryItem>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSURLQueryItem"))]
        #[method(setAuthorizationOptions:)]
        pub unsafe fn setAuthorizationOptions(
            &self,
            authorization_options: &NSArray<NSURLQueryItem>,
        );

        #[method(isUserInterfaceEnabled)]
        pub unsafe fn isUserInterfaceEnabled(&self) -> bool;

        #[method(setUserInterfaceEnabled:)]
        pub unsafe fn setUserInterfaceEnabled(&self, user_interface_enabled: bool);
    }
);
