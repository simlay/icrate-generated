//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Contacts::*;
use crate::CoreLocation::*;
use crate::Foundation::*;

pub type CLHeadingComponentValue = c_double;

extern_static!(kCLHeadingFilterNone: CLLocationDegrees);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreLocation_CLHeading")]
    #[cfg(not(any(target_os = "tvos")))]
    pub struct CLHeading;

    #[cfg(not(any(target_os = "tvos")))]
    #[cfg(feature = "CoreLocation_CLHeading")]
    unsafe impl ClassType for CLHeading {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CoreLocation_CLHeading")]
#[cfg(not(any(target_os = "tvos")))]
unsafe impl NSCoding for CLHeading {}

#[cfg(feature = "CoreLocation_CLHeading")]
#[cfg(not(any(target_os = "tvos")))]
unsafe impl NSCopying for CLHeading {}

#[cfg(feature = "CoreLocation_CLHeading")]
#[cfg(not(any(target_os = "tvos")))]
unsafe impl NSObjectProtocol for CLHeading {}

#[cfg(feature = "CoreLocation_CLHeading")]
#[cfg(not(any(target_os = "tvos")))]
unsafe impl NSSecureCoding for CLHeading {}

#[cfg(not(any(target_os = "tvos")))]
extern_methods!(
    #[cfg(feature = "CoreLocation_CLHeading")]
    #[cfg(not(any(target_os = "tvos")))]
    unsafe impl CLHeading {
        #[method(magneticHeading)]
        pub unsafe fn magneticHeading(&self) -> CLLocationDirection;

        #[method(trueHeading)]
        pub unsafe fn trueHeading(&self) -> CLLocationDirection;

        #[method(headingAccuracy)]
        pub unsafe fn headingAccuracy(&self) -> CLLocationDirection;

        #[method(x)]
        pub unsafe fn x(&self) -> CLHeadingComponentValue;

        #[method(y)]
        pub unsafe fn y(&self) -> CLHeadingComponentValue;

        #[method(z)]
        pub unsafe fn z(&self) -> CLHeadingComponentValue;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other timestamp)]
        pub unsafe fn timestamp(&self) -> Id<NSDate>;
    }
);

#[cfg(not(any(target_os = "tvos")))]
extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CoreLocation_CLHeading")]
    #[cfg(not(any(target_os = "tvos")))]
    unsafe impl CLHeading {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
