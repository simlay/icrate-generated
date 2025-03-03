//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameController::*;

pub type GCControllerDirectionPadValueChangedHandler =
    *mut Block<(NonNull<GCControllerDirectionPad>, c_float, c_float), ()>;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameController_GCControllerDirectionPad")]
    pub struct GCControllerDirectionPad;

    #[cfg(feature = "GameController_GCControllerDirectionPad")]
    unsafe impl ClassType for GCControllerDirectionPad {
        #[inherits(NSObject)]
        type Super = GCControllerElement;
    }
);

#[cfg(feature = "GameController_GCControllerDirectionPad")]
unsafe impl NSObjectProtocol for GCControllerDirectionPad {}

extern_methods!(
    #[cfg(feature = "GameController_GCControllerDirectionPad")]
    unsafe impl GCControllerDirectionPad {
        #[method(valueChangedHandler)]
        pub unsafe fn valueChangedHandler(&self) -> GCControllerDirectionPadValueChangedHandler;

        #[method(setValueChangedHandler:)]
        pub unsafe fn setValueChangedHandler(
            &self,
            value_changed_handler: GCControllerDirectionPadValueChangedHandler,
        );

        #[cfg(feature = "GameController_GCControllerAxisInput")]
        #[method_id(@__retain_semantics Other xAxis)]
        pub unsafe fn xAxis(&self) -> Id<GCControllerAxisInput>;

        #[cfg(feature = "GameController_GCControllerAxisInput")]
        #[method_id(@__retain_semantics Other yAxis)]
        pub unsafe fn yAxis(&self) -> Id<GCControllerAxisInput>;

        #[cfg(feature = "GameController_GCControllerButtonInput")]
        #[method_id(@__retain_semantics Other up)]
        pub unsafe fn up(&self) -> Id<GCControllerButtonInput>;

        #[cfg(feature = "GameController_GCControllerButtonInput")]
        #[method_id(@__retain_semantics Other down)]
        pub unsafe fn down(&self) -> Id<GCControllerButtonInput>;

        #[cfg(feature = "GameController_GCControllerButtonInput")]
        #[method_id(@__retain_semantics Other left)]
        pub unsafe fn left(&self) -> Id<GCControllerButtonInput>;

        #[cfg(feature = "GameController_GCControllerButtonInput")]
        #[method_id(@__retain_semantics Other right)]
        pub unsafe fn right(&self) -> Id<GCControllerButtonInput>;

        #[method(setValueForXAxis:yAxis:)]
        pub unsafe fn setValueForXAxis_yAxis(&self, x_axis: c_float, y_axis: c_float);
    }
);
