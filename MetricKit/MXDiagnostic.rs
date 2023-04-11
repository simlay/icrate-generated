//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::MetricKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MetricKit_MXDiagnostic")]
    /**
     @class         MXDiagnostic
    @abstract      An abstract class that describes a diagnostic report vended by MetricKit.
    @discussion    All supported diagnostics are subclasses of MXDiagnostic.
    */
    pub struct MXDiagnostic;

    #[cfg(feature = "MetricKit_MXDiagnostic")]
    unsafe impl ClassType for MXDiagnostic {
        type Super = NSObject;
    }
);

#[cfg(feature = "MetricKit_MXDiagnostic")]
/**
 @class         MXDiagnostic
@abstract      An abstract class that describes a diagnostic report vended by MetricKit.
@discussion    All supported diagnostics are subclasses of MXDiagnostic.
*/
unsafe impl NSCoding for MXDiagnostic {}

#[cfg(feature = "MetricKit_MXDiagnostic")]
/**
 @class         MXDiagnostic
@abstract      An abstract class that describes a diagnostic report vended by MetricKit.
@discussion    All supported diagnostics are subclasses of MXDiagnostic.
*/
unsafe impl NSObjectProtocol for MXDiagnostic {}

#[cfg(feature = "MetricKit_MXDiagnostic")]
/**
 @class         MXDiagnostic
@abstract      An abstract class that describes a diagnostic report vended by MetricKit.
@discussion    All supported diagnostics are subclasses of MXDiagnostic.
*/
unsafe impl NSSecureCoding for MXDiagnostic {}

extern_methods!(
    /**
     @class         MXDiagnostic
    @abstract      An abstract class that describes a diagnostic report vended by MetricKit.
    @discussion    All supported diagnostics are subclasses of MXDiagnostic.
    */
    #[cfg(feature = "MetricKit_MXDiagnostic")]
    unsafe impl MXDiagnostic {
        #[cfg(feature = "MetricKit_MXMetaData")]
        #[method_id(@__retain_semantics Other metaData)]
        pub unsafe fn metaData(&self) -> Id<MXMetaData>;

        #[cfg(feature = "Foundation_NSString")]
        /**
         @property      applicationVersion
        @abstract      An NSString representation of the application version from which this diagnostic was generated.
        */
        #[method_id(@__retain_semantics Other applicationVersion)]
        pub unsafe fn applicationVersion(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other JSONRepresentation)]
        pub unsafe fn JSONRepresentation(&self) -> Id<NSData>;

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method_id(@__retain_semantics Other dictionaryRepresentation)]
        pub unsafe fn dictionaryRepresentation(&self) -> Id<NSDictionary>;
    }
);
