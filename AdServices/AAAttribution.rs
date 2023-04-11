//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AdServices::*;
use crate::Foundation::*;

extern_static!(AAAttributionErrorDomain: &'static NSErrorDomain);

ns_error_enum!(
    #[underlying(NSInteger)]
    pub enum AAAttributionErrorCode {
        AAAttributionErrorCodeNetworkError = 1,
        AAAttributionErrorCodeInternalError = 2,
        AAAttributionErrorCodePlatformNotSupported = 3,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AdServices_AAAttribution")]
    /**
      @class AAAttribution

     @discussion
     This class contains a method that generates a token used to obtain the app's attribution from Apple’s Attribution Server.
    */
    pub struct AAAttribution;

    #[cfg(feature = "AdServices_AAAttribution")]
    unsafe impl ClassType for AAAttribution {
        type Super = NSObject;
    }
);

#[cfg(feature = "AdServices_AAAttribution")]
/**
  @class AAAttribution

 @discussion
 This class contains a method that generates a token used to obtain the app's attribution from Apple’s Attribution Server.
*/
unsafe impl NSObjectProtocol for AAAttribution {}

extern_methods!(
    /**
      @class AAAttribution

     @discussion
     This class contains a method that generates a token used to obtain the app's attribution from Apple’s Attribution Server.
    */
    #[cfg(feature = "AdServices_AAAttribution")]
    unsafe impl AAAttribution {
        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other attributionTokenWithError:_)]
        pub unsafe fn attributionTokenWithError() -> Result<Id<NSString>, Id<NSError>>;
    }
);
