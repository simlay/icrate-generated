//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::StoreKit::*;

#[cfg(not(any(target_os = "macos", target_os = "tvos", target_os = "watchos")))]
extern_protocol!(
    pub unsafe trait SKOverlayDelegate: NSObjectProtocol {
        #[cfg(all(feature = "Foundation_NSError", feature = "StoreKit_SKOverlay"))]
        #[optional]
        #[method(storeOverlay:didFailToLoadWithError:)]
        unsafe fn storeOverlay_didFailToLoadWithError(&self, overlay: &SKOverlay, error: &NSError);

        #[cfg(all(
            feature = "StoreKit_SKOverlay",
            feature = "StoreKit_SKOverlayTransitionContext"
        ))]
        #[optional]
        #[method(storeOverlay:willStartPresentation:)]
        unsafe fn storeOverlay_willStartPresentation(
            &self,
            overlay: &SKOverlay,
            transition_context: &SKOverlayTransitionContext,
        );

        #[cfg(all(
            feature = "StoreKit_SKOverlay",
            feature = "StoreKit_SKOverlayTransitionContext"
        ))]
        #[optional]
        #[method(storeOverlay:didFinishPresentation:)]
        unsafe fn storeOverlay_didFinishPresentation(
            &self,
            overlay: &SKOverlay,
            transition_context: &SKOverlayTransitionContext,
        );

        #[cfg(all(
            feature = "StoreKit_SKOverlay",
            feature = "StoreKit_SKOverlayTransitionContext"
        ))]
        #[optional]
        #[method(storeOverlay:willStartDismissal:)]
        unsafe fn storeOverlay_willStartDismissal(
            &self,
            overlay: &SKOverlay,
            transition_context: &SKOverlayTransitionContext,
        );

        #[cfg(all(
            feature = "StoreKit_SKOverlay",
            feature = "StoreKit_SKOverlayTransitionContext"
        ))]
        #[optional]
        #[method(storeOverlay:didFinishDismissal:)]
        unsafe fn storeOverlay_didFinishDismissal(
            &self,
            overlay: &SKOverlay,
            transition_context: &SKOverlayTransitionContext,
        );
    }

    unsafe impl ProtocolType for dyn SKOverlayDelegate {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "StoreKit_SKOverlay")]
    #[cfg(not(any(target_os = "macos", target_os = "tvos", target_os = "watchos")))]
    pub struct SKOverlay;

    #[cfg(feature = "StoreKit_SKOverlay")]
    unsafe impl ClassType for SKOverlay {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "StoreKit_SKOverlay")]
#[cfg(not(any(target_os = "macos", target_os = "tvos", target_os = "watchos")))]
unsafe impl NSObjectProtocol for SKOverlay {}

#[cfg(not(any(target_os = "macos", target_os = "tvos", target_os = "watchos")))]
extern_methods!(
    #[cfg(feature = "StoreKit_SKOverlay")]
    unsafe impl SKOverlay {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[cfg(feature = "StoreKit_SKOverlayConfiguration")]
        #[method_id(@__retain_semantics Init initWithConfiguration:)]
        pub unsafe fn initWithConfiguration(
            this: Option<Allocated<Self>>,
            configuration: &SKOverlayConfiguration,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<ProtocolObject<dyn SKOverlayDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&ProtocolObject<dyn SKOverlayDelegate>>);

        #[cfg(feature = "StoreKit_SKOverlayConfiguration")]
        #[method_id(@__retain_semantics Other configuration)]
        pub unsafe fn configuration(&self) -> Id<SKOverlayConfiguration>;
    }
);
