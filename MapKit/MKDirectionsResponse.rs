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
    #[cfg(feature = "MapKit_MKDirectionsResponse")]
    #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
    pub struct MKDirectionsResponse;

    #[cfg(feature = "MapKit_MKDirectionsResponse")]
    unsafe impl ClassType for MKDirectionsResponse {
        type Super = NSObject;
    }
);

#[cfg(feature = "MapKit_MKDirectionsResponse")]
unsafe impl NSObjectProtocol for MKDirectionsResponse {}

extern_methods!(
    #[cfg(feature = "MapKit_MKDirectionsResponse")]
    unsafe impl MKDirectionsResponse {
        #[cfg(feature = "MapKit_MKMapItem")]
        #[method_id(@__retain_semantics Other source)]
        pub unsafe fn source(&self) -> Id<MKMapItem>;

        #[cfg(feature = "MapKit_MKMapItem")]
        #[method_id(@__retain_semantics Other destination)]
        pub unsafe fn destination(&self) -> Id<MKMapItem>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "MapKit_MKRoute"))]
        #[method_id(@__retain_semantics Other routes)]
        pub unsafe fn routes(&self) -> Id<NSArray<MKRoute>>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MapKit_MKRoute")]
    #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
    pub struct MKRoute;

    #[cfg(feature = "MapKit_MKRoute")]
    unsafe impl ClassType for MKRoute {
        type Super = NSObject;
    }
);

#[cfg(feature = "MapKit_MKRoute")]
unsafe impl NSObjectProtocol for MKRoute {}

extern_methods!(
    #[cfg(feature = "MapKit_MKRoute")]
    unsafe impl MKRoute {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Id<NSString>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other advisoryNotices)]
        pub unsafe fn advisoryNotices(&self) -> Id<NSArray<NSString>>;

        #[method(distance)]
        pub unsafe fn distance(&self) -> CLLocationDistance;

        #[method(expectedTravelTime)]
        pub unsafe fn expectedTravelTime(&self) -> NSTimeInterval;

        #[method(transportType)]
        pub unsafe fn transportType(&self) -> MKDirectionsTransportType;

        #[cfg(feature = "MapKit_MKPolyline")]
        #[method_id(@__retain_semantics Other polyline)]
        pub unsafe fn polyline(&self) -> Id<MKPolyline>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "MapKit_MKRouteStep"))]
        #[method_id(@__retain_semantics Other steps)]
        pub unsafe fn steps(&self) -> Id<NSArray<MKRouteStep>>;

        #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
        #[method(hasTolls)]
        pub unsafe fn hasTolls(&self) -> bool;

        #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
        #[method(hasHighways)]
        pub unsafe fn hasHighways(&self) -> bool;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MapKit_MKRouteStep")]
    #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
    pub struct MKRouteStep;

    #[cfg(feature = "MapKit_MKRouteStep")]
    unsafe impl ClassType for MKRouteStep {
        type Super = NSObject;
    }
);

#[cfg(feature = "MapKit_MKRouteStep")]
unsafe impl NSObjectProtocol for MKRouteStep {}

extern_methods!(
    #[cfg(feature = "MapKit_MKRouteStep")]
    unsafe impl MKRouteStep {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other instructions)]
        pub unsafe fn instructions(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other notice)]
        pub unsafe fn notice(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "MapKit_MKPolyline")]
        #[method_id(@__retain_semantics Other polyline)]
        pub unsafe fn polyline(&self) -> Id<MKPolyline>;

        #[method(distance)]
        pub unsafe fn distance(&self) -> CLLocationDistance;

        #[method(transportType)]
        pub unsafe fn transportType(&self) -> MKDirectionsTransportType;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MapKit_MKETAResponse")]
    #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
    pub struct MKETAResponse;

    #[cfg(feature = "MapKit_MKETAResponse")]
    unsafe impl ClassType for MKETAResponse {
        type Super = NSObject;
    }
);

#[cfg(feature = "MapKit_MKETAResponse")]
unsafe impl NSObjectProtocol for MKETAResponse {}

extern_methods!(
    #[cfg(feature = "MapKit_MKETAResponse")]
    unsafe impl MKETAResponse {
        #[cfg(feature = "MapKit_MKMapItem")]
        #[method_id(@__retain_semantics Other source)]
        pub unsafe fn source(&self) -> Id<MKMapItem>;

        #[cfg(feature = "MapKit_MKMapItem")]
        #[method_id(@__retain_semantics Other destination)]
        pub unsafe fn destination(&self) -> Id<MKMapItem>;

        #[method(expectedTravelTime)]
        pub unsafe fn expectedTravelTime(&self) -> NSTimeInterval;

        #[method(distance)]
        pub unsafe fn distance(&self) -> CLLocationDistance;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other expectedArrivalDate)]
        pub unsafe fn expectedArrivalDate(&self) -> Id<NSDate>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other expectedDepartureDate)]
        pub unsafe fn expectedDepartureDate(&self) -> Id<NSDate>;

        #[method(transportType)]
        pub unsafe fn transportType(&self) -> MKDirectionsTransportType;
    }
);
