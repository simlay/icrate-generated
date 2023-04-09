//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AuthenticationServices_ASPasswordCredentialIdentity")]
    #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
    pub struct ASPasswordCredentialIdentity;

    #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
    #[cfg(feature = "AuthenticationServices_ASPasswordCredentialIdentity")]
    unsafe impl ClassType for ASPasswordCredentialIdentity {
        type Super = NSObject;
    }
);

#[cfg(feature = "AuthenticationServices_ASPasswordCredentialIdentity")]
#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
unsafe impl NSCoding for ASPasswordCredentialIdentity {}

#[cfg(feature = "AuthenticationServices_ASPasswordCredentialIdentity")]
#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
unsafe impl NSObjectProtocol for ASPasswordCredentialIdentity {}

#[cfg(feature = "AuthenticationServices_ASPasswordCredentialIdentity")]
#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
unsafe impl NSSecureCoding for ASPasswordCredentialIdentity {}

#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
extern_methods!(
    #[cfg(feature = "AuthenticationServices_ASPasswordCredentialIdentity")]
    unsafe impl ASPasswordCredentialIdentity {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[cfg(all(
            feature = "AuthenticationServices_ASCredentialServiceIdentifier",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Init initWithServiceIdentifier:user:recordIdentifier:)]
        pub unsafe fn initWithServiceIdentifier_user_recordIdentifier(
            this: Option<Allocated<Self>>,
            service_identifier: &ASCredentialServiceIdentifier,
            user: &NSString,
            record_identifier: Option<&NSString>,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "AuthenticationServices_ASCredentialServiceIdentifier",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other identityWithServiceIdentifier:user:recordIdentifier:)]
        pub unsafe fn identityWithServiceIdentifier_user_recordIdentifier(
            service_identifier: &ASCredentialServiceIdentifier,
            user: &NSString,
            record_identifier: Option<&NSString>,
        ) -> Id<Self>;

        #[cfg(feature = "AuthenticationServices_ASCredentialServiceIdentifier")]
        #[method_id(@__retain_semantics Other serviceIdentifier)]
        pub unsafe fn serviceIdentifier(&self) -> Id<ASCredentialServiceIdentifier>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other user)]
        pub unsafe fn user(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other recordIdentifier)]
        pub unsafe fn recordIdentifier(&self) -> Option<Id<NSString>>;

        #[method(rank)]
        pub unsafe fn rank(&self) -> NSInteger;

        #[method(setRank:)]
        pub unsafe fn setRank(&self, rank: NSInteger);
    }
);
