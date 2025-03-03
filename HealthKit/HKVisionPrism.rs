//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::HealthKit::*;
use crate::UniformTypeIdentifiers::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum HKPrismBase {
        HKPrismBaseNone = 0,
        HKPrismBaseUp = 1,
        HKPrismBaseDown = 2,
        HKPrismBaseIn = 3,
        HKPrismBaseOut = 4,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum HKVisionEye {
        HKVisionEyeLeft = 1,
        HKVisionEyeRight = 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "HealthKit_HKVisionPrism")]
    pub struct HKVisionPrism;

    #[cfg(feature = "HealthKit_HKVisionPrism")]
    unsafe impl ClassType for HKVisionPrism {
        type Super = NSObject;
    }
);

#[cfg(feature = "HealthKit_HKVisionPrism")]
unsafe impl NSCoding for HKVisionPrism {}

#[cfg(feature = "HealthKit_HKVisionPrism")]
unsafe impl NSObjectProtocol for HKVisionPrism {}

#[cfg(feature = "HealthKit_HKVisionPrism")]
unsafe impl NSSecureCoding for HKVisionPrism {}

extern_methods!(
    #[cfg(feature = "HealthKit_HKVisionPrism")]
    unsafe impl HKVisionPrism {
        #[cfg(feature = "HealthKit_HKQuantity")]
        #[method_id(@__retain_semantics Other amount)]
        pub unsafe fn amount(&self) -> Id<HKQuantity>;

        #[cfg(feature = "HealthKit_HKQuantity")]
        #[method_id(@__retain_semantics Other angle)]
        pub unsafe fn angle(&self) -> Id<HKQuantity>;

        #[cfg(feature = "HealthKit_HKQuantity")]
        #[method_id(@__retain_semantics Other verticalAmount)]
        pub unsafe fn verticalAmount(&self) -> Id<HKQuantity>;

        #[cfg(feature = "HealthKit_HKQuantity")]
        #[method_id(@__retain_semantics Other horizontalAmount)]
        pub unsafe fn horizontalAmount(&self) -> Id<HKQuantity>;

        #[method(verticalBase)]
        pub unsafe fn verticalBase(&self) -> HKPrismBase;

        #[method(horizontalBase)]
        pub unsafe fn horizontalBase(&self) -> HKPrismBase;

        #[method(eye)]
        pub unsafe fn eye(&self) -> HKVisionEye;

        #[cfg(feature = "HealthKit_HKQuantity")]
        #[method_id(@__retain_semantics Init initWithAmount:angle:eye:)]
        pub unsafe fn initWithAmount_angle_eye(
            this: Option<Allocated<Self>>,
            amount: &HKQuantity,
            angle: &HKQuantity,
            eye: HKVisionEye,
        ) -> Id<Self>;

        #[cfg(feature = "HealthKit_HKQuantity")]
        #[method_id(@__retain_semantics Init initWithVerticalAmount:verticalBase:horizontalAmount:horizontalBase:eye:)]
        pub unsafe fn initWithVerticalAmount_verticalBase_horizontalAmount_horizontalBase_eye(
            this: Option<Allocated<Self>>,
            vertical_amount: &HKQuantity,
            vertical_base: HKPrismBase,
            horizontal_amount: &HKQuantity,
            horizontal_base: HKPrismBase,
            eye: HKVisionEye,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
