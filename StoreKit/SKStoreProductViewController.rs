//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::StoreKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "StoreKit_SKStoreProductViewController")]
    #[cfg(not(any(target_os = "tvos")))]
    pub struct SKStoreProductViewController;

    #[cfg(not(any(target_os = "tvos")))]
    #[cfg(feature = "StoreKit_SKStoreProductViewController")]
    unsafe impl ClassType for SKStoreProductViewController {
        #[inherits(NSResponder, NSObject)]
        type Super = NSViewController;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "StoreKit_SKStoreProductViewController")]
#[cfg(not(any(target_os = "tvos")))]
unsafe impl NSCoding for SKStoreProductViewController {}

#[cfg(feature = "StoreKit_SKStoreProductViewController")]
#[cfg(not(any(target_os = "tvos")))]
unsafe impl NSEditor for SKStoreProductViewController {}

#[cfg(feature = "StoreKit_SKStoreProductViewController")]
#[cfg(not(any(target_os = "tvos")))]
unsafe impl NSObjectProtocol for SKStoreProductViewController {}

#[cfg(feature = "StoreKit_SKStoreProductViewController")]
#[cfg(not(any(target_os = "tvos")))]
unsafe impl NSSeguePerforming for SKStoreProductViewController {}

#[cfg(feature = "StoreKit_SKStoreProductViewController")]
#[cfg(not(any(target_os = "tvos")))]
unsafe impl NSUserInterfaceItemIdentification for SKStoreProductViewController {}

#[cfg(not(any(target_os = "tvos")))]
extern_methods!(
    #[cfg(feature = "StoreKit_SKStoreProductViewController")]
    #[cfg(not(any(target_os = "tvos")))]
    unsafe impl SKStoreProductViewController {
        #[cfg(not(any(target_os = "tvos")))]
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Id<ProtocolObject<dyn SKStoreProductViewControllerDelegate>>>;

        #[cfg(not(any(target_os = "tvos")))]
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn SKStoreProductViewControllerDelegate>>,
        );

        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString"
        ))]
        #[cfg(not(any(target_os = "tvos")))]
        #[method(loadProductWithParameters:completionBlock:)]
        pub unsafe fn loadProductWithParameters_completionBlock(
            &self,
            parameters: &NSDictionary<NSString, Object>,
            block: Option<&Block<(Bool, *mut NSError), ()>>,
        );

        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "StoreKit_SKAdImpression"
        ))]
        #[cfg(not(any(target_os = "macos", target_os = "tvos", target_os = "watchos")))]
        #[method(loadProductWithParameters:impression:completionBlock:)]
        pub unsafe fn loadProductWithParameters_impression_completionBlock(
            &self,
            parameters: &NSDictionary<NSString, Object>,
            impression: &SKAdImpression,
            block: Option<&Block<(Bool, *mut NSError), ()>>,
        );
    }
);

#[cfg(not(any(target_os = "tvos")))]
extern_methods!(
    /// Methods declared on superclass `NSViewController`
    #[cfg(feature = "StoreKit_SKStoreProductViewController")]
    #[cfg(not(any(target_os = "tvos")))]
    unsafe impl SKStoreProductViewController {
        #[cfg(feature = "Foundation_NSBundle")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Init initWithNibName:bundle:)]
        pub unsafe fn initWithNibName_bundle(
            this: Option<Allocated<Self>>,
            nib_name_or_nil: Option<&NSNibName>,
            nib_bundle_or_nil: Option<&NSBundle>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self>>;
    }
);

#[cfg(not(any(target_os = "tvos")))]
extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(feature = "StoreKit_SKStoreProductViewController")]
    #[cfg(not(any(target_os = "tvos")))]
    unsafe impl SKStoreProductViewController {
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

#[cfg(not(any(target_os = "tvos")))]
extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "StoreKit_SKStoreProductViewController")]
    #[cfg(not(any(target_os = "tvos")))]
    unsafe impl SKStoreProductViewController {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_protocol!(
    pub unsafe trait SKStoreProductViewControllerDelegate: NSObjectProtocol {
        #[cfg(feature = "StoreKit_SKStoreProductViewController")]
        #[cfg(not(any(target_os = "tvos")))]
        #[optional]
        #[method(productViewControllerDidFinish:)]
        unsafe fn productViewControllerDidFinish(
            &self,
            view_controller: &SKStoreProductViewController,
        );
    }

    unsafe impl ProtocolType for dyn SKStoreProductViewControllerDelegate {}
);

extern_static!(SKStoreProductParameterITunesItemIdentifier: &'static NSString);

extern_static!(SKStoreProductParameterProductIdentifier: &'static NSString);

extern_static!(SKStoreProductParameterCustomProductPageIdentifier: &'static NSString);

extern_static!(SKStoreProductParameterAffiliateToken: &'static NSString);

extern_static!(SKStoreProductParameterCampaignToken: &'static NSString);

extern_static!(SKStoreProductParameterProviderToken: &'static NSString);

extern_static!(SKStoreProductParameterAdvertisingPartnerToken: &'static NSString);
