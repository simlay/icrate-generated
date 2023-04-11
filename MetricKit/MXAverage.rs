//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::MetricKit::*;

__inner_extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MetricKit_MXAverage")]
    /**
     @class         MXAverage
    @abstract      A class representing metric data that is averaged.
    */
    pub struct MXAverage<UnitType: Message = Object, UnitTypeOwnership: Ownership = Shared> {
        _inner0: PhantomData<*mut (UnitType, UnitTypeOwnership)>,
        notunwindsafe: PhantomData<&'static mut ()>,
    }

    #[cfg(feature = "MetricKit_MXAverage")]
    unsafe impl<UnitType: Message, UnitTypeOwnership: Ownership> ClassType
        for MXAverage<UnitType, UnitTypeOwnership>
    {
        type Super = NSObject;
    }
);

#[cfg(feature = "MetricKit_MXAverage")]
/**
 @class         MXAverage
@abstract      A class representing metric data that is averaged.
*/
unsafe impl<UnitType: Message, UnitTypeOwnership: Ownership> NSCoding
    for MXAverage<UnitType, UnitTypeOwnership>
{
}

#[cfg(feature = "MetricKit_MXAverage")]
/**
 @class         MXAverage
@abstract      A class representing metric data that is averaged.
*/
unsafe impl<UnitType: Message, UnitTypeOwnership: Ownership> NSObjectProtocol
    for MXAverage<UnitType, UnitTypeOwnership>
{
}

#[cfg(feature = "MetricKit_MXAverage")]
/**
 @class         MXAverage
@abstract      A class representing metric data that is averaged.
*/
unsafe impl<UnitType: Message, UnitTypeOwnership: Ownership> NSSecureCoding
    for MXAverage<UnitType, UnitTypeOwnership>
{
}

extern_methods!(
    /**
     @class         MXAverage
    @abstract      A class representing metric data that is averaged.
    */
    #[cfg(feature = "MetricKit_MXAverage")]
    unsafe impl<UnitType: Message, UnitTypeOwnership: Ownership>
        MXAverage<UnitType, UnitTypeOwnership>
    {
        #[cfg(feature = "Foundation_NSMeasurement")]
        /**
         @property      averageMeasurement
        @abstract      An NSMeasurement that contains the average measurement.
        */
        #[method_id(@__retain_semantics Other averageMeasurement)]
        pub unsafe fn averageMeasurement(&self) -> Id<NSMeasurement<UnitType>>;

        /**
         @property      sampleCount
        @abstract      An NSInteger representation of the number of samples in the distribution used to formulate the average.
        @discussion    This value is negative if an unknown number of samples was used to compute the average.
        */
        #[method(sampleCount)]
        pub unsafe fn sampleCount(&self) -> NSInteger;

        /**
         @property      standardDeviation
        @abstract      An double representation of the standard deviation of the distribution.
        @discussion    This value is negative an unknown number of samples was used to compute the standard deviation.
        */
        #[method(standardDeviation)]
        pub unsafe fn standardDeviation(&self) -> c_double;
    }
);
