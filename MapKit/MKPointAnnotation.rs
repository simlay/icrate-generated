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
    #[cfg(feature = "MapKit_MKPointAnnotation")]
    #[cfg(not(any(target_os = "watchos")))]
    pub struct MKPointAnnotation;

    #[cfg(feature = "MapKit_MKPointAnnotation")]
    unsafe impl ClassType for MKPointAnnotation {
        #[inherits(NSObject)]
        type Super = MKShape;
    }
);

#[cfg(feature = "MapKit_MKPointAnnotation")]
unsafe impl MKAnnotation for MKPointAnnotation {}

#[cfg(feature = "MapKit_MKPointAnnotation")]
unsafe impl NSObjectProtocol for MKPointAnnotation {}

extern_methods!(
    #[cfg(feature = "MapKit_MKPointAnnotation")]
    unsafe impl MKPointAnnotation {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[cfg(not(any(target_os = "watchos")))]
        #[method_id(@__retain_semantics Init initWithCoordinate:)]
        pub unsafe fn initWithCoordinate(
            this: Option<Allocated<Self>>,
            coordinate: CLLocationCoordinate2D,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[cfg(not(any(target_os = "watchos")))]
        #[method_id(@__retain_semantics Init initWithCoordinate:title:subtitle:)]
        pub unsafe fn initWithCoordinate_title_subtitle(
            this: Option<Allocated<Self>>,
            coordinate: CLLocationCoordinate2D,
            title: Option<&NSString>,
            subtitle: Option<&NSString>,
        ) -> Id<Self>;

        #[method(coordinate)]
        pub unsafe fn coordinate(&self) -> CLLocationCoordinate2D;

        #[method(setCoordinate:)]
        pub unsafe fn setCoordinate(&self, coordinate: CLLocationCoordinate2D);
    }
);
