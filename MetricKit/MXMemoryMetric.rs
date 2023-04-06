//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::MetricKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MetricKit_MXMemoryMetric")]
    #[cfg(not(any(target_os = "macos")))]
    pub struct MXMemoryMetric;

    #[cfg(not(any(target_os = "macos")))]
    #[cfg(feature = "MetricKit_MXMemoryMetric")]
    unsafe impl ClassType for MXMemoryMetric {
        #[inherits(NSObject)]
        type Super = MXMetric;
    }
);

#[cfg(feature = "MetricKit_MXMemoryMetric")]
#[cfg(not(any(target_os = "macos")))]
unsafe impl NSCoding for MXMemoryMetric {}

#[cfg(feature = "MetricKit_MXMemoryMetric")]
#[cfg(not(any(target_os = "macos")))]
unsafe impl NSObjectProtocol for MXMemoryMetric {}

#[cfg(feature = "MetricKit_MXMemoryMetric")]
#[cfg(not(any(target_os = "macos")))]
unsafe impl NSSecureCoding for MXMemoryMetric {}

extern_methods!(
    #[cfg(feature = "MetricKit_MXMemoryMetric")]
    #[cfg(not(any(target_os = "macos")))]
    unsafe impl MXMemoryMetric {
        #[cfg(not(any(target_os = "macos")))]
        #[cfg(all(
            feature = "Foundation_NSMeasurement",
            feature = "Foundation_NSUnitInformationStorage"
        ))]
        #[method_id(@__retain_semantics Other peakMemoryUsage)]
        pub unsafe fn peakMemoryUsage(&self) -> Id<NSMeasurement<NSUnitInformationStorage>>;

        #[cfg(not(any(target_os = "macos")))]
        #[cfg(all(
            feature = "Foundation_NSUnitInformationStorage",
            feature = "MetricKit_MXAverage"
        ))]
        #[method_id(@__retain_semantics Other averageSuspendedMemory)]
        pub unsafe fn averageSuspendedMemory(&self) -> Id<MXAverage<NSUnitInformationStorage>>;
    }
);
