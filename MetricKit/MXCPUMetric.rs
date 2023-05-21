//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::MetricKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MetricKit_MXCPUMetric")]
    #[cfg(not(any(target_os = "macos")))]
    pub struct MXCPUMetric;

    #[cfg(not(any(target_os = "macos")))]
    #[cfg(feature = "MetricKit_MXCPUMetric")]
    unsafe impl ClassType for MXCPUMetric {
        #[inherits(NSObject)]
        type Super = MXMetric;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MetricKit_MXCPUMetric")]
#[cfg(not(any(target_os = "macos")))]
unsafe impl NSCoding for MXCPUMetric {}

#[cfg(feature = "MetricKit_MXCPUMetric")]
#[cfg(not(any(target_os = "macos")))]
unsafe impl NSObjectProtocol for MXCPUMetric {}

#[cfg(feature = "MetricKit_MXCPUMetric")]
#[cfg(not(any(target_os = "macos")))]
unsafe impl NSSecureCoding for MXCPUMetric {}

#[cfg(not(any(target_os = "macos")))]
extern_methods!(
    #[cfg(feature = "MetricKit_MXCPUMetric")]
    #[cfg(not(any(target_os = "macos")))]
    unsafe impl MXCPUMetric {
        #[cfg(all(
            feature = "Foundation_NSMeasurement",
            feature = "Foundation_NSUnitDuration"
        ))]
        #[method_id(@__retain_semantics Other cumulativeCPUTime)]
        pub unsafe fn cumulativeCPUTime(&self) -> Id<NSMeasurement<NSUnitDuration>>;

        #[cfg(all(feature = "Foundation_NSMeasurement", feature = "Foundation_NSUnit"))]
        #[cfg(not(any(target_os = "macos")))]
        #[method_id(@__retain_semantics Other cumulativeCPUInstructions)]
        pub unsafe fn cumulativeCPUInstructions(&self) -> Id<NSMeasurement<NSUnit>>;
    }
);

#[cfg(not(any(target_os = "macos")))]
extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MetricKit_MXCPUMetric")]
    #[cfg(not(any(target_os = "macos")))]
    unsafe impl MXCPUMetric {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
