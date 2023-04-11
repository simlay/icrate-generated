//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameController::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameController_GCDualSenseGamepad")]
    /**
     The GCDualSenseGamepad profile represents any supported DualSense controller.

    @see GCExtendedGamepad
    @see GCMotion
    */
    pub struct GCDualSenseGamepad;

    #[cfg(feature = "GameController_GCDualSenseGamepad")]
    unsafe impl ClassType for GCDualSenseGamepad {
        #[inherits(GCPhysicalInputProfile, NSObject)]
        type Super = GCExtendedGamepad;
    }
);

#[cfg(feature = "GameController_GCDualSenseGamepad")]
/**
 The GCDualSenseGamepad profile represents any supported DualSense controller.

@see GCExtendedGamepad
@see GCMotion
*/
unsafe impl NSObjectProtocol for GCDualSenseGamepad {}

extern_methods!(
    /**
     The GCDualSenseGamepad profile represents any supported DualSense controller.

    @see GCExtendedGamepad
    @see GCMotion
    */
    #[cfg(feature = "GameController_GCDualSenseGamepad")]
    unsafe impl GCDualSenseGamepad {
        #[cfg(feature = "GameController_GCControllerButtonInput")]
        /**
         DualSense controllers have a touchpad with a button and two-finger tracking.
        */
        #[method_id(@__retain_semantics Other touchpadButton)]
        pub unsafe fn touchpadButton(&self) -> Id<GCControllerButtonInput>;

        #[cfg(feature = "GameController_GCControllerDirectionPad")]
        #[method_id(@__retain_semantics Other touchpadPrimary)]
        pub unsafe fn touchpadPrimary(&self) -> Id<GCControllerDirectionPad>;

        #[cfg(feature = "GameController_GCControllerDirectionPad")]
        #[method_id(@__retain_semantics Other touchpadSecondary)]
        pub unsafe fn touchpadSecondary(&self) -> Id<GCControllerDirectionPad>;

        #[cfg(feature = "GameController_GCDualSenseAdaptiveTrigger")]
        /**
         Triggers are required to be analog inputs. Common uses would be acceleration and decelleration in a driving game for example.

        The DualSense has adaptive triggers, allowing you to specify a dynamic resistance force that is applied when pulling the trigger. This can,
        for example, be used to emulate the feeling of pulling back a bow string, firing a weapon, or pulling a lever.
        */
        #[method_id(@__retain_semantics Other leftTrigger)]
        pub unsafe fn leftTrigger(&self) -> Id<GCDualSenseAdaptiveTrigger>;

        #[cfg(feature = "GameController_GCDualSenseAdaptiveTrigger")]
        #[method_id(@__retain_semantics Other rightTrigger)]
        pub unsafe fn rightTrigger(&self) -> Id<GCDualSenseAdaptiveTrigger>;
    }
);
