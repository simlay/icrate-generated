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
    #[cfg(feature = "MapKit_MKMultiPolygon")]
    #[cfg(not(any(target_os = "watchos")))]
    pub struct MKMultiPolygon;

    #[cfg(not(any(target_os = "watchos")))]
    #[cfg(feature = "MapKit_MKMultiPolygon")]
    unsafe impl ClassType for MKMultiPolygon {
        #[inherits(NSObject)]
        type Super = MKShape;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MapKit_MKMultiPolygon")]
#[cfg(not(any(target_os = "watchos")))]
unsafe impl MKAnnotation for MKMultiPolygon {}

#[cfg(feature = "MapKit_MKMultiPolygon")]
#[cfg(not(any(target_os = "watchos")))]
unsafe impl MKOverlay for MKMultiPolygon {}

#[cfg(feature = "MapKit_MKMultiPolygon")]
#[cfg(not(any(target_os = "watchos")))]
unsafe impl NSObjectProtocol for MKMultiPolygon {}

#[cfg(not(any(target_os = "watchos")))]
extern_methods!(
    #[cfg(feature = "MapKit_MKMultiPolygon")]
    #[cfg(not(any(target_os = "watchos")))]
    unsafe impl MKMultiPolygon {
        #[cfg(all(feature = "Foundation_NSArray", feature = "MapKit_MKPolygon"))]
        #[method_id(@__retain_semantics Init initWithPolygons:)]
        pub unsafe fn initWithPolygons(
            this: Option<Allocated<Self>>,
            polygons: &NSArray<MKPolygon>,
        ) -> Id<Self>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "MapKit_MKPolygon"))]
        #[method_id(@__retain_semantics Other polygons)]
        pub unsafe fn polygons(&self) -> Id<NSArray<MKPolygon>>;
    }
);

#[cfg(not(any(target_os = "watchos")))]
extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MapKit_MKMultiPolygon")]
    #[cfg(not(any(target_os = "watchos")))]
    unsafe impl MKMultiPolygon {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
