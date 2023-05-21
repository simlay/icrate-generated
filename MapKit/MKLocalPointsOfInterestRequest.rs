//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Contacts::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::MapKit::*;

#[cfg(not(any(target_os = "watchos")))]
extern_static!(MKPointsOfInterestRequestMaxRadius: CLLocationDistance);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MapKit_MKLocalPointsOfInterestRequest")]
    #[cfg(not(any(target_os = "watchos")))]
    pub struct MKLocalPointsOfInterestRequest;

    #[cfg(not(any(target_os = "watchos")))]
    #[cfg(feature = "MapKit_MKLocalPointsOfInterestRequest")]
    unsafe impl ClassType for MKLocalPointsOfInterestRequest {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MapKit_MKLocalPointsOfInterestRequest")]
#[cfg(not(any(target_os = "watchos")))]
unsafe impl NSCopying for MKLocalPointsOfInterestRequest {}

#[cfg(feature = "MapKit_MKLocalPointsOfInterestRequest")]
#[cfg(not(any(target_os = "watchos")))]
unsafe impl NSObjectProtocol for MKLocalPointsOfInterestRequest {}

#[cfg(not(any(target_os = "watchos")))]
extern_methods!(
    #[cfg(feature = "MapKit_MKLocalPointsOfInterestRequest")]
    #[cfg(not(any(target_os = "watchos")))]
    unsafe impl MKLocalPointsOfInterestRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithCenterCoordinate:radius:)]
        pub unsafe fn initWithCenterCoordinate_radius(
            this: Option<Allocated<Self>>,
            coordinate: CLLocationCoordinate2D,
            radius: CLLocationDistance,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithCoordinateRegion:)]
        pub unsafe fn initWithCoordinateRegion(
            this: Option<Allocated<Self>>,
            region: MKCoordinateRegion,
        ) -> Id<Self>;

        #[method(coordinate)]
        pub unsafe fn coordinate(&self) -> CLLocationCoordinate2D;

        #[method(radius)]
        pub unsafe fn radius(&self) -> CLLocationDistance;

        #[method(region)]
        pub unsafe fn region(&self) -> MKCoordinateRegion;

        #[cfg(feature = "MapKit_MKPointOfInterestFilter")]
        #[method_id(@__retain_semantics Other pointOfInterestFilter)]
        pub unsafe fn pointOfInterestFilter(&self) -> Option<Id<MKPointOfInterestFilter>>;

        #[cfg(feature = "MapKit_MKPointOfInterestFilter")]
        #[method(setPointOfInterestFilter:)]
        pub unsafe fn setPointOfInterestFilter(
            &self,
            point_of_interest_filter: Option<&MKPointOfInterestFilter>,
        );
    }
);

#[cfg(not(any(target_os = "watchos")))]
extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MapKit_MKLocalPointsOfInterestRequest")]
    #[cfg(not(any(target_os = "watchos")))]
    unsafe impl MKLocalPointsOfInterestRequest {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
