//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::FileProvider::*;
use crate::Foundation::*;
use crate::UniformTypeIdentifiers::*;

extern_protocol!(
    pub unsafe trait NSFileProviderServiceSource {
        /**
         The service name that uniquely identifies the service (using reverse domain
        name notation for you service name is recommended).
        */
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

        /**
         Indicates whether access to the service is restricted.

        A restricted service can only be accessed by processes that can manage the domain the service is attached to. It is only accessible
        through `-[NSFileProviderManager getServiceWithName:itemIdentifier:completionHandler:]`
        */
        #[optional]
        #[method(isRestricted)]
        unsafe fn isRestricted(&self) -> bool;
    }

    unsafe impl ProtocolType for dyn NSFileProviderServiceSource {}
);

extern_methods!(
    /**
     A file provider can override the method in this category to return service
    sources that provide custom communication channels to client applications.
    The service sources must be tied to the item identified by @c itemIdentifier.
    Client applications can retrieve the list of supported services by calling
    @c -[NSFileManager getFileProviderServicesForItemAtURL:] for a specific item URL.
    */
    /// NSFileProviderService
    #[cfg(feature = "FileProvider_NSFileProviderExtension")]
    unsafe impl NSFileProviderExtension {
        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSError"))]
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
