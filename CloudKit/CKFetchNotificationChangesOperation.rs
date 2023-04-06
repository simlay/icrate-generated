//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CloudKit::*;
use crate::CoreLocation::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CloudKit_CKFetchNotificationChangesOperation")]
    #[deprecated = "Instead of iterating notifications to enumerate changed record zones, use CKDatabaseSubscription, CKFetchDatabaseChangesOperation, and CKFetchRecordZoneChangesOperation"]
    pub struct CKFetchNotificationChangesOperation;

    #[deprecated = "Instead of iterating notifications to enumerate changed record zones, use CKDatabaseSubscription, CKFetchDatabaseChangesOperation, and CKFetchRecordZoneChangesOperation"]
    #[cfg(feature = "CloudKit_CKFetchNotificationChangesOperation")]
    unsafe impl ClassType for CKFetchNotificationChangesOperation {
        #[inherits(NSOperation, NSObject)]
        type Super = CKOperation;
    }
);

#[cfg(feature = "CloudKit_CKFetchNotificationChangesOperation")]
unsafe impl NSObjectProtocol for CKFetchNotificationChangesOperation {}

extern_methods!(
    #[cfg(feature = "CloudKit_CKFetchNotificationChangesOperation")]
    unsafe impl CKFetchNotificationChangesOperation {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[cfg(feature = "CloudKit_CKServerChangeToken")]
        #[method_id(@__retain_semantics Init initWithPreviousServerChangeToken:)]
        pub unsafe fn initWithPreviousServerChangeToken(
            this: Option<Allocated<Self>>,
            previous_server_change_token: Option<&CKServerChangeToken>,
        ) -> Id<Self>;

        #[cfg(feature = "CloudKit_CKServerChangeToken")]
        #[method_id(@__retain_semantics Other previousServerChangeToken)]
        pub unsafe fn previousServerChangeToken(&self) -> Option<Id<CKServerChangeToken>>;

        #[cfg(feature = "CloudKit_CKServerChangeToken")]
        #[method(setPreviousServerChangeToken:)]
        pub unsafe fn setPreviousServerChangeToken(
            &self,
            previous_server_change_token: Option<&CKServerChangeToken>,
        );

        #[method(resultsLimit)]
        pub unsafe fn resultsLimit(&self) -> NSUInteger;

        #[method(setResultsLimit:)]
        pub unsafe fn setResultsLimit(&self, results_limit: NSUInteger);

        #[method(moreComing)]
        pub unsafe fn moreComing(&self) -> bool;

        #[cfg(feature = "CloudKit_CKNotification")]
        #[method(notificationChangedBlock)]
        pub unsafe fn notificationChangedBlock(&self)
            -> *mut Block<(NonNull<CKNotification>,), ()>;

        #[cfg(feature = "CloudKit_CKNotification")]
        #[method(setNotificationChangedBlock:)]
        pub unsafe fn setNotificationChangedBlock(
            &self,
            notification_changed_block: Option<&Block<(NonNull<CKNotification>,), ()>>,
        );

        #[cfg(all(
            feature = "CloudKit_CKServerChangeToken",
            feature = "Foundation_NSError"
        ))]
        #[method(fetchNotificationChangesCompletionBlock)]
        pub unsafe fn fetchNotificationChangesCompletionBlock(
            &self,
        ) -> *mut Block<(*mut CKServerChangeToken, *mut NSError), ()>;

        #[cfg(all(
            feature = "CloudKit_CKServerChangeToken",
            feature = "Foundation_NSError"
        ))]
        #[method(setFetchNotificationChangesCompletionBlock:)]
        pub unsafe fn setFetchNotificationChangesCompletionBlock(
            &self,
            fetch_notification_changes_completion_block: Option<
                &Block<(*mut CKServerChangeToken, *mut NSError), ()>,
            >,
        );
    }
);
