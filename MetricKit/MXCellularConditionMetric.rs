//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::MetricKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MetricKit_MXCellularConditionMetric")]
    #[cfg(not(any(target_os = "macos")))]
    pub struct MXCellularConditionMetric;

    #[cfg(feature = "MetricKit_MXCellularConditionMetric")]
    unsafe impl ClassType for MXCellularConditionMetric {
        #[inherits(NSObject)]
        type Super = MXMetric;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MetricKit_MXCellularConditionMetric")]
#[cfg(not(any(target_os = "macos")))]
unsafe impl NSCoding for MXCellularConditionMetric {}

#[cfg(feature = "MetricKit_MXCellularConditionMetric")]
#[cfg(not(any(target_os = "macos")))]
unsafe impl NSObjectProtocol for MXCellularConditionMetric {}

#[cfg(feature = "MetricKit_MXCellularConditionMetric")]
#[cfg(not(any(target_os = "macos")))]
unsafe impl NSSecureCoding for MXCellularConditionMetric {}

#[cfg(not(any(target_os = "macos")))]
extern_methods!(
    #[cfg(feature = "MetricKit_MXCellularConditionMetric")]
    unsafe impl MXCellularConditionMetric {
        #[cfg(all(
            feature = "MetricKit_MXHistogram",
            feature = "MetricKit_MXUnitSignalBars"
        ))]
        #[method_id(@__retain_semantics Other histogrammedCellularConditionTime)]
        pub unsafe fn histogrammedCellularConditionTime(&self)
            -> Id<MXHistogram<MXUnitSignalBars>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MetricKit_MXCellularConditionMetric")]
    unsafe impl MXCellularConditionMetric {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
