//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::ExternalAccessory::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum EAWiFiUnconfiguredAccessoryBrowserState {
        EAWiFiUnconfiguredAccessoryBrowserStateWiFiUnavailable = 0,
        EAWiFiUnconfiguredAccessoryBrowserStateStopped = 1,
        EAWiFiUnconfiguredAccessoryBrowserStateSearching = 2,
        EAWiFiUnconfiguredAccessoryBrowserStateConfiguring = 3,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum EAWiFiUnconfiguredAccessoryConfigurationStatus {
        EAWiFiUnconfiguredAccessoryConfigurationStatusSuccess = 0,
        EAWiFiUnconfiguredAccessoryConfigurationStatusUserCancelledConfiguration = 1,
        EAWiFiUnconfiguredAccessoryConfigurationStatusFailed = 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "ExternalAccessory_EAWiFiUnconfiguredAccessoryBrowser")]
    #[cfg(not(any(target_os = "macos")))]
    pub struct EAWiFiUnconfiguredAccessoryBrowser;

    #[cfg(not(any(target_os = "macos")))]
    #[cfg(feature = "ExternalAccessory_EAWiFiUnconfiguredAccessoryBrowser")]
    unsafe impl ClassType for EAWiFiUnconfiguredAccessoryBrowser {
        type Super = NSObject;
    }
);

#[cfg(feature = "ExternalAccessory_EAWiFiUnconfiguredAccessoryBrowser")]
#[cfg(not(any(target_os = "macos")))]
unsafe impl NSObjectProtocol for EAWiFiUnconfiguredAccessoryBrowser {}

#[cfg(not(any(target_os = "macos")))]
extern_methods!(
    #[cfg(feature = "ExternalAccessory_EAWiFiUnconfiguredAccessoryBrowser")]
    unsafe impl EAWiFiUnconfiguredAccessoryBrowser {
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Id<ProtocolObject<dyn EAWiFiUnconfiguredAccessoryBrowserDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn EAWiFiUnconfiguredAccessoryBrowserDelegate>>,
        );

        #[cfg(all(
            feature = "ExternalAccessory_EAWiFiUnconfiguredAccessory",
            feature = "Foundation_NSSet"
        ))]
        #[method_id(@__retain_semantics Other unconfiguredAccessories)]
        pub unsafe fn unconfiguredAccessories(&self) -> Id<NSSet<EAWiFiUnconfiguredAccessory>>;

        #[cfg(feature = "Foundation_NSPredicate")]
        #[cfg(not(any(target_os = "macos", target_os = "tvos")))]
        #[method(startSearchingForUnconfiguredAccessoriesMatchingPredicate:)]
        pub unsafe fn startSearchingForUnconfiguredAccessoriesMatchingPredicate(
            &self,
            predicate: Option<&NSPredicate>,
        );

        #[cfg(not(any(target_os = "macos", target_os = "tvos")))]
        #[method(stopSearchingForUnconfiguredAccessories)]
        pub unsafe fn stopSearchingForUnconfiguredAccessories(&self);

        #[cfg(all(
            feature = "ExternalAccessory_EAWiFiUnconfiguredAccessory",
            feature = "ExternalAccessory_UIViewController"
        ))]
        #[cfg(not(any(target_os = "macos", target_os = "tvos")))]
        #[method(configureAccessory:withConfigurationUIOnViewController:)]
        pub unsafe fn configureAccessory_withConfigurationUIOnViewController(
            &self,
            accessory: &EAWiFiUnconfiguredAccessory,
            view_controller: &UIViewController,
        );
    }
);

extern_protocol!(
    pub unsafe trait EAWiFiUnconfiguredAccessoryBrowserDelegate: NSObjectProtocol {
        #[cfg(feature = "ExternalAccessory_EAWiFiUnconfiguredAccessoryBrowser")]
        #[cfg(not(any(target_os = "macos", target_os = "tvos")))]
        #[method(accessoryBrowser:didUpdateState:)]
        unsafe fn accessoryBrowser_didUpdateState(
            &self,
            browser: &EAWiFiUnconfiguredAccessoryBrowser,
            state: EAWiFiUnconfiguredAccessoryBrowserState,
        );

        #[cfg(all(
            feature = "ExternalAccessory_EAWiFiUnconfiguredAccessory",
            feature = "ExternalAccessory_EAWiFiUnconfiguredAccessoryBrowser",
            feature = "Foundation_NSSet"
        ))]
        #[cfg(not(any(target_os = "macos", target_os = "tvos")))]
        #[method(accessoryBrowser:didFindUnconfiguredAccessories:)]
        unsafe fn accessoryBrowser_didFindUnconfiguredAccessories(
            &self,
            browser: &EAWiFiUnconfiguredAccessoryBrowser,
            accessories: &NSSet<EAWiFiUnconfiguredAccessory>,
        );

        #[cfg(all(
            feature = "ExternalAccessory_EAWiFiUnconfiguredAccessory",
            feature = "ExternalAccessory_EAWiFiUnconfiguredAccessoryBrowser",
            feature = "Foundation_NSSet"
        ))]
        #[cfg(not(any(target_os = "macos", target_os = "tvos")))]
        #[method(accessoryBrowser:didRemoveUnconfiguredAccessories:)]
        unsafe fn accessoryBrowser_didRemoveUnconfiguredAccessories(
            &self,
            browser: &EAWiFiUnconfiguredAccessoryBrowser,
            accessories: &NSSet<EAWiFiUnconfiguredAccessory>,
        );

        #[cfg(all(
            feature = "ExternalAccessory_EAWiFiUnconfiguredAccessory",
            feature = "ExternalAccessory_EAWiFiUnconfiguredAccessoryBrowser"
        ))]
        #[cfg(not(any(target_os = "macos", target_os = "tvos")))]
        #[method(accessoryBrowser:didFinishConfiguringAccessory:withStatus:)]
        unsafe fn accessoryBrowser_didFinishConfiguringAccessory_withStatus(
            &self,
            browser: &EAWiFiUnconfiguredAccessoryBrowser,
            accessory: &EAWiFiUnconfiguredAccessory,
            status: EAWiFiUnconfiguredAccessoryConfigurationStatus,
        );
    }

    unsafe impl ProtocolType for dyn EAWiFiUnconfiguredAccessoryBrowserDelegate {}
);
