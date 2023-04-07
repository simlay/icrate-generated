//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AuthenticationServices::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    #[cfg(not(any(target_os = "watchos")))]
    pub enum ASAuthorizationAppleIDButtonType {
        #[cfg(not(any(target_os = "watchos")))]
        ASAuthorizationAppleIDButtonTypeSignIn = 0,
        #[cfg(not(any(target_os = "watchos")))]
        ASAuthorizationAppleIDButtonTypeContinue = 1,
        #[cfg(not(any(target_os = "watchos")))]
        ASAuthorizationAppleIDButtonTypeSignUp = 2,
        #[cfg(not(any(target_os = "watchos")))]
        ASAuthorizationAppleIDButtonTypeDefault = ASAuthorizationAppleIDButtonTypeSignIn,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    #[cfg(not(any(target_os = "watchos")))]
    pub enum ASAuthorizationAppleIDButtonStyle {
        #[cfg(not(any(target_os = "watchos")))]
        ASAuthorizationAppleIDButtonStyleWhite = 0,
        #[cfg(not(any(target_os = "watchos")))]
        ASAuthorizationAppleIDButtonStyleWhiteOutline = 1,
        #[cfg(not(any(target_os = "watchos")))]
        ASAuthorizationAppleIDButtonStyleBlack = 2,
    }
);

extern_methods!(
    #[cfg(feature = "AuthenticationServices_ASAuthorizationAppleIDButton")]
    #[cfg(not(any(target_os = "watchos")))]
    unsafe impl ASAuthorizationAppleIDButton {
        #[cfg(not(any(target_os = "watchos")))]
        #[method_id(@__retain_semantics Other buttonWithType:style:)]
        pub unsafe fn buttonWithType_style(
            r#type: ASAuthorizationAppleIDButtonType,
            style: ASAuthorizationAppleIDButtonStyle,
        ) -> Id<Self>;

        #[cfg(not(any(target_os = "watchos")))]
        #[method_id(@__retain_semantics Init initWithAuthorizationButtonType:authorizationButtonStyle:)]
        pub unsafe fn initWithAuthorizationButtonType_authorizationButtonStyle(
            this: Option<Allocated<Self>>,
            r#type: ASAuthorizationAppleIDButtonType,
            style: ASAuthorizationAppleIDButtonStyle,
        ) -> Id<Self>;

        #[cfg(not(any(target_os = "watchos")))]
        #[cfg(not(any(target_os = "tvos")))]
        #[method(cornerRadius)]
        pub unsafe fn cornerRadius(&self) -> CGFloat;

        #[cfg(not(any(target_os = "watchos")))]
        #[cfg(not(any(target_os = "tvos")))]
        #[method(setCornerRadius:)]
        pub unsafe fn setCornerRadius(&self, corner_radius: CGFloat);
    }
);
