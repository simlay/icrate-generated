//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::FileProvider::*;
use crate::Foundation::*;
use crate::UniformTypeIdentifiers::*;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSFileProviderCreateItemOptions {
        NSFileProviderCreateItemMayAlreadyExist = 1 << 0,
        NSFileProviderCreateItemDeletionConflicted = 1 << 1,
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSFileProviderDeleteItemOptions {
        NSFileProviderDeleteItemRecursive = 1 << 0,
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    #[cfg(not(any(target_os = "ios")))]
    pub enum NSFileProviderMaterializationFlags {
        #[cfg(not(any(target_os = "ios")))]
        NSFileProviderMaterializationFlagsKnownSparseRanges = 1 << 0,
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    #[cfg(not(any(target_os = "ios")))]
    pub enum NSFileProviderFetchContentsOptions {
        #[cfg(not(any(target_os = "ios")))]
        NSFileProviderFetchContentsOptionsStrictVersioning = 1 << 0,
    }
);

extern_protocol!(
    pub unsafe trait NSFileProviderEnumerating: NSObjectProtocol {
        #[cfg(all(
            feature = "FileProvider_NSFileProviderRequest",
            feature = "Foundation_NSError"
        ))]
        #[method_id(@__retain_semantics Other enumeratorForContainerItemIdentifier:request:error:_)]
        unsafe fn enumeratorForContainerItemIdentifier_request_error(
            &self,
            container_item_identifier: &NSFileProviderItemIdentifier,
            request: &NSFileProviderRequest,
        ) -> Result<Id<ProtocolObject<dyn NSFileProviderEnumerator>>, Id<NSError>>;
    }

    unsafe impl ProtocolType for dyn NSFileProviderEnumerating {}
);

extern_protocol!(
    pub unsafe trait NSFileProviderReplicatedExtension:
        NSFileProviderEnumerating + NSObjectProtocol
    {
        #[cfg(feature = "FileProvider_NSFileProviderDomain")]
        #[method_id(@__retain_semantics Init initWithDomain:)]
        unsafe fn initWithDomain(
            this: Option<Allocated<Self>>,
            domain: &NSFileProviderDomain,
        ) -> Id<Self>;

        #[method(invalidate)]
        unsafe fn invalidate(&self);

        #[cfg(all(
            feature = "FileProvider_NSFileProviderRequest",
            feature = "Foundation_NSError",
            feature = "Foundation_NSProgress"
        ))]
        #[method_id(@__retain_semantics Other itemForIdentifier:request:completionHandler:)]
        unsafe fn itemForIdentifier_request_completionHandler(
            &self,
            identifier: &NSFileProviderItemIdentifier,
            request: &NSFileProviderRequest,
            completion_handler: &Block<(*mut NSFileProviderItem, *mut NSError), ()>,
        ) -> Id<NSProgress>;

        #[cfg(all(
            feature = "FileProvider_NSFileProviderItemVersion",
            feature = "FileProvider_NSFileProviderRequest",
            feature = "Foundation_NSError",
            feature = "Foundation_NSProgress",
            feature = "Foundation_NSURL"
        ))]
        #[method_id(@__retain_semantics Other fetchContentsForItemWithIdentifier:version:request:completionHandler:)]
        unsafe fn fetchContentsForItemWithIdentifier_version_request_completionHandler(
            &self,
            item_identifier: &NSFileProviderItemIdentifier,
            requested_version: Option<&NSFileProviderItemVersion>,
            request: &NSFileProviderRequest,
            completion_handler: &Block<(*mut NSURL, *mut NSFileProviderItem, *mut NSError), ()>,
        ) -> Id<NSProgress>;

        #[cfg(all(
            feature = "FileProvider_NSFileProviderRequest",
            feature = "Foundation_NSError",
            feature = "Foundation_NSProgress",
            feature = "Foundation_NSURL"
        ))]
        #[method_id(@__retain_semantics Other createItemBasedOnTemplate:fields:contents:options:request:completionHandler:)]
        unsafe fn createItemBasedOnTemplate_fields_contents_options_request_completionHandler(
            &self,
            item_template: &NSFileProviderItem,
            fields: NSFileProviderItemFields,
            url: Option<&NSURL>,
            options: NSFileProviderCreateItemOptions,
            request: &NSFileProviderRequest,
            completion_handler: &Block<
                (
                    *mut NSFileProviderItem,
                    NSFileProviderItemFields,
                    Bool,
                    *mut NSError,
                ),
                (),
            >,
        ) -> Id<NSProgress>;

        #[cfg(all(
            feature = "FileProvider_NSFileProviderItemVersion",
            feature = "FileProvider_NSFileProviderRequest",
            feature = "Foundation_NSError",
            feature = "Foundation_NSProgress",
            feature = "Foundation_NSURL"
        ))]
        #[method_id(@__retain_semantics Other modifyItem:baseVersion:changedFields:contents:options:request:completionHandler:)]
        unsafe fn modifyItem_baseVersion_changedFields_contents_options_request_completionHandler(
            &self,
            item: &NSFileProviderItem,
            version: &NSFileProviderItemVersion,
            changed_fields: NSFileProviderItemFields,
            new_contents: Option<&NSURL>,
            options: NSFileProviderModifyItemOptions,
            request: &NSFileProviderRequest,
            completion_handler: &Block<
                (
                    *mut NSFileProviderItem,
                    NSFileProviderItemFields,
                    Bool,
                    *mut NSError,
                ),
                (),
            >,
        ) -> Id<NSProgress>;

        #[cfg(all(
            feature = "FileProvider_NSFileProviderItemVersion",
            feature = "FileProvider_NSFileProviderRequest",
            feature = "Foundation_NSError",
            feature = "Foundation_NSProgress"
        ))]
        #[method_id(@__retain_semantics Other deleteItemWithIdentifier:baseVersion:options:request:completionHandler:)]
        unsafe fn deleteItemWithIdentifier_baseVersion_options_request_completionHandler(
            &self,
            identifier: &NSFileProviderItemIdentifier,
            version: &NSFileProviderItemVersion,
            options: NSFileProviderDeleteItemOptions,
            request: &NSFileProviderRequest,
            completion_handler: &Block<(*mut NSError,), ()>,
        ) -> Id<NSProgress>;

        #[optional]
        #[method(importDidFinishWithCompletionHandler:)]
        unsafe fn importDidFinishWithCompletionHandler(&self, completion_handler: &Block<(), ()>);

        #[optional]
        #[method(materializedItemsDidChangeWithCompletionHandler:)]
        unsafe fn materializedItemsDidChangeWithCompletionHandler(
            &self,
            completion_handler: &Block<(), ()>,
        );

        #[optional]
        #[method(pendingItemsDidChangeWithCompletionHandler:)]
        unsafe fn pendingItemsDidChangeWithCompletionHandler(
            &self,
            completion_handler: &Block<(), ()>,
        );
    }

    unsafe impl ProtocolType for dyn NSFileProviderReplicatedExtension {}
);

