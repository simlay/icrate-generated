//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
typed_extensible_enum!(
    pub type ASAuthorizationProviderAuthorizationOperation = NSString;
);

#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
extern_static!(
    ASAuthorizationProviderAuthorizationOperationConfigurationRemoved:
        &'static ASAuthorizationProviderAuthorizationOperation
);

#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
extern_static!(
    ASAuthorizationProviderAuthorizationOperationDirectRequest:
        &'static ASAuthorizationProviderAuthorizationOperation
);

#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
extern_protocol!(
    pub unsafe trait ASAuthorizationProviderExtensionAuthorizationRequestHandler:
        NSObjectProtocol
    {
        #[cfg(
            feature = "AuthenticationServices_ASAuthorizationProviderExtensionAuthorizationRequest"
        )]
        #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
        #[method(beginAuthorizationWithRequest:)]
        unsafe fn beginAuthorizationWithRequest(
            &self,
            request: &ASAuthorizationProviderExtensionAuthorizationRequest,
        );

        #[cfg(
            feature = "AuthenticationServices_ASAuthorizationProviderExtensionAuthorizationRequest"
        )]
        #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
        #[optional]
        #[method(cancelAuthorizationWithRequest:)]
        unsafe fn cancelAuthorizationWithRequest(
            &self,
            request: &ASAuthorizationProviderExtensionAuthorizationRequest,
        );
    }

    unsafe impl ProtocolType for dyn ASAuthorizationProviderExtensionAuthorizationRequestHandler {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AuthenticationServices_ASAuthorizationProviderExtensionAuthorizationRequest")]
    #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
    pub struct ASAuthorizationProviderExtensionAuthorizationRequest;

    #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
    #[cfg(feature = "AuthenticationServices_ASAuthorizationProviderExtensionAuthorizationRequest")]
    unsafe impl ClassType for ASAuthorizationProviderExtensionAuthorizationRequest {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AuthenticationServices_ASAuthorizationProviderExtensionAuthorizationRequest")]
#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
unsafe impl NSObjectProtocol for ASAuthorizationProviderExtensionAuthorizationRequest {}

#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
extern_methods!(
    #[cfg(feature = "AuthenticationServices_ASAuthorizationProviderExtensionAuthorizationRequest")]
    #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
    unsafe impl ASAuthorizationProviderExtensionAuthorizationRequest {
        #[method(doNotHandle)]
        pub unsafe fn doNotHandle(&self);

        #[method(cancel)]
        pub unsafe fn cancel(&self);

        #[method(complete)]
        pub unsafe fn complete(&self);

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method(completeWithHTTPAuthorizationHeaders:)]
        pub unsafe fn completeWithHTTPAuthorizationHeaders(
            &self,
            http_authorization_headers: &NSDictionary<NSString, NSString>,
        );

        #[cfg(all(
            feature = "Foundation_NSData",
            feature = "Foundation_NSHTTPURLResponse"
        ))]
        #[method(completeWithHTTPResponse:httpBody:)]
        pub unsafe fn completeWithHTTPResponse_httpBody(
            &self,
            http_response: &NSHTTPURLResponse,
            http_body: Option<&NSData>,
        );

        #[cfg(
            feature = "AuthenticationServices_ASAuthorizationProviderExtensionAuthorizationResult"
        )]
        #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
        #[method(completeWithAuthorizationResult:)]
        pub unsafe fn completeWithAuthorizationResult(
            &self,
            authorization_result: &ASAuthorizationProviderExtensionAuthorizationResult,
        );

        #[cfg(feature = "Foundation_NSError")]
        #[method(completeWithError:)]
        pub unsafe fn completeWithError(&self, error: &NSError);

        #[cfg(feature = "Foundation_NSError")]
        #[method(presentAuthorizationViewControllerWithCompletion:)]
        pub unsafe fn presentAuthorizationViewControllerWithCompletion(
            &self,
            completion: &Block<(Bool, *mut NSError), ()>,
        );

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other url)]
        pub unsafe fn url(&self) -> Id<NSURL>;

        #[method_id(@__retain_semantics Other requestedOperation)]
        pub unsafe fn requestedOperation(
            &self,
        ) -> Id<ASAuthorizationProviderAuthorizationOperation>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other httpHeaders)]
        pub unsafe fn httpHeaders(&self) -> Id<NSDictionary<NSString, NSString>>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other httpBody)]
        pub unsafe fn httpBody(&self) -> Id<NSData>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other realm)]
        pub unsafe fn realm(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method_id(@__retain_semantics Other extensionData)]
        pub unsafe fn extensionData(&self) -> Id<NSDictionary>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other callerBundleIdentifier)]
        pub unsafe fn callerBundleIdentifier(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method_id(@__retain_semantics Other authorizationOptions)]
        pub unsafe fn authorizationOptions(&self) -> Id<NSDictionary>;

        #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
        #[method(isCallerManaged)]
        pub unsafe fn isCallerManaged(&self) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
        #[method_id(@__retain_semantics Other callerTeamIdentifier)]
        pub unsafe fn callerTeamIdentifier(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
        #[method_id(@__retain_semantics Other localizedCallerDisplayName)]
        pub unsafe fn localizedCallerDisplayName(&self) -> Id<NSString>;

        #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
        #[method(isUserInterfaceEnabled)]
        pub unsafe fn isUserInterfaceEnabled(&self) -> bool;

        #[cfg(feature = "AuthenticationServices_ASAuthorizationProviderExtensionLoginManager")]
        #[cfg(not(any(target_os = "ios", target_os = "tvos", target_os = "watchos")))]
        #[method_id(@__retain_semantics Other loginManager)]
        pub unsafe fn loginManager(
            &self,
        ) -> Option<Id<ASAuthorizationProviderExtensionLoginManager>>;
    }
);

#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AuthenticationServices_ASAuthorizationProviderExtensionAuthorizationRequest")]
    #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
    unsafe impl ASAuthorizationProviderExtensionAuthorizationRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
