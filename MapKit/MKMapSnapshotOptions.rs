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

extern_methods!(
    #[cfg(feature = "MapKit_MKMapSnapshotOptions")]
    #[cfg(not(any(target_os = "watchos")))]
    unsafe impl MKMapSnapshotOptions {
        #[cfg(not(any(target_os = "watchos")))]
        #[cfg(feature = "MapKit_MKMapCamera")]
        #[method_id(@__retain_semantics Other camera)]
        pub unsafe fn camera(&self) -> Id<MKMapCamera>;

        #[cfg(not(any(target_os = "watchos")))]
        #[cfg(feature = "MapKit_MKMapCamera")]
        #[method(setCamera:)]
        pub unsafe fn setCamera(&self, camera: &MKMapCamera);

        #[cfg(not(any(target_os = "watchos")))]
        #[method(mapRect)]
        pub unsafe fn mapRect(&self) -> MKMapRect;

        #[cfg(not(any(target_os = "watchos")))]
        #[method(setMapRect:)]
        pub unsafe fn setMapRect(&self, map_rect: MKMapRect);

        #[cfg(not(any(target_os = "watchos")))]
        #[method(region)]
        pub unsafe fn region(&self) -> MKCoordinateRegion;

        #[cfg(not(any(target_os = "watchos")))]
        #[method(setRegion:)]
        pub unsafe fn setRegion(&self, region: MKCoordinateRegion);

        #[cfg(not(any(target_os = "watchos")))]
        #[method(mapType)]
        pub unsafe fn mapType(&self) -> MKMapType;

        #[cfg(not(any(target_os = "watchos")))]
        #[method(setMapType:)]
        pub unsafe fn setMapType(&self, map_type: MKMapType);

        #[cfg(not(any(target_os = "watchos")))]
        #[cfg(feature = "MapKit_MKPointOfInterestFilter")]
        #[cfg(not(any(target_os = "watchos")))]
        #[method_id(@__retain_semantics Other pointOfInterestFilter)]
        pub unsafe fn pointOfInterestFilter(&self) -> Option<Id<MKPointOfInterestFilter>>;

        #[cfg(not(any(target_os = "watchos")))]
        #[cfg(feature = "MapKit_MKPointOfInterestFilter")]
        #[cfg(not(any(target_os = "watchos")))]
        #[method(setPointOfInterestFilter:)]
        pub unsafe fn setPointOfInterestFilter(
            &self,
            point_of_interest_filter: Option<&MKPointOfInterestFilter>,
        );

        #[cfg(not(any(target_os = "watchos")))]
        #[deprecated = "Use pointOfInterestFilter"]
        #[cfg(not(any(target_os = "watchos")))]
        #[method(showsPointsOfInterest)]
        pub unsafe fn showsPointsOfInterest(&self) -> bool;

        #[cfg(not(any(target_os = "watchos")))]
        #[deprecated = "Use pointOfInterestFilter"]
        #[cfg(not(any(target_os = "watchos")))]
        #[method(setShowsPointsOfInterest:)]
        pub unsafe fn setShowsPointsOfInterest(&self, shows_points_of_interest: bool);

        #[cfg(not(any(target_os = "watchos")))]
        #[method(showsBuildings)]
        pub unsafe fn showsBuildings(&self) -> bool;

        #[cfg(not(any(target_os = "watchos")))]
        #[method(setShowsBuildings:)]
        pub unsafe fn setShowsBuildings(&self, shows_buildings: bool);

        #[cfg(not(any(target_os = "watchos")))]
        #[method(size)]
        pub unsafe fn size(&self) -> NSSize;

        #[cfg(not(any(target_os = "watchos")))]
        #[method(setSize:)]
        pub unsafe fn setSize(&self, size: NSSize);

        #[cfg(not(any(target_os = "watchos")))]
        #[cfg(feature = "AppKit_NSAppearance")]
        #[cfg(not(any(target_os = "ios", target_os = "tvos", target_os = "watchos")))]
        #[method_id(@__retain_semantics Other appearance)]
        pub unsafe fn appearance(&self) -> Option<Id<NSAppearance>>;

        #[cfg(not(any(target_os = "watchos")))]
        #[cfg(feature = "AppKit_NSAppearance")]
        #[cfg(not(any(target_os = "ios", target_os = "tvos", target_os = "watchos")))]
        #[method(setAppearance:)]
        pub unsafe fn setAppearance(&self, appearance: Option<&NSAppearance>);
    }
);
