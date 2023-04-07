//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Contacts::*;
use crate::CoreLocation::*;
use crate::Foundation::*;

#[cfg(not(any(target_os = "macos", target_os = "tvos", target_os = "watchos")))]
extern_static!(CLLocationPushServiceErrorDomain: Option<&'static NSErrorDomain>);

ns_error_enum!(
    #[underlying(NSInteger)]
    #[cfg(not(any(target_os = "macos", target_os = "tvos", target_os = "watchos")))]
    pub enum CLLocationPushServiceError {
        #[cfg(not(any(target_os = "macos", target_os = "tvos", target_os = "watchos")))]
        CLLocationPushServiceErrorUnknown = 0,
        #[cfg(not(any(target_os = "macos", target_os = "tvos", target_os = "watchos")))]
        CLLocationPushServiceErrorMissingPushExtension = 1,
        #[cfg(not(any(target_os = "macos", target_os = "tvos", target_os = "watchos")))]
        CLLocationPushServiceErrorMissingPushServerEnvironment = 2,
        #[cfg(not(any(target_os = "macos", target_os = "tvos", target_os = "watchos")))]
        CLLocationPushServiceErrorMissingEntitlement = 3,
    }
);
