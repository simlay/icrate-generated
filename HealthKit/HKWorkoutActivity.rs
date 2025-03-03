//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::HealthKit::*;
use crate::UniformTypeIdentifiers::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "HealthKit_HKWorkoutActivity")]
    pub struct HKWorkoutActivity;

    #[cfg(feature = "HealthKit_HKWorkoutActivity")]
    unsafe impl ClassType for HKWorkoutActivity {
        type Super = NSObject;
    }
);

#[cfg(feature = "HealthKit_HKWorkoutActivity")]
unsafe impl NSObjectProtocol for HKWorkoutActivity {}

extern_methods!(
    #[cfg(feature = "HealthKit_HKWorkoutActivity")]
    unsafe impl HKWorkoutActivity {
        #[cfg(feature = "Foundation_NSUUID")]
        #[method_id(@__retain_semantics Other UUID)]
        pub unsafe fn UUID(&self) -> Id<NSUUID>;

        #[cfg(feature = "HealthKit_HKWorkoutConfiguration")]
        #[method_id(@__retain_semantics Other workoutConfiguration)]
        pub unsafe fn workoutConfiguration(&self) -> Id<HKWorkoutConfiguration>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other startDate)]
        pub unsafe fn startDate(&self) -> Id<NSDate>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other endDate)]
        pub unsafe fn endDate(&self) -> Option<Id<NSDate>>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other metadata)]
        pub unsafe fn metadata(&self) -> Option<Id<NSDictionary<NSString, Object>>>;

        #[method(duration)]
        pub unsafe fn duration(&self) -> NSTimeInterval;

        #[cfg(all(feature = "Foundation_NSArray", feature = "HealthKit_HKWorkoutEvent"))]
        #[method_id(@__retain_semantics Other workoutEvents)]
        pub unsafe fn workoutEvents(&self) -> Id<NSArray<HKWorkoutEvent>>;

        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "HealthKit_HKQuantityType",
            feature = "HealthKit_HKStatistics"
        ))]
        #[method_id(@__retain_semantics Other allStatistics)]
        pub unsafe fn allStatistics(&self) -> Id<NSDictionary<HKQuantityType, HKStatistics>>;

        #[cfg(all(
            feature = "HealthKit_HKQuantityType",
            feature = "HealthKit_HKStatistics"
        ))]
        #[method_id(@__retain_semantics Other statisticsForType:)]
        pub unsafe fn statisticsForType(
            &self,
            quantity_type: &HKQuantityType,
        ) -> Option<Id<HKStatistics>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[cfg(all(
            feature = "Foundation_NSDate",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString",
            feature = "HealthKit_HKWorkoutConfiguration"
        ))]
        #[method_id(@__retain_semantics Init initWithWorkoutConfiguration:startDate:endDate:metadata:)]
        pub unsafe fn initWithWorkoutConfiguration_startDate_endDate_metadata(
            this: Option<Allocated<Self>>,
            workout_configuration: &HKWorkoutConfiguration,
            start_date: &NSDate,
            end_date: Option<&NSDate>,
            metadata: Option<&NSDictionary<NSString, Object>>,
        ) -> Id<Self>;
    }
);

extern_static!(HKPredicateKeyPathWorkoutActivityType: &'static NSString);

extern_static!(HKPredicateKeyPathWorkoutActivityDuration: &'static NSString);

extern_static!(HKPredicateKeyPathWorkoutActivityStartDate: &'static NSString);

extern_static!(HKPredicateKeyPathWorkoutActivityEndDate: &'static NSString);

extern_static!(HKPredicateKeyPathWorkoutActivitySumQuantity: &'static NSString);

extern_static!(HKPredicateKeyPathWorkoutActivityMinimumQuantity: &'static NSString);

extern_static!(HKPredicateKeyPathWorkoutActivityMaximumQuantity: &'static NSString);

extern_static!(HKPredicateKeyPathWorkoutActivityAverageQuantity: &'static NSString);
