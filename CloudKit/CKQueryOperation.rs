//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CloudKit::*;
use crate::CoreLocation::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CloudKit_CKQueryCursor")]
    pub struct CKQueryCursor;

    #[cfg(feature = "CloudKit_CKQueryCursor")]
    unsafe impl ClassType for CKQueryCursor {
        type Super = NSObject;
    }
);

#[cfg(feature = "CloudKit_CKQueryCursor")]
unsafe impl NSCoding for CKQueryCursor {}

#[cfg(feature = "CloudKit_CKQueryCursor")]
unsafe impl NSObjectProtocol for CKQueryCursor {}

#[cfg(feature = "CloudKit_CKQueryCursor")]
unsafe impl NSSecureCoding for CKQueryCursor {}

extern_methods!(
    #[cfg(feature = "CloudKit_CKQueryCursor")]
    unsafe impl CKQueryCursor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_static!(CKQueryOperationMaximumResults: NSUInteger);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CloudKit_CKQueryOperation")]
    pub struct CKQueryOperation;

    #[cfg(feature = "CloudKit_CKQueryOperation")]
    unsafe impl ClassType for CKQueryOperation {
        #[inherits(CKOperation, NSOperation, NSObject)]
        type Super = CKDatabaseOperation;
    }
);

#[cfg(feature = "CloudKit_CKQueryOperation")]
unsafe impl NSObjectProtocol for CKQueryOperation {}

extern_methods!(
    #[cfg(feature = "CloudKit_CKQueryOperation")]
    unsafe impl CKQueryOperation {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[cfg(feature = "CloudKit_CKQuery")]
        #[method_id(@__retain_semantics Init initWithQuery:)]
        pub unsafe fn initWithQuery(this: Option<Allocated<Self>>, query: &CKQuery) -> Id<Self>;

        #[cfg(feature = "CloudKit_CKQueryCursor")]
        #[method_id(@__retain_semantics Init initWithCursor:)]
        pub unsafe fn initWithCursor(
            this: Option<Allocated<Self>>,
            cursor: &CKQueryCursor,
        ) -> Id<Self>;

        #[cfg(feature = "CloudKit_CKQuery")]
        #[method_id(@__retain_semantics Other query)]
        pub unsafe fn query(&self) -> Option<Id<CKQuery>>;

        #[cfg(feature = "CloudKit_CKQuery")]
        #[method(setQuery:)]
        pub unsafe fn setQuery(&self, query: Option<&CKQuery>);

        #[cfg(feature = "CloudKit_CKQueryCursor")]
        #[method_id(@__retain_semantics Other cursor)]
        pub unsafe fn cursor(&self) -> Option<Id<CKQueryCursor>>;

        #[cfg(feature = "CloudKit_CKQueryCursor")]
        #[method(setCursor:)]
        pub unsafe fn setCursor(&self, cursor: Option<&CKQueryCursor>);

        #[cfg(feature = "CloudKit_CKRecordZoneID")]
        #[method_id(@__retain_semantics Other zoneID)]
        pub unsafe fn zoneID(&self) -> Option<Id<CKRecordZoneID>>;

        #[cfg(feature = "CloudKit_CKRecordZoneID")]
        #[method(setZoneID:)]
        pub unsafe fn setZoneID(&self, zone_id: Option<&CKRecordZoneID>);

        #[method(resultsLimit)]
        pub unsafe fn resultsLimit(&self) -> NSUInteger;

        #[method(setResultsLimit:)]
        pub unsafe fn setResultsLimit(&self, results_limit: NSUInteger);

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other desiredKeys)]
        pub unsafe fn desiredKeys(&self) -> Option<Id<NSArray<CKRecordFieldKey>>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setDesiredKeys:)]
        pub unsafe fn setDesiredKeys(&self, desired_keys: Option<&NSArray<CKRecordFieldKey>>);

        #[cfg(feature = "CloudKit_CKRecord")]
        #[deprecated = "Use recordMatchedBlock instead, which surfaces per-record errors"]
        #[method(recordFetchedBlock)]
        pub unsafe fn recordFetchedBlock(&self) -> *mut Block<(NonNull<CKRecord>,), ()>;

        #[cfg(feature = "CloudKit_CKRecord")]
        #[deprecated = "Use recordMatchedBlock instead, which surfaces per-record errors"]
        #[method(setRecordFetchedBlock:)]
        pub unsafe fn setRecordFetchedBlock(
            &self,
            record_fetched_block: Option<&Block<(NonNull<CKRecord>,), ()>>,
        );

        #[cfg(all(
            feature = "CloudKit_CKRecord",
            feature = "CloudKit_CKRecordID",
            feature = "Foundation_NSError"
        ))]
        #[method(recordMatchedBlock)]
        pub unsafe fn recordMatchedBlock(
            &self,
        ) -> *mut Block<(NonNull<CKRecordID>, *mut CKRecord, *mut NSError), ()>;

        #[cfg(all(
            feature = "CloudKit_CKRecord",
            feature = "CloudKit_CKRecordID",
            feature = "Foundation_NSError"
        ))]
        #[method(setRecordMatchedBlock:)]
        pub unsafe fn setRecordMatchedBlock(
            &self,
            record_matched_block: Option<
                &Block<(NonNull<CKRecordID>, *mut CKRecord, *mut NSError), ()>,
            >,
        );

        #[cfg(all(feature = "CloudKit_CKQueryCursor", feature = "Foundation_NSError"))]
        #[method(queryCompletionBlock)]
        pub unsafe fn queryCompletionBlock(
            &self,
        ) -> *mut Block<(*mut CKQueryCursor, *mut NSError), ()>;

        #[cfg(all(feature = "CloudKit_CKQueryCursor", feature = "Foundation_NSError"))]
        #[method(setQueryCompletionBlock:)]
        pub unsafe fn setQueryCompletionBlock(
            &self,
            query_completion_block: Option<&Block<(*mut CKQueryCursor, *mut NSError), ()>>,
        );
    }
);
