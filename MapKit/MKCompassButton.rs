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
    #[cfg(feature = "MapKit_MKCompassButton")]
    #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
    pub struct MKCompassButton;

    #[cfg(feature = "MapKit_MKCompassButton")]
    unsafe impl ClassType for MKCompassButton {
        #[inherits(NSResponder, NSObject)]
        type Super = NSView;
    }
);

#[cfg(feature = "MapKit_MKCompassButton")]
unsafe impl NSAccessibility for MKCompassButton {}

#[cfg(feature = "MapKit_MKCompassButton")]
unsafe impl NSAccessibilityElementProtocol for MKCompassButton {}

#[cfg(feature = "MapKit_MKCompassButton")]
unsafe impl NSAnimatablePropertyContainer for MKCompassButton {}

#[cfg(feature = "MapKit_MKCompassButton")]
unsafe impl NSAppearanceCustomization for MKCompassButton {}

#[cfg(feature = "MapKit_MKCompassButton")]
unsafe impl NSCoding for MKCompassButton {}

#[cfg(feature = "MapKit_MKCompassButton")]
unsafe impl NSDraggingDestination for MKCompassButton {}

#[cfg(feature = "MapKit_MKCompassButton")]
unsafe impl NSObjectProtocol for MKCompassButton {}

#[cfg(feature = "MapKit_MKCompassButton")]
unsafe impl NSUserInterfaceItemIdentification for MKCompassButton {}

extern_methods!(
    #[cfg(feature = "MapKit_MKCompassButton")]
    unsafe impl MKCompassButton {
        #[cfg(feature = "MapKit_MKMapView")]
        #[method_id(@__retain_semantics Other compassButtonWithMapView:)]
        pub unsafe fn compassButtonWithMapView(map_view: Option<&MKMapView>) -> Id<Self>;

        #[cfg(feature = "MapKit_MKMapView")]
        #[method_id(@__retain_semantics Other mapView)]
        pub unsafe fn mapView(&self) -> Option<Id<MKMapView>>;

        #[cfg(feature = "MapKit_MKMapView")]
        #[method(setMapView:)]
        pub unsafe fn setMapView(&self, map_view: Option<&MKMapView>);

        #[method(compassVisibility)]
        pub unsafe fn compassVisibility(&self) -> MKFeatureVisibility;

        #[method(setCompassVisibility:)]
        pub unsafe fn setCompassVisibility(&self, compass_visibility: MKFeatureVisibility);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSView`
    #[cfg(feature = "MapKit_MKCompassButton")]
    unsafe impl MKCompassButton {
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Option<Allocated<Self>>, frame_rect: NSRect) -> Id<Self>;
    }
);
