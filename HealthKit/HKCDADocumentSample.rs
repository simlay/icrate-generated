//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::HealthKit::*;
use crate::UniformTypeIdentifiers::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "HealthKit_HKCDADocumentSample")]
    pub struct HKCDADocumentSample;

    #[cfg(feature = "HealthKit_HKCDADocumentSample")]
    unsafe impl ClassType for HKCDADocumentSample {
        #[inherits(HKSample, HKObject, NSObject)]
        type Super = HKDocumentSample;
    }
);

#[cfg(feature = "HealthKit_HKCDADocumentSample")]
unsafe impl NSCoding for HKCDADocumentSample {}

#[cfg(feature = "HealthKit_HKCDADocumentSample")]
unsafe impl NSObjectProtocol for HKCDADocumentSample {}

#[cfg(feature = "HealthKit_HKCDADocumentSample")]
unsafe impl NSSecureCoding for HKCDADocumentSample {}

extern_methods!(
    #[cfg(feature = "HealthKit_HKCDADocumentSample")]
    unsafe impl HKCDADocumentSample {
        #[cfg(feature = "HealthKit_HKCDADocument")]
        #[method_id(@__retain_semantics Other document)]
        pub unsafe fn document(&self) -> Option<Id<HKCDADocument>>;

        #[cfg(all(
            feature = "Foundation_NSData",
            feature = "Foundation_NSDate",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other CDADocumentSampleWithData:startDate:endDate:metadata:validationError:_)]
        pub unsafe fn CDADocumentSampleWithData_startDate_endDate_metadata_validationError(
            document_data: &NSData,
            start_date: &NSDate,
            end_date: &NSDate,
            metadata: Option<&NSDictionary<NSString, Object>>,
        ) -> Result<Id<Self>, Id<NSError>>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "HealthKit_HKCDADocument")]
    pub struct HKCDADocument;

    #[cfg(feature = "HealthKit_HKCDADocument")]
    unsafe impl ClassType for HKCDADocument {
        type Super = NSObject;
    }
);

#[cfg(feature = "HealthKit_HKCDADocument")]
unsafe impl NSObjectProtocol for HKCDADocument {}

extern_methods!(
    #[cfg(feature = "HealthKit_HKCDADocument")]
    unsafe impl HKCDADocument {
        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other documentData)]
        pub unsafe fn documentData(&self) -> Option<Id<NSData>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other patientName)]
        pub unsafe fn patientName(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other authorName)]
        pub unsafe fn authorName(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other custodianName)]
        pub unsafe fn custodianName(&self) -> Id<NSString>;
    }
);

extern_static!(HKPredicateKeyPathCDATitle: &'static NSString);

extern_static!(HKPredicateKeyPathCDAPatientName: &'static NSString);

extern_static!(HKPredicateKeyPathCDAAuthorName: &'static NSString);

extern_static!(HKPredicateKeyPathCDACustodianName: &'static NSString);

extern_static!(HKDetailedCDAValidationErrorKey: &'static NSString);
