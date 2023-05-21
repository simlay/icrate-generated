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
    #[cfg(feature = "MapKit_MKUserLocationView")]
    #[cfg(not(any(target_os = "watchos")))]
    pub struct MKUserLocationView;

    #[cfg(not(any(target_os = "watchos")))]
    #[cfg(feature = "MapKit_MKUserLocationView")]
    unsafe impl ClassType for MKUserLocationView {
        #[inherits(NSView, NSResponder, NSObject)]
        type Super = MKAnnotationView;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MapKit_MKUserLocationView")]
#[cfg(not(any(target_os = "watchos")))]
unsafe impl NSAccessibility for MKUserLocationView {}

#[cfg(feature = "MapKit_MKUserLocationView")]
#[cfg(not(any(target_os = "watchos")))]
unsafe impl NSAccessibilityElementProtocol for MKUserLocationView {}

#[cfg(feature = "MapKit_MKUserLocationView")]
#[cfg(not(any(target_os = "watchos")))]
unsafe impl NSAnimatablePropertyContainer for MKUserLocationView {}

#[cfg(feature = "MapKit_MKUserLocationView")]
#[cfg(not(any(target_os = "watchos")))]
unsafe impl NSAppearanceCustomization for MKUserLocationView {}

#[cfg(feature = "MapKit_MKUserLocationView")]
#[cfg(not(any(target_os = "watchos")))]
unsafe impl NSCoding for MKUserLocationView {}

#[cfg(feature = "MapKit_MKUserLocationView")]
#[cfg(not(any(target_os = "watchos")))]
unsafe impl NSDraggingDestination for MKUserLocationView {}

#[cfg(feature = "MapKit_MKUserLocationView")]
#[cfg(not(any(target_os = "watchos")))]
unsafe impl NSObjectProtocol for MKUserLocationView {}

#[cfg(feature = "MapKit_MKUserLocationView")]
#[cfg(not(any(target_os = "watchos")))]
unsafe impl NSUserInterfaceItemIdentification for MKUserLocationView {}

#[cfg(not(any(target_os = "watchos")))]
extern_methods!(
    #[cfg(feature = "MapKit_MKUserLocationView")]
    #[cfg(not(any(target_os = "watchos")))]
    unsafe impl MKUserLocationView {}
);

#[cfg(not(any(target_os = "watchos")))]
extern_methods!(
    /// Methods declared on superclass `MKAnnotationView`
    #[cfg(feature = "MapKit_MKUserLocationView")]
    #[cfg(not(any(target_os = "watchos")))]
    unsafe impl MKUserLocationView {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithAnnotation:reuseIdentifier:)]
        pub unsafe fn initWithAnnotation_reuseIdentifier(
            this: Option<Allocated<Self>>,
            annotation: Option<&ProtocolObject<dyn MKAnnotation>>,
            reuse_identifier: Option<&NSString>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            a_decoder: &NSCoder,
        ) -> Option<Id<Self>>;
    }
);

#[cfg(not(any(target_os = "watchos")))]
extern_methods!(
    /// Methods declared on superclass `NSView`
    #[cfg(feature = "MapKit_MKUserLocationView")]
    #[cfg(not(any(target_os = "watchos")))]
    unsafe impl MKUserLocationView {
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Option<Allocated<Self>>, frame_rect: NSRect) -> Id<Self>;
    }
);

#[cfg(not(any(target_os = "watchos")))]
extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(feature = "MapKit_MKUserLocationView")]
    #[cfg(not(any(target_os = "watchos")))]
    unsafe impl MKUserLocationView {
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

#[cfg(not(any(target_os = "watchos")))]
extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MapKit_MKUserLocationView")]
    #[cfg(not(any(target_os = "watchos")))]
    unsafe impl MKUserLocationView {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
