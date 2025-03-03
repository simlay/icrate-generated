//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::HealthKit::*;
use crate::UniformTypeIdentifiers::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "HealthKit_HKVerifiableClinicalRecordQuery")]
    pub struct HKVerifiableClinicalRecordQuery;

    #[cfg(feature = "HealthKit_HKVerifiableClinicalRecordQuery")]
    unsafe impl ClassType for HKVerifiableClinicalRecordQuery {
        #[inherits(NSObject)]
        type Super = HKQuery;
    }
);

#[cfg(feature = "HealthKit_HKVerifiableClinicalRecordQuery")]
unsafe impl NSObjectProtocol for HKVerifiableClinicalRecordQuery {}

extern_methods!(
    #[cfg(feature = "HealthKit_HKVerifiableClinicalRecordQuery")]
    unsafe impl HKVerifiableClinicalRecordQuery {
        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other recordTypes)]
        pub unsafe fn recordTypes(&self) -> Id<NSArray<NSString>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other sourceTypes)]
        pub unsafe fn sourceTypes(&self) -> Id<NSArray<HKVerifiableClinicalRecordSourceType>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "Foundation_NSPredicate",
            feature = "Foundation_NSString",
            feature = "HealthKit_HKVerifiableClinicalRecord"
        ))]
        #[method_id(@__retain_semantics Init initWithRecordTypes:predicate:resultsHandler:)]
        pub unsafe fn initWithRecordTypes_predicate_resultsHandler(
            this: Option<Allocated<Self>>,
            record_types: &NSArray<NSString>,
            predicate: Option<&NSPredicate>,
            results_handler: &Block<
                (
                    NonNull<HKVerifiableClinicalRecordQuery>,
                    *mut NSArray<HKVerifiableClinicalRecord>,
                    *mut NSError,
                ),
                (),
            >,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "Foundation_NSPredicate",
            feature = "Foundation_NSString",
            feature = "HealthKit_HKVerifiableClinicalRecord"
        ))]
        #[method_id(@__retain_semantics Init initWithRecordTypes:sourceTypes:predicate:resultsHandler:)]
        pub unsafe fn initWithRecordTypes_sourceTypes_predicate_resultsHandler(
            this: Option<Allocated<Self>>,
            record_types: &NSArray<NSString>,
            source_types: &NSArray<HKVerifiableClinicalRecordSourceType>,
            predicate: Option<&NSPredicate>,
            results_handler: &Block<
                (
                    NonNull<HKVerifiableClinicalRecordQuery>,
                    *mut NSArray<HKVerifiableClinicalRecord>,
                    *mut NSError,
                ),
                (),
            >,
        ) -> Id<Self>;
    }
);
