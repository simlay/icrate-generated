//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreData_NSBatchDeleteRequest")]
    /**
       Used to request that Core Data do a batch deletion of objects without faulting in
      objects to be deleted.
      May not be supported by all store types.
      WARNING:
      It is up to the developer creating the request to ensure that changes made by the request to
      the underlying store do not violate any validation rules specified in the model beyond basic
      delete rules (for example, minimum relationship counts).
    */
    pub struct NSBatchDeleteRequest;

    #[cfg(feature = "CoreData_NSBatchDeleteRequest")]
    unsafe impl ClassType for NSBatchDeleteRequest {
        #[inherits(NSObject)]
        type Super = NSPersistentStoreRequest;
    }
);

#[cfg(feature = "CoreData_NSBatchDeleteRequest")]
/**
   Used to request that Core Data do a batch deletion of objects without faulting in
  objects to be deleted.
  May not be supported by all store types.
  WARNING:
  It is up to the developer creating the request to ensure that changes made by the request to
  the underlying store do not violate any validation rules specified in the model beyond basic
  delete rules (for example, minimum relationship counts).
*/
unsafe impl NSObjectProtocol for NSBatchDeleteRequest {}

extern_methods!(
    /**
       Used to request that Core Data do a batch deletion of objects without faulting in
      objects to be deleted.
      May not be supported by all store types.
      WARNING:
      It is up to the developer creating the request to ensure that changes made by the request to
      the underlying store do not violate any validation rules specified in the model beyond basic
      delete rules (for example, minimum relationship counts).
    */
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

        /**
          The type of result that should be returned from this request. Defaults to NSBatchDeleteResultTypeStatusOnly
        */
        #[method(resultType)]
        pub unsafe fn resultType(&self) -> NSBatchDeleteRequestResultType;

        /**
          The type of result that should be returned from this request. Defaults to NSBatchDeleteResultTypeStatusOnly
        */
        #[method(setResultType:)]
        pub unsafe fn setResultType(&self, result_type: NSBatchDeleteRequestResultType);

        #[cfg(feature = "CoreData_NSFetchRequest")]
        #[method_id(@__retain_semantics Other fetchRequest)]
        pub unsafe fn fetchRequest(&self) -> Id<NSFetchRequest>;
    }
);
