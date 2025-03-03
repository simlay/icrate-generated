//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameController::*;

pub type GCGamepadValueChangedHandler =
    *mut Block<(NonNull<GCGamepad>, NonNull<GCControllerElement>), ()>;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameController_GCGamepad")]
    #[deprecated]
    pub struct GCGamepad;

    #[cfg(feature = "GameController_GCGamepad")]
    unsafe impl ClassType for GCGamepad {
        #[inherits(NSObject)]
        type Super = GCPhysicalInputProfile;
    }
);

#[cfg(feature = "GameController_GCGamepad")]
unsafe impl NSObjectProtocol for GCGamepad {}

extern_methods!(
    #[cfg(feature = "GameController_GCGamepad")]
    unsafe impl GCGamepad {
        #[cfg(feature = "GameController_GCController")]
        #[method_id(@__retain_semantics Other controller)]
        pub unsafe fn controller(&self) -> Option<Id<GCController>>;

        #[method(valueChangedHandler)]
        pub unsafe fn valueChangedHandler(&self) -> GCGamepadValueChangedHandler;

        #[method(setValueChangedHandler:)]
        pub unsafe fn setValueChangedHandler(
            &self,
            value_changed_handler: GCGamepadValueChangedHandler,
        );

        #[cfg(feature = "GameController_GCGamepadSnapshot")]
        #[method_id(@__retain_semantics Other saveSnapshot)]
        pub unsafe fn saveSnapshot(&self) -> Id<GCGamepadSnapshot>;

        #[cfg(feature = "GameController_GCControllerDirectionPad")]
        #[method_id(@__retain_semantics Other dpad)]
        pub unsafe fn dpad(&self) -> Id<GCControllerDirectionPad>;

        #[cfg(feature = "GameController_GCControllerButtonInput")]
        #[method_id(@__retain_semantics Other buttonA)]
        pub unsafe fn buttonA(&self) -> Id<GCControllerButtonInput>;

        #[cfg(feature = "GameController_GCControllerButtonInput")]
        #[method_id(@__retain_semantics Other buttonB)]
        pub unsafe fn buttonB(&self) -> Id<GCControllerButtonInput>;

        #[cfg(feature = "GameController_GCControllerButtonInput")]
        #[method_id(@__retain_semantics Other buttonX)]
        pub unsafe fn buttonX(&self) -> Id<GCControllerButtonInput>;

        #[cfg(feature = "GameController_GCControllerButtonInput")]
        #[method_id(@__retain_semantics Other buttonY)]
        pub unsafe fn buttonY(&self) -> Id<GCControllerButtonInput>;

        #[cfg(feature = "GameController_GCControllerButtonInput")]
        #[method_id(@__retain_semantics Other leftShoulder)]
        pub unsafe fn leftShoulder(&self) -> Id<GCControllerButtonInput>;

        #[cfg(feature = "GameController_GCControllerButtonInput")]
        #[method_id(@__retain_semantics Other rightShoulder)]
        pub unsafe fn rightShoulder(&self) -> Id<GCControllerButtonInput>;
    }
);
