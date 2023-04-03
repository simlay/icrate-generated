//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSObjectController")]
    #[cfg(not(any(target_os = "ios")))]
    pub struct NSObjectController;

    #[cfg(feature = "AppKit_NSObjectController")]
    unsafe impl ClassType for NSObjectController {
        #[inherits(NSObject)]
        type Super = NSController;
    }
);

#[cfg(feature = "AppKit_NSObjectController")]
unsafe impl NSCoding for NSObjectController {}

#[cfg(feature = "AppKit_NSObjectController")]
unsafe impl NSEditor for NSObjectController {}

#[cfg(feature = "AppKit_NSObjectController")]
unsafe impl NSEditorRegistration for NSObjectController {}

#[cfg(feature = "AppKit_NSObjectController")]
unsafe impl NSObjectProtocol for NSObjectController {}

extern_methods!(
    #[cfg(feature = "AppKit_NSObjectController")]
    unsafe impl NSObjectController {
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Init initWithContent:)]
        pub unsafe fn initWithContent(
            this: Option<Allocated<Self>>,
            content: Option<&Object>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self>>;

        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other content)]
        pub unsafe fn content(&self) -> Option<Id<Object>>;

        #[cfg(not(any(target_os = "ios")))]
        #[method(setContent:)]
        pub unsafe fn setContent(&self, content: Option<&Object>);

        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other selection)]
        pub unsafe fn selection(&self) -> Id<Object>;

        #[cfg(feature = "Foundation_NSArray")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other selectedObjects)]
        pub unsafe fn selectedObjects(&self) -> Id<NSArray>;

        #[cfg(not(any(target_os = "ios")))]
        #[method(automaticallyPreparesContent)]
        pub unsafe fn automaticallyPreparesContent(&self) -> bool;

        #[cfg(not(any(target_os = "ios")))]
        #[method(setAutomaticallyPreparesContent:)]
        pub unsafe fn setAutomaticallyPreparesContent(&self, automatically_prepares_content: bool);

        #[cfg(not(any(target_os = "ios")))]
        #[method(prepareContent)]
        pub unsafe fn prepareContent(&self);

        #[cfg(not(any(target_os = "ios")))]
        #[method(objectClass)]
        pub unsafe fn objectClass(&self) -> Option<&'static Class>;

        #[cfg(not(any(target_os = "ios")))]
        #[method(setObjectClass:)]
        pub unsafe fn setObjectClass(&self, object_class: Option<&Class>);

        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics New newObject)]
        pub unsafe fn newObject(&self) -> Id<Object>;

        #[cfg(not(any(target_os = "ios")))]
        #[method(addObject:)]
        pub unsafe fn addObject(&self, object: &Object);

        #[cfg(not(any(target_os = "ios")))]
        #[method(removeObject:)]
        pub unsafe fn removeObject(&self, object: &Object);

        #[cfg(not(any(target_os = "ios")))]
        #[method(isEditable)]
        pub unsafe fn isEditable(&self) -> bool;

        #[cfg(not(any(target_os = "ios")))]
        #[method(setEditable:)]
        pub unsafe fn setEditable(&self, editable: bool);

        #[cfg(not(any(target_os = "ios")))]
        #[method(add:)]
        pub unsafe fn add(&self, sender: Option<&Object>);

        #[cfg(not(any(target_os = "ios")))]
        #[method(canAdd)]
        pub unsafe fn canAdd(&self) -> bool;

        #[cfg(not(any(target_os = "ios")))]
        #[method(remove:)]
        pub unsafe fn remove(&self, sender: Option<&Object>);

        #[cfg(not(any(target_os = "ios")))]
        #[method(canRemove)]
        pub unsafe fn canRemove(&self) -> bool;

        #[cfg(not(any(target_os = "ios")))]
        #[method(validateUserInterfaceItem:)]
        pub unsafe fn validateUserInterfaceItem(
            &self,
            item: &ProtocolObject<dyn NSValidatedUserInterfaceItem>,
        ) -> bool;
    }
);

extern_methods!(
    /// NSManagedController
    #[cfg(feature = "AppKit_NSObjectController")]
    unsafe impl NSObjectController {
        #[cfg(feature = "CoreData_NSManagedObjectContext")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other managedObjectContext)]
        pub unsafe fn managedObjectContext(&self) -> Option<Id<NSManagedObjectContext>>;

        #[cfg(feature = "CoreData_NSManagedObjectContext")]
        #[cfg(not(any(target_os = "ios")))]
        #[method(setManagedObjectContext:)]
        pub unsafe fn setManagedObjectContext(
            &self,
            managed_object_context: Option<&NSManagedObjectContext>,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other entityName)]
        pub unsafe fn entityName(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[cfg(not(any(target_os = "ios")))]
        #[method(setEntityName:)]
        pub unsafe fn setEntityName(&self, entity_name: Option<&NSString>);

        #[cfg(feature = "Foundation_NSPredicate")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other fetchPredicate)]
        pub unsafe fn fetchPredicate(&self) -> Option<Id<NSPredicate>>;

        #[cfg(feature = "Foundation_NSPredicate")]
        #[cfg(not(any(target_os = "ios")))]
        #[method(setFetchPredicate:)]
        pub unsafe fn setFetchPredicate(&self, fetch_predicate: Option<&NSPredicate>);

        #[cfg(all(feature = "CoreData_NSFetchRequest", feature = "Foundation_NSError"))]
        #[cfg(not(any(target_os = "ios")))]
        #[method(fetchWithRequest:merge:error:_)]
        pub unsafe fn fetchWithRequest_merge_error(
            &self,
            fetch_request: Option<&NSFetchRequest>,
            merge: bool,
        ) -> Result<(), Id<NSError>>;

        #[cfg(not(any(target_os = "ios")))]
        #[method(fetch:)]
        pub unsafe fn fetch(&self, sender: Option<&Object>);

        #[cfg(not(any(target_os = "ios")))]
        #[method(usesLazyFetching)]
        pub unsafe fn usesLazyFetching(&self) -> bool;

        #[cfg(not(any(target_os = "ios")))]
        #[method(setUsesLazyFetching:)]
        pub unsafe fn setUsesLazyFetching(&self, uses_lazy_fetching: bool);

        #[cfg(feature = "CoreData_NSFetchRequest")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other defaultFetchRequest)]
        pub unsafe fn defaultFetchRequest(&self) -> Id<NSFetchRequest>;
    }
);
