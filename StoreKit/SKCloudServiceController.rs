//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::StoreKit::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum SKCloudServiceAuthorizationStatus {
        SKCloudServiceAuthorizationStatusNotDetermined = 0,
        SKCloudServiceAuthorizationStatusDenied = 1,
        SKCloudServiceAuthorizationStatusRestricted = 2,
        SKCloudServiceAuthorizationStatusAuthorized = 3,
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum SKCloudServiceCapability {
        SKCloudServiceCapabilityNone = 0,
        SKCloudServiceCapabilityMusicCatalogPlayback = 1 << 0,
        SKCloudServiceCapabilityMusicCatalogSubscriptionEligible = 1 << 1,
        SKCloudServiceCapabilityAddToCloudMusicLibrary = 1 << 8,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "StoreKit_SKCloudServiceController")]
    pub struct SKCloudServiceController;

    #[cfg(feature = "StoreKit_SKCloudServiceController")]
    unsafe impl ClassType for SKCloudServiceController {
        type Super = NSObject;
    }
);

#[cfg(feature = "StoreKit_SKCloudServiceController")]
unsafe impl NSObjectProtocol for SKCloudServiceController {}

extern_methods!(
    #[cfg(feature = "StoreKit_SKCloudServiceController")]
    unsafe impl SKCloudServiceController {
        #[method(authorizationStatus)]
        pub unsafe fn authorizationStatus() -> SKCloudServiceAuthorizationStatus;

        #[method(requestAuthorization:)]
        pub unsafe fn requestAuthorization(
            completion_handler: &Block<(SKCloudServiceAuthorizationStatus,), ()>,
        );

        #[cfg(feature = "Foundation_NSError")]
        #[method(requestCapabilitiesWithCompletionHandler:)]
        pub unsafe fn requestCapabilitiesWithCompletionHandler(
            &self,
            completion_handler: &Block<(SKCloudServiceCapability, *mut NSError), ()>,
        );

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[method(requestStorefrontCountryCodeWithCompletionHandler:)]
        pub unsafe fn requestStorefrontCountryCodeWithCompletionHandler(
            &self,
            completion_handler: &Block<(*mut NSString, *mut NSError), ()>,
        );

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[method(requestStorefrontIdentifierWithCompletionHandler:)]
        pub unsafe fn requestStorefrontIdentifierWithCompletionHandler(
            &self,
            completion_handler: &Block<(*mut NSString, *mut NSError), ()>,
        );

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[method(requestUserTokenForDeveloperToken:completionHandler:)]
        pub unsafe fn requestUserTokenForDeveloperToken_completionHandler(
            &self,
            developer_token: &NSString,
            completion_handler: &Block<(*mut NSString, *mut NSError), ()>,
        );

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[deprecated]
        #[cfg(not(any(target_os = "macos", target_os = "tvos", target_os = "watchos")))]
        #[method(requestPersonalizationTokenForClientToken:withCompletionHandler:)]
        pub unsafe fn requestPersonalizationTokenForClientToken_withCompletionHandler(
            &self,
            client_token: &NSString,
            completion_handler: &Block<(*mut NSString, *mut NSError), ()>,
        );
    }
);

extern_static!(SKCloudServiceCapabilitiesDidChangeNotification: &'static NSNotificationName);

extern_static!(SKStorefrontCountryCodeDidChangeNotification: &'static NSNotificationName);

extern_static!(SKStorefrontIdentifierDidChangeNotification: &'static NSNotificationName);
