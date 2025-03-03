//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::UserNotifications::*;

typed_extensible_enum!(
    pub type UNNotificationSoundName = NSString;
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UserNotifications_UNNotificationSound")]
    pub struct UNNotificationSound;

    #[cfg(feature = "UserNotifications_UNNotificationSound")]
    unsafe impl ClassType for UNNotificationSound {
        type Super = NSObject;
    }
);

#[cfg(feature = "UserNotifications_UNNotificationSound")]
unsafe impl NSCoding for UNNotificationSound {}

#[cfg(feature = "UserNotifications_UNNotificationSound")]
unsafe impl NSObjectProtocol for UNNotificationSound {}

#[cfg(feature = "UserNotifications_UNNotificationSound")]
unsafe impl NSSecureCoding for UNNotificationSound {}

extern_methods!(
    #[cfg(feature = "UserNotifications_UNNotificationSound")]
    unsafe impl UNNotificationSound {
        #[method_id(@__retain_semantics Other defaultSound)]
        pub unsafe fn defaultSound() -> Id<UNNotificationSound>;

        #[method_id(@__retain_semantics Other defaultRingtoneSound)]
        pub unsafe fn defaultRingtoneSound() -> Id<UNNotificationSound>;

        #[method_id(@__retain_semantics Other defaultCriticalSound)]
        pub unsafe fn defaultCriticalSound() -> Id<UNNotificationSound>;

        #[method_id(@__retain_semantics Other defaultCriticalSoundWithAudioVolume:)]
        pub unsafe fn defaultCriticalSoundWithAudioVolume(volume: c_float) -> Id<Self>;

        #[method_id(@__retain_semantics Other soundNamed:)]
        pub unsafe fn soundNamed(name: &UNNotificationSoundName) -> Id<Self>;

        #[method_id(@__retain_semantics Other ringtoneSoundNamed:)]
        pub unsafe fn ringtoneSoundNamed(name: &UNNotificationSoundName) -> Id<Self>;

        #[method_id(@__retain_semantics Other criticalSoundNamed:)]
        pub unsafe fn criticalSoundNamed(name: &UNNotificationSoundName) -> Id<Self>;

        #[method_id(@__retain_semantics Other criticalSoundNamed:withAudioVolume:)]
        pub unsafe fn criticalSoundNamed_withAudioVolume(
            name: &UNNotificationSoundName,
            volume: c_float,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);
