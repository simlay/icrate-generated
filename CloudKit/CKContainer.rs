//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CloudKit::*;
use crate::CoreLocation::*;
use crate::Foundation::*;

extern_static!(CKCurrentUserDefaultName: &'static NSString);

extern_static!(CKOwnerDefaultName: &'static NSString);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CloudKit_CKContainer")]
    /**
      @class CKContainer

      @abstract A CKContainer, and its CKDatabases, are the main entry points into the CloudKit framework.

      @discussion
      Several methods in CloudKit accept completion handlers to indicate when they're completed.
      All CKOperation subclasses include progress and completion blocks to report significant events in their lifecycles.
      Each of these handlers and blocks is invoked on a non-main serial queue.  The receiver is responsible for handling the message on a different queue or thread if it is required.
    */
    pub struct CKContainer;

    #[cfg(feature = "CloudKit_CKContainer")]
    unsafe impl ClassType for CKContainer {
        type Super = NSObject;
    }
);

#[cfg(feature = "CloudKit_CKContainer")]
/**
  @class CKContainer

  @abstract A CKContainer, and its CKDatabases, are the main entry points into the CloudKit framework.

  @discussion
  Several methods in CloudKit accept completion handlers to indicate when they're completed.
  All CKOperation subclasses include progress and completion blocks to report significant events in their lifecycles.
  Each of these handlers and blocks is invoked on a non-main serial queue.  The receiver is responsible for handling the message on a different queue or thread if it is required.
*/
unsafe impl NSObjectProtocol for CKContainer {}

extern_methods!(
    /**
      @class CKContainer

      @abstract A CKContainer, and its CKDatabases, are the main entry points into the CloudKit framework.

      @discussion
      Several methods in CloudKit accept completion handlers to indicate when they're completed.
      All CKOperation subclasses include progress and completion blocks to report significant events in their lifecycles.
      Each of these handlers and blocks is invoked on a non-main serial queue.  The receiver is responsible for handling the message on a different queue or thread if it is required.
    */
    #[cfg(feature = "CloudKit_CKContainer")]
    unsafe impl CKContainer {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Other defaultContainer)]
        pub unsafe fn defaultContainer() -> Id<CKContainer>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other containerWithIdentifier:)]
        pub unsafe fn containerWithIdentifier(container_identifier: &NSString) -> Id<CKContainer>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other containerIdentifier)]
        pub unsafe fn containerIdentifier(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "CloudKit_CKOperation")]
        #[method(addOperation:)]
        pub unsafe fn addOperation(&self, operation: &CKOperation);
    }
);

