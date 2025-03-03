//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Contacts::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::MapKit::*;

ns_enum!(
    #[underlying(NSUInteger)]
    #[deprecated = "Use MKPinAnnotationView's pinTintColor instead"]
    pub enum MKPinAnnotationColor {
        #[deprecated = "Use MKPinAnnotationView's pinTintColor instead"]
        MKPinAnnotationColorRed = 0,
        #[deprecated = "Use MKPinAnnotationView's pinTintColor instead"]
        MKPinAnnotationColorGreen = 1,
        #[deprecated = "Use MKPinAnnotationView's pinTintColor instead"]
        MKPinAnnotationColorPurple = 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MapKit_MKPinAnnotationView")]
    #[deprecated]
    pub struct MKPinAnnotationView;

    #[cfg(feature = "MapKit_MKPinAnnotationView")]
    unsafe impl ClassType for MKPinAnnotationView {
        #[inherits(NSView, NSResponder, NSObject)]
        type Super = MKAnnotationView;
    }
);

#[cfg(feature = "MapKit_MKPinAnnotationView")]
unsafe impl NSAccessibility for MKPinAnnotationView {}

#[cfg(feature = "MapKit_MKPinAnnotationView")]
unsafe impl NSAccessibilityElementProtocol for MKPinAnnotationView {}

#[cfg(feature = "MapKit_MKPinAnnotationView")]
unsafe impl NSAnimatablePropertyContainer for MKPinAnnotationView {}

#[cfg(feature = "MapKit_MKPinAnnotationView")]
unsafe impl NSAppearanceCustomization for MKPinAnnotationView {}

#[cfg(feature = "MapKit_MKPinAnnotationView")]
unsafe impl NSCoding for MKPinAnnotationView {}

#[cfg(feature = "MapKit_MKPinAnnotationView")]
unsafe impl NSDraggingDestination for MKPinAnnotationView {}

#[cfg(feature = "MapKit_MKPinAnnotationView")]
unsafe impl NSObjectProtocol for MKPinAnnotationView {}

#[cfg(feature = "MapKit_MKPinAnnotationView")]
unsafe impl NSUserInterfaceItemIdentification for MKPinAnnotationView {}

extern_methods!(
    #[cfg(feature = "MapKit_MKPinAnnotationView")]
    unsafe impl MKPinAnnotationView {
        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Other redPinColor)]
        pub unsafe fn redPinColor() -> Id<NSColor>;

        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Other greenPinColor)]
        pub unsafe fn greenPinColor() -> Id<NSColor>;

        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Other purplePinColor)]
        pub unsafe fn purplePinColor() -> Id<NSColor>;

        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Other pinTintColor)]
        pub unsafe fn pinTintColor(&self) -> Option<Id<NSColor>>;

        #[cfg(feature = "AppKit_NSColor")]
        #[method(setPinTintColor:)]
        pub unsafe fn setPinTintColor(&self, pin_tint_color: Option<&NSColor>);

        #[method(animatesDrop)]
        pub unsafe fn animatesDrop(&self) -> bool;

        #[method(setAnimatesDrop:)]
        pub unsafe fn setAnimatesDrop(&self, animates_drop: bool);

        #[deprecated = "Use pinTintColor instead"]
        #[method(pinColor)]
        pub unsafe fn pinColor(&self) -> MKPinAnnotationColor;

        #[deprecated = "Use pinTintColor instead"]
        #[method(setPinColor:)]
        pub unsafe fn setPinColor(&self, pin_color: MKPinAnnotationColor);
    }
);

extern_methods!(
    /// Methods declared on superclass `MKAnnotationView`
    #[cfg(feature = "MapKit_MKPinAnnotationView")]
    unsafe impl MKPinAnnotationView {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithAnnotation:reuseIdentifier:)]
        pub unsafe fn initWithAnnotation_reuseIdentifier(
            this: Option<Allocated<Self>>,
            annotation: Option<&ProtocolObject<dyn MKAnnotation>>,
            reuse_identifier: Option<&NSString>,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSView`
    #[cfg(feature = "MapKit_MKPinAnnotationView")]
    unsafe impl MKPinAnnotationView {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Option<Allocated<Self>>, frame_rect: NSRect) -> Id<Self>;
    }
);
