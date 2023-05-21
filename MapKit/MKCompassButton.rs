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

    #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
    #[cfg(feature = "MapKit_MKCompassButton")]
    unsafe impl ClassType for MKCompassButton {
        #[inherits(NSResponder, NSObject)]
        type Super = NSView;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MapKit_MKCompassButton")]
#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
unsafe impl NSAccessibility for MKCompassButton {}

#[cfg(feature = "MapKit_MKCompassButton")]
#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
unsafe impl NSAccessibilityElementProtocol for MKCompassButton {}

#[cfg(feature = "MapKit_MKCompassButton")]
#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
unsafe impl NSAnimatablePropertyContainer for MKCompassButton {}

#[cfg(feature = "MapKit_MKCompassButton")]
#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
unsafe impl NSAppearanceCustomization for MKCompassButton {}

#[cfg(feature = "MapKit_MKCompassButton")]
#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
unsafe impl NSCoding for MKCompassButton {}

#[cfg(feature = "MapKit_MKCompassButton")]
#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
unsafe impl NSDraggingDestination for MKCompassButton {}

#[cfg(feature = "MapKit_MKCompassButton")]
#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
unsafe impl NSObjectProtocol for MKCompassButton {}

#[cfg(feature = "MapKit_MKCompassButton")]
#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
unsafe impl NSUserInterfaceItemIdentification for MKCompassButton {}

#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
extern_methods!(
    #[cfg(feature = "MapKit_MKCompassButton")]
    #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
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

#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
extern_methods!(
    /// Methods declared on superclass `NSView`
    #[cfg(feature = "MapKit_MKCompassButton")]
    #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
    unsafe impl MKCompassButton {
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Option<Allocated<Self>>, frame_rect: NSRect) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self>>;
    }
);

#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(feature = "MapKit_MKCompassButton")]
    #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
    unsafe impl MKCompassButton {
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MapKit_MKCompassButton")]
    #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
    unsafe impl MKCompassButton {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
