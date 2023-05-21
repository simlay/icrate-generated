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
    #[cfg(feature = "MapKit_MKMultiPolylineRenderer")]
    #[cfg(not(any(target_os = "watchos")))]
    pub struct MKMultiPolylineRenderer;

    #[cfg(not(any(target_os = "watchos")))]
    #[cfg(feature = "MapKit_MKMultiPolylineRenderer")]
    unsafe impl ClassType for MKMultiPolylineRenderer {
        #[inherits(MKOverlayRenderer, NSObject)]
        type Super = MKOverlayPathRenderer;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MapKit_MKMultiPolylineRenderer")]
#[cfg(not(any(target_os = "watchos")))]
unsafe impl NSObjectProtocol for MKMultiPolylineRenderer {}

#[cfg(not(any(target_os = "watchos")))]
extern_methods!(
    #[cfg(feature = "MapKit_MKMultiPolylineRenderer")]
    #[cfg(not(any(target_os = "watchos")))]
    unsafe impl MKMultiPolylineRenderer {
        #[cfg(feature = "MapKit_MKMultiPolyline")]
        #[method_id(@__retain_semantics Init initWithMultiPolyline:)]
        pub unsafe fn initWithMultiPolyline(
            this: Option<Allocated<Self>>,
            multi_polyline: &MKMultiPolyline,
        ) -> Id<Self>;

        #[cfg(feature = "MapKit_MKMultiPolyline")]
        #[method_id(@__retain_semantics Other multiPolyline)]
        pub unsafe fn multiPolyline(&self) -> Id<MKMultiPolyline>;
    }
);

#[cfg(not(any(target_os = "watchos")))]
extern_methods!(
    /// Methods declared on superclass `MKOverlayRenderer`
    #[cfg(feature = "MapKit_MKMultiPolylineRenderer")]
    #[cfg(not(any(target_os = "watchos")))]
    unsafe impl MKMultiPolylineRenderer {
        #[method_id(@__retain_semantics Init initWithOverlay:)]
        pub unsafe fn initWithOverlay(
            this: Option<Allocated<Self>>,
            overlay: &ProtocolObject<dyn MKOverlay>,
        ) -> Id<Self>;
    }
);

#[cfg(not(any(target_os = "watchos")))]
extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MapKit_MKMultiPolylineRenderer")]
    #[cfg(not(any(target_os = "watchos")))]
    unsafe impl MKMultiPolylineRenderer {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
