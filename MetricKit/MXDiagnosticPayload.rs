//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::MetricKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MetricKit_MXDiagnosticPayload")]
    /**
     @class         MXDiagnosticPayload
    @abstract      A wrapper class which contains a diagnostic payload and associated properties of that payload.
    @discussion    MXDiagnosticPayload encapsulates currently supported diagnostics that can be vended by MetricKit. Arrays of MXDiangostic subclasses on MXDiagnosticPayload are nullable. If an array of MXDiagnostic subclasses is nil, it indicates that the diagnostics are not available for this payload.
    @discussion    MXDiagnosticPayload exposes a convenience function, JSONRepresentation, to convert the contents of the payload to a human readable JSON. This should be used in conjunction with other APIs that accept NSData.
    @discussion    An MXDiagnosticPayload contains diagnostics that cover a 24 hour period of application usage. The properties timeStampBegin and timeStampEnd should be used to determine which time range the payload covers.
    @discussion    It is possible for an MXDiagnosticPayload to cover regions of time where an application was updated, and thus each MXDiagnostic subclass will contain its own application version string. This is in contrast to MXMetricPayload, where only the latest application version string is included as metadata of the payload. Each MXDiagnostic subclass application version string should be inspected prior to processing.
    */
    pub struct MXDiagnosticPayload;

    #[cfg(feature = "MetricKit_MXDiagnosticPayload")]
    unsafe impl ClassType for MXDiagnosticPayload {
        type Super = NSObject;
    }
);

#[cfg(feature = "MetricKit_MXDiagnosticPayload")]
/**
 @class         MXDiagnosticPayload
@abstract      A wrapper class which contains a diagnostic payload and associated properties of that payload.
@discussion    MXDiagnosticPayload encapsulates currently supported diagnostics that can be vended by MetricKit. Arrays of MXDiangostic subclasses on MXDiagnosticPayload are nullable. If an array of MXDiagnostic subclasses is nil, it indicates that the diagnostics are not available for this payload.
@discussion    MXDiagnosticPayload exposes a convenience function, JSONRepresentation, to convert the contents of the payload to a human readable JSON. This should be used in conjunction with other APIs that accept NSData.
@discussion    An MXDiagnosticPayload contains diagnostics that cover a 24 hour period of application usage. The properties timeStampBegin and timeStampEnd should be used to determine which time range the payload covers.
@discussion    It is possible for an MXDiagnosticPayload to cover regions of time where an application was updated, and thus each MXDiagnostic subclass will contain its own application version string. This is in contrast to MXMetricPayload, where only the latest application version string is included as metadata of the payload. Each MXDiagnostic subclass application version string should be inspected prior to processing.
*/
unsafe impl NSCoding for MXDiagnosticPayload {}

#[cfg(feature = "MetricKit_MXDiagnosticPayload")]
/**
 @class         MXDiagnosticPayload
@abstract      A wrapper class which contains a diagnostic payload and associated properties of that payload.
@discussion    MXDiagnosticPayload encapsulates currently supported diagnostics that can be vended by MetricKit. Arrays of MXDiangostic subclasses on MXDiagnosticPayload are nullable. If an array of MXDiagnostic subclasses is nil, it indicates that the diagnostics are not available for this payload.
@discussion    MXDiagnosticPayload exposes a convenience function, JSONRepresentation, to convert the contents of the payload to a human readable JSON. This should be used in conjunction with other APIs that accept NSData.
@discussion    An MXDiagnosticPayload contains diagnostics that cover a 24 hour period of application usage. The properties timeStampBegin and timeStampEnd should be used to determine which time range the payload covers.
@discussion    It is possible for an MXDiagnosticPayload to cover regions of time where an application was updated, and thus each MXDiagnostic subclass will contain its own application version string. This is in contrast to MXMetricPayload, where only the latest application version string is included as metadata of the payload. Each MXDiagnostic subclass application version string should be inspected prior to processing.
*/
unsafe impl NSObjectProtocol for MXDiagnosticPayload {}

#[cfg(feature = "MetricKit_MXDiagnosticPayload")]
/**
 @class         MXDiagnosticPayload
@abstract      A wrapper class which contains a diagnostic payload and associated properties of that payload.
@discussion    MXDiagnosticPayload encapsulates currently supported diagnostics that can be vended by MetricKit. Arrays of MXDiangostic subclasses on MXDiagnosticPayload are nullable. If an array of MXDiagnostic subclasses is nil, it indicates that the diagnostics are not available for this payload.
@discussion    MXDiagnosticPayload exposes a convenience function, JSONRepresentation, to convert the contents of the payload to a human readable JSON. This should be used in conjunction with other APIs that accept NSData.
@discussion    An MXDiagnosticPayload contains diagnostics that cover a 24 hour period of application usage. The properties timeStampBegin and timeStampEnd should be used to determine which time range the payload covers.
@discussion    It is possible for an MXDiagnosticPayload to cover regions of time where an application was updated, and thus each MXDiagnostic subclass will contain its own application version string. This is in contrast to MXMetricPayload, where only the latest application version string is included as metadata of the payload. Each MXDiagnostic subclass application version string should be inspected prior to processing.
*/
unsafe impl NSSecureCoding for MXDiagnosticPayload {}

