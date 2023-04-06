//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::UserNotifications::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "UserNotifications_UNNotificationServiceExtension")]
    #[cfg(not(any(target_os = "tvos")))]
    pub struct UNNotificationServiceExtension;

    #[cfg(not(any(target_os = "tvos")))]
    #[cfg(feature = "UserNotifications_UNNotificationServiceExtension")]
    unsafe impl ClassType for UNNotificationServiceExtension {
        type Super = NSObject;
    }
);

#[cfg(feature = "UserNotifications_UNNotificationServiceExtension")]
#[cfg(not(any(target_os = "tvos")))]
unsafe impl NSObjectProtocol for UNNotificationServiceExtension {}

extern_methods!(
    #[cfg(feature = "UserNotifications_UNNotificationServiceExtension")]
    #[cfg(not(any(target_os = "tvos")))]
    unsafe impl UNNotificationServiceExtension {
        #[cfg(not(any(target_os = "tvos")))]
        #[cfg(all(
            feature = "UserNotifications_UNNotificationContent",
            feature = "UserNotifications_UNNotificationRequest"
        ))]
        #[method(didReceiveNotificationRequest:withContentHandler:)]
        pub unsafe fn didReceiveNotificationRequest_withContentHandler(
            &self,
            request: &UNNotificationRequest,
            content_handler: &Block<(NonNull<UNNotificationContent>,), ()>,
        );

        #[cfg(not(any(target_os = "tvos")))]
        #[method(serviceExtensionTimeWillExpire)]
        pub unsafe fn serviceExtensionTimeWillExpire(&self);
    }
);
