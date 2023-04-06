//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::StoreKit::*;

#[cfg(not(any(target_os = "watchos")))]
ns_closed_enum!(
    #[underlying(NSInteger)]
    pub enum SKProductStorePromotionVisibility {
        #[cfg(not(any(target_os = "watchos")))]
        SKProductStorePromotionVisibilityDefault = 0,
        #[cfg(not(any(target_os = "watchos")))]
        SKProductStorePromotionVisibilityShow = 1,
        #[cfg(not(any(target_os = "watchos")))]
        SKProductStorePromotionVisibilityHide = 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "StoreKit_SKProductStorePromotionController")]
    #[cfg(not(any(target_os = "watchos")))]
    pub struct SKProductStorePromotionController;

    #[cfg(not(any(target_os = "watchos")))]
    #[cfg(feature = "StoreKit_SKProductStorePromotionController")]
    unsafe impl ClassType for SKProductStorePromotionController {
        type Super = NSObject;
    }
);

#[cfg(feature = "StoreKit_SKProductStorePromotionController")]
#[cfg(not(any(target_os = "watchos")))]
unsafe impl NSObjectProtocol for SKProductStorePromotionController {}

extern_methods!(
    #[cfg(feature = "StoreKit_SKProductStorePromotionController")]
    #[cfg(not(any(target_os = "watchos")))]
    unsafe impl SKProductStorePromotionController {
        #[cfg(not(any(target_os = "watchos")))]
        #[cfg(not(any(target_os = "watchos")))]
        #[method_id(@__retain_semantics Other defaultController)]
        pub unsafe fn defaultController() -> Id<Self>;

        #[cfg(not(any(target_os = "watchos")))]
        #[cfg(all(feature = "Foundation_NSError", feature = "StoreKit_SKProduct"))]
        #[cfg(not(any(target_os = "watchos")))]
        #[method(fetchStorePromotionVisibilityForProduct:completionHandler:)]
        pub unsafe fn fetchStorePromotionVisibilityForProduct_completionHandler(
            &self,
            product: &SKProduct,
            completion_handler: Option<
                &Block<(SKProductStorePromotionVisibility, *mut NSError), ()>,
            >,
        );

        #[cfg(not(any(target_os = "watchos")))]
        #[cfg(all(feature = "Foundation_NSError", feature = "StoreKit_SKProduct"))]
        #[cfg(not(any(target_os = "watchos")))]
        #[method(updateStorePromotionVisibility:forProduct:completionHandler:)]
        pub unsafe fn updateStorePromotionVisibility_forProduct_completionHandler(
            &self,
            promotion_visibility: SKProductStorePromotionVisibility,
            product: &SKProduct,
            completion_handler: Option<&Block<(*mut NSError,), ()>>,
        );

        #[cfg(not(any(target_os = "watchos")))]
        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "StoreKit_SKProduct"
        ))]
        #[cfg(not(any(target_os = "watchos")))]
        #[method(fetchStorePromotionOrderWithCompletionHandler:)]
        pub unsafe fn fetchStorePromotionOrderWithCompletionHandler(
            &self,
            completion_handler: Option<&Block<(NonNull<NSArray<SKProduct>>, *mut NSError), ()>>,
        );

        #[cfg(not(any(target_os = "watchos")))]
        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "StoreKit_SKProduct"
        ))]
        #[cfg(not(any(target_os = "watchos")))]
        #[method(updateStorePromotionOrder:completionHandler:)]
        pub unsafe fn updateStorePromotionOrder_completionHandler(
            &self,
            promotion_order: &NSArray<SKProduct>,
            completion_handler: Option<&Block<(*mut NSError,), ()>>,
        );
    }
);
