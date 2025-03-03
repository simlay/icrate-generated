//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::PhotoKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "PhotoKit_PHAsset")]
    pub struct PHAsset;

    #[cfg(feature = "PhotoKit_PHAsset")]
    unsafe impl ClassType for PHAsset {
        #[inherits(NSObject)]
        type Super = PHObject;
    }
);

#[cfg(feature = "PhotoKit_PHAsset")]
unsafe impl NSObjectProtocol for PHAsset {}

extern_methods!(
    #[cfg(feature = "PhotoKit_PHAsset")]
    unsafe impl PHAsset {
        #[method(playbackStyle)]
        pub unsafe fn playbackStyle(&self) -> PHAssetPlaybackStyle;

        #[method(mediaType)]
        pub unsafe fn mediaType(&self) -> PHAssetMediaType;

        #[method(mediaSubtypes)]
        pub unsafe fn mediaSubtypes(&self) -> PHAssetMediaSubtype;

        #[method(pixelWidth)]
        pub unsafe fn pixelWidth(&self) -> NSUInteger;

        #[method(pixelHeight)]
        pub unsafe fn pixelHeight(&self) -> NSUInteger;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other creationDate)]
        pub unsafe fn creationDate(&self) -> Option<Id<NSDate>>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other modificationDate)]
        pub unsafe fn modificationDate(&self) -> Option<Id<NSDate>>;

        #[cfg(feature = "CoreLocation_CLLocation")]
        #[method_id(@__retain_semantics Other location)]
        pub unsafe fn location(&self) -> Option<Id<CLLocation>>;

        #[method(duration)]
        pub unsafe fn duration(&self) -> NSTimeInterval;

        #[method(isHidden)]
        pub unsafe fn isHidden(&self) -> bool;

        #[method(isFavorite)]
        pub unsafe fn isFavorite(&self) -> bool;

        #[deprecated = "No longer supported"]
        #[method(isSyncFailureHidden)]
        pub unsafe fn isSyncFailureHidden(&self) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other burstIdentifier)]
        pub unsafe fn burstIdentifier(&self) -> Option<Id<NSString>>;

        #[method(burstSelectionTypes)]
        pub unsafe fn burstSelectionTypes(&self) -> PHAssetBurstSelectionType;

        #[method(representsBurst)]
        pub unsafe fn representsBurst(&self) -> bool;

        #[method(sourceType)]
        pub unsafe fn sourceType(&self) -> PHAssetSourceType;

        #[method(hasAdjustments)]
        pub unsafe fn hasAdjustments(&self) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other adjustmentFormatIdentifier)]
        pub unsafe fn adjustmentFormatIdentifier(&self) -> Option<Id<NSString>>;

        #[method(canPerformEditOperation:)]
        pub unsafe fn canPerformEditOperation(&self, edit_operation: PHAssetEditOperation) -> bool;

        #[cfg(all(
            feature = "PhotoKit_PHAssetCollection",
            feature = "PhotoKit_PHFetchOptions",
            feature = "PhotoKit_PHFetchResult"
        ))]
        #[method_id(@__retain_semantics Other fetchAssetsInAssetCollection:options:)]
        pub unsafe fn fetchAssetsInAssetCollection_options(
            asset_collection: &PHAssetCollection,
            options: Option<&PHFetchOptions>,
        ) -> Id<PHFetchResult<PHAsset>>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSString",
            feature = "PhotoKit_PHFetchOptions",
            feature = "PhotoKit_PHFetchResult"
        ))]
        #[method_id(@__retain_semantics Other fetchAssetsWithLocalIdentifiers:options:)]
        pub unsafe fn fetchAssetsWithLocalIdentifiers_options(
            identifiers: &NSArray<NSString>,
            options: Option<&PHFetchOptions>,
        ) -> Id<PHFetchResult<PHAsset>>;

        #[cfg(all(
            feature = "PhotoKit_PHAssetCollection",
            feature = "PhotoKit_PHFetchOptions",
            feature = "PhotoKit_PHFetchResult"
        ))]
        #[method_id(@__retain_semantics Other fetchKeyAssetsInAssetCollection:options:)]
        pub unsafe fn fetchKeyAssetsInAssetCollection_options(
            asset_collection: &PHAssetCollection,
            options: Option<&PHFetchOptions>,
        ) -> Option<Id<PHFetchResult<PHAsset>>>;

        #[cfg(all(
            feature = "Foundation_NSString",
            feature = "PhotoKit_PHFetchOptions",
            feature = "PhotoKit_PHFetchResult"
        ))]
        #[method_id(@__retain_semantics Other fetchAssetsWithBurstIdentifier:options:)]
        pub unsafe fn fetchAssetsWithBurstIdentifier_options(
            burst_identifier: &NSString,
            options: Option<&PHFetchOptions>,
        ) -> Id<PHFetchResult<PHAsset>>;

        #[cfg(all(
            feature = "PhotoKit_PHFetchOptions",
            feature = "PhotoKit_PHFetchResult"
        ))]
        #[method_id(@__retain_semantics Other fetchAssetsWithOptions:)]
        pub unsafe fn fetchAssetsWithOptions(
            options: Option<&PHFetchOptions>,
        ) -> Id<PHFetchResult<PHAsset>>;

        #[cfg(all(
            feature = "PhotoKit_PHFetchOptions",
            feature = "PhotoKit_PHFetchResult"
        ))]
        #[method_id(@__retain_semantics Other fetchAssetsWithMediaType:options:)]
        pub unsafe fn fetchAssetsWithMediaType_options(
            media_type: PHAssetMediaType,
            options: Option<&PHFetchOptions>,
        ) -> Id<PHFetchResult<PHAsset>>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSURL",
            feature = "PhotoKit_PHFetchOptions",
            feature = "PhotoKit_PHFetchResult"
        ))]
        #[deprecated = "Will be removed in a future release"]
        #[method_id(@__retain_semantics Other fetchAssetsWithALAssetURLs:options:)]
        pub unsafe fn fetchAssetsWithALAssetURLs_options(
            asset_ur_ls: &NSArray<NSURL>,
            options: Option<&PHFetchOptions>,
        ) -> Id<PHFetchResult<PHAsset>>;
    }
);
