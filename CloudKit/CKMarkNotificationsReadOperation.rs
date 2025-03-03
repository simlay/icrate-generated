//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CloudKit::*;
use crate::CoreLocation::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CloudKit_CKMarkNotificationsReadOperation")]
    #[deprecated = "Instead of iterating notifications, consider using CKDatabaseSubscription, CKFetchDatabaseChangesOperation, and CKFetchRecordZoneChangesOperation as appropriate"]
    pub struct CKMarkNotificationsReadOperation;

    #[cfg(feature = "CloudKit_CKMarkNotificationsReadOperation")]
    unsafe impl ClassType for CKMarkNotificationsReadOperation {
        #[inherits(NSOperation, NSObject)]
        type Super = CKOperation;
    }
);

#[cfg(feature = "CloudKit_CKMarkNotificationsReadOperation")]
unsafe impl NSObjectProtocol for CKMarkNotificationsReadOperation {}

extern_methods!(
    #[cfg(feature = "CloudKit_CKMarkNotificationsReadOperation")]
    unsafe impl CKMarkNotificationsReadOperation {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[cfg(all(feature = "CloudKit_CKNotificationID", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Init initWithNotificationIDsToMarkRead:)]
        pub unsafe fn initWithNotificationIDsToMarkRead(
            this: Option<Allocated<Self>>,
            notification_i_ds: &NSArray<CKNotificationID>,
        ) -> Id<Self>;

        #[cfg(all(feature = "CloudKit_CKNotificationID", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other notificationIDs)]
        pub unsafe fn notificationIDs(&self) -> Option<Id<NSArray<CKNotificationID>>>;

        #[cfg(all(feature = "CloudKit_CKNotificationID", feature = "Foundation_NSArray"))]
        #[method(setNotificationIDs:)]
        pub unsafe fn setNotificationIDs(
            &self,
            notification_i_ds: Option<&NSArray<CKNotificationID>>,
        );

        #[cfg(all(
            feature = "CloudKit_CKNotificationID",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError"
        ))]
        #[method(markNotificationsReadCompletionBlock)]
        pub unsafe fn markNotificationsReadCompletionBlock(
            &self,
        ) -> *mut Block<(*mut NSArray<CKNotificationID>, *mut NSError), ()>;

        #[cfg(all(
            feature = "CloudKit_CKNotificationID",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError"
        ))]
        #[method(setMarkNotificationsReadCompletionBlock:)]
        pub unsafe fn setMarkNotificationsReadCompletionBlock(
            &self,
            mark_notifications_read_completion_block: Option<
                &Block<(*mut NSArray<CKNotificationID>, *mut NSError), ()>,
            >,
        );
    }
);
