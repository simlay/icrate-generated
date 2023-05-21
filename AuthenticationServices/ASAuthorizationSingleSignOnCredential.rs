//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AuthenticationServices_ASAuthorizationSingleSignOnCredential")]
    #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
    pub struct ASAuthorizationSingleSignOnCredential;

    #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
    #[cfg(feature = "AuthenticationServices_ASAuthorizationSingleSignOnCredential")]
    unsafe impl ClassType for ASAuthorizationSingleSignOnCredential {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AuthenticationServices_ASAuthorizationSingleSignOnCredential")]
#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
unsafe impl ASAuthorizationCredential for ASAuthorizationSingleSignOnCredential {}

#[cfg(feature = "AuthenticationServices_ASAuthorizationSingleSignOnCredential")]
#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
unsafe impl NSCoding for ASAuthorizationSingleSignOnCredential {}

#[cfg(feature = "AuthenticationServices_ASAuthorizationSingleSignOnCredential")]
#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
unsafe impl NSCopying for ASAuthorizationSingleSignOnCredential {}

#[cfg(feature = "AuthenticationServices_ASAuthorizationSingleSignOnCredential")]
#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
unsafe impl NSObjectProtocol for ASAuthorizationSingleSignOnCredential {}

#[cfg(feature = "AuthenticationServices_ASAuthorizationSingleSignOnCredential")]
#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
unsafe impl NSSecureCoding for ASAuthorizationSingleSignOnCredential {}

#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
extern_methods!(
    #[cfg(feature = "AuthenticationServices_ASAuthorizationSingleSignOnCredential")]
    #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
    unsafe impl ASAuthorizationSingleSignOnCredential {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other state)]
        pub unsafe fn state(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other accessToken)]
        pub unsafe fn accessToken(&self) -> Option<Id<NSData>>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other identityToken)]
        pub unsafe fn identityToken(&self) -> Option<Id<NSData>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other authorizedScopes)]
        pub unsafe fn authorizedScopes(&self) -> Id<NSArray<ASAuthorizationScope>>;

        #[cfg(feature = "Foundation_NSHTTPURLResponse")]
        #[method_id(@__retain_semantics Other authenticatedResponse)]
        pub unsafe fn authenticatedResponse(&self) -> Option<Id<NSHTTPURLResponse>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
        #[method_id(@__retain_semantics Other privateKeys)]
        pub unsafe fn privateKeys(&self) -> Id<NSArray>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);
