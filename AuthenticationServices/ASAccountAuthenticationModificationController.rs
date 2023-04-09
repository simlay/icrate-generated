//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

#[cfg(not(any(target_os = "macos", target_os = "tvos", target_os = "watchos")))]
extern_protocol!(
    pub unsafe trait ASAccountAuthenticationModificationControllerDelegate:
        NSObjectProtocol
    {
        #[cfg(all(
            feature = "AuthenticationServices_ASAccountAuthenticationModificationController",
            feature = "AuthenticationServices_ASAccountAuthenticationModificationRequest",
            feature = "Foundation_NSDictionary"
        ))]
        #[optional]
        #[method(accountAuthenticationModificationController:didSuccessfullyCompleteRequest:withUserInfo:)]
        unsafe fn accountAuthenticationModificationController_didSuccessfullyCompleteRequest_withUserInfo(
            &self,
            controller: &ASAccountAuthenticationModificationController,
            request: &ASAccountAuthenticationModificationRequest,
            user_info: Option<&NSDictionary>,
        );

        #[cfg(all(
            feature = "AuthenticationServices_ASAccountAuthenticationModificationController",
            feature = "AuthenticationServices_ASAccountAuthenticationModificationRequest",
            feature = "Foundation_NSError"
        ))]
        #[optional]
        #[method(accountAuthenticationModificationController:didFailRequest:withError:)]
        unsafe fn accountAuthenticationModificationController_didFailRequest_withError(
            &self,
            controller: &ASAccountAuthenticationModificationController,
            request: &ASAccountAuthenticationModificationRequest,
            error: &NSError,
        );
    }

    unsafe impl ProtocolType for dyn ASAccountAuthenticationModificationControllerDelegate {}
);

#[cfg(not(any(target_os = "macos", target_os = "tvos", target_os = "watchos")))]
extern_protocol!(
    pub unsafe trait ASAccountAuthenticationModificationControllerPresentationContextProviding:
        NSObjectProtocol
    {
        #[cfg(feature = "AuthenticationServices_ASAccountAuthenticationModificationController")]
        #[method_id(@__retain_semantics Other presentationAnchorForAccountAuthenticationModificationController:)]
        unsafe fn presentationAnchorForAccountAuthenticationModificationController(
            &self,
            controller: &ASAccountAuthenticationModificationController,
        ) -> Id<ASPresentationAnchor>;
    }

    unsafe impl ProtocolType
        for dyn ASAccountAuthenticationModificationControllerPresentationContextProviding
    {
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AuthenticationServices_ASAccountAuthenticationModificationController")]
    #[cfg(not(any(target_os = "macos", target_os = "tvos", target_os = "watchos")))]
    pub struct ASAccountAuthenticationModificationController;

    #[cfg(not(any(target_os = "macos", target_os = "tvos", target_os = "watchos")))]
    #[cfg(feature = "AuthenticationServices_ASAccountAuthenticationModificationController")]
    unsafe impl ClassType for ASAccountAuthenticationModificationController {
        type Super = NSObject;
    }
);

#[cfg(feature = "AuthenticationServices_ASAccountAuthenticationModificationController")]
#[cfg(not(any(target_os = "macos", target_os = "tvos", target_os = "watchos")))]
unsafe impl NSObjectProtocol for ASAccountAuthenticationModificationController {}

#[cfg(not(any(target_os = "macos", target_os = "tvos", target_os = "watchos")))]
extern_methods!(
    #[cfg(feature = "AuthenticationServices_ASAccountAuthenticationModificationController")]
    unsafe impl ASAccountAuthenticationModificationController {
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Id<ProtocolObject<dyn ASAccountAuthenticationModificationControllerDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<
                &ProtocolObject<dyn ASAccountAuthenticationModificationControllerDelegate>,
            >,
        );

        #[method_id(@__retain_semantics Other presentationContextProvider)]
        pub unsafe fn presentationContextProvider(
            &self,
        ) -> Option<
            Id<
                ProtocolObject<
                    dyn ASAccountAuthenticationModificationControllerPresentationContextProviding,
                >,
            >,
        >;

        #[method(setPresentationContextProvider:)]
        pub unsafe fn setPresentationContextProvider(
            &self,
            presentation_context_provider: Option<
                &ProtocolObject<
                    dyn ASAccountAuthenticationModificationControllerPresentationContextProviding,
                >,
            >,
        );

        #[cfg(feature = "AuthenticationServices_ASAccountAuthenticationModificationRequest")]
        #[method(performRequest:)]
        pub unsafe fn performRequest(&self, request: &ASAccountAuthenticationModificationRequest);
    }
);
