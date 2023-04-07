//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::UserNotifications::*;

ns_options!(
    #[underlying(NSUInteger)]
    #[cfg(not(any(target_os = "tvos")))]
    pub enum UNNotificationActionOptions {
        #[cfg(not(any(target_os = "tvos")))]
        UNNotificationActionOptionAuthenticationRequired = 1 << 0,
        #[cfg(not(any(target_os = "tvos")))]
        UNNotificationActionOptionDestructive = 1 << 1,
        #[cfg(not(any(target_os = "tvos")))]
        UNNotificationActionOptionForeground = 1 << 2,
    }
);

#[cfg(not(any(target_os = "tvos")))]
extern_static!(UNNotificationActionOptionNone: UNNotificationActionOptions = 0);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UserNotifications_UNNotificationAction")]
    #[cfg(not(any(target_os = "tvos")))]
    pub struct UNNotificationAction;

    #[cfg(not(any(target_os = "tvos")))]
    #[cfg(feature = "UserNotifications_UNNotificationAction")]
    unsafe impl ClassType for UNNotificationAction {
        type Super = NSObject;
    }
);

#[cfg(feature = "UserNotifications_UNNotificationAction")]
#[cfg(not(any(target_os = "tvos")))]
unsafe impl NSCoding for UNNotificationAction {}

#[cfg(feature = "UserNotifications_UNNotificationAction")]
#[cfg(not(any(target_os = "tvos")))]
unsafe impl NSObjectProtocol for UNNotificationAction {}

#[cfg(feature = "UserNotifications_UNNotificationAction")]
#[cfg(not(any(target_os = "tvos")))]
unsafe impl NSSecureCoding for UNNotificationAction {}

extern_methods!(
    #[cfg(feature = "UserNotifications_UNNotificationAction")]
    #[cfg(not(any(target_os = "tvos")))]
    unsafe impl UNNotificationAction {
        #[cfg(not(any(target_os = "tvos")))]
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Id<NSString>;

        #[cfg(not(any(target_os = "tvos")))]
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Id<NSString>;

        #[cfg(not(any(target_os = "tvos")))]
        #[method(options)]
        pub unsafe fn options(&self) -> UNNotificationActionOptions;

        #[cfg(not(any(target_os = "tvos")))]
        #[cfg(feature = "UserNotifications_UNNotificationActionIcon")]
        #[method_id(@__retain_semantics Other icon)]
        pub unsafe fn icon(&self) -> Option<Id<UNNotificationActionIcon>>;

        #[cfg(not(any(target_os = "tvos")))]
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other actionWithIdentifier:title:options:)]
        pub unsafe fn actionWithIdentifier_title_options(
            identifier: &NSString,
            title: &NSString,
            options: UNNotificationActionOptions,
        ) -> Id<Self>;

        #[cfg(not(any(target_os = "tvos")))]
        #[cfg(all(
            feature = "Foundation_NSString",
            feature = "UserNotifications_UNNotificationActionIcon"
        ))]
        #[method_id(@__retain_semantics Other actionWithIdentifier:title:options:icon:)]
        pub unsafe fn actionWithIdentifier_title_options_icon(
            identifier: &NSString,
            title: &NSString,
            options: UNNotificationActionOptions,
            icon: Option<&UNNotificationActionIcon>,
        ) -> Id<Self>;

        #[cfg(not(any(target_os = "tvos")))]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UserNotifications_UNTextInputNotificationAction")]
    #[cfg(not(any(target_os = "tvos")))]
    pub struct UNTextInputNotificationAction;

    #[cfg(not(any(target_os = "tvos")))]
    #[cfg(feature = "UserNotifications_UNTextInputNotificationAction")]
    unsafe impl ClassType for UNTextInputNotificationAction {
        #[inherits(NSObject)]
        type Super = UNNotificationAction;
    }
);

#[cfg(feature = "UserNotifications_UNTextInputNotificationAction")]
#[cfg(not(any(target_os = "tvos")))]
unsafe impl NSCoding for UNTextInputNotificationAction {}

#[cfg(feature = "UserNotifications_UNTextInputNotificationAction")]
#[cfg(not(any(target_os = "tvos")))]
unsafe impl NSObjectProtocol for UNTextInputNotificationAction {}

#[cfg(feature = "UserNotifications_UNTextInputNotificationAction")]
#[cfg(not(any(target_os = "tvos")))]
unsafe impl NSSecureCoding for UNTextInputNotificationAction {}

extern_methods!(
    #[cfg(feature = "UserNotifications_UNTextInputNotificationAction")]
    #[cfg(not(any(target_os = "tvos")))]
    unsafe impl UNTextInputNotificationAction {
        #[cfg(not(any(target_os = "tvos")))]
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other actionWithIdentifier:title:options:textInputButtonTitle:textInputPlaceholder:)]
        pub unsafe fn actionWithIdentifier_title_options_textInputButtonTitle_textInputPlaceholder(
            identifier: &NSString,
            title: &NSString,
            options: UNNotificationActionOptions,
            text_input_button_title: &NSString,
            text_input_placeholder: &NSString,
        ) -> Id<Self>;

        #[cfg(not(any(target_os = "tvos")))]
        #[cfg(all(
            feature = "Foundation_NSString",
            feature = "UserNotifications_UNNotificationActionIcon"
        ))]
        #[method_id(@__retain_semantics Other actionWithIdentifier:title:options:icon:textInputButtonTitle:textInputPlaceholder:)]
        pub unsafe fn actionWithIdentifier_title_options_icon_textInputButtonTitle_textInputPlaceholder(
            identifier: &NSString,
            title: &NSString,
            options: UNNotificationActionOptions,
            icon: Option<&UNNotificationActionIcon>,
            text_input_button_title: &NSString,
            text_input_placeholder: &NSString,
        ) -> Id<Self>;

        #[cfg(not(any(target_os = "tvos")))]
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other textInputButtonTitle)]
        pub unsafe fn textInputButtonTitle(&self) -> Id<NSString>;

        #[cfg(not(any(target_os = "tvos")))]
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other textInputPlaceholder)]
        pub unsafe fn textInputPlaceholder(&self) -> Id<NSString>;
    }
);

extern_methods!(
    /// Methods declared on superclass `UNNotificationAction`
    #[cfg(feature = "UserNotifications_UNTextInputNotificationAction")]
    #[cfg(not(any(target_os = "tvos")))]
    unsafe impl UNTextInputNotificationAction {
        #[cfg(not(any(target_os = "tvos")))]
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other actionWithIdentifier:title:options:)]
        pub unsafe fn actionWithIdentifier_title_options(
            identifier: &NSString,
            title: &NSString,
            options: UNNotificationActionOptions,
        ) -> Id<Self>;

        #[cfg(not(any(target_os = "tvos")))]
        #[cfg(all(
            feature = "Foundation_NSString",
            feature = "UserNotifications_UNNotificationActionIcon"
        ))]
        #[method_id(@__retain_semantics Other actionWithIdentifier:title:options:icon:)]
        pub unsafe fn actionWithIdentifier_title_options_icon(
            identifier: &NSString,
            title: &NSString,
            options: UNNotificationActionOptions,
            icon: Option<&UNNotificationActionIcon>,
        ) -> Id<Self>;
    }
);
