//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::IdentityLookup::*;

ns_enum!(
    #[underlying(NSInteger)]
    #[cfg(not(any(target_os = "macos")))]
    pub enum ILClassificationAction {
        #[cfg(not(any(target_os = "macos")))]
        ILClassificationActionNone = 0,
        #[cfg(not(any(target_os = "macos")))]
        ILClassificationActionReportNotJunk = 1,
        #[cfg(not(any(target_os = "macos")))]
        ILClassificationActionReportJunk = 2,
        #[cfg(not(any(target_os = "macos")))]
        ILClassificationActionReportJunkAndBlockSender = 3,
    }
);
