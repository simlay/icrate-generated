//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CloudKit::*;
use crate::CoreLocation::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CloudKit_CKQuery")]
    pub struct CKQuery;

    #[cfg(feature = "CloudKit_CKQuery")]
    unsafe impl ClassType for CKQuery {
        type Super = NSObject;
    }
);

#[cfg(feature = "CloudKit_CKQuery")]
unsafe impl NSCoding for CKQuery {}

#[cfg(feature = "CloudKit_CKQuery")]
unsafe impl NSObjectProtocol for CKQuery {}

#[cfg(feature = "CloudKit_CKQuery")]
unsafe impl NSSecureCoding for CKQuery {}

extern_methods!(
    #[cfg(feature = "CloudKit_CKQuery")]
    unsafe impl CKQuery {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Option<Allocated<Self>>, a_decoder: &NSCoder)
            -> Id<Self>;

        #[cfg(feature = "Foundation_NSPredicate")]
        #[method_id(@__retain_semantics Init initWithRecordType:predicate:)]
        pub unsafe fn initWithRecordType_predicate(
            this: Option<Allocated<Self>>,
            record_type: &CKRecordType,
            predicate: &NSPredicate,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Other recordType)]
        pub unsafe fn recordType(&self) -> Id<CKRecordType>;

        #[cfg(feature = "Foundation_NSPredicate")]
        #[method_id(@__retain_semantics Other predicate)]
        pub unsafe fn predicate(&self) -> Id<NSPredicate>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSSortDescriptor"
        ))]
        #[method_id(@__retain_semantics Other sortDescriptors)]
        pub unsafe fn sortDescriptors(&self) -> Option<Id<NSArray<NSSortDescriptor>>>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSSortDescriptor"
        ))]
        #[method(setSortDescriptors:)]
        pub unsafe fn setSortDescriptors(
            &self,
            sort_descriptors: Option<&NSArray<NSSortDescriptor>>,
        );
    }
);
