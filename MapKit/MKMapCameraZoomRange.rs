//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Contacts::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::MapKit::*;

#[cfg(not(any(target_os = "watchos")))]
extern_static!(MKMapCameraZoomDefault: CLLocationDistance);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MapKit_MKMapCameraZoomRange")]
    #[cfg(not(any(target_os = "watchos")))]
    pub struct MKMapCameraZoomRange;

    #[cfg(feature = "MapKit_MKMapCameraZoomRange")]
    unsafe impl ClassType for MKMapCameraZoomRange {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MapKit_MKMapCameraZoomRange")]
#[cfg(not(any(target_os = "watchos")))]
unsafe impl NSCoding for MKMapCameraZoomRange {}

#[cfg(feature = "MapKit_MKMapCameraZoomRange")]
#[cfg(not(any(target_os = "watchos")))]
unsafe impl NSCopying for MKMapCameraZoomRange {}

#[cfg(feature = "MapKit_MKMapCameraZoomRange")]
#[cfg(not(any(target_os = "watchos")))]
unsafe impl NSObjectProtocol for MKMapCameraZoomRange {}

#[cfg(feature = "MapKit_MKMapCameraZoomRange")]
#[cfg(not(any(target_os = "watchos")))]
unsafe impl NSSecureCoding for MKMapCameraZoomRange {}

#[cfg(not(any(target_os = "watchos")))]
extern_methods!(
    #[cfg(feature = "MapKit_MKMapCameraZoomRange")]
    unsafe impl MKMapCameraZoomRange {
        #[method_id(@__retain_semantics Init initWithMinCenterCoordinateDistance:maxCenterCoordinateDistance:)]
        pub unsafe fn initWithMinCenterCoordinateDistance_maxCenterCoordinateDistance(
            this: Option<Allocated<Self>>,
            min_distance: CLLocationDistance,
            max_distance: CLLocationDistance,
        ) -> Option<Id<Self>>;

        #[method_id(@__retain_semantics Init initWithMinCenterCoordinateDistance:)]
        pub unsafe fn initWithMinCenterCoordinateDistance(
            this: Option<Allocated<Self>>,
            min_distance: CLLocationDistance,
        ) -> Option<Id<Self>>;

        #[method_id(@__retain_semantics Init initWithMaxCenterCoordinateDistance:)]
        pub unsafe fn initWithMaxCenterCoordinateDistance(
            this: Option<Allocated<Self>>,
            max_distance: CLLocationDistance,
        ) -> Option<Id<Self>>;

        #[method(minCenterCoordinateDistance)]
        pub unsafe fn minCenterCoordinateDistance(&self) -> CLLocationDistance;

        #[method(maxCenterCoordinateDistance)]
        pub unsafe fn maxCenterCoordinateDistance(&self) -> CLLocationDistance;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MapKit_MKMapCameraZoomRange")]
    unsafe impl MKMapCameraZoomRange {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
