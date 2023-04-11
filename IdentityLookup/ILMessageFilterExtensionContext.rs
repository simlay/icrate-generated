//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::IdentityLookup::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "IdentityLookup_ILMessageFilterExtensionContext")]
    /**
      Represents a MessageFilter extension request's context.
    */
    pub struct ILMessageFilterExtensionContext;

    #[cfg(feature = "IdentityLookup_ILMessageFilterExtensionContext")]
    unsafe impl ClassType for ILMessageFilterExtensionContext {
        #[inherits(NSObject)]
        type Super = NSExtensionContext;
    }
);

#[cfg(feature = "IdentityLookup_ILMessageFilterExtensionContext")]
/**
  Represents a MessageFilter extension request's context.
*/
unsafe impl NSObjectProtocol for ILMessageFilterExtensionContext {}

extern_methods!(
    /**
      Represents a MessageFilter extension request's context.
    */
    #[cfg(feature = "IdentityLookup_ILMessageFilterExtensionContext")]
    unsafe impl ILMessageFilterExtensionContext {
        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "IdentityLookup_ILNetworkResponse"
        ))]
        #[method(deferQueryRequestToNetworkWithCompletion:)]
        pub unsafe fn deferQueryRequestToNetworkWithCompletion(
            &self,
            completion: &Block<(*mut ILNetworkResponse, *mut NSError), ()>,
        );
    }
);
