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
    #[cfg(feature = "MapKit_MKMapCamera")]
    #[cfg(not(any(target_os = "watchos")))]
    pub struct MKMapCamera;

    #[cfg(not(any(target_os = "watchos")))]
    #[cfg(feature = "MapKit_MKMapCamera")]
    unsafe impl ClassType for MKMapCamera {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MapKit_MKMapCamera")]
#[cfg(not(any(target_os = "watchos")))]
unsafe impl NSCoding for MKMapCamera {}

#[cfg(feature = "MapKit_MKMapCamera")]
#[cfg(not(any(target_os = "watchos")))]
unsafe impl NSCopying for MKMapCamera {}

#[cfg(feature = "MapKit_MKMapCamera")]
#[cfg(not(any(target_os = "watchos")))]
unsafe impl NSObjectProtocol for MKMapCamera {}

#[cfg(feature = "MapKit_MKMapCamera")]
#[cfg(not(any(target_os = "watchos")))]
unsafe impl NSSecureCoding for MKMapCamera {}

#[cfg(not(any(target_os = "watchos")))]
extern_methods!(
    #[cfg(feature = "MapKit_MKMapCamera")]
    #[cfg(not(any(target_os = "watchos")))]
    unsafe impl MKMapCamera {
        #[method(centerCoordinate)]
        pub unsafe fn centerCoordinate(&self) -> CLLocationCoordinate2D;

        #[method(setCenterCoordinate:)]
        pub unsafe fn setCenterCoordinate(&self, center_coordinate: CLLocationCoordinate2D);

        #[cfg(not(any(target_os = "watchos")))]
        #[method(centerCoordinateDistance)]
        pub unsafe fn centerCoordinateDistance(&self) -> CLLocationDistance;

        #[cfg(not(any(target_os = "watchos")))]
        #[method(setCenterCoordinateDistance:)]
        pub unsafe fn setCenterCoordinateDistance(
            &self,
            center_coordinate_distance: CLLocationDistance,
        );

        #[method(heading)]
        pub unsafe fn heading(&self) -> CLLocationDirection;

        #[method(setHeading:)]
        pub unsafe fn setHeading(&self, heading: CLLocationDirection);

        #[method(pitch)]
        pub unsafe fn pitch(&self) -> CGFloat;

        #[method(setPitch:)]
        pub unsafe fn setPitch(&self, pitch: CGFloat);

        #[deprecated = "Use centerCoordinateDistance"]
        #[cfg(not(any(target_os = "watchos")))]
        #[method(altitude)]
        pub unsafe fn altitude(&self) -> CLLocationDistance;

        #[deprecated = "Use centerCoordinateDistance"]
        #[cfg(not(any(target_os = "watchos")))]
        #[method(setAltitude:)]
        pub unsafe fn setAltitude(&self, altitude: CLLocationDistance);

        #[method_id(@__retain_semantics Other camera)]
        pub unsafe fn camera() -> Id<Self>;

        #[method_id(@__retain_semantics Other cameraLookingAtCenterCoordinate:fromEyeCoordinate:eyeAltitude:)]
        pub unsafe fn cameraLookingAtCenterCoordinate_fromEyeCoordinate_eyeAltitude(
            center_coordinate: CLLocationCoordinate2D,
            eye_coordinate: CLLocationCoordinate2D,
            eye_altitude: CLLocationDistance,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Other cameraLookingAtCenterCoordinate:fromDistance:pitch:heading:)]
        pub unsafe fn cameraLookingAtCenterCoordinate_fromDistance_pitch_heading(
            center_coordinate: CLLocationCoordinate2D,
            distance: CLLocationDistance,
            pitch: CGFloat,
            heading: CLLocationDirection,
        ) -> Id<Self>;

        #[cfg(feature = "MapKit_MKMapItem")]
        #[cfg(not(any(target_os = "watchos")))]
        #[method_id(@__retain_semantics Other cameraLookingAtMapItem:forViewSize:allowPitch:)]
        pub unsafe fn cameraLookingAtMapItem_forViewSize_allowPitch(
            map_item: &MKMapItem,
            view_size: CGSize,
            allow_pitch: bool,
        ) -> Id<Self>;
    }
);

#[cfg(not(any(target_os = "watchos")))]
extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MapKit_MKMapCamera")]
    #[cfg(not(any(target_os = "watchos")))]
    unsafe impl MKMapCamera {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
