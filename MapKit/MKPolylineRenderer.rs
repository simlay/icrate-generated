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
    #[cfg(feature = "MapKit_MKPolylineRenderer")]
    #[cfg(not(any(target_os = "watchos")))]
    pub struct MKPolylineRenderer;

    #[cfg(not(any(target_os = "watchos")))]
    #[cfg(feature = "MapKit_MKPolylineRenderer")]
    unsafe impl ClassType for MKPolylineRenderer {
        #[inherits(MKOverlayRenderer, NSObject)]
        type Super = MKOverlayPathRenderer;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MapKit_MKPolylineRenderer")]
#[cfg(not(any(target_os = "watchos")))]
unsafe impl NSObjectProtocol for MKPolylineRenderer {}

#[cfg(not(any(target_os = "watchos")))]
extern_methods!(
    #[cfg(feature = "MapKit_MKPolylineRenderer")]
    #[cfg(not(any(target_os = "watchos")))]
    unsafe impl MKPolylineRenderer {
        #[cfg(feature = "MapKit_MKPolyline")]
        #[method_id(@__retain_semantics Init initWithPolyline:)]
        pub unsafe fn initWithPolyline(
            this: Option<Allocated<Self>>,
            polyline: &MKPolyline,
        ) -> Id<Self>;

        #[cfg(feature = "MapKit_MKPolyline")]
        #[method_id(@__retain_semantics Other polyline)]
        pub unsafe fn polyline(&self) -> Id<MKPolyline>;

        #[cfg(not(any(target_os = "watchos")))]
        #[method(strokeStart)]
        pub unsafe fn strokeStart(&self) -> CGFloat;

        #[cfg(not(any(target_os = "watchos")))]
        #[method(setStrokeStart:)]
        pub unsafe fn setStrokeStart(&self, stroke_start: CGFloat);

        #[cfg(not(any(target_os = "watchos")))]
        #[method(strokeEnd)]
        pub unsafe fn strokeEnd(&self) -> CGFloat;

        #[cfg(not(any(target_os = "watchos")))]
        #[method(setStrokeEnd:)]
        pub unsafe fn setStrokeEnd(&self, stroke_end: CGFloat);
    }
);

#[cfg(not(any(target_os = "watchos")))]
extern_methods!(
    /// Methods declared on superclass `MKOverlayRenderer`
    #[cfg(feature = "MapKit_MKPolylineRenderer")]
    #[cfg(not(any(target_os = "watchos")))]
    unsafe impl MKPolylineRenderer {
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
    #[cfg(feature = "MapKit_MKPolylineRenderer")]
    #[cfg(not(any(target_os = "watchos")))]
    unsafe impl MKPolylineRenderer {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
