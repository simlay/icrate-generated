//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameController::*;

extern_static!(GCRacingWheelDidConnectNotification: &'static NSString);

extern_static!(GCRacingWheelDidDisconnectNotification: &'static NSString);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameController_GCRacingWheel")]
    pub struct GCRacingWheel;

    #[cfg(feature = "GameController_GCRacingWheel")]
    unsafe impl ClassType for GCRacingWheel {
        type Super = NSObject;
    }
);

#[cfg(feature = "GameController_GCRacingWheel")]
unsafe impl GCDevice for GCRacingWheel {}

#[cfg(feature = "GameController_GCRacingWheel")]
unsafe impl NSObjectProtocol for GCRacingWheel {}

extern_methods!(
    #[cfg(feature = "GameController_GCRacingWheel")]
    unsafe impl GCRacingWheel {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[cfg(feature = "Foundation_NSSet")]
        /**
         Get the collection of racing wheels currently attached to the system.

        @see GCRacingWheelDidConnectNotification
        @see GCRacingWheelDidDisconnectNotification
        */
        #[method_id(@__retain_semantics Other connectedRacingWheels)]
        pub unsafe fn connectedRacingWheels() -> Id<NSSet<GCRacingWheel>>;

        #[cfg(feature = "Foundation_NSError")]
        #[method(acquireDeviceWithError:_)]
        pub unsafe fn acquireDeviceWithError(&self) -> Result<(), Id<NSError>>;

        #[method(relinquishDevice)]
        pub unsafe fn relinquishDevice(&self);

        /**
         Checks if the racing wheel has been acquired by the application.

        This property is observable.
        */
        #[method(isAcquired)]
        pub unsafe fn isAcquired(&self) -> bool;

        #[cfg(feature = "GameController_GCRacingWheelInput")]
        /**
          Get the physical input profile for the racing wheel.
        */
        #[method_id(@__retain_semantics Other wheelInput)]
        pub unsafe fn wheelInput(&self) -> Id<GCRacingWheelInput>;

        /**
         A GCRacingWheel may represent a real device managed by the operating system,
        or a snapshot created by the developer.

        @see capture
        */
        #[method(isSnapshot)]
        pub unsafe fn isSnapshot(&self) -> bool;

        #[method_id(@__retain_semantics Other capture)]
        pub unsafe fn capture(&self) -> Id<GCRacingWheel>;
    }
);
