//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AuthenticationServices_ASAuthorizationSingleSignOnProvider")]
    pub struct ASAuthorizationSingleSignOnProvider;

    #[cfg(feature = "AuthenticationServices_ASAuthorizationSingleSignOnProvider")]
    unsafe impl ClassType for ASAuthorizationSingleSignOnProvider {
        type Super = NSObject;
    }
);

#[cfg(feature = "AuthenticationServices_ASAuthorizationSingleSignOnProvider")]
unsafe impl ASAuthorizationProvider for ASAuthorizationSingleSignOnProvider {}

#[cfg(feature = "AuthenticationServices_ASAuthorizationSingleSignOnProvider")]
unsafe impl NSObjectProtocol for ASAuthorizationSingleSignOnProvider {}

extern_methods!(
    #[cfg(feature = "AuthenticationServices_ASAuthorizationSingleSignOnProvider")]
    unsafe impl ASAuthorizationSingleSignOnProvider {
        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other authorizationProviderWithIdentityProviderURL:)]
        pub unsafe fn authorizationProviderWithIdentityProviderURL(url: &NSURL) -> Id<Self>;

        #[cfg(feature = "AuthenticationServices_ASAuthorizationSingleSignOnRequest")]
        #[method_id(@__retain_semantics Other createRequest)]
        pub unsafe fn createRequest(&self) -> Id<ASAuthorizationSingleSignOnRequest>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other url)]
        pub unsafe fn url(&self) -> Id<NSURL>;

        #[method(canPerformAuthorization)]
        pub unsafe fn canPerformAuthorization(&self) -> bool;
    }
);