extern_methods!(
    /**
     @class         MXDiagnosticPayload
    @abstract      A wrapper class which contains a diagnostic payload and associated properties of that payload.
    @discussion    MXDiagnosticPayload encapsulates currently supported diagnostics that can be vended by MetricKit. Arrays of MXDiangostic subclasses on MXDiagnosticPayload are nullable. If an array of MXDiagnostic subclasses is nil, it indicates that the diagnostics are not available for this payload.
    @discussion    MXDiagnosticPayload exposes a convenience function, JSONRepresentation, to convert the contents of the payload to a human readable JSON. This should be used in conjunction with other APIs that accept NSData.
    @discussion    An MXDiagnosticPayload contains diagnostics that cover a 24 hour period of application usage. The properties timeStampBegin and timeStampEnd should be used to determine which time range the payload covers.
    @discussion    It is possible for an MXDiagnosticPayload to cover regions of time where an application was updated, and thus each MXDiagnostic subclass will contain its own application version string. This is in contrast to MXMetricPayload, where only the latest application version string is included as metadata of the payload. Each MXDiagnostic subclass application version string should be inspected prior to processing.
    */
    #[cfg(feature = "MetricKit_MXDiagnosticPayload")]
    unsafe impl MXDiagnosticPayload {
        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "MetricKit_MXCPUExceptionDiagnostic"
        ))]
        /**
         @property      cpuExceptionDiagnostics
        @abstract      An array containing CPU exception diagnostics for this application.
        */
        #[method_id(@__retain_semantics Other cpuExceptionDiagnostics)]
        pub unsafe fn cpuExceptionDiagnostics(
            &self,
        ) -> Option<Id<NSArray<MXCPUExceptionDiagnostic>>>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "MetricKit_MXDiskWriteExceptionDiagnostic"
        ))]
        /**
         @property      diskWriteExceptionDiagnostics
        @abstract      An array containing disk write exception diagnostics for this application.
        */
        #[method_id(@__retain_semantics Other diskWriteExceptionDiagnostics)]
        pub unsafe fn diskWriteExceptionDiagnostics(
            &self,
        ) -> Option<Id<NSArray<MXDiskWriteExceptionDiagnostic>>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "MetricKit_MXHangDiagnostic"))]
        /**
         @property      hangDiagnostics
        @abstract      An array containing hang diagnostics for this application.
        */
        #[method_id(@__retain_semantics Other hangDiagnostics)]
        pub unsafe fn hangDiagnostics(&self) -> Option<Id<NSArray<MXHangDiagnostic>>>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "MetricKit_MXAppLaunchDiagnostic"
        ))]
        /**
         @property      appLaunchDiagnostics
        @abstract      An array containing app launch diagnostics for this application.
        */
        #[method_id(@__retain_semantics Other appLaunchDiagnostics)]
        pub unsafe fn appLaunchDiagnostics(&self) -> Option<Id<NSArray<MXAppLaunchDiagnostic>>>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "MetricKit_MXCrashDiagnostic"
        ))]
        /**
         @property      crashDiagnostics
        @abstract      An array containing crash diagnostics for this application.
        */
        #[method_id(@__retain_semantics Other crashDiagnostics)]
        pub unsafe fn crashDiagnostics(&self) -> Option<Id<NSArray<MXCrashDiagnostic>>>;

        #[cfg(feature = "Foundation_NSDate")]
        /**
         @property      timeStampBegin
        @abstract      An NSDate object that indicates the start time for which the payload was generated.
        */
        #[method_id(@__retain_semantics Other timeStampBegin)]
        pub unsafe fn timeStampBegin(&self) -> Id<NSDate>;

        #[cfg(feature = "Foundation_NSDate")]
        /**
         @property      timeStampEnd
        @abstract      An NSDate object that indicates the end time for which the payload was generated.
        */
        #[method_id(@__retain_semantics Other timeStampEnd)]
        pub unsafe fn timeStampEnd(&self) -> Id<NSDate>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other JSONRepresentation)]
        pub unsafe fn JSONRepresentation(&self) -> Id<NSData>;

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method_id(@__retain_semantics Other dictionaryRepresentation)]
        pub unsafe fn dictionaryRepresentation(&self) -> Id<NSDictionary>;
    }
);
