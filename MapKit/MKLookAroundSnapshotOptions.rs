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
    #[cfg(feature = "MapKit_MKLookAroundSnapshotOptions")]
    pub struct MKLookAroundSnapshotOptions;

    #[cfg(feature = "MapKit_MKLookAroundSnapshotOptions")]
    unsafe impl ClassType for MKLookAroundSnapshotOptions {
        type Super = NSObject;
    }
);

#[cfg(feature = "MapKit_MKLookAroundSnapshotOptions")]
unsafe impl NSObjectProtocol for MKLookAroundSnapshotOptions {}

extern_methods!(
    #[cfg(feature = "MapKit_MKLookAroundSnapshotOptions")]
    unsafe impl MKLookAroundSnapshotOptions {
        #[cfg(feature = "MapKit_MKPointOfInterestFilter")]
        #[method_id(@__retain_semantics Other pointOfInterestFilter)]
        pub unsafe fn pointOfInterestFilter(&self) -> Option<Id<MKPointOfInterestFilter>>;

        #[cfg(feature = "MapKit_MKPointOfInterestFilter")]
        #[method(setPointOfInterestFilter:)]
        pub unsafe fn setPointOfInterestFilter(
            &self,
            point_of_interest_filter: Option<&MKPointOfInterestFilter>,
        );

        #[method(size)]
        pub unsafe fn size(&self) -> CGSize;

        #[method(setSize:)]
        pub unsafe fn setSize(&self, size: CGSize);
    }
);
