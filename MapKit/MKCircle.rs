//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Contacts::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::MapKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MapKit_MKCircle")]
    #[cfg(not(any(target_os = "watchos")))]
    pub struct MKCircle;

    #[cfg(not(any(target_os = "watchos")))]
    #[cfg(feature = "MapKit_MKCircle")]
    unsafe impl ClassType for MKCircle {
        #[inherits(NSObject)]
        type Super = MKShape;
    }
);

#[cfg(feature = "MapKit_MKCircle")]
#[cfg(not(any(target_os = "watchos")))]
unsafe impl MKAnnotation for MKCircle {}

#[cfg(feature = "MapKit_MKCircle")]
#[cfg(not(any(target_os = "watchos")))]
unsafe impl MKOverlay for MKCircle {}

#[cfg(feature = "MapKit_MKCircle")]
#[cfg(not(any(target_os = "watchos")))]
unsafe impl NSObjectProtocol for MKCircle {}

extern_methods!(
    #[cfg(feature = "MapKit_MKCircle")]
    #[cfg(not(any(target_os = "watchos")))]
    unsafe impl MKCircle {
        #[cfg(not(any(target_os = "watchos")))]
        #[method_id(@__retain_semantics Other circleWithCenterCoordinate:radius:)]
        pub unsafe fn circleWithCenterCoordinate_radius(
            coord: CLLocationCoordinate2D,
            radius: CLLocationDistance,
        ) -> Id<Self>;

        #[cfg(not(any(target_os = "watchos")))]
        #[method_id(@__retain_semantics Other circleWithMapRect:)]
        pub unsafe fn circleWithMapRect(map_rect: MKMapRect) -> Id<Self>;

        #[cfg(not(any(target_os = "watchos")))]
        #[method(coordinate)]
        pub unsafe fn coordinate(&self) -> CLLocationCoordinate2D;

        #[cfg(not(any(target_os = "watchos")))]
        #[method(radius)]
        pub unsafe fn radius(&self) -> CLLocationDistance;

        #[cfg(not(any(target_os = "watchos")))]
        #[method(boundingMapRect)]
        pub unsafe fn boundingMapRect(&self) -> MKMapRect;
    }
);
