//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AuthenticationServices_ASWebAuthenticationSessionWebBrowserSessionManager")]
    #[cfg(not(any(target_os = "ios", target_os = "tvos")))]
    pub struct ASWebAuthenticationSessionWebBrowserSessionManager;

    #[cfg(feature = "AuthenticationServices_ASWebAuthenticationSessionWebBrowserSessionManager")]
    unsafe impl ClassType for ASWebAuthenticationSessionWebBrowserSessionManager {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AuthenticationServices_ASWebAuthenticationSessionWebBrowserSessionManager")]
#[cfg(not(any(target_os = "ios", target_os = "tvos")))]
unsafe impl NSObjectProtocol for ASWebAuthenticationSessionWebBrowserSessionManager {}

#[cfg(not(any(target_os = "ios", target_os = "tvos")))]
extern_methods!(
    #[cfg(feature = "AuthenticationServices_ASWebAuthenticationSessionWebBrowserSessionManager")]
    unsafe impl ASWebAuthenticationSessionWebBrowserSessionManager {
        #[method_id(@__retain_semantics Other sharedManager)]
        pub unsafe fn sharedManager() -> Id<ASWebAuthenticationSessionWebBrowserSessionManager>;

        #[method_id(@__retain_semantics Other sessionHandler)]
        pub unsafe fn sessionHandler(
            &self,
        ) -> Id<ProtocolObject<dyn ASWebAuthenticationSessionWebBrowserSessionHandling>>;

        #[method(setSessionHandler:)]
        pub unsafe fn setSessionHandler(
            &self,
            session_handler: &ProtocolObject<
                dyn ASWebAuthenticationSessionWebBrowserSessionHandling,
            >,
        );

        #[method(wasLaunchedByAuthenticationServices)]
        pub unsafe fn wasLaunchedByAuthenticationServices(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AuthenticationServices_ASWebAuthenticationSessionWebBrowserSessionManager")]
    unsafe impl ASWebAuthenticationSessionWebBrowserSessionManager {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
