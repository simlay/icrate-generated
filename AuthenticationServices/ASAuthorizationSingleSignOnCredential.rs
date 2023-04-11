//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AuthenticationServices_ASAuthorizationSingleSignOnCredential")]
    pub struct ASAuthorizationSingleSignOnCredential;

    #[cfg(feature = "AuthenticationServices_ASAuthorizationSingleSignOnCredential")]
    unsafe impl ClassType for ASAuthorizationSingleSignOnCredential {
        type Super = NSObject;
    }
);

#[cfg(feature = "AuthenticationServices_ASAuthorizationSingleSignOnCredential")]
unsafe impl ASAuthorizationCredential for ASAuthorizationSingleSignOnCredential {}

#[cfg(feature = "AuthenticationServices_ASAuthorizationSingleSignOnCredential")]
unsafe impl NSCoding for ASAuthorizationSingleSignOnCredential {}

#[cfg(feature = "AuthenticationServices_ASAuthorizationSingleSignOnCredential")]
unsafe impl NSObjectProtocol for ASAuthorizationSingleSignOnCredential {}

#[cfg(feature = "AuthenticationServices_ASAuthorizationSingleSignOnCredential")]
unsafe impl NSSecureCoding for ASAuthorizationSingleSignOnCredential {}

extern_methods!(
    #[cfg(feature = "AuthenticationServices_ASAuthorizationSingleSignOnCredential")]
    unsafe impl ASAuthorizationSingleSignOnCredential {
        #[cfg(feature = "Foundation_NSString")]
        /**
          @abstract A state returned from the AuthenticationServices extension.
        */
        #[method_id(@__retain_semantics Other state)]
        pub unsafe fn state(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSData")]
        /**
          @abstract An access token used to access other systems with the authorized scopes.
        */
        #[method_id(@__retain_semantics Other accessToken)]
        pub unsafe fn accessToken(&self) -> Option<Id<NSData>>;

        #[cfg(feature = "Foundation_NSData")]
        /**
          @abstract A JSON Web Token (JWT) used to communicate information about the identity of the user in a secure way to the app.
        */
        #[method_id(@__retain_semantics Other identityToken)]
        pub unsafe fn identityToken(&self) -> Option<Id<NSData>>;

        #[cfg(feature = "Foundation_NSArray")]
        /**
          @abstract This value will contain a list of scopes for which the user provided authorization.  These may contain a subset of the requested scopes on @see ASAuthorizationOpenIDRequest.  The application should query this value to identify which scopes were returned as it maybe different from ones requested.
        */
        #[method_id(@__retain_semantics Other authorizedScopes)]
        pub unsafe fn authorizedScopes(&self) -> Id<NSArray<ASAuthorizationScope>>;

        #[cfg(feature = "Foundation_NSHTTPURLResponse")]
        /**
          @abstract The complete AuthenticationServices extension response with the additional outputs used by the specific technology used by the Authorization Server instance and AuthenticationServices Extension.
        @note for some operations all properties can be null and the response will indicate just successful result of the operation.
        */
        #[method_id(@__retain_semantics Other authenticatedResponse)]
        pub unsafe fn authenticatedResponse(&self) -> Option<Id<NSHTTPURLResponse>>;

        #[cfg(feature = "Foundation_NSArray")]
        /**
          @abstract Private SecKeys returned from the AuthenticationServices extension.
        */
        #[method_id(@__retain_semantics Other privateKeys)]
        pub unsafe fn privateKeys(&self) -> Id<NSArray>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);
