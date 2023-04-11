//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreLocation::*;
use crate::EventKit::*;
use crate::Foundation::*;
use crate::MapKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "EventKit_EKStructuredLocation")]
    pub struct EKStructuredLocation;

    #[cfg(feature = "EventKit_EKStructuredLocation")]
    unsafe impl ClassType for EKStructuredLocation {
        #[inherits(NSObject)]
        type Super = EKObject;
    }
);

#[cfg(feature = "EventKit_EKStructuredLocation")]
unsafe impl NSObjectProtocol for EKStructuredLocation {}

extern_methods!(
    #[cfg(feature = "EventKit_EKStructuredLocation")]
    unsafe impl EKStructuredLocation {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other locationWithTitle:)]
        pub unsafe fn locationWithTitle(title: &NSString) -> Id<Self>;

        #[cfg(feature = "MapKit_MKMapItem")]
        #[method_id(@__retain_semantics Other locationWithMapItem:)]
        pub unsafe fn locationWithMapItem(map_item: &MKMapItem) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: Option<&NSString>);

        #[cfg(feature = "CoreLocation_CLLocation")]
        #[method_id(@__retain_semantics Other geoLocation)]
        pub unsafe fn geoLocation(&self) -> Option<Id<CLLocation>>;

        #[cfg(feature = "CoreLocation_CLLocation")]
        #[method(setGeoLocation:)]
        pub unsafe fn setGeoLocation(&self, geo_location: Option<&CLLocation>);

        /**
          0 = use default, unit is meters
        */
        #[method(radius)]
        pub unsafe fn radius(&self) -> c_double;

        /**
          0 = use default, unit is meters
        */
        #[method(setRadius:)]
        pub unsafe fn setRadius(&self, radius: c_double);
    }
);
