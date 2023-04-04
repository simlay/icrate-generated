//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::MetricKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MetricKit_MXAppResponsivenessMetric")]
    #[cfg(not(any(target_os = "macos")))]
    pub struct MXAppResponsivenessMetric;

    #[cfg(feature = "MetricKit_MXAppResponsivenessMetric")]
    unsafe impl ClassType for MXAppResponsivenessMetric {
        #[inherits(NSObject)]
        type Super = MXMetric;
    }
);

#[cfg(feature = "MetricKit_MXAppResponsivenessMetric")]
unsafe impl NSCoding for MXAppResponsivenessMetric {}

#[cfg(feature = "MetricKit_MXAppResponsivenessMetric")]
unsafe impl NSObjectProtocol for MXAppResponsivenessMetric {}

#[cfg(feature = "MetricKit_MXAppResponsivenessMetric")]
unsafe impl NSSecureCoding for MXAppResponsivenessMetric {}

extern_methods!(
    #[cfg(feature = "MetricKit_MXAppResponsivenessMetric")]
    unsafe impl MXAppResponsivenessMetric {
        #[cfg(all(
            feature = "Foundation_NSUnitDuration",
            feature = "MetricKit_MXHistogram"
        ))]
        #[method_id(@__retain_semantics Other histogrammedApplicationHangTime)]
        pub unsafe fn histogrammedApplicationHangTime(&self) -> Id<MXHistogram<NSUnitDuration>>;
    }
);
