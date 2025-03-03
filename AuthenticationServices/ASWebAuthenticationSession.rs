//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

extern_static!(ASWebAuthenticationSessionErrorDomain: &'static NSErrorDomain);

ns_error_enum!(
    #[underlying(NSInteger)]
    pub enum ASWebAuthenticationSessionErrorCode {
        ASWebAuthenticationSessionErrorCodeCanceledLogin = 1,
        ASWebAuthenticationSessionErrorCodePresentationContextNotProvided = 2,
        ASWebAuthenticationSessionErrorCodePresentationContextInvalid = 3,
    }
);

pub type ASWebAuthenticationSessionCompletionHandler = *mut Block<(*mut NSURL, *mut NSError), ()>;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AuthenticationServices_ASWebAuthenticationSession")]
    pub struct ASWebAuthenticationSession;

    #[cfg(feature = "AuthenticationServices_ASWebAuthenticationSession")]
    unsafe impl ClassType for ASWebAuthenticationSession {
        type Super = NSObject;
    }
);

#[cfg(feature = "AuthenticationServices_ASWebAuthenticationSession")]
unsafe impl NSObjectProtocol for ASWebAuthenticationSession {}

extern_methods!(
    #[cfg(feature = "AuthenticationServices_ASWebAuthenticationSession")]
    unsafe impl ASWebAuthenticationSession {
        #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSURL"))]
        #[method_id(@__retain_semantics Init initWithURL:callbackURLScheme:completionHandler:)]
        pub unsafe fn initWithURL_callbackURLScheme_completionHandler(
            this: Option<Allocated<Self>>,
            url: &NSURL,
            callback_url_scheme: Option<&NSString>,
            completion_handler: ASWebAuthenticationSessionCompletionHandler,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Other presentationContextProvider)]
        pub unsafe fn presentationContextProvider(
            &self,
        ) -> Option<Id<ProtocolObject<dyn ASWebAuthenticationPresentationContextProviding>>>;

        #[method(setPresentationContextProvider:)]
        pub unsafe fn setPresentationContextProvider(
            &self,
            presentation_context_provider: Option<
                &ProtocolObject<dyn ASWebAuthenticationPresentationContextProviding>,
            >,
        );

        #[method(prefersEphemeralWebBrowserSession)]
        pub unsafe fn prefersEphemeralWebBrowserSession(&self) -> bool;

        #[method(setPrefersEphemeralWebBrowserSession:)]
        pub unsafe fn setPrefersEphemeralWebBrowserSession(
            &self,
            prefers_ephemeral_web_browser_session: bool,
        );

        #[method(canStart)]
        pub unsafe fn canStart(&self) -> bool;

        #[method(start)]
        pub unsafe fn start(&self) -> bool;

        #[method(cancel)]
        pub unsafe fn cancel(&self);

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_protocol!(
    pub unsafe trait ASWebAuthenticationPresentationContextProviding:
        NSObjectProtocol
    {
        #[cfg(feature = "AuthenticationServices_ASWebAuthenticationSession")]
        #[method_id(@__retain_semantics Other presentationAnchorForWebAuthenticationSession:)]
        unsafe fn presentationAnchorForWebAuthenticationSession(
            &self,
            session: &ASWebAuthenticationSession,
        ) -> Id<ASPresentationAnchor>;
    }

    unsafe impl ProtocolType for dyn ASWebAuthenticationPresentationContextProviding {}
);
