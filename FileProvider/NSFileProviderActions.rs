//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::FileProvider::*;
use crate::Foundation::*;
use crate::UniformTypeIdentifiers::*;

typed_extensible_enum!(
    pub type NSFileProviderExtensionActionIdentifier = NSString;
);

extern_methods!(
    /// NSFileProviderActions
    #[cfg(feature = "FileProvider_NSFileProviderExtension")]
    #[cfg(not(any(target_os = "macos")))]
    unsafe impl NSFileProviderExtension {
        #[cfg(not(any(target_os = "macos")))]
        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
        #[method(importDocumentAtURL:toParentItemIdentifier:completionHandler:)]
        pub unsafe fn importDocumentAtURL_toParentItemIdentifier_completionHandler(
            &self,
            file_url: &NSURL,
            parent_item_identifier: &NSFileProviderItemIdentifier,
            completion_handler: &Block<(*mut NSFileProviderItem, *mut NSError), ()>,
        );

        #[cfg(not(any(target_os = "macos")))]
        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[method(createDirectoryWithName:inParentItemIdentifier:completionHandler:)]
        pub unsafe fn createDirectoryWithName_inParentItemIdentifier_completionHandler(
            &self,
            directory_name: &NSString,
            parent_item_identifier: &NSFileProviderItemIdentifier,
            completion_handler: &Block<(*mut NSFileProviderItem, *mut NSError), ()>,
        );

        #[cfg(not(any(target_os = "macos")))]
        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[method(renameItemWithIdentifier:toName:completionHandler:)]
        pub unsafe fn renameItemWithIdentifier_toName_completionHandler(
            &self,
            item_identifier: &NSFileProviderItemIdentifier,
            item_name: &NSString,
            completion_handler: &Block<(*mut NSFileProviderItem, *mut NSError), ()>,
        );

        #[cfg(not(any(target_os = "macos")))]
        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[method(reparentItemWithIdentifier:toParentItemWithIdentifier:newName:completionHandler:)]
        pub unsafe fn reparentItemWithIdentifier_toParentItemWithIdentifier_newName_completionHandler(
            &self,
            item_identifier: &NSFileProviderItemIdentifier,
            parent_item_identifier: &NSFileProviderItemIdentifier,
            new_name: Option<&NSString>,
            completion_handler: &Block<(*mut NSFileProviderItem, *mut NSError), ()>,
        );

        #[cfg(not(any(target_os = "macos")))]
        #[cfg(feature = "Foundation_NSError")]
        #[method(trashItemWithIdentifier:completionHandler:)]
        pub unsafe fn trashItemWithIdentifier_completionHandler(
            &self,
            item_identifier: &NSFileProviderItemIdentifier,
            completion_handler: &Block<(*mut NSFileProviderItem, *mut NSError), ()>,
        );

        #[cfg(not(any(target_os = "macos")))]
        #[cfg(feature = "Foundation_NSError")]
        #[method(untrashItemWithIdentifier:toParentItemIdentifier:completionHandler:)]
        pub unsafe fn untrashItemWithIdentifier_toParentItemIdentifier_completionHandler(
            &self,
            item_identifier: &NSFileProviderItemIdentifier,
            parent_item_identifier: Option<&NSFileProviderItemIdentifier>,
            completion_handler: &Block<(*mut NSFileProviderItem, *mut NSError), ()>,
        );

        #[cfg(not(any(target_os = "macos")))]
        #[cfg(feature = "Foundation_NSError")]
        #[method(deleteItemWithIdentifier:completionHandler:)]
        pub unsafe fn deleteItemWithIdentifier_completionHandler(
            &self,
            item_identifier: &NSFileProviderItemIdentifier,
            completion_handler: &Block<(*mut NSError,), ()>,
        );

        #[cfg(not(any(target_os = "macos")))]
        #[cfg(all(feature = "Foundation_NSDate", feature = "Foundation_NSError"))]
        #[method(setLastUsedDate:forItemIdentifier:completionHandler:)]
        pub unsafe fn setLastUsedDate_forItemIdentifier_completionHandler(
            &self,
            last_used_date: Option<&NSDate>,
            item_identifier: &NSFileProviderItemIdentifier,
            completion_handler: &Block<(*mut NSFileProviderItem, *mut NSError), ()>,
        );

        #[cfg(not(any(target_os = "macos")))]
        #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSError"))]
        #[method(setTagData:forItemIdentifier:completionHandler:)]
        pub unsafe fn setTagData_forItemIdentifier_completionHandler(
            &self,
            tag_data: Option<&NSData>,
            item_identifier: &NSFileProviderItemIdentifier,
            completion_handler: &Block<(*mut NSFileProviderItem, *mut NSError), ()>,
        );

        #[cfg(not(any(target_os = "macos")))]
        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSNumber"))]
        #[method(setFavoriteRank:forItemIdentifier:completionHandler:)]
        pub unsafe fn setFavoriteRank_forItemIdentifier_completionHandler(
            &self,
            favorite_rank: Option<&NSNumber>,
            item_identifier: &NSFileProviderItemIdentifier,
            completion_handler: &Block<(*mut NSFileProviderItem, *mut NSError), ()>,
        );
    }
);
