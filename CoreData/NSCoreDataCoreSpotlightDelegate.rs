//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_static!(
    NSCoreDataCoreSpotlightDelegateIndexDidUpdateNotification: &'static NSNotificationName
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreData_NSCoreDataCoreSpotlightDelegate")]
    #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
    pub struct NSCoreDataCoreSpotlightDelegate;

    #[cfg(feature = "CoreData_NSCoreDataCoreSpotlightDelegate")]
    unsafe impl ClassType for NSCoreDataCoreSpotlightDelegate {
        type Super = NSObject;
    }
);

#[cfg(feature = "CoreData_NSCoreDataCoreSpotlightDelegate")]
unsafe impl NSObjectProtocol for NSCoreDataCoreSpotlightDelegate {}

extern_methods!(
    #[cfg(feature = "CoreData_NSCoreDataCoreSpotlightDelegate")]
    unsafe impl NSCoreDataCoreSpotlightDelegate {
        #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
        #[method(isIndexingEnabled)]
        pub unsafe fn isIndexingEnabled(&self) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other domainIdentifier)]
        pub unsafe fn domainIdentifier(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other indexName)]
        pub unsafe fn indexName(&self) -> Option<Id<NSString>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[cfg(all(
            feature = "CoreData_NSPersistentStoreCoordinator",
            feature = "CoreData_NSPersistentStoreDescription"
        ))]
        #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
        #[method_id(@__retain_semantics Init initForStoreWithDescription:coordinator:)]
        pub unsafe fn initForStoreWithDescription_coordinator(
            this: Option<Allocated<Self>>,
            description: &NSPersistentStoreDescription,
            psc: &NSPersistentStoreCoordinator,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "CoreData_NSManagedObjectModel",
            feature = "CoreData_NSPersistentStoreDescription"
        ))]
        #[deprecated]
        #[method_id(@__retain_semantics Init initForStoreWithDescription:model:)]
        pub unsafe fn initForStoreWithDescription_model(
            this: Option<Allocated<Self>>,
            description: &NSPersistentStoreDescription,
            model: &NSManagedObjectModel,
        ) -> Id<Self>;

        #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
        #[method(startSpotlightIndexing)]
        pub unsafe fn startSpotlightIndexing(&self);

        #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
        #[method(stopSpotlightIndexing)]
        pub unsafe fn stopSpotlightIndexing(&self);

        #[cfg(feature = "Foundation_NSError")]
        #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
        #[method(deleteSpotlightIndexWithCompletionHandler:)]
        pub unsafe fn deleteSpotlightIndexWithCompletionHandler(
            &self,
            completion_handler: &Block<(*mut NSError,), ()>,
        );
    }
);
