//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::HealthKit::*;
use crate::UniformTypeIdentifiers::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "HealthKit_HKHeartbeatSeriesSample")]
    pub struct HKHeartbeatSeriesSample;

    #[cfg(feature = "HealthKit_HKHeartbeatSeriesSample")]
    unsafe impl ClassType for HKHeartbeatSeriesSample {
        #[inherits(HKSample, HKObject, NSObject)]
        type Super = HKSeriesSample;
    }
);

#[cfg(feature = "HealthKit_HKHeartbeatSeriesSample")]
unsafe impl NSCoding for HKHeartbeatSeriesSample {}

#[cfg(feature = "HealthKit_HKHeartbeatSeriesSample")]
unsafe impl NSObjectProtocol for HKHeartbeatSeriesSample {}

#[cfg(feature = "HealthKit_HKHeartbeatSeriesSample")]
unsafe impl NSSecureCoding for HKHeartbeatSeriesSample {}

extern_methods!(
    #[cfg(feature = "HealthKit_HKHeartbeatSeriesSample")]
    unsafe impl HKHeartbeatSeriesSample {}
);
