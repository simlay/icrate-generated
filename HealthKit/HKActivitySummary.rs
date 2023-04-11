//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::HealthKit::*;
use crate::UniformTypeIdentifiers::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "HealthKit_HKActivitySummary")]
    /**
     @class         HKActivitySummary
    @abstract      An object that represents a summary of a user's activity for a given day.
    */
    pub struct HKActivitySummary;

    #[cfg(feature = "HealthKit_HKActivitySummary")]
    unsafe impl ClassType for HKActivitySummary {
        type Super = NSObject;
    }
);

#[cfg(feature = "HealthKit_HKActivitySummary")]
/**
 @class         HKActivitySummary
@abstract      An object that represents a summary of a user's activity for a given day.
*/
unsafe impl NSCoding for HKActivitySummary {}

#[cfg(feature = "HealthKit_HKActivitySummary")]
/**
 @class         HKActivitySummary
@abstract      An object that represents a summary of a user's activity for a given day.
*/
unsafe impl NSObjectProtocol for HKActivitySummary {}

#[cfg(feature = "HealthKit_HKActivitySummary")]
/**
 @class         HKActivitySummary
@abstract      An object that represents a summary of a user's activity for a given day.
*/
unsafe impl NSSecureCoding for HKActivitySummary {}

extern_methods!(
    /**
     @class         HKActivitySummary
    @abstract      An object that represents a summary of a user's activity for a given day.
    */
    #[cfg(feature = "HealthKit_HKActivitySummary")]
    unsafe impl HKActivitySummary {
        #[cfg(all(
            feature = "Foundation_NSCalendar",
            feature = "Foundation_NSDateComponents"
        ))]
        #[method_id(@__retain_semantics Other dateComponentsForCalendar:)]
        pub unsafe fn dateComponentsForCalendar(
            &self,
            calendar: &NSCalendar,
        ) -> Id<NSDateComponents>;

        /**
         @property      activityMoveMode
        @abstract      The move mode of this activity summary
        @discussion    The move mode of an activity summary determines if activeEnergyBurned or appleMoveTime are used for the move ring.
        */
        #[method(activityMoveMode)]
        pub unsafe fn activityMoveMode(&self) -> HKActivityMoveMode;

        /**
         @property      activityMoveMode
        @abstract      The move mode of this activity summary
        @discussion    The move mode of an activity summary determines if activeEnergyBurned or appleMoveTime are used for the move ring.
        */
        #[method(setActivityMoveMode:)]
        pub unsafe fn setActivityMoveMode(&self, activity_move_mode: HKActivityMoveMode);

        #[cfg(feature = "HealthKit_HKQuantity")]
        /**
         @property      activeEnergyBurned
        @abstract      The amount of active energy that the user burned.
        @discussion    This quantity is compatible with energy units.
        */
        #[method_id(@__retain_semantics Other activeEnergyBurned)]
        pub unsafe fn activeEnergyBurned(&self) -> Id<HKQuantity>;

        #[cfg(feature = "HealthKit_HKQuantity")]
        /**
         @property      activeEnergyBurned
        @abstract      The amount of active energy that the user burned.
        @discussion    This quantity is compatible with energy units.
        */
        #[method(setActiveEnergyBurned:)]
        pub unsafe fn setActiveEnergyBurned(&self, active_energy_burned: &HKQuantity);

        #[cfg(feature = "HealthKit_HKQuantity")]
        /**
         @property      appleMoveTime
        @abstract      The amount of move time that the user performed.
        @discussion    This quantity is compatible with time units. The measurement criteria of
        move time time is defined by Apple.
        */
        #[method_id(@__retain_semantics Other appleMoveTime)]
        pub unsafe fn appleMoveTime(&self) -> Id<HKQuantity>;

        #[cfg(feature = "HealthKit_HKQuantity")]
        /**
         @property      appleMoveTime
        @abstract      The amount of move time that the user performed.
        @discussion    This quantity is compatible with time units. The measurement criteria of
        move time time is defined by Apple.
        */
        #[method(setAppleMoveTime:)]
        pub unsafe fn setAppleMoveTime(&self, apple_move_time: &HKQuantity);

        #[cfg(feature = "HealthKit_HKQuantity")]
        /**
         @property      appleExerciseTime
        @abstract      The amount of exercise time that the user performed.
        @discussion    This quantity is compatible with time units. The measurement criteria of
        exercise time is defined by Apple.
        */
        #[method_id(@__retain_semantics Other appleExerciseTime)]
        pub unsafe fn appleExerciseTime(&self) -> Id<HKQuantity>;

        #[cfg(feature = "HealthKit_HKQuantity")]
        /**
         @property      appleExerciseTime
        @abstract      The amount of exercise time that the user performed.
        @discussion    This quantity is compatible with time units. The measurement criteria of
        exercise time is defined by Apple.
        */
        #[method(setAppleExerciseTime:)]
        pub unsafe fn setAppleExerciseTime(&self, apple_exercise_time: &HKQuantity);

        #[cfg(feature = "HealthKit_HKQuantity")]
        /**
         @property      appleStandHours
        @abstract      The number of stand hours that the user earned.
        @discussion     This quantity is compatible with the count unit. The measurement criteria of
        stand hours is defined by Apple.
        */
        #[method_id(@__retain_semantics Other appleStandHours)]
        pub unsafe fn appleStandHours(&self) -> Id<HKQuantity>;

        #[cfg(feature = "HealthKit_HKQuantity")]
        /**
         @property      appleStandHours
        @abstract      The number of stand hours that the user earned.
        @discussion     This quantity is compatible with the count unit. The measurement criteria of
        stand hours is defined by Apple.
        */
        #[method(setAppleStandHours:)]
        pub unsafe fn setAppleStandHours(&self, apple_stand_hours: &HKQuantity);

        #[cfg(feature = "HealthKit_HKQuantity")]
        /**
         @property      activeEnergyBurnedGoal
        @abstract      The user's active energy goal for the day.
        @discussion    This quantity is compatible with energy units.
        */
        #[method_id(@__retain_semantics Other activeEnergyBurnedGoal)]
        pub unsafe fn activeEnergyBurnedGoal(&self) -> Id<HKQuantity>;

        #[cfg(feature = "HealthKit_HKQuantity")]
        /**
         @property      activeEnergyBurnedGoal
        @abstract      The user's active energy goal for the day.
        @discussion    This quantity is compatible with energy units.
        */
        #[method(setActiveEnergyBurnedGoal:)]
        pub unsafe fn setActiveEnergyBurnedGoal(&self, active_energy_burned_goal: &HKQuantity);

        #[cfg(feature = "HealthKit_HKQuantity")]
        /**
         @property      appleMoveTimeGoal
        @abstract      The user's move time goal for the day.
        @discussion    This quantity is compatible with time units.
        */
        #[method_id(@__retain_semantics Other appleMoveTimeGoal)]
        pub unsafe fn appleMoveTimeGoal(&self) -> Id<HKQuantity>;

        #[cfg(feature = "HealthKit_HKQuantity")]
        /**
         @property      appleMoveTimeGoal
        @abstract      The user's move time goal for the day.
        @discussion    This quantity is compatible with time units.
        */
        #[method(setAppleMoveTimeGoal:)]
        pub unsafe fn setAppleMoveTimeGoal(&self, apple_move_time_goal: &HKQuantity);

        #[cfg(feature = "HealthKit_HKQuantity")]
        /**
         @property      appleExerciseTimeGoal
        @abstract      The user's exercise time goal for the day.
        @discussion    This quantity is compatible with time units.
        */
        #[deprecated]
        #[method_id(@__retain_semantics Other appleExerciseTimeGoal)]
        pub unsafe fn appleExerciseTimeGoal(&self) -> Id<HKQuantity>;

        #[cfg(feature = "HealthKit_HKQuantity")]
        /**
         @property      appleExerciseTimeGoal
        @abstract      The user's exercise time goal for the day.
        @discussion    This quantity is compatible with time units.
        */
        #[deprecated]
        #[method(setAppleExerciseTimeGoal:)]
        pub unsafe fn setAppleExerciseTimeGoal(&self, apple_exercise_time_goal: &HKQuantity);

        #[cfg(feature = "HealthKit_HKQuantity")]
        /**
         @property      exerciseTimeGoal
        @abstract      The user's exercise time goal for the day.
        @discussion    This quantity is compatible with time units.
        */
        #[method_id(@__retain_semantics Other exerciseTimeGoal)]
        pub unsafe fn exerciseTimeGoal(&self) -> Option<Id<HKQuantity>>;

        #[cfg(feature = "HealthKit_HKQuantity")]
        /**
         @property      exerciseTimeGoal
        @abstract      The user's exercise time goal for the day.
        @discussion    This quantity is compatible with time units.
        */
        #[method(setExerciseTimeGoal:)]
        pub unsafe fn setExerciseTimeGoal(&self, exercise_time_goal: Option<&HKQuantity>);

        #[cfg(feature = "HealthKit_HKQuantity")]
        /**
         @property      appleStandHoursGoal
        @abstract      The user's active stand hours goal for the day.
        @discussion    This quantity is compatible with the count unit.
        */
        #[deprecated]
        #[method_id(@__retain_semantics Other appleStandHoursGoal)]
        pub unsafe fn appleStandHoursGoal(&self) -> Id<HKQuantity>;

        #[cfg(feature = "HealthKit_HKQuantity")]
        /**
         @property      appleStandHoursGoal
        @abstract      The user's active stand hours goal for the day.
        @discussion    This quantity is compatible with the count unit.
        */
        #[deprecated]
        #[method(setAppleStandHoursGoal:)]
        pub unsafe fn setAppleStandHoursGoal(&self, apple_stand_hours_goal: &HKQuantity);

        #[cfg(feature = "HealthKit_HKQuantity")]
        /**
         @property      standHoursGoal
        @abstract      The user's active stand hours goal for the day.
        @discussion    This quantity is compatible with the count unit.
        */
        #[method_id(@__retain_semantics Other standHoursGoal)]
        pub unsafe fn standHoursGoal(&self) -> Option<Id<HKQuantity>>;

        #[cfg(feature = "HealthKit_HKQuantity")]
        /**
         @property      standHoursGoal
        @abstract      The user's active stand hours goal for the day.
        @discussion    This quantity is compatible with the count unit.
        */
        #[method(setStandHoursGoal:)]
        pub unsafe fn setStandHoursGoal(&self, stand_hours_goal: Option<&HKQuantity>);
    }
);

extern_static!(HKPredicateKeyPathDateComponents: &'static NSString);
