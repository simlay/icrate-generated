//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::UserNotifications::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UserNotifications_UNNotificationTrigger")]
    pub struct UNNotificationTrigger;

    #[cfg(feature = "UserNotifications_UNNotificationTrigger")]
    unsafe impl ClassType for UNNotificationTrigger {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "UserNotifications_UNNotificationTrigger")]
unsafe impl NSCoding for UNNotificationTrigger {}

#[cfg(feature = "UserNotifications_UNNotificationTrigger")]
unsafe impl NSCopying for UNNotificationTrigger {}

#[cfg(feature = "UserNotifications_UNNotificationTrigger")]
unsafe impl NSObjectProtocol for UNNotificationTrigger {}

#[cfg(feature = "UserNotifications_UNNotificationTrigger")]
unsafe impl NSSecureCoding for UNNotificationTrigger {}

extern_methods!(
    #[cfg(feature = "UserNotifications_UNNotificationTrigger")]
    unsafe impl UNNotificationTrigger {
        #[method(repeats)]
        pub unsafe fn repeats(&self) -> bool;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "UserNotifications_UNNotificationTrigger")]
    unsafe impl UNNotificationTrigger {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UserNotifications_UNPushNotificationTrigger")]
    pub struct UNPushNotificationTrigger;

    #[cfg(feature = "UserNotifications_UNPushNotificationTrigger")]
    unsafe impl ClassType for UNPushNotificationTrigger {
        #[inherits(NSObject)]
        type Super = UNNotificationTrigger;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "UserNotifications_UNPushNotificationTrigger")]
unsafe impl NSCoding for UNPushNotificationTrigger {}

#[cfg(feature = "UserNotifications_UNPushNotificationTrigger")]
unsafe impl NSCopying for UNPushNotificationTrigger {}

#[cfg(feature = "UserNotifications_UNPushNotificationTrigger")]
unsafe impl NSObjectProtocol for UNPushNotificationTrigger {}

#[cfg(feature = "UserNotifications_UNPushNotificationTrigger")]
unsafe impl NSSecureCoding for UNPushNotificationTrigger {}

extern_methods!(
    #[cfg(feature = "UserNotifications_UNPushNotificationTrigger")]
    unsafe impl UNPushNotificationTrigger {}
);

extern_methods!(
    /// Methods declared on superclass `UNNotificationTrigger`
    #[cfg(feature = "UserNotifications_UNPushNotificationTrigger")]
    unsafe impl UNPushNotificationTrigger {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "UserNotifications_UNPushNotificationTrigger")]
    unsafe impl UNPushNotificationTrigger {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UserNotifications_UNTimeIntervalNotificationTrigger")]
    pub struct UNTimeIntervalNotificationTrigger;

    #[cfg(feature = "UserNotifications_UNTimeIntervalNotificationTrigger")]
    unsafe impl ClassType for UNTimeIntervalNotificationTrigger {
        #[inherits(NSObject)]
        type Super = UNNotificationTrigger;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "UserNotifications_UNTimeIntervalNotificationTrigger")]
unsafe impl NSCoding for UNTimeIntervalNotificationTrigger {}

#[cfg(feature = "UserNotifications_UNTimeIntervalNotificationTrigger")]
unsafe impl NSCopying for UNTimeIntervalNotificationTrigger {}

#[cfg(feature = "UserNotifications_UNTimeIntervalNotificationTrigger")]
unsafe impl NSObjectProtocol for UNTimeIntervalNotificationTrigger {}

#[cfg(feature = "UserNotifications_UNTimeIntervalNotificationTrigger")]
unsafe impl NSSecureCoding for UNTimeIntervalNotificationTrigger {}

extern_methods!(
    #[cfg(feature = "UserNotifications_UNTimeIntervalNotificationTrigger")]
    unsafe impl UNTimeIntervalNotificationTrigger {
        #[method(timeInterval)]
        pub unsafe fn timeInterval(&self) -> NSTimeInterval;

        #[method_id(@__retain_semantics Other triggerWithTimeInterval:repeats:)]
        pub unsafe fn triggerWithTimeInterval_repeats(
            time_interval: NSTimeInterval,
            repeats: bool,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other nextTriggerDate)]
        pub unsafe fn nextTriggerDate(&self) -> Option<Id<NSDate>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `UNNotificationTrigger`
    #[cfg(feature = "UserNotifications_UNTimeIntervalNotificationTrigger")]
    unsafe impl UNTimeIntervalNotificationTrigger {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "UserNotifications_UNTimeIntervalNotificationTrigger")]
    unsafe impl UNTimeIntervalNotificationTrigger {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UserNotifications_UNCalendarNotificationTrigger")]
    pub struct UNCalendarNotificationTrigger;

    #[cfg(feature = "UserNotifications_UNCalendarNotificationTrigger")]
    unsafe impl ClassType for UNCalendarNotificationTrigger {
        #[inherits(NSObject)]
        type Super = UNNotificationTrigger;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "UserNotifications_UNCalendarNotificationTrigger")]
