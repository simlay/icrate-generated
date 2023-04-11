//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::HealthKit::*;
use crate::UniformTypeIdentifiers::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "HealthKit_HKContactsLensSpecification")]
    /**
     @class         HKContactsLensSpecification
    @abstract      An object subclass representing lens specification for contacts
    */
    pub struct HKContactsLensSpecification;

    #[cfg(feature = "HealthKit_HKContactsLensSpecification")]
    unsafe impl ClassType for HKContactsLensSpecification {
        #[inherits(NSObject)]
        type Super = HKLensSpecification;
    }
);

#[cfg(feature = "HealthKit_HKContactsLensSpecification")]
/**
 @class         HKContactsLensSpecification
@abstract      An object subclass representing lens specification for contacts
*/
unsafe impl NSCoding for HKContactsLensSpecification {}

#[cfg(feature = "HealthKit_HKContactsLensSpecification")]
/**
 @class         HKContactsLensSpecification
@abstract      An object subclass representing lens specification for contacts
*/
unsafe impl NSObjectProtocol for HKContactsLensSpecification {}

#[cfg(feature = "HealthKit_HKContactsLensSpecification")]
/**
 @class         HKContactsLensSpecification
@abstract      An object subclass representing lens specification for contacts
*/
unsafe impl NSSecureCoding for HKContactsLensSpecification {}

extern_methods!(
    /**
     @class         HKContactsLensSpecification
    @abstract      An object subclass representing lens specification for contacts
    */
    #[cfg(feature = "HealthKit_HKContactsLensSpecification")]
    unsafe impl HKContactsLensSpecification {
        #[cfg(feature = "HealthKit_HKQuantity")]
        /**
         @property      baseCurve
        @abstract      The curvature of the back surface of the lens (measured in mm)
        */
        #[method_id(@__retain_semantics Other baseCurve)]
        pub unsafe fn baseCurve(&self) -> Option<Id<HKQuantity>>;

        #[cfg(feature = "HealthKit_HKQuantity")]
        /**
         @property      diameter
        @abstract      The width of the lens from edge to edge (measured in mm)
        */
        #[method_id(@__retain_semantics Other diameter)]
        pub unsafe fn diameter(&self) -> Option<Id<HKQuantity>>;

        #[cfg(feature = "HealthKit_HKQuantity")]
        #[method_id(@__retain_semantics Init initWithSphere:cylinder:axis:addPower:baseCurve:diameter:)]
        pub unsafe fn initWithSphere_cylinder_axis_addPower_baseCurve_diameter(
            this: Option<Allocated<Self>>,
            sphere: &HKQuantity,
            cylinder: Option<&HKQuantity>,
            axis: Option<&HKQuantity>,
            add_power: Option<&HKQuantity>,
            base_curve: Option<&HKQuantity>,
            diameter: Option<&HKQuantity>,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
