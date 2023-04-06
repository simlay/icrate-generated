//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
extern_static!(ASCredentialIdentityStoreErrorDomain: &'static NSErrorDomain);

#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
ns_error_enum!(
    #[underlying(NSInteger)]
    pub enum ASCredentialIdentityStoreErrorCode {
        #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
        ASCredentialIdentityStoreErrorCodeInternalError = 0,
        #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
        ASCredentialIdentityStoreErrorCodeStoreDisabled = 1,
        #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
        ASCredentialIdentityStoreErrorCodeStoreBusy = 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AuthenticationServices_ASCredentialIdentityStore")]
    #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
    pub struct ASCredentialIdentityStore;

    #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
    #[cfg(feature = "AuthenticationServices_ASCredentialIdentityStore")]
    unsafe impl ClassType for ASCredentialIdentityStore {
        type Super = NSObject;
    }
);

#[cfg(feature = "AuthenticationServices_ASCredentialIdentityStore")]
#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
unsafe impl NSObjectProtocol for ASCredentialIdentityStore {}

extern_methods!(
    #[cfg(feature = "AuthenticationServices_ASCredentialIdentityStore")]
    #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
    unsafe impl ASCredentialIdentityStore {
        #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
        #[method_id(@__retain_semantics Other sharedStore)]
        pub unsafe fn sharedStore() -> Id<ASCredentialIdentityStore>;

        #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
        #[cfg(feature = "AuthenticationServices_ASCredentialIdentityStoreState")]
        #[method(getCredentialIdentityStoreStateWithCompletion:)]
        pub unsafe fn getCredentialIdentityStoreStateWithCompletion(
            &self,
            completion: &Block<(NonNull<ASCredentialIdentityStoreState>,), ()>,
        );

        #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
        #[cfg(all(
            feature = "AuthenticationServices_ASPasswordCredentialIdentity",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError"
        ))]
        #[method(saveCredentialIdentities:completion:)]
        pub unsafe fn saveCredentialIdentities_completion(
            &self,
            credential_identities: &NSArray<ASPasswordCredentialIdentity>,
            completion: Option<&Block<(Bool, *mut NSError), ()>>,
        );

        #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
        #[cfg(all(
            feature = "AuthenticationServices_ASPasswordCredentialIdentity",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError"
        ))]
        #[method(removeCredentialIdentities:completion:)]
        pub unsafe fn removeCredentialIdentities_completion(
            &self,
            credential_identities: &NSArray<ASPasswordCredentialIdentity>,
            completion: Option<&Block<(Bool, *mut NSError), ()>>,
        );

        #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
        #[cfg(feature = "Foundation_NSError")]
        #[method(removeAllCredentialIdentitiesWithCompletion:)]
        pub unsafe fn removeAllCredentialIdentitiesWithCompletion(
            &self,
            completion: Option<&Block<(Bool, *mut NSError), ()>>,
        );

        #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
        #[cfg(all(
            feature = "AuthenticationServices_ASPasswordCredentialIdentity",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError"
        ))]
        #[method(replaceCredentialIdentitiesWithIdentities:completion:)]
        pub unsafe fn replaceCredentialIdentitiesWithIdentities_completion(
            &self,
            new_credential_identities: &NSArray<ASPasswordCredentialIdentity>,
            completion: Option<&Block<(Bool, *mut NSError), ()>>,
        );
    }
);
