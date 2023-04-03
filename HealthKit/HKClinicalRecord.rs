//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::HealthKit::*;
use crate::UniformTypeIdentifiers::*;

extern_static!(HKPredicateKeyPathClinicalRecordFHIRResourceIdentifier: &'static NSString);

extern_static!(HKPredicateKeyPathClinicalRecordFHIRResourceType: &'static NSString);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "HealthKit_HKClinicalRecord")]
    #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
    pub struct HKClinicalRecord;

    #[cfg(feature = "HealthKit_HKClinicalRecord")]
    unsafe impl ClassType for HKClinicalRecord {
        #[inherits(HKObject, NSObject)]
        type Super = HKSample;
    }
);

#[cfg(feature = "HealthKit_HKClinicalRecord")]
unsafe impl NSCoding for HKClinicalRecord {}

#[cfg(feature = "HealthKit_HKClinicalRecord")]
unsafe impl NSObjectProtocol for HKClinicalRecord {}

#[cfg(feature = "HealthKit_HKClinicalRecord")]
unsafe impl NSSecureCoding for HKClinicalRecord {}

extern_methods!(
    #[cfg(feature = "HealthKit_HKClinicalRecord")]
    unsafe impl HKClinicalRecord {
        #[cfg(feature = "HealthKit_HKClinicalType")]
        #[method_id(@__retain_semantics Other clinicalType)]
        pub unsafe fn clinicalType(&self) -> Id<HKClinicalType>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other displayName)]
        pub unsafe fn displayName(&self) -> Id<NSString>;

        #[cfg(feature = "HealthKit_HKFHIRResource")]
        #[method_id(@__retain_semantics Other FHIRResource)]
        pub unsafe fn FHIRResource(&self) -> Option<Id<HKFHIRResource>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
