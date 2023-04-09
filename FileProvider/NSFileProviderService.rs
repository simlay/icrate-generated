//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::FileProvider::*;
use crate::Foundation::*;
use crate::UniformTypeIdentifiers::*;

extern_protocol!(
    pub unsafe trait NSFileProviderServiceSource {
        #[method_id(@__retain_semantics Other serviceName)]
        unsafe fn serviceName(&self) -> Id<NSFileProviderServiceName>;

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSXPCListenerEndpoint"
        ))]
        #[method_id(@__retain_semantics Other makeListenerEndpointAndReturnError:_)]
        unsafe fn makeListenerEndpointAndReturnError(
            &self,
        ) -> Result<Id<NSXPCListenerEndpoint>, Id<NSError>>;

        #[optional]
        #[method(isRestricted)]
        unsafe fn isRestricted(&self) -> bool;
    }

    unsafe impl ProtocolType for dyn NSFileProviderServiceSource {}
);

#[cfg(not(any(target_os = "macos")))]
extern_methods!(
    /// NSFileProviderService
    #[cfg(feature = "FileProvider_NSFileProviderExtension")]
    unsafe impl NSFileProviderExtension {
        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSError"))]
        #[cfg(not(any(target_os = "macos")))]
        #[method_id(@__retain_semantics Other supportedServiceSourcesForItemIdentifier:error:_)]
        pub unsafe fn supportedServiceSourcesForItemIdentifier_error(
            &self,
            item_identifier: &NSFileProviderItemIdentifier,
        ) -> Result<Id<NSArray<ProtocolObject<dyn NSFileProviderServiceSource>>>, Id<NSError>>;
    }
);

extern_methods!(
    /// NSFileProviderService
    #[cfg(feature = "FileProvider_NSFileProviderManager")]
    unsafe impl NSFileProviderManager {
        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSFileProviderService"
        ))]
        #[method(getServiceWithName:itemIdentifier:completionHandler:)]
        pub unsafe fn getServiceWithName_itemIdentifier_completionHandler(
            &self,
            service_name: &NSFileProviderServiceName,
            item_identifier: &NSFileProviderItemIdentifier,
            completion_handler: &Block<(*mut NSFileProviderService, *mut NSError), ()>,
        );
    }
);
