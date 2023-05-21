//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::LocalAuthentication::*;

#[cfg(not(any(target_os = "watchos")))]
ns_enum!(
    #[underlying(NSInteger)]
    pub enum LARightState {
        #[cfg(not(any(target_os = "watchos")))]
        LARightStateUnknown = 0,
        #[cfg(not(any(target_os = "watchos")))]
        LARightStateAuthorizing = 1,
        #[cfg(not(any(target_os = "watchos")))]
        LARightStateAuthorized = 2,
        #[cfg(not(any(target_os = "watchos")))]
        LARightStateNotAuthorized = 3,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "LocalAuthentication_LARight")]
    #[cfg(not(any(target_os = "watchos")))]
    pub struct LARight;

    #[cfg(not(any(target_os = "watchos")))]
    #[cfg(feature = "LocalAuthentication_LARight")]
    unsafe impl ClassType for LARight {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "LocalAuthentication_LARight")]
#[cfg(not(any(target_os = "watchos")))]
unsafe impl NSObjectProtocol for LARight {}

#[cfg(not(any(target_os = "watchos")))]
extern_methods!(
    #[cfg(feature = "LocalAuthentication_LARight")]
    #[cfg(not(any(target_os = "watchos")))]
    unsafe impl LARight {
        #[method(state)]
        pub unsafe fn state(&self) -> LARightState;

        #[method(tag)]
        pub unsafe fn tag(&self) -> NSInteger;

        #[method(setTag:)]
        pub unsafe fn setTag(&self, tag: NSInteger);

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[cfg(feature = "LocalAuthentication_LAAuthenticationRequirement")]
        #[method_id(@__retain_semantics Init initWithRequirement:)]
        pub unsafe fn initWithRequirement(
            this: Option<Allocated<Self>>,
            requirement: &LAAuthenticationRequirement,
        ) -> Id<Self>;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[method(authorizeWithLocalizedReason:completion:)]
        pub unsafe fn authorizeWithLocalizedReason_completion(
            &self,
            localized_reason: &NSString,
            handler: &Block<(*mut NSError,), ()>,
        );

        #[cfg(feature = "Foundation_NSError")]
        #[method(checkCanAuthorizeWithCompletion:)]
        pub unsafe fn checkCanAuthorizeWithCompletion(&self, handler: &Block<(*mut NSError,), ()>);

        #[method(deauthorizeWithCompletion:)]
        pub unsafe fn deauthorizeWithCompletion(&self, handler: &Block<(), ()>);
    }
);

#[cfg(not(any(target_os = "watchos")))]
extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "LocalAuthentication_LARight")]
    #[cfg(not(any(target_os = "watchos")))]
    unsafe impl LARight {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