unsafe impl NSCoding for UNCalendarNotificationTrigger {}

#[cfg(feature = "UserNotifications_UNCalendarNotificationTrigger")]
unsafe impl NSCopying for UNCalendarNotificationTrigger {}

#[cfg(feature = "UserNotifications_UNCalendarNotificationTrigger")]
unsafe impl NSObjectProtocol for UNCalendarNotificationTrigger {}

#[cfg(feature = "UserNotifications_UNCalendarNotificationTrigger")]
unsafe impl NSSecureCoding for UNCalendarNotificationTrigger {}

extern_methods!(
    #[cfg(feature = "UserNotifications_UNCalendarNotificationTrigger")]
    unsafe impl UNCalendarNotificationTrigger {
        #[cfg(feature = "Foundation_NSDateComponents")]
        #[method_id(@__retain_semantics Other dateComponents)]
        pub unsafe fn dateComponents(&self) -> Id<NSDateComponents>;

        #[cfg(feature = "Foundation_NSDateComponents")]
        #[method_id(@__retain_semantics Other triggerWithDateMatchingComponents:repeats:)]
        pub unsafe fn triggerWithDateMatchingComponents_repeats(
            date_components: &NSDateComponents,
            repeats: bool,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other nextTriggerDate)]
        pub unsafe fn nextTriggerDate(&self) -> Option<Id<NSDate>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `UNNotificationTrigger`
    #[cfg(feature = "UserNotifications_UNCalendarNotificationTrigger")]
    unsafe impl UNCalendarNotificationTrigger {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "UserNotifications_UNCalendarNotificationTrigger")]
    unsafe impl UNCalendarNotificationTrigger {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UserNotifications_UNLocationNotificationTrigger")]
    #[cfg(not(any(target_os = "macos", target_os = "tvos")))]
    pub struct UNLocationNotificationTrigger;

    #[cfg(feature = "UserNotifications_UNLocationNotificationTrigger")]
    unsafe impl ClassType for UNLocationNotificationTrigger {
        #[inherits(NSObject)]
        type Super = UNNotificationTrigger;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "UserNotifications_UNLocationNotificationTrigger")]
#[cfg(not(any(target_os = "macos", target_os = "tvos")))]
unsafe impl NSCoding for UNLocationNotificationTrigger {}

#[cfg(feature = "UserNotifications_UNLocationNotificationTrigger")]
#[cfg(not(any(target_os = "macos", target_os = "tvos")))]
unsafe impl NSCopying for UNLocationNotificationTrigger {}

#[cfg(feature = "UserNotifications_UNLocationNotificationTrigger")]
#[cfg(not(any(target_os = "macos", target_os = "tvos")))]
unsafe impl NSObjectProtocol for UNLocationNotificationTrigger {}

#[cfg(feature = "UserNotifications_UNLocationNotificationTrigger")]
#[cfg(not(any(target_os = "macos", target_os = "tvos")))]
unsafe impl NSSecureCoding for UNLocationNotificationTrigger {}

#[cfg(not(any(target_os = "macos", target_os = "tvos")))]
extern_methods!(
    #[cfg(feature = "UserNotifications_UNLocationNotificationTrigger")]
    unsafe impl UNLocationNotificationTrigger {
        #[cfg(feature = "CoreLocation_CLRegion")]
        #[method_id(@__retain_semantics Other region)]
        pub unsafe fn region(&self) -> Id<CLRegion>;

        #[cfg(feature = "CoreLocation_CLRegion")]
        #[method_id(@__retain_semantics Other triggerWithRegion:repeats:)]
        pub unsafe fn triggerWithRegion_repeats(region: &CLRegion, repeats: bool) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `UNNotificationTrigger`
    #[cfg(feature = "UserNotifications_UNLocationNotificationTrigger")]
    unsafe impl UNLocationNotificationTrigger {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "UserNotifications_UNLocationNotificationTrigger")]
    unsafe impl UNLocationNotificationTrigger {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
