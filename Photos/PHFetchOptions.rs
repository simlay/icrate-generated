//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::PhotoKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "PhotoKit_PHFetchOptions")]
    pub struct PHFetchOptions;

    #[cfg(feature = "PhotoKit_PHFetchOptions")]
    unsafe impl ClassType for PHFetchOptions {
        type Super = NSObject;
    }
);

#[cfg(feature = "PhotoKit_PHFetchOptions")]
unsafe impl NSObjectProtocol for PHFetchOptions {}

extern_methods!(
    #[cfg(feature = "PhotoKit_PHFetchOptions")]
    unsafe impl PHFetchOptions {
        #[cfg(feature = "Foundation_NSPredicate")]
        /**
          Some predicates / sorts may be suboptimal and we will log
        */
        #[method_id(@__retain_semantics Other predicate)]
        pub unsafe fn predicate(&self) -> Option<Id<NSPredicate>>;

        #[cfg(feature = "Foundation_NSPredicate")]
        /**
          Some predicates / sorts may be suboptimal and we will log
        */
        #[method(setPredicate:)]
        pub unsafe fn setPredicate(&self, predicate: Option<&NSPredicate>);

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSSortDescriptor"
        ))]
        #[method_id(@__retain_semantics Other sortDescriptors)]
        pub unsafe fn sortDescriptors(&self) -> Option<Id<NSArray<NSSortDescriptor>>>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSSortDescriptor"
        ))]
        #[method(setSortDescriptors:)]
        pub unsafe fn setSortDescriptors(
            &self,
            sort_descriptors: Option<&NSArray<NSSortDescriptor>>,
        );

        /**
          Whether hidden assets are included in fetch results. Defaults to NO
        */
        #[method(includeHiddenAssets)]
        pub unsafe fn includeHiddenAssets(&self) -> bool;

        /**
          Whether hidden assets are included in fetch results. Defaults to NO
        */
        #[method(setIncludeHiddenAssets:)]
        pub unsafe fn setIncludeHiddenAssets(&self, include_hidden_assets: bool);

        /**
          Whether hidden burst assets are included in fetch results. Defaults to NO
        */
        #[method(includeAllBurstAssets)]
        pub unsafe fn includeAllBurstAssets(&self) -> bool;

        /**
          Whether hidden burst assets are included in fetch results. Defaults to NO
        */
        #[method(setIncludeAllBurstAssets:)]
        pub unsafe fn setIncludeAllBurstAssets(&self, include_all_burst_assets: bool);

        /**
          The asset source types included in the fetch results.  Defaults to PHAssetSourceTypeNone.
         If set to PHAssetSourceTypeNone the asset source types included in the fetch results are inferred from the type of query performed (see PHAsset fetchAssetsWithOptions:)
        */
        #[method(includeAssetSourceTypes)]
        pub unsafe fn includeAssetSourceTypes(&self) -> PHAssetSourceType;

        /**
          The asset source types included in the fetch results.  Defaults to PHAssetSourceTypeNone.
         If set to PHAssetSourceTypeNone the asset source types included in the fetch results are inferred from the type of query performed (see PHAsset fetchAssetsWithOptions:)
        */
        #[method(setIncludeAssetSourceTypes:)]
        pub unsafe fn setIncludeAssetSourceTypes(
            &self,
            include_asset_source_types: PHAssetSourceType,
        );

        /**
          Limits the maximum number of objects returned in the fetch result, a value of 0 means no limit.  Defaults to 0.
        */
        #[method(fetchLimit)]
        pub unsafe fn fetchLimit(&self) -> NSUInteger;

        /**
          Limits the maximum number of objects returned in the fetch result, a value of 0 means no limit.  Defaults to 0.
        */
        #[method(setFetchLimit:)]
        pub unsafe fn setFetchLimit(&self, fetch_limit: NSUInteger);

        /**
          Whether the owner of this object is interested in incremental change details for the results of this fetch (see PHChange)
         Defaults to YES
        */
        #[method(wantsIncrementalChangeDetails)]
        pub unsafe fn wantsIncrementalChangeDetails(&self) -> bool;

        /**
          Whether the owner of this object is interested in incremental change details for the results of this fetch (see PHChange)
         Defaults to YES
        */
        #[method(setWantsIncrementalChangeDetails:)]
        pub unsafe fn setWantsIncrementalChangeDetails(
            &self,
            wants_incremental_change_details: bool,
        );
    }
);
