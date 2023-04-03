//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::LocalAuthentication::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum LAPolicy {
        #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
        LAPolicyDeviceOwnerAuthenticationWithBiometrics = 1,
        LAPolicyDeviceOwnerAuthentication = 2,
        #[cfg(not(any(target_os = "ios", target_os = "tvos", target_os = "watchos")))]
        LAPolicyDeviceOwnerAuthenticationWithWatch = 3,
        #[cfg(not(any(target_os = "ios", target_os = "tvos", target_os = "watchos")))]
        LAPolicyDeviceOwnerAuthenticationWithBiometricsOrWatch = 4,
        #[cfg(not(any(target_os = "ios", target_os = "macos")))]
        LAPolicyDeviceOwnerAuthenticationWithWristDetection = 5,
    }
);

extern_static!(LATouchIDAuthenticationMaximumAllowableReuseDuration: NSTimeInterval);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum LAAccessControlOperation {
        LAAccessControlOperationCreateItem = 0,
        LAAccessControlOperationUseItem = 1,
        LAAccessControlOperationCreateKey = 2,
        LAAccessControlOperationUseKeySign = 3,
        LAAccessControlOperationUseKeyDecrypt = 4,
        LAAccessControlOperationUseKeyKeyExchange = 5,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
    pub enum LABiometryType {
        LABiometryTypeNone = 0,
        #[deprecated]
        LABiometryNone = LABiometryTypeNone,
        #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
        LABiometryTypeTouchID = 1,
        LABiometryTypeFaceID = 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "LocalAuthentication_LAContext")]
    pub struct LAContext;

    #[cfg(feature = "LocalAuthentication_LAContext")]
    unsafe impl ClassType for LAContext {
        type Super = NSObject;
    }
);

#[cfg(feature = "LocalAuthentication_LAContext")]
unsafe impl NSObjectProtocol for LAContext {}

extern_methods!(
    #[cfg(feature = "LocalAuthentication_LAContext")]
    unsafe impl LAContext {
        #[cfg(feature = "Foundation_NSError")]
        #[method(canEvaluatePolicy:error:_)]
        pub unsafe fn canEvaluatePolicy_error(&self, policy: LAPolicy) -> Result<(), Id<NSError>>;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[method(evaluatePolicy:localizedReason:reply:)]
        pub unsafe fn evaluatePolicy_localizedReason_reply(
            &self,
            policy: LAPolicy,
            localized_reason: &NSString,
            reply: &Block<(Bool, *mut NSError), ()>,
        );

        #[method(invalidate)]
        pub unsafe fn invalidate(&self);

        #[cfg(feature = "Foundation_NSData")]
        #[method(setCredential:type:)]
        pub unsafe fn setCredential_type(
            &self,
            credential: Option<&NSData>,
            r#type: LACredentialType,
        ) -> bool;

        #[method(isCredentialSet:)]
        pub unsafe fn isCredentialSet(&self, r#type: LACredentialType) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
        #[method_id(@__retain_semantics Other localizedFallbackTitle)]
        pub unsafe fn localizedFallbackTitle(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
        #[method(setLocalizedFallbackTitle:)]
        pub unsafe fn setLocalizedFallbackTitle(&self, localized_fallback_title: Option<&NSString>);

        #[cfg(feature = "Foundation_NSNumber")]
        #[deprecated = "No longer supported"]
        #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
        #[method_id(@__retain_semantics Other maxBiometryFailures)]
        pub unsafe fn maxBiometryFailures(&self) -> Option<Id<NSNumber>>;

        #[cfg(feature = "Foundation_NSNumber")]
        #[deprecated = "No longer supported"]
        #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
        #[method(setMaxBiometryFailures:)]
        pub unsafe fn setMaxBiometryFailures(&self, max_biometry_failures: Option<&NSNumber>);

        #[cfg(feature = "Foundation_NSString")]
        #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
        #[method_id(@__retain_semantics Other localizedCancelTitle)]
        pub unsafe fn localizedCancelTitle(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
        #[method(setLocalizedCancelTitle:)]
        pub unsafe fn setLocalizedCancelTitle(&self, localized_cancel_title: Option<&NSString>);

        #[cfg(feature = "Foundation_NSData")]
        #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
        #[method_id(@__retain_semantics Other evaluatedPolicyDomainState)]
        pub unsafe fn evaluatedPolicyDomainState(&self) -> Option<Id<NSData>>;

        #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
        #[method(touchIDAuthenticationAllowableReuseDuration)]
        pub unsafe fn touchIDAuthenticationAllowableReuseDuration(&self) -> NSTimeInterval;

        #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
        #[method(setTouchIDAuthenticationAllowableReuseDuration:)]
        pub unsafe fn setTouchIDAuthenticationAllowableReuseDuration(
            &self,
            touch_id_authentication_allowable_reuse_duration: NSTimeInterval,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
        #[method_id(@__retain_semantics Other localizedReason)]
        pub unsafe fn localizedReason(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
        #[method(setLocalizedReason:)]
        pub unsafe fn setLocalizedReason(&self, localized_reason: &NSString);

        #[method(interactionNotAllowed)]
        pub unsafe fn interactionNotAllowed(&self) -> bool;

        #[method(setInteractionNotAllowed:)]
        pub unsafe fn setInteractionNotAllowed(&self, interaction_not_allowed: bool);

        #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
        #[method(biometryType)]
        pub unsafe fn biometryType(&self) -> LABiometryType;
    }
);