extern_methods!(
    /**
      @discussion
      Database properties:
      Records in a public database
      - By default are world readable, owner writable.
      - Can be locked down by Roles, a process done in the Developer Portal, a web interface.  Roles are not present in the client API.
      - Are visible to the application developer via the Developer Portal.
      - Do not contribute to the owner's iCloud account storage quota.
      Records in a private database
      - By default are only owner readable and owner writable.
      - Are not visible to the application developer via the Developer Portal.
      - Are counted towards the owner's iCloud account storage quota.
      Records in a shared database
      - Are available to share participants based on the permissions of the enclosing CKShare
      - Are not visible to the application developer via the Developer Portal.
      - Are counted towards the originating owner's iCloud account storage quota.
    */
    /// Database
    #[cfg(feature = "CloudKit_CKContainer")]
    unsafe impl CKContainer {
        #[cfg(feature = "CloudKit_CKDatabase")]
        #[method_id(@__retain_semantics Other privateCloudDatabase)]
        pub unsafe fn privateCloudDatabase(&self) -> Id<CKDatabase>;

        #[cfg(feature = "CloudKit_CKDatabase")]
        #[method_id(@__retain_semantics Other publicCloudDatabase)]
        pub unsafe fn publicCloudDatabase(&self) -> Id<CKDatabase>;

        #[cfg(feature = "CloudKit_CKDatabase")]
        #[method_id(@__retain_semantics Other sharedCloudDatabase)]
        pub unsafe fn sharedCloudDatabase(&self) -> Id<CKDatabase>;

        #[cfg(feature = "CloudKit_CKDatabase")]
        #[method_id(@__retain_semantics Other databaseWithDatabaseScope:)]
        pub unsafe fn databaseWithDatabaseScope(
            &self,
            database_scope: CKDatabaseScope,
        ) -> Id<CKDatabase>;
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    /**
      @enum CKAccountStatus
      @constant CKAccountStatusCouldNotDetermine An error occurred when getting the account status, consult the corresponding NSError.
      @constant CKAccountStatusAvailable The iCloud account credentials are available for this application
      @constant CKAccountStatusRestricted Parental Controls / Device Management has denied access to iCloud account credentials
      @constant CKAccountStatusNoAccount No iCloud account is logged in on this device
      @constant CKAccountStatusTemporarilyUnavailable An iCloud account is logged in but not ready. The user can be asked to verify their
      credentials in Settings app.
    */
    pub enum CKAccountStatus {
        CKAccountStatusCouldNotDetermine = 0,
        CKAccountStatusAvailable = 1,
        CKAccountStatusRestricted = 2,
        CKAccountStatusNoAccount = 3,
        CKAccountStatusTemporarilyUnavailable = 4,
    }
);

extern_static!(CKAccountChangedNotification: &'static NSString);

extern_methods!(
    /// AccountStatus
    #[cfg(feature = "CloudKit_CKContainer")]
    unsafe impl CKContainer {
        #[cfg(feature = "Foundation_NSError")]
        #[method(accountStatusWithCompletionHandler:)]
        pub unsafe fn accountStatusWithCompletionHandler(
            &self,
            completion_handler: &Block<(CKAccountStatus, *mut NSError), ()>,
        );
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum CKApplicationPermissions {
        CKApplicationPermissionUserDiscoverability = 1 << 0,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    /**
      @enum CKApplicationPermissionStatus
      @constant CKApplicationPermissionStatusInitialState The user has not made a decision for this application permission.
      @constant CKApplicationPermissionStatusCouldNotComplete An error occurred when getting or setting the application permission status, consult the corresponding NSError
      @constant CKApplicationPermissionStatusDenied The user has denied this application permission
      @constant CKApplicationPermissionStatusGranted The user has granted this application permission
    */
    pub enum CKApplicationPermissionStatus {
        CKApplicationPermissionStatusInitialState = 0,
        CKApplicationPermissionStatusCouldNotComplete = 1,
        CKApplicationPermissionStatusDenied = 2,
        CKApplicationPermissionStatusGranted = 3,
    }
);

pub type CKApplicationPermissionBlock =
    *mut Block<(CKApplicationPermissionStatus, *mut NSError), ()>;

extern_methods!(
    /// ApplicationPermission
    #[cfg(feature = "CloudKit_CKContainer")]
    unsafe impl CKContainer {
        #[method(statusForApplicationPermission:completionHandler:)]
        pub unsafe fn statusForApplicationPermission_completionHandler(
            &self,
            application_permission: CKApplicationPermissions,
            completion_handler: CKApplicationPermissionBlock,
        );

        #[method(requestApplicationPermission:completionHandler:)]
        pub unsafe fn requestApplicationPermission_completionHandler(
            &self,
            application_permission: CKApplicationPermissions,
            completion_handler: CKApplicationPermissionBlock,
        );
    }
);

extern_methods!(
    /// UserRecords
    #[cfg(feature = "CloudKit_CKContainer")]
    unsafe impl CKContainer {
        #[cfg(all(feature = "CloudKit_CKRecordID", feature = "Foundation_NSError"))]
        #[method(fetchUserRecordIDWithCompletionHandler:)]
        pub unsafe fn fetchUserRecordIDWithCompletionHandler(
            &self,
            completion_handler: &Block<(*mut CKRecordID, *mut NSError), ()>,
        );

        #[cfg(all(
            feature = "CloudKit_CKUserIdentity",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError"
        ))]
        #[method(discoverAllIdentitiesWithCompletionHandler:)]
        pub unsafe fn discoverAllIdentitiesWithCompletionHandler(
            &self,
            completion_handler: &Block<(*mut NSArray<CKUserIdentity>, *mut NSError), ()>,
        );

        #[cfg(all(
            feature = "CloudKit_CKUserIdentity",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString"
        ))]
        #[method(discoverUserIdentityWithEmailAddress:completionHandler:)]
        pub unsafe fn discoverUserIdentityWithEmailAddress_completionHandler(
            &self,
            email: &NSString,
            completion_handler: &Block<(*mut CKUserIdentity, *mut NSError), ()>,
        );

        #[cfg(all(
            feature = "CloudKit_CKUserIdentity",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString"
        ))]
        #[method(discoverUserIdentityWithPhoneNumber:completionHandler:)]
        pub unsafe fn discoverUserIdentityWithPhoneNumber_completionHandler(
            &self,
            phone_number: &NSString,
            completion_handler: &Block<(*mut CKUserIdentity, *mut NSError), ()>,
        );

        #[cfg(all(
            feature = "CloudKit_CKRecordID",
            feature = "CloudKit_CKUserIdentity",
            feature = "Foundation_NSError"
        ))]
        #[method(discoverUserIdentityWithUserRecordID:completionHandler:)]
        pub unsafe fn discoverUserIdentityWithUserRecordID_completionHandler(
            &self,
            user_record_id: &CKRecordID,
            completion_handler: &Block<(*mut CKUserIdentity, *mut NSError), ()>,
        );
    }
);