extern_protocol!(
    pub unsafe trait NSFileProviderIncrementalContentFetching: NSObjectProtocol {
        #[cfg(all(
            feature = "FileProvider_NSFileProviderItemVersion",
            feature = "FileProvider_NSFileProviderRequest",
            feature = "Foundation_NSError",
            feature = "Foundation_NSProgress",
            feature = "Foundation_NSURL"
        ))]
        #[method_id(@__retain_semantics Other fetchContentsForItemWithIdentifier:version:usingExistingContentsAtURL:existingVersion:request:completionHandler:)]
        unsafe fn fetchContentsForItemWithIdentifier_version_usingExistingContentsAtURL_existingVersion_request_completionHandler(
            &self,
            item_identifier: &NSFileProviderItemIdentifier,
            requested_version: Option<&NSFileProviderItemVersion>,
            existing_contents: &NSURL,
            existing_version: &NSFileProviderItemVersion,
            request: &NSFileProviderRequest,
            completion_handler: &Block<(*mut NSURL, *mut NSFileProviderItem, *mut NSError), ()>,
        ) -> Id<NSProgress>;
    }

    unsafe impl ProtocolType for dyn NSFileProviderIncrementalContentFetching {}
);

extern_protocol!(
    pub unsafe trait NSFileProviderServicing: NSObjectProtocol {
        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "Foundation_NSProgress"
        ))]
        #[method_id(@__retain_semantics Other supportedServiceSourcesForItemIdentifier:completionHandler:)]
        unsafe fn supportedServiceSourcesForItemIdentifier_completionHandler(
            &self,
            item_identifier: &NSFileProviderItemIdentifier,
            completion_handler: &Block<
                (
                    *mut NSArray<ProtocolObject<dyn NSFileProviderServiceSource>>,
                    *mut NSError,
                ),
                (),
            >,
        ) -> Id<NSProgress>;
    }

    unsafe impl ProtocolType for dyn NSFileProviderServicing {}
);

