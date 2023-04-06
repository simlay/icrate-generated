//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameController::*;

#[cfg(not(any(target_os = "ios", target_os = "tvos")))]
extern_static!(GCRacingWheelDidConnectNotification: &'static NSString);

#[cfg(not(any(target_os = "ios", target_os = "tvos")))]
extern_static!(GCRacingWheelDidDisconnectNotification: &'static NSString);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameController_GCRacingWheel")]
    #[cfg(not(any(target_os = "ios", target_os = "tvos")))]
    pub struct GCRacingWheel;

    #[cfg(not(any(target_os = "ios", target_os = "tvos")))]
    #[cfg(feature = "GameController_GCRacingWheel")]
    unsafe impl ClassType for GCRacingWheel {
        type Super = NSObject;
    }
);

#[cfg(feature = "GameController_GCRacingWheel")]
#[cfg(not(any(target_os = "ios", target_os = "tvos")))]
unsafe impl GCDevice for GCRacingWheel {}

#[cfg(feature = "GameController_GCRacingWheel")]
#[cfg(not(any(target_os = "ios", target_os = "tvos")))]
unsafe impl NSObjectProtocol for GCRacingWheel {}

extern_methods!(
    #[cfg(feature = "GameController_GCRacingWheel")]
    #[cfg(not(any(target_os = "ios", target_os = "tvos")))]
    unsafe impl GCRacingWheel {
        #[cfg(not(any(target_os = "ios", target_os = "tvos")))]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[cfg(not(any(target_os = "ios", target_os = "tvos")))]
        #[cfg(feature = "Foundation_NSSet")]
        #[method_id(@__retain_semantics Other connectedRacingWheels)]
        pub unsafe fn connectedRacingWheels() -> Id<NSSet<GCRacingWheel>>;

        #[cfg(not(any(target_os = "ios", target_os = "tvos")))]
        #[cfg(feature = "Foundation_NSError")]
        #[method(acquireDeviceWithError:_)]
        pub unsafe fn acquireDeviceWithError(&self) -> Result<(), Id<NSError>>;

        #[cfg(not(any(target_os = "ios", target_os = "tvos")))]
        #[method(relinquishDevice)]
        pub unsafe fn relinquishDevice(&self);

        #[cfg(not(any(target_os = "ios", target_os = "tvos")))]
        #[method(isAcquired)]
        pub unsafe fn isAcquired(&self) -> bool;

        #[cfg(not(any(target_os = "ios", target_os = "tvos")))]
        #[cfg(feature = "GameController_GCRacingWheelInput")]
        #[method_id(@__retain_semantics Other wheelInput)]
        pub unsafe fn wheelInput(&self) -> Id<GCRacingWheelInput>;

        #[cfg(not(any(target_os = "ios", target_os = "tvos")))]
        #[method(isSnapshot)]
        pub unsafe fn isSnapshot(&self) -> bool;

        #[cfg(not(any(target_os = "ios", target_os = "tvos")))]
        #[method_id(@__retain_semantics Other capture)]
        pub unsafe fn capture(&self) -> Id<GCRacingWheel>;
    }
);
