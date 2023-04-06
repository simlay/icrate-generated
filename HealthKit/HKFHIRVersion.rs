//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::HealthKit::*;
use crate::UniformTypeIdentifiers::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "HealthKit_HKFHIRVersion")]
    #[cfg(not(any(target_os = "watchos")))]
    pub struct HKFHIRVersion;

    #[cfg(not(any(target_os = "watchos")))]
    #[cfg(feature = "HealthKit_HKFHIRVersion")]
    unsafe impl ClassType for HKFHIRVersion {
        type Super = NSObject;
    }
);

#[cfg(feature = "HealthKit_HKFHIRVersion")]
#[cfg(not(any(target_os = "watchos")))]
unsafe impl NSCoding for HKFHIRVersion {}

#[cfg(feature = "HealthKit_HKFHIRVersion")]
#[cfg(not(any(target_os = "watchos")))]
unsafe impl NSObjectProtocol for HKFHIRVersion {}

#[cfg(feature = "HealthKit_HKFHIRVersion")]
#[cfg(not(any(target_os = "watchos")))]
unsafe impl NSSecureCoding for HKFHIRVersion {}

extern_methods!(
    #[cfg(feature = "HealthKit_HKFHIRVersion")]
    #[cfg(not(any(target_os = "watchos")))]
    unsafe impl HKFHIRVersion {
        #[cfg(not(any(target_os = "watchos")))]
        #[method(majorVersion)]
        pub unsafe fn majorVersion(&self) -> NSInteger;

        #[cfg(not(any(target_os = "watchos")))]
        #[method(minorVersion)]
        pub unsafe fn minorVersion(&self) -> NSInteger;

        #[cfg(not(any(target_os = "watchos")))]
        #[method(patchVersion)]
        pub unsafe fn patchVersion(&self) -> NSInteger;

        #[cfg(not(any(target_os = "watchos")))]
        #[method_id(@__retain_semantics Other FHIRRelease)]
        pub unsafe fn FHIRRelease(&self) -> Id<HKFHIRRelease>;

        #[cfg(not(any(target_os = "watchos")))]
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other stringRepresentation)]
        pub unsafe fn stringRepresentation(&self) -> Id<NSString>;

        #[cfg(not(any(target_os = "watchos")))]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[cfg(not(any(target_os = "watchos")))]
        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other versionFromVersionString:error:_)]
        pub unsafe fn versionFromVersionString_error(
            version_string: &NSString,
        ) -> Result<Id<Self>, Id<NSError>>;

        #[cfg(not(any(target_os = "watchos")))]
        #[method_id(@__retain_semantics Other primaryDSTU2Version)]
        pub unsafe fn primaryDSTU2Version() -> Id<Self>;

        #[cfg(not(any(target_os = "watchos")))]
        #[method_id(@__retain_semantics Other primaryR4Version)]
        pub unsafe fn primaryR4Version() -> Id<Self>;
    }
);
