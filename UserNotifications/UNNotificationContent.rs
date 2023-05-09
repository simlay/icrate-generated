//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::UserNotifications::*;

extern_protocol!(
    pub unsafe trait UNNotificationContentProviding: NSObjectProtocol {}

    unsafe impl ProtocolType for dyn UNNotificationContentProviding {}
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum UNNotificationInterruptionLevel {
        UNNotificationInterruptionLevelPassive = 0,
        UNNotificationInterruptionLevelActive = 1,
        UNNotificationInterruptionLevelTimeSensitive = 2,
        UNNotificationInterruptionLevelCritical = 3,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UserNotifications_UNNotificationContent")]
    pub struct UNNotificationContent;

    #[cfg(feature = "UserNotifications_UNNotificationContent")]
    unsafe impl ClassType for UNNotificationContent {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "UserNotifications_UNNotificationContent")]
unsafe impl NSCoding for UNNotificationContent {}

#[cfg(feature = "UserNotifications_UNNotificationContent")]
unsafe impl NSCopying for UNNotificationContent {}

#[cfg(feature = "UserNotifications_UNNotificationContent")]
unsafe impl NSMutableCopying for UNNotificationContent {}

#[cfg(feature = "UserNotifications_UNNotificationContent")]
unsafe impl NSObjectProtocol for UNNotificationContent {}

#[cfg(feature = "UserNotifications_UNNotificationContent")]
unsafe impl NSSecureCoding for UNNotificationContent {}

extern_methods!(
    #[cfg(feature = "UserNotifications_UNNotificationContent")]
    unsafe impl UNNotificationContent {
        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "UserNotifications_UNNotificationAttachment"
        ))]
        #[cfg(not(any(target_os = "tvos")))]
        #[method_id(@__retain_semantics Other attachments)]
        pub unsafe fn attachments(&self) -> Id<NSArray<UNNotificationAttachment>>;

        #[cfg(feature = "Foundation_NSNumber")]
        #[method_id(@__retain_semantics Other badge)]
        pub unsafe fn badge(&self) -> Option<Id<NSNumber>>;

        #[cfg(feature = "Foundation_NSString")]
        #[cfg(not(any(target_os = "tvos")))]
        #[method_id(@__retain_semantics Other body)]
        pub unsafe fn body(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[cfg(not(any(target_os = "tvos")))]
        #[method_id(@__retain_semantics Other categoryIdentifier)]
        pub unsafe fn categoryIdentifier(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[cfg(not(any(target_os = "macos", target_os = "tvos")))]
        #[method_id(@__retain_semantics Other launchImageName)]
        pub unsafe fn launchImageName(&self) -> Id<NSString>;

        #[cfg(feature = "UserNotifications_UNNotificationSound")]
        #[cfg(not(any(target_os = "tvos")))]
        #[method_id(@__retain_semantics Other sound)]
        pub unsafe fn sound(&self) -> Option<Id<UNNotificationSound>>;

        #[cfg(feature = "Foundation_NSString")]
        #[cfg(not(any(target_os = "tvos")))]
        #[method_id(@__retain_semantics Other subtitle)]
        pub unsafe fn subtitle(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[cfg(not(any(target_os = "tvos")))]
        #[method_id(@__retain_semantics Other threadIdentifier)]
        pub unsafe fn threadIdentifier(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[cfg(not(any(target_os = "tvos")))]
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSDictionary")]
        #[cfg(not(any(target_os = "tvos")))]
        #[method_id(@__retain_semantics Other userInfo)]
        pub unsafe fn userInfo(&self) -> Id<NSDictionary>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "summaryArgument is ignored"]
        #[method_id(@__retain_semantics Other summaryArgument)]
        pub unsafe fn summaryArgument(&self) -> Id<NSString>;

        #[deprecated = "summaryArgumentCount is ignored"]
        #[method(summaryArgumentCount)]
        pub unsafe fn summaryArgumentCount(&self) -> NSUInteger;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other targetContentIdentifier)]
        pub unsafe fn targetContentIdentifier(&self) -> Option<Id<NSString>>;

        #[method(interruptionLevel)]
        pub unsafe fn interruptionLevel(&self) -> UNNotificationInterruptionLevel;

        #[method(relevanceScore)]
        pub unsafe fn relevanceScore(&self) -> c_double;

        #[cfg(feature = "Foundation_NSString")]
        #[cfg(not(any(target_os = "tvos")))]
        #[method_id(@__retain_semantics Other filterCriteria)]
        pub unsafe fn filterCriteria(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSError")]
        #[method_id(@__retain_semantics Other contentByUpdatingWithProvider:error:_)]
        pub unsafe fn contentByUpdatingWithProvider_error(
            &self,
            provider: &ProtocolObject<dyn UNNotificationContentProviding>,
        ) -> Result<Id<UNNotificationContent>, Id<NSError>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "UserNotifications_UNNotificationContent")]
    unsafe impl UNNotificationContent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UserNotifications_UNMutableNotificationContent")]
    pub struct UNMutableNotificationContent;

    #[cfg(feature = "UserNotifications_UNMutableNotificationContent")]
    unsafe impl ClassType for UNMutableNotificationContent {
        #[inherits(NSObject)]
        type Super = UNNotificationContent;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "UserNotifications_UNMutableNotificationContent")]
