//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::IdentityLookup::*;

extern_static!(ILMessageFilterErrorDomain: &'static NSErrorDomain);

ns_error_enum!(
    #[underlying(NSInteger)]
    #[cfg(not(any(target_os = "macos")))]
    pub enum ILMessageFilterError {
        #[cfg(not(any(target_os = "macos")))]
        ILMessageFilterErrorSystem = 1,
        #[cfg(not(any(target_os = "macos")))]
        ILMessageFilterErrorInvalidNetworkURL = 2,
        #[cfg(not(any(target_os = "macos")))]
        ILMessageFilterErrorNetworkURLUnauthorized = 3,
        #[cfg(not(any(target_os = "macos")))]
        ILMessageFilterErrorNetworkRequestFailed = 4,
        #[cfg(not(any(target_os = "macos")))]
        ILMessageFilterErrorRedundantNetworkDeferral = 5,
    }
);
