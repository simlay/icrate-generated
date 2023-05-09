//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreData_NSBatchDeleteRequest")]
    pub struct NSBatchDeleteRequest;

    #[cfg(feature = "CoreData_NSBatchDeleteRequest")]
    unsafe impl ClassType for NSBatchDeleteRequest {
        #[inherits(NSObject)]
        type Super = NSPersistentStoreRequest;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CoreData_NSBatchDeleteRequest")]
unsafe impl NSCopying for NSBatchDeleteRequest {}

#[cfg(feature = "CoreData_NSBatchDeleteRequest")]
unsafe impl NSObjectProtocol for NSBatchDeleteRequest {}

extern_methods!(
    #[cfg(feature = "CoreData_NSBatchDeleteRequest")]
    unsafe impl NSBatchDeleteRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[cfg(feature = "CoreData_NSFetchRequest")]
        #[method_id(@__retain_semantics Init initWithFetchRequest:)]
        pub unsafe fn initWithFetchRequest(
            this: Option<Allocated<Self>>,
            fetch: &NSFetchRequest,
        ) -> Id<Self>;

        #[cfg(all(feature = "CoreData_NSManagedObjectID", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Init initWithObjectIDs:)]
        pub unsafe fn initWithObjectIDs(
            this: Option<Allocated<Self>>,
            objects: &NSArray<NSManagedObjectID>,
        ) -> Id<Self>;

        #[method(resultType)]
        pub unsafe fn resultType(&self) -> NSBatchDeleteRequestResultType;

        #[method(setResultType:)]
        pub unsafe fn setResultType(&self, result_type: NSBatchDeleteRequestResultType);

        #[cfg(feature = "CoreData_NSFetchRequest")]
        #[method_id(@__retain_semantics Other fetchRequest)]
        pub unsafe fn fetchRequest(&self) -> Id<NSFetchRequest>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CoreData_NSBatchDeleteRequest")]
    unsafe impl NSBatchDeleteRequest {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
