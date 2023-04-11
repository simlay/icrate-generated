//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::DeviceCheck::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "DeviceCheck_DCDevice")]
    pub struct DCDevice;

    #[cfg(feature = "DeviceCheck_DCDevice")]
    unsafe impl ClassType for DCDevice {
        type Super = NSObject;
    }
);

#[cfg(feature = "DeviceCheck_DCDevice")]
unsafe impl NSObjectProtocol for DCDevice {}

extern_methods!(
    #[cfg(feature = "DeviceCheck_DCDevice")]
    unsafe impl DCDevice {
        /**
         The current device.
        */
        #[method_id(@__retain_semantics Other currentDevice)]
        pub unsafe fn currentDevice() -> Id<DCDevice>;

        /**
         Check if this API is supported on the current device.
        */
        #[method(isSupported)]
        pub unsafe fn isSupported(&self) -> bool;

        #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSError"))]
        #[method(generateTokenWithCompletionHandler:)]
        pub unsafe fn generateTokenWithCompletionHandler(
            &self,
            completion: &Block<(*mut NSData, *mut NSError), ()>,
        );
    }
);