extern_methods!(
    /// Sharing
    #[cfg(feature = "CloudKit_CKContainer")]
    unsafe impl CKContainer {
        #[cfg(all(
            feature = "CloudKit_CKShareParticipant",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString"
        ))]
        #[method(fetchShareParticipantWithEmailAddress:completionHandler:)]
        pub unsafe fn fetchShareParticipantWithEmailAddress_completionHandler(
            &self,
            email_address: &NSString,
            completion_handler: &Block<(*mut CKShareParticipant, *mut NSError), ()>,
        );

        #[cfg(all(
            feature = "CloudKit_CKShareParticipant",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString"
        ))]
        #[method(fetchShareParticipantWithPhoneNumber:completionHandler:)]
        pub unsafe fn fetchShareParticipantWithPhoneNumber_completionHandler(
            &self,
            phone_number: &NSString,
            completion_handler: &Block<(*mut CKShareParticipant, *mut NSError), ()>,
        );

        #[cfg(all(
            feature = "CloudKit_CKRecordID",
            feature = "CloudKit_CKShareParticipant",
            feature = "Foundation_NSError"
        ))]
        #[method(fetchShareParticipantWithUserRecordID:completionHandler:)]
        pub unsafe fn fetchShareParticipantWithUserRecordID_completionHandler(
            &self,
            user_record_id: &CKRecordID,
            completion_handler: &Block<(*mut CKShareParticipant, *mut NSError), ()>,
        );

        #[cfg(all(
            feature = "CloudKit_CKShareMetadata",
            feature = "Foundation_NSError",
            feature = "Foundation_NSURL"
        ))]
        #[method(fetchShareMetadataWithURL:completionHandler:)]
        pub unsafe fn fetchShareMetadataWithURL_completionHandler(
            &self,
            url: &NSURL,
            completion_handler: &Block<(*mut CKShareMetadata, *mut NSError), ()>,
        );

        #[cfg(all(
            feature = "CloudKit_CKShare",
            feature = "CloudKit_CKShareMetadata",
            feature = "Foundation_NSError"
        ))]
        #[method(acceptShareMetadata:completionHandler:)]
        pub unsafe fn acceptShareMetadata_completionHandler(
            &self,
            metadata: &CKShareMetadata,
            completion_handler: &Block<(*mut CKShare, *mut NSError), ()>,
        );
    }
);

extern_methods!(
    /// CKLongLivedOperations
    #[cfg(feature = "CloudKit_CKContainer")]
    unsafe impl CKContainer {
        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSError"))]
        #[method(fetchAllLongLivedOperationIDsWithCompletionHandler:)]
        pub unsafe fn fetchAllLongLivedOperationIDsWithCompletionHandler(
            &self,
            completion_handler: &Block<(*mut NSArray<CKOperationID>, *mut NSError), ()>,
        );

        #[cfg(all(feature = "CloudKit_CKOperation", feature = "Foundation_NSError"))]
        #[method(fetchLongLivedOperationWithID:completionHandler:)]
        pub unsafe fn fetchLongLivedOperationWithID_completionHandler(
            &self,
            operation_id: &CKOperationID,
            completion_handler: &Block<(*mut CKOperation, *mut NSError), ()>,
        );
    }
);
