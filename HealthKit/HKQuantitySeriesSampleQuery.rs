//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::HealthKit::*;
use crate::UniformTypeIdentifiers::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "HealthKit_HKQuantitySeriesSampleQuery")]
    pub struct HKQuantitySeriesSampleQuery;

    #[cfg(feature = "HealthKit_HKQuantitySeriesSampleQuery")]
    unsafe impl ClassType for HKQuantitySeriesSampleQuery {
        #[inherits(NSObject)]
        type Super = HKQuery;
    }
);

#[cfg(feature = "HealthKit_HKQuantitySeriesSampleQuery")]
unsafe impl NSObjectProtocol for HKQuantitySeriesSampleQuery {}

extern_methods!(
    #[cfg(feature = "HealthKit_HKQuantitySeriesSampleQuery")]
    unsafe impl HKQuantitySeriesSampleQuery {
        /**
         @property      includeSample
        @abstract      Include owning HKQuantitySample in quantityHandler handler.
        @discussion    Default value is NO.
        If includeSample is set then the quantitySample parameter of quantityHandler will
        be non-nil anytime the quantity parameter is non-nil.
        Specifying this option has a performance cost.
        This property may not be modified once the query has been executed.
        */
        #[method(includeSample)]
        pub unsafe fn includeSample(&self) -> bool;

        /**
         @property      includeSample
        @abstract      Include owning HKQuantitySample in quantityHandler handler.
        @discussion    Default value is NO.
        If includeSample is set then the quantitySample parameter of quantityHandler will
        be non-nil anytime the quantity parameter is non-nil.
        Specifying this option has a performance cost.
        This property may not be modified once the query has been executed.
        */
        #[method(setIncludeSample:)]
        pub unsafe fn setIncludeSample(&self, include_sample: bool);

        /**
         @property      orderByQuantitySampleStartDate
        @abstract      Order enumerated results first by quantitySample.startDate,
        then by the quantity's dateInterval.startDate.
        @discussion    Default value is NO.
        All quantities owned by a given quantitySample will be
        enumerated before any quantities owned by any other quantity sample,
        and the quantity samples will be enumerated in their startDate order.
        Note that individual quantities may not be returned in their
        dateInterval.startDate order if more than one quantitySample overlap in time.
        This property may not be modified once the query has been executed.
        */
        #[method(orderByQuantitySampleStartDate)]
        pub unsafe fn orderByQuantitySampleStartDate(&self) -> bool;

        /**
         @property      orderByQuantitySampleStartDate
        @abstract      Order enumerated results first by quantitySample.startDate,
        then by the quantity's dateInterval.startDate.
        @discussion    Default value is NO.
        All quantities owned by a given quantitySample will be
        enumerated before any quantities owned by any other quantity sample,
        and the quantity samples will be enumerated in their startDate order.
        Note that individual quantities may not be returned in their
        dateInterval.startDate order if more than one quantitySample overlap in time.
        This property may not be modified once the query has been executed.
        */
        #[method(setOrderByQuantitySampleStartDate:)]
        pub unsafe fn setOrderByQuantitySampleStartDate(
            &self,
            order_by_quantity_sample_start_date: bool,
        );

        #[cfg(all(
            feature = "Foundation_NSDateInterval",
            feature = "Foundation_NSError",
            feature = "Foundation_NSPredicate",
            feature = "HealthKit_HKQuantity",
            feature = "HealthKit_HKQuantitySample",
            feature = "HealthKit_HKQuantityType"
        ))]
        #[method_id(@__retain_semantics Init initWithQuantityType:predicate:quantityHandler:)]
        pub unsafe fn initWithQuantityType_predicate_quantityHandler(
            this: Option<Allocated<Self>>,
            quantity_type: &HKQuantityType,
            predicate: Option<&NSPredicate>,
            quantity_handler: &Block<
                (
                    NonNull<HKQuantitySeriesSampleQuery>,
                    *mut HKQuantity,
                    *mut NSDateInterval,
                    *mut HKQuantitySample,
                    Bool,
                    *mut NSError,
                ),
                (),
            >,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "Foundation_NSDate",
            feature = "Foundation_NSError",
            feature = "HealthKit_HKQuantity",
            feature = "HealthKit_HKQuantitySample"
        ))]
        #[deprecated]
        #[method_id(@__retain_semantics Init initWithSample:quantityHandler:)]
        pub unsafe fn initWithSample_quantityHandler(
            this: Option<Allocated<Self>>,
            quantity_sample: &HKQuantitySample,
            quantity_handler: &Block<
                (
                    NonNull<HKQuantitySeriesSampleQuery>,
                    *mut HKQuantity,
                    *mut NSDate,
                    Bool,
                    *mut NSError,
                ),
                (),
            >,
        ) -> Id<Self>;
    }
);
