//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::UserNotifications::*;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum UNNotificationCategoryOptions {
        UNNotificationCategoryOptionCustomDismissAction = 1 << 0,
        UNNotificationCategoryOptionAllowInCarPlay = 1 << 1,
        UNNotificationCategoryOptionHiddenPreviewsShowTitle = 1 << 2,
        UNNotificationCategoryOptionHiddenPreviewsShowSubtitle = 1 << 3,
        #[deprecated = "Announcement option is ignored"]
        UNNotificationCategoryOptionAllowAnnouncement = 1 << 4,
    }
);

extern_static!(UNNotificationCategoryOptionNone: UNNotificationCategoryOptions = 0);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UserNotifications_UNNotificationCategory")]
    pub struct UNNotificationCategory;

    #[cfg(feature = "UserNotifications_UNNotificationCategory")]
    unsafe impl ClassType for UNNotificationCategory {
        type Super = NSObject;
    }
);

#[cfg(feature = "UserNotifications_UNNotificationCategory")]
unsafe impl NSCoding for UNNotificationCategory {}

#[cfg(feature = "UserNotifications_UNNotificationCategory")]
unsafe impl NSObjectProtocol for UNNotificationCategory {}

#[cfg(feature = "UserNotifications_UNNotificationCategory")]
unsafe impl NSSecureCoding for UNNotificationCategory {}

extern_methods!(
    #[cfg(feature = "UserNotifications_UNNotificationCategory")]
    unsafe impl UNNotificationCategory {
        #[cfg(feature = "Foundation_NSString")]
        /**
          The unique identifier for this category. The UNNotificationCategory's actions will be displayed on notifications when the UNNotificationCategory's identifier matches the UNNotificationRequest's categoryIdentifier.
        */
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Id<NSString>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "UserNotifications_UNNotificationAction"
        ))]
        /**
          The UNNotificationActions in the order they will be displayed.
        */
        #[method_id(@__retain_semantics Other actions)]
        pub unsafe fn actions(&self) -> Id<NSArray<UNNotificationAction>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        /**
          The intents supported support for notifications of this category. See <Intents/INIntentIdentifiers.h> for possible values.
        */
        #[method_id(@__retain_semantics Other intentIdentifiers)]
        pub unsafe fn intentIdentifiers(&self) -> Id<NSArray<NSString>>;

        #[method(options)]
        pub unsafe fn options(&self) -> UNNotificationCategoryOptions;

        #[cfg(feature = "Foundation_NSString")]
        /**
          The format string that will replace the notification body if previews are hidden.
        */
        #[method_id(@__retain_semantics Other hiddenPreviewsBodyPlaceholder)]
        pub unsafe fn hiddenPreviewsBodyPlaceholder(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        /**
          A format string for a summary description when notifications from this category are grouped together.
         It should contain descriptive text and format arguments that will be replaced with the information
         from the notifications that have been grouped together. The arguments are replaced with the number
         of notifications and the list created by joining the argument in each grouped notification.
         For example: "%u new messages from %@".
         The arguments list is optional, "%u new messages" is also accepted.
        */
        #[method_id(@__retain_semantics Other categorySummaryFormat)]
        pub unsafe fn categorySummaryFormat(&self) -> Id<NSString>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSString",
            feature = "UserNotifications_UNNotificationAction"
        ))]
        #[method_id(@__retain_semantics Other categoryWithIdentifier:actions:intentIdentifiers:options:)]
        pub unsafe fn categoryWithIdentifier_actions_intentIdentifiers_options(
            identifier: &NSString,
            actions: &NSArray<UNNotificationAction>,
            intent_identifiers: &NSArray<NSString>,
            options: UNNotificationCategoryOptions,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSString",
            feature = "UserNotifications_UNNotificationAction"
        ))]
        #[method_id(@__retain_semantics Other categoryWithIdentifier:actions:intentIdentifiers:hiddenPreviewsBodyPlaceholder:options:)]
        pub unsafe fn categoryWithIdentifier_actions_intentIdentifiers_hiddenPreviewsBodyPlaceholder_options(
            identifier: &NSString,
            actions: &NSArray<UNNotificationAction>,
            intent_identifiers: &NSArray<NSString>,
            hidden_previews_body_placeholder: &NSString,
            options: UNNotificationCategoryOptions,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSString",
            feature = "UserNotifications_UNNotificationAction"
        ))]
        #[method_id(@__retain_semantics Other categoryWithIdentifier:actions:intentIdentifiers:hiddenPreviewsBodyPlaceholder:categorySummaryFormat:options:)]
        pub unsafe fn categoryWithIdentifier_actions_intentIdentifiers_hiddenPreviewsBodyPlaceholder_categorySummaryFormat_options(
            identifier: &NSString,
            actions: &NSArray<UNNotificationAction>,
            intent_identifiers: &NSArray<NSString>,
            hidden_previews_body_placeholder: Option<&NSString>,
            category_summary_format: Option<&NSString>,
            options: UNNotificationCategoryOptions,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);