extern_protocol!(
    pub unsafe trait NSFileProviderThumbnailing: NSObjectProtocol {
        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSData",
            feature = "Foundation_NSError",
            feature = "Foundation_NSProgress"
        ))]
        #[method_id(@__retain_semantics Other fetchThumbnailsForItemIdentifiers:requestedSize:perThumbnailCompletionHandler:completionHandler:)]
        unsafe fn fetchThumbnailsForItemIdentifiers_requestedSize_perThumbnailCompletionHandler_completionHandler(
            &self,
            item_identifiers: &NSArray<NSFileProviderItemIdentifier>,
            size: CGSize,
            per_thumbnail_completion_handler: &Block<
                (
                    NonNull<NSFileProviderItemIdentifier>,
                    *mut NSData,
                    *mut NSError,
                ),
                (),
            >,
            completion_handler: &Block<(*mut NSError,), ()>,
        ) -> Id<NSProgress>;
    }

    unsafe impl ProtocolType for dyn NSFileProviderThumbnailing {}
);

extern_protocol!(
    pub unsafe trait NSFileProviderCustomAction: NSObjectProtocol {
        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "Foundation_NSProgress"
        ))]
        #[method_id(@__retain_semantics Other performActionWithIdentifier:onItemsWithIdentifiers:completionHandler:)]
        unsafe fn performActionWithIdentifier_onItemsWithIdentifiers_completionHandler(
            &self,
            action_identifier: &NSFileProviderExtensionActionIdentifier,
            item_identifiers: &NSArray<NSFileProviderItemIdentifier>,
            completion_handler: &Block<(*mut NSError,), ()>,
        ) -> Id<NSProgress>;
    }

    unsafe impl ProtocolType for dyn NSFileProviderCustomAction {}
);

extern_protocol!(
    #[cfg(not(any(target_os = "ios")))]
    pub unsafe trait NSFileProviderUserInteractionSuppressing: NSObjectProtocol {
        #[cfg(feature = "Foundation_NSString")]
        #[method(setInteractionSuppressed:forIdentifier:)]
        unsafe fn setInteractionSuppressed_forIdentifier(
            &self,
            suppression: bool,
            suppression_identifier: &NSString,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method(isInteractionSuppressedForIdentifier:)]
        unsafe fn isInteractionSuppressedForIdentifier(
            &self,
            suppression_identifier: &NSString,
        ) -> bool;
    }

    #[cfg(not(any(target_os = "ios")))]
    unsafe impl ProtocolType for dyn NSFileProviderUserInteractionSuppressing {}
);

extern_protocol!(
    pub unsafe trait NSFileProviderDomainState: NSObjectProtocol {
        #[cfg(feature = "FileProvider_NSFileProviderDomainVersion")]
        #[method_id(@__retain_semantics Other domainVersion)]
        unsafe fn domainVersion(&self) -> Id<NSFileProviderDomainVersion>;

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method_id(@__retain_semantics Other userInfo)]
        unsafe fn userInfo(&self) -> Id<NSDictionary>;
    }

    unsafe impl ProtocolType for dyn NSFileProviderDomainState {}
);

extern_protocol!(
    #[cfg(not(any(target_os = "ios")))]
    pub unsafe trait NSFileProviderPartialContentFetching: NSObjectProtocol {
        #[cfg(all(
            feature = "FileProvider_NSFileProviderItemVersion",
            feature = "FileProvider_NSFileProviderRequest",
            feature = "Foundation_NSError",
            feature = "Foundation_NSProgress",
            feature = "Foundation_NSURL"
        ))]
        #[method_id(@__retain_semantics Other fetchPartialContentsForItemWithIdentifier:version:request:minimalRange:aligningTo:options:completionHandler:)]
        unsafe fn fetchPartialContentsForItemWithIdentifier_version_request_minimalRange_aligningTo_options_completionHandler(
            &self,
            item_identifier: &NSFileProviderItemIdentifier,
            requested_version: &NSFileProviderItemVersion,
            request: &NSFileProviderRequest,
            requested_range: NSRange,
            alignment: NSUInteger,
            options: NSFileProviderFetchContentsOptions,
            completion_handler: &Block<
                (
                    *mut NSURL,
                    *mut NSFileProviderItem,
                    NSRange,
                    NSFileProviderMaterializationFlags,
                    *mut NSError,
                ),
                (),
            >,
        ) -> Id<NSProgress>;
    }

    #[cfg(not(any(target_os = "ios")))]
    unsafe impl ProtocolType for dyn NSFileProviderPartialContentFetching {}
);
