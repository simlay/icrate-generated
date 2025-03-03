//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::FileProvider::*;
use crate::FileProviderUI::*;
use crate::Foundation::*;

extern_static!(FPUIErrorDomain: &'static NSString);

typed_extensible_enum!(
    pub type FPUIActionIdentifier = NSString;
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum FPUIExtensionErrorCode {
        FPUIExtensionErrorCodeUserCancelled = 0,
        FPUIExtensionErrorCodeFailed = 1,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "FileProviderUI_FPUIActionExtensionContext")]
    pub struct FPUIActionExtensionContext;

    #[cfg(feature = "FileProviderUI_FPUIActionExtensionContext")]
    unsafe impl ClassType for FPUIActionExtensionContext {
        #[inherits(NSObject)]
        type Super = NSExtensionContext;
    }
);

#[cfg(feature = "FileProviderUI_FPUIActionExtensionContext")]
unsafe impl NSObjectProtocol for FPUIActionExtensionContext {}

extern_methods!(
    #[cfg(feature = "FileProviderUI_FPUIActionExtensionContext")]
    unsafe impl FPUIActionExtensionContext {
        #[method_id(@__retain_semantics Other domainIdentifier)]
        pub unsafe fn domainIdentifier(&self) -> Option<Id<NSFileProviderDomainIdentifier>>;

        #[method(completeRequest)]
        pub unsafe fn completeRequest(&self);

        #[cfg(feature = "Foundation_NSArray")]
        #[method(completeRequestReturningItems:completionHandler:)]
        pub unsafe fn completeRequestReturningItems_completionHandler(
            &self,
            items: Option<&NSArray>,
            completion_handler: Option<&Block<(Bool,), ()>>,
        );

        #[cfg(feature = "Foundation_NSError")]
        #[method(cancelRequestWithError:)]
        pub unsafe fn cancelRequestWithError(&self, error: &NSError);
    }
);
