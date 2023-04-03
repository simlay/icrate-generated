//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameController::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameController_GCSteeringWheelElement")]
    #[cfg(not(any(target_os = "ios")))]
    pub struct GCSteeringWheelElement;

    #[cfg(feature = "GameController_GCSteeringWheelElement")]
    unsafe impl ClassType for GCSteeringWheelElement {
        type Super = NSObject;
    }
);

#[cfg(feature = "GameController_GCSteeringWheelElement")]
unsafe impl GCAxisElement for GCSteeringWheelElement {}

#[cfg(feature = "GameController_GCSteeringWheelElement")]
unsafe impl GCPhysicalInputElement for GCSteeringWheelElement {}

#[cfg(feature = "GameController_GCSteeringWheelElement")]
unsafe impl NSObjectProtocol for GCSteeringWheelElement {}

extern_methods!(
    #[cfg(feature = "GameController_GCSteeringWheelElement")]
    unsafe impl GCSteeringWheelElement {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method(maximumDegreesOfRotation)]
        pub unsafe fn maximumDegreesOfRotation(&self) -> c_float;
    }
);
