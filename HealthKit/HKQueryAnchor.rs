//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::HealthKit::*;
use crate::UniformTypeIdentifiers::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "HealthKit_HKQueryAnchor")]
    pub struct HKQueryAnchor;

    #[cfg(feature = "HealthKit_HKQueryAnchor")]
    unsafe impl ClassType for HKQueryAnchor {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "HealthKit_HKQueryAnchor")]
unsafe impl NSCoding for HKQueryAnchor {}

#[cfg(feature = "HealthKit_HKQueryAnchor")]
unsafe impl NSCopying for HKQueryAnchor {}

#[cfg(feature = "HealthKit_HKQueryAnchor")]
unsafe impl NSObjectProtocol for HKQueryAnchor {}

#[cfg(feature = "HealthKit_HKQueryAnchor")]
unsafe impl NSSecureCoding for HKQueryAnchor {}

extern_methods!(
    #[cfg(feature = "HealthKit_HKQueryAnchor")]
    unsafe impl HKQueryAnchor {
        #[method_id(@__retain_semantics Other anchorFromValue:)]
        pub unsafe fn anchorFromValue(value: NSUInteger) -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "HealthKit_HKQueryAnchor")]
    unsafe impl HKQueryAnchor {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
