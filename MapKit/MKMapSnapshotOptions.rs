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
    #[cfg(feature = "MapKit_MKMapSnapshotOptions")]
    #[cfg(not(any(target_os = "watchos")))]
    pub struct MKMapSnapshotOptions;

    #[cfg(not(any(target_os = "watchos")))]
    #[cfg(feature = "MapKit_MKMapSnapshotOptions")]
    unsafe impl ClassType for MKMapSnapshotOptions {
        type Super = NSObject;
    }
);

#[cfg(feature = "MapKit_MKMapSnapshotOptions")]
#[cfg(not(any(target_os = "watchos")))]
unsafe impl NSObjectProtocol for MKMapSnapshotOptions {}

#[cfg(not(any(target_os = "watchos")))]
extern_methods!(
    #[cfg(feature = "MapKit_MKMapSnapshotOptions")]
    unsafe impl MKMapSnapshotOptions {
        #[cfg(feature = "MapKit_MKMapCamera")]
        #[method_id(@__retain_semantics Other camera)]
        pub unsafe fn camera(&self) -> Id<MKMapCamera>;

        #[cfg(feature = "MapKit_MKMapCamera")]
        #[method(setCamera:)]
        pub unsafe fn setCamera(&self, camera: &MKMapCamera);

        #[method(mapRect)]
        pub unsafe fn mapRect(&self) -> MKMapRect;

        #[method(setMapRect:)]
        pub unsafe fn setMapRect(&self, map_rect: MKMapRect);

        #[method(region)]
        pub unsafe fn region(&self) -> MKCoordinateRegion;

        #[method(setRegion:)]
        pub unsafe fn setRegion(&self, region: MKCoordinateRegion);

        #[method(mapType)]
        pub unsafe fn mapType(&self) -> MKMapType;

        #[method(setMapType:)]
        pub unsafe fn setMapType(&self, map_type: MKMapType);

        #[cfg(feature = "MapKit_MKPointOfInterestFilter")]
        #[cfg(not(any(target_os = "watchos")))]
        #[method_id(@__retain_semantics Other pointOfInterestFilter)]
        pub unsafe fn pointOfInterestFilter(&self) -> Option<Id<MKPointOfInterestFilter>>;

        #[cfg(feature = "MapKit_MKPointOfInterestFilter")]
        #[cfg(not(any(target_os = "watchos")))]
        #[method(setPointOfInterestFilter:)]
        pub unsafe fn setPointOfInterestFilter(
            &self,
            point_of_interest_filter: Option<&MKPointOfInterestFilter>,
        );

        #[deprecated = "Use pointOfInterestFilter"]
        #[cfg(not(any(target_os = "watchos")))]
        #[method(showsPointsOfInterest)]
        pub unsafe fn showsPointsOfInterest(&self) -> bool;

        #[deprecated = "Use pointOfInterestFilter"]
        #[cfg(not(any(target_os = "watchos")))]
        #[method(setShowsPointsOfInterest:)]
        pub unsafe fn setShowsPointsOfInterest(&self, shows_points_of_interest: bool);

        #[method(showsBuildings)]
        pub unsafe fn showsBuildings(&self) -> bool;

        #[method(setShowsBuildings:)]
        pub unsafe fn setShowsBuildings(&self, shows_buildings: bool);

        #[method(size)]
        pub unsafe fn size(&self) -> NSSize;

        #[method(setSize:)]
        pub unsafe fn setSize(&self, size: NSSize);

        #[cfg(feature = "AppKit_NSAppearance")]
        #[cfg(not(any(target_os = "ios", target_os = "tvos", target_os = "watchos")))]
        #[method_id(@__retain_semantics Other appearance)]
        pub unsafe fn appearance(&self) -> Option<Id<NSAppearance>>;

        #[cfg(feature = "AppKit_NSAppearance")]
        #[cfg(not(any(target_os = "ios", target_os = "tvos", target_os = "watchos")))]
        #[method(setAppearance:)]
        pub unsafe fn setAppearance(&self, appearance: Option<&NSAppearance>);
    }
);
