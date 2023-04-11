//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::PhotoKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "PhotoKit_PHCollectionListChangeRequest")]
    /**
      PHCollectionListChangeRequest can only be created or used within a -[PHPhotoLibrary performChanges:] or -[PHPhotoLibrary performChangesAndWait:] block.
    */
    pub struct PHCollectionListChangeRequest;

    #[cfg(feature = "PhotoKit_PHCollectionListChangeRequest")]
    unsafe impl ClassType for PHCollectionListChangeRequest {
        #[inherits(NSObject)]
        type Super = PHChangeRequest;
    }
);

#[cfg(feature = "PhotoKit_PHCollectionListChangeRequest")]
/**
  PHCollectionListChangeRequest can only be created or used within a -[PHPhotoLibrary performChanges:] or -[PHPhotoLibrary performChangesAndWait:] block.
*/
unsafe impl NSObjectProtocol for PHCollectionListChangeRequest {}

extern_methods!(
    /**
      PHCollectionListChangeRequest can only be created or used within a -[PHPhotoLibrary performChanges:] or -[PHPhotoLibrary performChangesAndWait:] block.
    */
    #[cfg(feature = "PhotoKit_PHCollectionListChangeRequest")]
    unsafe impl PHCollectionListChangeRequest {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other creationRequestForCollectionListWithTitle:)]
        pub unsafe fn creationRequestForCollectionListWithTitle(title: &NSString) -> Id<Self>;

        #[cfg(feature = "PhotoKit_PHObjectPlaceholder")]
        /**
          This can be used to fetch the newly created collection list after the change block has completed by using -localIdentifier
         It can also be added directly to collection lists within the current change block
        */
        #[method_id(@__retain_semantics Other placeholderForCreatedCollectionList)]
        pub unsafe fn placeholderForCreatedCollectionList(&self) -> Id<PHObjectPlaceholder>;

        #[method(deleteCollectionLists:)]
        pub unsafe fn deleteCollectionLists(
            collection_lists: &ProtocolObject<dyn NSFastEnumeration>,
        );

        #[cfg(feature = "PhotoKit_PHCollectionList")]
        #[method_id(@__retain_semantics Other changeRequestForCollectionList:)]
        pub unsafe fn changeRequestForCollectionList(
            collection_list: &PHCollectionList,
        ) -> Option<Id<Self>>;

        #[cfg(all(
            feature = "PhotoKit_PHCollection",
            feature = "PhotoKit_PHCollectionList",
            feature = "PhotoKit_PHFetchResult"
        ))]
        #[method_id(@__retain_semantics Other changeRequestForCollectionList:childCollections:)]
        pub unsafe fn changeRequestForCollectionList_childCollections(
            collection_list: &PHCollectionList,
            child_collections: &PHFetchResult<PHCollection>,
        ) -> Option<Id<Self>>;

        #[cfg(all(feature = "PhotoKit_PHCollection", feature = "PhotoKit_PHFetchResult"))]
        #[method_id(@__retain_semantics Other changeRequestForTopLevelCollectionListUserCollections:)]
        pub unsafe fn changeRequestForTopLevelCollectionListUserCollections(
            child_collections: &PHFetchResult<PHCollection>,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: &NSString);

        #[method(addChildCollections:)]
        pub unsafe fn addChildCollections(
            &self,
            collections: &ProtocolObject<dyn NSFastEnumeration>,
        );

        #[cfg(feature = "Foundation_NSIndexSet")]
        #[method(insertChildCollections:atIndexes:)]
        pub unsafe fn insertChildCollections_atIndexes(
            &self,
            collections: &ProtocolObject<dyn NSFastEnumeration>,
            indexes: &NSIndexSet,
        );

        #[method(removeChildCollections:)]
        pub unsafe fn removeChildCollections(
            &self,
            collections: &ProtocolObject<dyn NSFastEnumeration>,
        );

        #[cfg(feature = "Foundation_NSIndexSet")]
        #[method(removeChildCollectionsAtIndexes:)]
        pub unsafe fn removeChildCollectionsAtIndexes(&self, indexes: &NSIndexSet);

        #[cfg(feature = "Foundation_NSIndexSet")]
        #[method(replaceChildCollectionsAtIndexes:withChildCollections:)]
        pub unsafe fn replaceChildCollectionsAtIndexes_withChildCollections(
            &self,
            indexes: &NSIndexSet,
            collections: &ProtocolObject<dyn NSFastEnumeration>,
        );

        #[cfg(feature = "Foundation_NSIndexSet")]
        #[method(moveChildCollectionsAtIndexes:toIndex:)]
        pub unsafe fn moveChildCollectionsAtIndexes_toIndex(
            &self,
            indexes: &NSIndexSet,
            to_index: NSUInteger,
        );
    }
);
