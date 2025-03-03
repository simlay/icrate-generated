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
    #[cfg(feature = "MapKit_MKLocalSearchResponse")]
    pub struct MKLocalSearchResponse;

    #[cfg(feature = "MapKit_MKLocalSearchResponse")]
    unsafe impl ClassType for MKLocalSearchResponse {
        type Super = NSObject;
    }
);

#[cfg(feature = "MapKit_MKLocalSearchResponse")]
unsafe impl NSObjectProtocol for MKLocalSearchResponse {}

extern_methods!(
    #[cfg(feature = "MapKit_MKLocalSearchResponse")]
    unsafe impl MKLocalSearchResponse {
        #[cfg(all(feature = "Foundation_NSArray", feature = "MapKit_MKMapItem"))]
        #[method_id(@__retain_semantics Other mapItems)]
        pub unsafe fn mapItems(&self) -> Id<NSArray<MKMapItem>>;

        #[method(boundingRegion)]
        pub unsafe fn boundingRegion(&self) -> MKCoordinateRegion;
    }
);
