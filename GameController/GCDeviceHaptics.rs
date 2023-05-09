//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameController::*;

typed_enum!(
    pub type GCHapticsLocality = NSString;
);

extern_static!(GCHapticsLocalityDefault: &'static GCHapticsLocality);

extern_static!(GCHapticsLocalityAll: &'static GCHapticsLocality);

extern_static!(GCHapticsLocalityHandles: &'static GCHapticsLocality);

extern_static!(GCHapticsLocalityLeftHandle: &'static GCHapticsLocality);

extern_static!(GCHapticsLocalityRightHandle: &'static GCHapticsLocality);

extern_static!(GCHapticsLocalityTriggers: &'static GCHapticsLocality);

extern_static!(GCHapticsLocalityLeftTrigger: &'static GCHapticsLocality);

extern_static!(GCHapticsLocalityRightTrigger: &'static GCHapticsLocality);

extern_static!(GCHapticDurationInfinite: c_float);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameController_GCDeviceHaptics")]
    pub struct GCDeviceHaptics;

    #[cfg(feature = "GameController_GCDeviceHaptics")]
    unsafe impl ClassType for GCDeviceHaptics {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "GameController_GCDeviceHaptics")]
unsafe impl NSObjectProtocol for GCDeviceHaptics {}

extern_methods!(
    #[cfg(feature = "GameController_GCDeviceHaptics")]
    unsafe impl GCDeviceHaptics {
        #[cfg(feature = "Foundation_NSSet")]
        #[method_id(@__retain_semantics Other supportedLocalities)]
        pub unsafe fn supportedLocalities(&self) -> Id<NSSet<GCHapticsLocality>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[cfg(feature = "GameController_CHHapticEngine")]
        #[method_id(@__retain_semantics Other createEngineWithLocality:)]
        pub unsafe fn createEngineWithLocality(
            &self,
            locality: &GCHapticsLocality,
        ) -> Option<Id<CHHapticEngine>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "GameController_GCDeviceHaptics")]
    unsafe impl GCDeviceHaptics {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
