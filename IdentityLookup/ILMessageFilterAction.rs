//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::IdentityLookup::*;

#[cfg(not(any(target_os = "macos")))]
ns_enum!(
    #[underlying(NSInteger)]
    pub enum ILMessageFilterAction {
        #[cfg(not(any(target_os = "macos")))]
        ILMessageFilterActionNone = 0,
        #[cfg(not(any(target_os = "macos")))]
        ILMessageFilterActionAllow = 1,
        #[cfg(not(any(target_os = "macos")))]
        ILMessageFilterActionJunk = 2,
        #[deprecated]
        #[cfg(not(any(target_os = "macos")))]
        ILMessageFilterActionFilter = ILMessageFilterActionJunk,
        #[cfg(not(any(target_os = "macos")))]
        ILMessageFilterActionPromotion = 3,
        #[cfg(not(any(target_os = "macos")))]
        ILMessageFilterActionTransaction = 4,
    }
);

#[cfg(not(any(target_os = "macos")))]
ns_enum!(
    #[underlying(NSInteger)]
    pub enum ILMessageFilterSubAction {
        #[cfg(not(any(target_os = "macos")))]
        ILMessageFilterSubActionNone = 0,
        #[cfg(not(any(target_os = "macos")))]
        ILMessageFilterSubActionTransactionalOthers = 10000,
        #[cfg(not(any(target_os = "macos")))]
        ILMessageFilterSubActionTransactionalFinance = 10001,
        #[cfg(not(any(target_os = "macos")))]
        ILMessageFilterSubActionTransactionalOrders = 10002,
        #[cfg(not(any(target_os = "macos")))]
        ILMessageFilterSubActionTransactionalReminders = 10003,
        #[cfg(not(any(target_os = "macos")))]
        ILMessageFilterSubActionTransactionalHealth = 10004,
        #[cfg(not(any(target_os = "macos")))]
        ILMessageFilterSubActionTransactionalWeather = 10005,
        #[cfg(not(any(target_os = "macos")))]
        ILMessageFilterSubActionTransactionalCarrier = 10006,
        #[cfg(not(any(target_os = "macos")))]
        ILMessageFilterSubActionTransactionalRewards = 10007,
        #[cfg(not(any(target_os = "macos")))]
        ILMessageFilterSubActionTransactionalPublicServices = 10008,
        #[cfg(not(any(target_os = "macos")))]
        ILMessageFilterSubActionPromotionalOthers = 20000,
        #[cfg(not(any(target_os = "macos")))]
        ILMessageFilterSubActionPromotionalOffers = 20001,
        #[cfg(not(any(target_os = "macos")))]
        ILMessageFilterSubActionPromotionalCoupons = 20002,
    }
);
