//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::MetricKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MetricKit_MXDiskWriteExceptionDiagnostic")]
    /**
     @class         MXDiskWriteExceptionDiagnostic
    @abstract      An MXDiagnostic subclass that encapsulates disk write exception reports.
    @discussion    Disk write exceptions occur when your application writes data excessively to disk.
    */
    pub struct MXDiskWriteExceptionDiagnostic;

    #[cfg(feature = "MetricKit_MXDiskWriteExceptionDiagnostic")]
    unsafe impl ClassType for MXDiskWriteExceptionDiagnostic {
        #[inherits(NSObject)]
        type Super = MXDiagnostic;
    }
);

#[cfg(feature = "MetricKit_MXDiskWriteExceptionDiagnostic")]
/**
 @class         MXDiskWriteExceptionDiagnostic
@abstract      An MXDiagnostic subclass that encapsulates disk write exception reports.
@discussion    Disk write exceptions occur when your application writes data excessively to disk.
*/
unsafe impl NSCoding for MXDiskWriteExceptionDiagnostic {}

#[cfg(feature = "MetricKit_MXDiskWriteExceptionDiagnostic")]
/**
 @class         MXDiskWriteExceptionDiagnostic
@abstract      An MXDiagnostic subclass that encapsulates disk write exception reports.
@discussion    Disk write exceptions occur when your application writes data excessively to disk.
*/
unsafe impl NSObjectProtocol for MXDiskWriteExceptionDiagnostic {}

#[cfg(feature = "MetricKit_MXDiskWriteExceptionDiagnostic")]
/**
 @class         MXDiskWriteExceptionDiagnostic
@abstract      An MXDiagnostic subclass that encapsulates disk write exception reports.
@discussion    Disk write exceptions occur when your application writes data excessively to disk.
*/
unsafe impl NSSecureCoding for MXDiskWriteExceptionDiagnostic {}

extern_methods!(
    /**
     @class         MXDiskWriteExceptionDiagnostic
    @abstract      An MXDiagnostic subclass that encapsulates disk write exception reports.
    @discussion    Disk write exceptions occur when your application writes data excessively to disk.
    */
    #[cfg(feature = "MetricKit_MXDiskWriteExceptionDiagnostic")]
    unsafe impl MXDiskWriteExceptionDiagnostic {
        #[cfg(feature = "MetricKit_MXCallStackTree")]
        /**
         @property      callStackTree
        @abstract      The application call stack tree associated with the excessive disk writes.
        */
        #[method_id(@__retain_semantics Other callStackTree)]
        pub unsafe fn callStackTree(&self) -> Id<MXCallStackTree>;

        #[cfg(all(
            feature = "Foundation_NSMeasurement",
            feature = "Foundation_NSUnitInformationStorage"
        ))]
        /**
         @property      totalWritesCaused
        @abstract      Total disk writes caused in the scope of this disk write exception.
        @discussion    Dimensioned as NSUnitInformationStorage.
        */
        #[method_id(@__retain_semantics Other totalWritesCaused)]
        pub unsafe fn totalWritesCaused(&self) -> Id<NSMeasurement<NSUnitInformationStorage>>;
    }
);
