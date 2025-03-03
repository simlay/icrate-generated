//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreData_NSPersistentStoreDescription")]
    pub struct NSPersistentStoreDescription;

    #[cfg(feature = "CoreData_NSPersistentStoreDescription")]
    unsafe impl ClassType for NSPersistentStoreDescription {
        type Super = NSObject;
    }
);

#[cfg(feature = "CoreData_NSPersistentStoreDescription")]
unsafe impl NSObjectProtocol for NSPersistentStoreDescription {}

extern_methods!(
    #[cfg(feature = "CoreData_NSPersistentStoreDescription")]
    unsafe impl NSPersistentStoreDescription {
        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other persistentStoreDescriptionWithURL:)]
        pub unsafe fn persistentStoreDescriptionWithURL(url: &NSURL) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other type)]
        pub unsafe fn r#type(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setType:)]
        pub unsafe fn setType(&self, r#type: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other configuration)]
        pub unsafe fn configuration(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setConfiguration:)]
        pub unsafe fn setConfiguration(&self, configuration: Option<&NSString>);

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other URL)]
        pub unsafe fn URL(&self) -> Option<Id<NSURL>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method(setURL:)]
        pub unsafe fn setURL(&self, url: Option<&NSURL>);

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other options)]
        pub unsafe fn options(&self) -> Id<NSDictionary<NSString, NSObject>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setOption:forKey:)]
        pub unsafe fn setOption_forKey(&self, option: Option<&NSObject>, key: &NSString);

        #[method(isReadOnly)]
        pub unsafe fn isReadOnly(&self) -> bool;

        #[method(setReadOnly:)]
        pub unsafe fn setReadOnly(&self, read_only: bool);

        #[method(timeout)]
        pub unsafe fn timeout(&self) -> NSTimeInterval;

        #[method(setTimeout:)]
        pub unsafe fn setTimeout(&self, timeout: NSTimeInterval);

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other sqlitePragmas)]
        pub unsafe fn sqlitePragmas(&self) -> Id<NSDictionary<NSString, NSObject>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setValue:forPragmaNamed:)]
        pub unsafe fn setValue_forPragmaNamed(&self, value: Option<&NSObject>, name: &NSString);

        #[method(shouldAddStoreAsynchronously)]
        pub unsafe fn shouldAddStoreAsynchronously(&self) -> bool;

        #[method(setShouldAddStoreAsynchronously:)]
        pub unsafe fn setShouldAddStoreAsynchronously(&self, should_add_store_asynchronously: bool);

        #[method(shouldMigrateStoreAutomatically)]
        pub unsafe fn shouldMigrateStoreAutomatically(&self) -> bool;

        #[method(setShouldMigrateStoreAutomatically:)]
        pub unsafe fn setShouldMigrateStoreAutomatically(
            &self,
            should_migrate_store_automatically: bool,
        );

        #[method(shouldInferMappingModelAutomatically)]
        pub unsafe fn shouldInferMappingModelAutomatically(&self) -> bool;

        #[method(setShouldInferMappingModelAutomatically:)]
        pub unsafe fn setShouldInferMappingModelAutomatically(
            &self,
            should_infer_mapping_model_automatically: bool,
        );

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Init initWithURL:)]
        pub unsafe fn initWithURL(this: Option<Allocated<Self>>, url: &NSURL) -> Id<Self>;
    }
);

extern_methods!(
    /// NSPersistentCloudKitContainerAdditions
    #[cfg(feature = "CoreData_NSPersistentStoreDescription")]
    unsafe impl NSPersistentStoreDescription {
        #[cfg(feature = "CoreData_NSPersistentCloudKitContainerOptions")]
        #[method_id(@__retain_semantics Other cloudKitContainerOptions)]
        pub unsafe fn cloudKitContainerOptions(
            &self,
        ) -> Option<Id<NSPersistentCloudKitContainerOptions>>;

        #[cfg(feature = "CoreData_NSPersistentCloudKitContainerOptions")]
        #[method(setCloudKitContainerOptions:)]
        pub unsafe fn setCloudKitContainerOptions(
            &self,
            cloud_kit_container_options: Option<&NSPersistentCloudKitContainerOptions>,
        );
    }
);
