//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameController::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameController_GCDualShockGamepad")]
    /**
     The GCDualShockGamepad profile represents any supported DualShock 4 controller.

    @see GCExtendedGamepad
    @see GCMotion
    */
    pub struct GCDualShockGamepad;

    #[cfg(feature = "GameController_GCDualShockGamepad")]
    unsafe impl ClassType for GCDualShockGamepad {
        #[inherits(GCPhysicalInputProfile, NSObject)]
        type Super = GCExtendedGamepad;
    }
);

#[cfg(feature = "GameController_GCDualShockGamepad")]
/**
 The GCDualShockGamepad profile represents any supported DualShock 4 controller.

@see GCExtendedGamepad
@see GCMotion
*/
unsafe impl NSObjectProtocol for GCDualShockGamepad {}

extern_methods!(
    /**
     The GCDualShockGamepad profile represents any supported DualShock 4 controller.

    @see GCExtendedGamepad
    @see GCMotion
    */
    #[cfg(feature = "GameController_GCDualShockGamepad")]
    unsafe impl GCDualShockGamepad {
        #[cfg(feature = "GameController_GCControllerButtonInput")]
        /**
         DualShock controllers have a touchpad with a button and two-finger tracking.
        */
        #[method_id(@__retain_semantics Other touchpadButton)]
        pub unsafe fn touchpadButton(&self) -> Option<Id<GCControllerButtonInput>>;

        #[cfg(feature = "GameController_GCControllerDirectionPad")]
        #[method_id(@__retain_semantics Other touchpadPrimary)]
        pub unsafe fn touchpadPrimary(&self) -> Option<Id<GCControllerDirectionPad>>;

        #[cfg(feature = "GameController_GCControllerDirectionPad")]
        #[method_id(@__retain_semantics Other touchpadSecondary)]
        pub unsafe fn touchpadSecondary(&self) -> Option<Id<GCControllerDirectionPad>>;
    }
);
