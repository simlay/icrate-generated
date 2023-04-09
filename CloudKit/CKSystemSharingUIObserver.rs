//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CloudKit::*;
use crate::CoreLocation::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CloudKit_CKSystemSharingUIObserver")]
    #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
    pub struct CKSystemSharingUIObserver;

    #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
    #[cfg(feature = "CloudKit_CKSystemSharingUIObserver")]
    unsafe impl ClassType for CKSystemSharingUIObserver {
        type Super = NSObject;
    }
);

#[cfg(feature = "CloudKit_CKSystemSharingUIObserver")]
#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
unsafe impl NSObjectProtocol for CKSystemSharingUIObserver {}

#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
extern_methods!(
    #[cfg(feature = "CloudKit_CKSystemSharingUIObserver")]
    unsafe impl CKSystemSharingUIObserver {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[cfg(feature = "CloudKit_CKContainer")]
        #[method_id(@__retain_semantics Init initWithContainer:)]
        pub unsafe fn initWithContainer(
            this: Option<Allocated<Self>>,
            container: &CKContainer,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "CloudKit_CKRecordID",
            feature = "CloudKit_CKShare",
            feature = "Foundation_NSError"
        ))]
        #[method(systemSharingUIDidSaveShareBlock)]
        pub unsafe fn systemSharingUIDidSaveShareBlock(
            &self,
        ) -> *mut Block<(NonNull<CKRecordID>, *mut CKShare, *mut NSError), ()>;

        #[cfg(all(
            feature = "CloudKit_CKRecordID",
            feature = "CloudKit_CKShare",
            feature = "Foundation_NSError"
        ))]
        #[method(setSystemSharingUIDidSaveShareBlock:)]
        pub unsafe fn setSystemSharingUIDidSaveShareBlock(
            &self,
            system_sharing_ui_did_save_share_block: Option<
                &Block<(NonNull<CKRecordID>, *mut CKShare, *mut NSError), ()>,
            >,
        );

        #[cfg(all(feature = "CloudKit_CKRecordID", feature = "Foundation_NSError"))]
        #[method(systemSharingUIDidStopSharingBlock)]
        pub unsafe fn systemSharingUIDidStopSharingBlock(
            &self,
        ) -> *mut Block<(NonNull<CKRecordID>, *mut NSError), ()>;

        #[cfg(all(feature = "CloudKit_CKRecordID", feature = "Foundation_NSError"))]
        #[method(setSystemSharingUIDidStopSharingBlock:)]
        pub unsafe fn setSystemSharingUIDidStopSharingBlock(
            &self,
            system_sharing_ui_did_stop_sharing_block: Option<
                &Block<(NonNull<CKRecordID>, *mut NSError), ()>,
            >,
        );
    }
);
