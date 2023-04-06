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

extern_methods!(
    #[cfg(feature = "MetricKit_MXCPUMetric")]
    #[cfg(not(any(target_os = "macos")))]
    unsafe impl MXCPUMetric {
        #[cfg(not(any(target_os = "macos")))]
        #[cfg(all(
            feature = "Foundation_NSMeasurement",
            feature = "Foundation_NSUnitDuration"
        ))]
        #[method_id(@__retain_semantics Other cumulativeCPUTime)]
        pub unsafe fn cumulativeCPUTime(&self) -> Id<NSMeasurement<NSUnitDuration>>;

        #[cfg(not(any(target_os = "macos")))]
        #[cfg(all(feature = "Foundation_NSMeasurement", feature = "Foundation_NSUnit"))]
        #[cfg(not(any(target_os = "macos")))]
        #[method_id(@__retain_semantics Other cumulativeCPUInstructions)]
        pub unsafe fn cumulativeCPUInstructions(&self) -> Id<NSMeasurement<NSUnit>>;
    }
);
