//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AuthenticationServices_ASAuthorizationRequest")]
    pub struct ASAuthorizationRequest;

    #[cfg(feature = "AuthenticationServices_ASAuthorizationRequest")]
    unsafe impl ClassType for ASAuthorizationRequest {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AuthenticationServices_ASAuthorizationRequest")]
unsafe impl NSCoding for ASAuthorizationRequest {}

#[cfg(feature = "AuthenticationServices_ASAuthorizationRequest")]
unsafe impl NSCopying for ASAuthorizationRequest {}

#[cfg(feature = "AuthenticationServices_ASAuthorizationRequest")]
unsafe impl NSObjectProtocol for ASAuthorizationRequest {}

#[cfg(feature = "AuthenticationServices_ASAuthorizationRequest")]
unsafe impl NSSecureCoding for ASAuthorizationRequest {}

extern_methods!(
    #[cfg(feature = "AuthenticationServices_ASAuthorizationRequest")]
    unsafe impl ASAuthorizationRequest {
        #[method_id(@__retain_semantics Other provider)]
        pub unsafe fn provider(&self) -> Id<ProtocolObject<dyn ASAuthorizationProvider>>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);
