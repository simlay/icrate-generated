//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::MetricKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MetricKit_MXDiskWriteExceptionDiagnostic")]
    pub struct MXDiskWriteExceptionDiagnostic;

    #[cfg(feature = "MetricKit_MXDiskWriteExceptionDiagnostic")]
    unsafe impl ClassType for MXDiskWriteExceptionDiagnostic {
        #[inherits(NSObject)]
        type Super = MXDiagnostic;
    }
);

#[cfg(feature = "MetricKit_MXDiskWriteExceptionDiagnostic")]
unsafe impl NSCoding for MXDiskWriteExceptionDiagnostic {}

#[cfg(feature = "MetricKit_MXDiskWriteExceptionDiagnostic")]
unsafe impl NSObjectProtocol for MXDiskWriteExceptionDiagnostic {}

#[cfg(feature = "MetricKit_MXDiskWriteExceptionDiagnostic")]
unsafe impl NSSecureCoding for MXDiskWriteExceptionDiagnostic {}

extern_methods!(
    #[cfg(feature = "MetricKit_MXDiskWriteExceptionDiagnostic")]
    unsafe impl MXDiskWriteExceptionDiagnostic {
        #[cfg(feature = "MetricKit_MXCallStackTree")]
        #[method_id(@__retain_semantics Other callStackTree)]
        pub unsafe fn callStackTree(&self) -> Id<MXCallStackTree>;

        #[cfg(all(
            feature = "Foundation_NSMeasurement",
            feature = "Foundation_NSUnitInformationStorage"
        ))]
        #[method_id(@__retain_semantics Other totalWritesCaused)]
        pub unsafe fn totalWritesCaused(&self) -> Id<NSMeasurement<NSUnitInformationStorage>>;
    }
);
