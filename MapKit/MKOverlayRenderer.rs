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
    #[cfg(feature = "MapKit_MKOverlayRenderer")]
    #[cfg(not(any(target_os = "watchos")))]
    pub struct MKOverlayRenderer;

    #[cfg(not(any(target_os = "watchos")))]
    #[cfg(feature = "MapKit_MKOverlayRenderer")]
    unsafe impl ClassType for MKOverlayRenderer {
        type Super = NSObject;
    }
);

#[cfg(feature = "MapKit_MKOverlayRenderer")]
#[cfg(not(any(target_os = "watchos")))]
unsafe impl NSObjectProtocol for MKOverlayRenderer {}

extern_methods!(
    #[cfg(feature = "MapKit_MKOverlayRenderer")]
    #[cfg(not(any(target_os = "watchos")))]
    unsafe impl MKOverlayRenderer {
        #[cfg(not(any(target_os = "watchos")))]
        #[method_id(@__retain_semantics Init initWithOverlay:)]
        pub unsafe fn initWithOverlay(
            this: Option<Allocated<Self>>,
            overlay: &ProtocolObject<dyn MKOverlay>,
        ) -> Id<Self>;

        #[cfg(not(any(target_os = "watchos")))]
        #[method_id(@__retain_semantics Other overlay)]
        pub unsafe fn overlay(&self) -> Id<ProtocolObject<dyn MKOverlay>>;

        #[cfg(not(any(target_os = "watchos")))]
        #[method(pointForMapPoint:)]
        pub unsafe fn pointForMapPoint(&self, map_point: MKMapPoint) -> CGPoint;

        #[cfg(not(any(target_os = "watchos")))]
        #[method(mapPointForPoint:)]
        pub unsafe fn mapPointForPoint(&self, point: CGPoint) -> MKMapPoint;

        #[cfg(not(any(target_os = "watchos")))]
        #[method(rectForMapRect:)]
        pub unsafe fn rectForMapRect(&self, map_rect: MKMapRect) -> CGRect;

        #[cfg(not(any(target_os = "watchos")))]
        #[method(mapRectForRect:)]
        pub unsafe fn mapRectForRect(&self, rect: CGRect) -> MKMapRect;

        #[cfg(not(any(target_os = "watchos")))]
        #[method(canDrawMapRect:zoomScale:)]
        pub unsafe fn canDrawMapRect_zoomScale(
            &self,
            map_rect: MKMapRect,
            zoom_scale: MKZoomScale,
        ) -> bool;

        #[cfg(not(any(target_os = "watchos")))]
        #[method(setNeedsDisplay)]
        pub unsafe fn setNeedsDisplay(&self);

        #[cfg(not(any(target_os = "watchos")))]
        #[method(setNeedsDisplayInMapRect:)]
        pub unsafe fn setNeedsDisplayInMapRect(&self, map_rect: MKMapRect);

        #[cfg(not(any(target_os = "watchos")))]
        #[method(setNeedsDisplayInMapRect:zoomScale:)]
        pub unsafe fn setNeedsDisplayInMapRect_zoomScale(
            &self,
            map_rect: MKMapRect,
            zoom_scale: MKZoomScale,
        );

        #[cfg(not(any(target_os = "watchos")))]
        #[method(alpha)]
        pub unsafe fn alpha(&self) -> CGFloat;

        #[cfg(not(any(target_os = "watchos")))]
        #[method(setAlpha:)]
        pub unsafe fn setAlpha(&self, alpha: CGFloat);

        #[cfg(not(any(target_os = "watchos")))]
        #[method(contentScaleFactor)]
        pub unsafe fn contentScaleFactor(&self) -> CGFloat;
    }
);

extern_fn!(
    #[cfg(not(any(target_os = "watchos")))]
    pub unsafe fn MKRoadWidthAtZoomScale(zoom_scale: MKZoomScale) -> CGFloat;
);
