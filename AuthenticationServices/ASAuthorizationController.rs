//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

extern_protocol!(
    pub struct ASAuthorizationControllerDelegate;

    unsafe impl ProtocolType for ASAuthorizationControllerDelegate {
        #[cfg(all(
            feature = "AuthenticationServices_ASAuthorization",
            feature = "AuthenticationServices_ASAuthorizationController"
        ))]
        #[optional]
        #[method(authorizationController:didCompleteWithAuthorization:)]
        pub unsafe fn authorizationController_didCompleteWithAuthorization(
            &self,
            controller: &ASAuthorizationController,
            authorization: &ASAuthorization,
        );

        #[cfg(all(
            feature = "AuthenticationServices_ASAuthorizationController",
            feature = "Foundation_NSError"
        ))]
        #[optional]
        #[method(authorizationController:didCompleteWithError:)]
        pub unsafe fn authorizationController_didCompleteWithError(
            &self,
            controller: &ASAuthorizationController,
            error: &NSError,
        );

        #[cfg(feature = "AuthenticationServices_ASAuthorizationController")]
        #[optional]
        #[method(authorizationController:didCompleteWithCustomMethod:)]
        pub unsafe fn authorizationController_didCompleteWithCustomMethod(
            &self,
            controller: &ASAuthorizationController,
            method: &ASAuthorizationCustomMethod,
        );
    }
);

extern_protocol!(
    pub struct ASAuthorizationControllerPresentationContextProviding;

    unsafe impl ProtocolType for ASAuthorizationControllerPresentationContextProviding {
        #[cfg(feature = "AuthenticationServices_ASAuthorizationController")]
        #[method_id(@__retain_semantics Other presentationAnchorForAuthorizationController:)]
        pub unsafe fn presentationAnchorForAuthorizationController(
            &self,
            controller: &ASAuthorizationController,
        ) -> Id<ASPresentationAnchor, Shared>;
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum ASAuthorizationControllerRequestOptions {
        ASAuthorizationControllerRequestOptionPreferImmediatelyAvailableCredentials = 1 << 0,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AuthenticationServices_ASAuthorizationController")]
    pub struct ASAuthorizationController;

    #[cfg(feature = "AuthenticationServices_ASAuthorizationController")]
    unsafe impl ClassType for ASAuthorizationController {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "AuthenticationServices_ASAuthorizationController")]
    unsafe impl ASAuthorizationController {
        #[cfg(all(
            feature = "AuthenticationServices_ASAuthorizationRequest",
            feature = "Foundation_NSArray"
        ))]
        #[method_id(@__retain_semantics Other authorizationRequests)]
        pub unsafe fn authorizationRequests(&self) -> Id<NSArray<ASAuthorizationRequest>, Shared>;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<ASAuthorizationControllerDelegate, Shared>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&ASAuthorizationControllerDelegate>);

        #[method_id(@__retain_semantics Other presentationContextProvider)]
        pub unsafe fn presentationContextProvider(
            &self,
        ) -> Option<Id<ASAuthorizationControllerPresentationContextProviding, Shared>>;

        #[method(setPresentationContextProvider:)]
        pub unsafe fn setPresentationContextProvider(
            &self,
            presentationContextProvider: Option<
                &ASAuthorizationControllerPresentationContextProviding,
            >,
        );

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other customAuthorizationMethods)]
        pub unsafe fn customAuthorizationMethods(
            &self,
        ) -> Id<NSArray<ASAuthorizationCustomMethod>, Shared>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setCustomAuthorizationMethods:)]
        pub unsafe fn setCustomAuthorizationMethods(
            &self,
            customAuthorizationMethods: &NSArray<ASAuthorizationCustomMethod>,
        );

        #[cfg(all(
            feature = "AuthenticationServices_ASAuthorizationRequest",
            feature = "Foundation_NSArray"
        ))]
        #[method_id(@__retain_semantics Init initWithAuthorizationRequests:)]
        pub unsafe fn initWithAuthorizationRequests(
            this: Option<Allocated<Self>>,
            authorizationRequests: &NSArray<ASAuthorizationRequest>,
        ) -> Id<Self, Shared>;

        #[method(performRequests)]
        pub unsafe fn performRequests(&self);

        #[method(performAutoFillAssistedRequests)]
        pub unsafe fn performAutoFillAssistedRequests(&self);

        #[method(performRequestsWithOptions:)]
        pub unsafe fn performRequestsWithOptions(
            &self,
            options: ASAuthorizationControllerRequestOptions,
        );

        #[method(cancel)]
        pub unsafe fn cancel(&self);

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self, Shared>;
    }
);