unsafe impl NSCoding for UNMutableNotificationContent {}

#[cfg(feature = "UserNotifications_UNMutableNotificationContent")]
unsafe impl NSCopying for UNMutableNotificationContent {}

#[cfg(feature = "UserNotifications_UNMutableNotificationContent")]
unsafe impl NSMutableCopying for UNMutableNotificationContent {}

#[cfg(feature = "UserNotifications_UNMutableNotificationContent")]
unsafe impl NSObjectProtocol for UNMutableNotificationContent {}

#[cfg(feature = "UserNotifications_UNMutableNotificationContent")]
unsafe impl NSSecureCoding for UNMutableNotificationContent {}

extern_methods!(
    #[cfg(feature = "UserNotifications_UNMutableNotificationContent")]
    unsafe impl UNMutableNotificationContent {
        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "UserNotifications_UNNotificationAttachment"
        ))]
        #[cfg(not(any(target_os = "tvos")))]
        #[method_id(@__retain_semantics Other attachments)]
        pub unsafe fn attachments(&self) -> Id<NSArray<UNNotificationAttachment>>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "UserNotifications_UNNotificationAttachment"
        ))]
        #[cfg(not(any(target_os = "tvos")))]
        #[method(setAttachments:)]
        pub unsafe fn setAttachments(&self, attachments: &NSArray<UNNotificationAttachment>);

        #[cfg(feature = "Foundation_NSNumber")]
        #[method_id(@__retain_semantics Other badge)]
        pub unsafe fn badge(&self) -> Option<Id<NSNumber>>;

        #[cfg(feature = "Foundation_NSNumber")]
        #[method(setBadge:)]
        pub unsafe fn setBadge(&self, badge: Option<&NSNumber>);

        #[cfg(feature = "Foundation_NSString")]
        #[cfg(not(any(target_os = "tvos")))]
        #[method_id(@__retain_semantics Other body)]
        pub unsafe fn body(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[cfg(not(any(target_os = "tvos")))]
        #[method(setBody:)]
        pub unsafe fn setBody(&self, body: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[cfg(not(any(target_os = "tvos")))]
        #[method_id(@__retain_semantics Other categoryIdentifier)]
        pub unsafe fn categoryIdentifier(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[cfg(not(any(target_os = "tvos")))]
        #[method(setCategoryIdentifier:)]
        pub unsafe fn setCategoryIdentifier(&self, category_identifier: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[cfg(not(any(target_os = "macos", target_os = "tvos")))]
        #[method_id(@__retain_semantics Other launchImageName)]
        pub unsafe fn launchImageName(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[cfg(not(any(target_os = "macos", target_os = "tvos")))]
        #[method(setLaunchImageName:)]
        pub unsafe fn setLaunchImageName(&self, launch_image_name: &NSString);

        #[cfg(feature = "UserNotifications_UNNotificationSound")]
        #[cfg(not(any(target_os = "tvos")))]
        #[method_id(@__retain_semantics Other sound)]
        pub unsafe fn sound(&self) -> Option<Id<UNNotificationSound>>;

        #[cfg(feature = "UserNotifications_UNNotificationSound")]
        #[cfg(not(any(target_os = "tvos")))]
        #[method(setSound:)]
        pub unsafe fn setSound(&self, sound: Option<&UNNotificationSound>);

        #[cfg(feature = "Foundation_NSString")]
        #[cfg(not(any(target_os = "tvos")))]
        #[method_id(@__retain_semantics Other subtitle)]
        pub unsafe fn subtitle(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[cfg(not(any(target_os = "tvos")))]
        #[method(setSubtitle:)]
        pub unsafe fn setSubtitle(&self, subtitle: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[cfg(not(any(target_os = "tvos")))]
        #[method_id(@__retain_semantics Other threadIdentifier)]
        pub unsafe fn threadIdentifier(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[cfg(not(any(target_os = "tvos")))]
        #[method(setThreadIdentifier:)]
        pub unsafe fn setThreadIdentifier(&self, thread_identifier: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[cfg(not(any(target_os = "tvos")))]
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[cfg(not(any(target_os = "tvos")))]
        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: &NSString);

        #[cfg(feature = "Foundation_NSDictionary")]
        #[cfg(not(any(target_os = "tvos")))]
        #[method_id(@__retain_semantics Other userInfo)]
        pub unsafe fn userInfo(&self) -> Id<NSDictionary>;

        #[cfg(feature = "Foundation_NSDictionary")]
        #[cfg(not(any(target_os = "tvos")))]
        #[method(setUserInfo:)]
        pub unsafe fn setUserInfo(&self, user_info: &NSDictionary);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "summaryArgument is ignored"]
        #[method_id(@__retain_semantics Other summaryArgument)]
        pub unsafe fn summaryArgument(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "summaryArgument is ignored"]
        #[method(setSummaryArgument:)]
        pub unsafe fn setSummaryArgument(&self, summary_argument: &NSString);

        #[deprecated = "summaryArgumentCount is ignored"]
        #[method(summaryArgumentCount)]
        pub unsafe fn summaryArgumentCount(&self) -> NSUInteger;

        #[deprecated = "summaryArgumentCount is ignored"]
        #[method(setSummaryArgumentCount:)]
        pub unsafe fn setSummaryArgumentCount(&self, summary_argument_count: NSUInteger);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other targetContentIdentifier)]
        pub unsafe fn targetContentIdentifier(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setTargetContentIdentifier:)]
        pub unsafe fn setTargetContentIdentifier(
            &self,
            target_content_identifier: Option<&NSString>,
        );

        #[method(interruptionLevel)]
        pub unsafe fn interruptionLevel(&self) -> UNNotificationInterruptionLevel;

        #[method(setInterruptionLevel:)]
        pub unsafe fn setInterruptionLevel(
            &self,
            interruption_level: UNNotificationInterruptionLevel,
        );

        #[method(relevanceScore)]
        pub unsafe fn relevanceScore(&self) -> c_double;

        #[method(setRelevanceScore:)]
        pub unsafe fn setRelevanceScore(&self, relevance_score: c_double);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other filterCriteria)]
        pub unsafe fn filterCriteria(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setFilterCriteria:)]
        pub unsafe fn setFilterCriteria(&self, filter_criteria: Option<&NSString>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "UserNotifications_UNMutableNotificationContent")]
    unsafe impl UNMutableNotificationContent {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
