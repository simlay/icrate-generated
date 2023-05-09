//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::MetricKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MetricKit_MXDiskIOMetric")]
    #[cfg(not(any(target_os = "macos")))]
    pub struct MXDiskIOMetric;

    #[cfg(feature = "MetricKit_MXDiskIOMetric")]
    unsafe impl ClassType for MXDiskIOMetric {
        #[inherits(NSObject)]
        type Super = MXMetric;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MetricKit_MXDiskIOMetric")]
#[cfg(not(any(target_os = "macos")))]
unsafe impl NSCoding for MXDiskIOMetric {}

#[cfg(feature = "MetricKit_MXDiskIOMetric")]
#[cfg(not(any(target_os = "macos")))]
unsafe impl NSObjectProtocol for MXDiskIOMetric {}

#[cfg(feature = "MetricKit_MXDiskIOMetric")]
#[cfg(not(any(target_os = "macos")))]
unsafe impl NSSecureCoding for MXDiskIOMetric {}

#[cfg(not(any(target_os = "macos")))]
extern_methods!(
    #[cfg(feature = "MetricKit_MXDiskIOMetric")]
    unsafe impl MXDiskIOMetric {
        #[cfg(all(
            feature = "Foundation_NSMeasurement",
            feature = "Foundation_NSUnitInformationStorage"
        ))]
        #[method_id(@__retain_semantics Other cumulativeLogicalWrites)]
        pub unsafe fn cumulativeLogicalWrites(&self)
            -> Id<NSMeasurement<NSUnitInformationStorage>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MetricKit_MXDiskIOMetric")]
    unsafe impl MXDiskIOMetric {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
