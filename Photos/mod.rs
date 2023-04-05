//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#![allow(unused_imports)]
#![allow(deprecated)]
#[path = "PHAdjustmentData.rs"]
mod __PHAdjustmentData;
#[path = "PHAsset.rs"]
mod __PHAsset;
#[path = "PHAssetChangeRequest.rs"]
mod __PHAssetChangeRequest;
#[path = "PHAssetCollectionChangeRequest.rs"]
mod __PHAssetCollectionChangeRequest;
#[path = "PHAssetCreationRequest.rs"]
mod __PHAssetCreationRequest;
#[path = "PHAssetResource.rs"]
mod __PHAssetResource;
#[path = "PHAssetResourceManager.rs"]
mod __PHAssetResourceManager;
#[path = "PHChange.rs"]
mod __PHChange;
#[path = "PHChangeRequest.rs"]
mod __PHChangeRequest;
#[path = "PHCloudIdentifier.rs"]
mod __PHCloudIdentifier;
#[path = "PHCollection.rs"]
mod __PHCollection;
#[path = "PHCollectionListChangeRequest.rs"]
mod __PHCollectionListChangeRequest;
#[path = "PHContentEditingInput.rs"]
mod __PHContentEditingInput;
#[path = "PHContentEditingOutput.rs"]
mod __PHContentEditingOutput;
#[path = "PHError.rs"]
mod __PHError;
#[path = "PHFetchOptions.rs"]
mod __PHFetchOptions;
#[path = "PHFetchResult.rs"]
mod __PHFetchResult;
#[path = "PHImageManager.rs"]
mod __PHImageManager;
#[path = "PHLivePhoto.rs"]
mod __PHLivePhoto;
#[path = "PHLivePhotoEditingContext.rs"]
mod __PHLivePhotoEditingContext;
#[path = "PHObject.rs"]
mod __PHObject;
#[path = "PHPersistentChange.rs"]
mod __PHPersistentChange;
#[path = "PHPersistentChangeFetchResult.rs"]
mod __PHPersistentChangeFetchResult;
#[path = "PHPersistentChangeToken.rs"]
mod __PHPersistentChangeToken;
#[path = "PHPersistentObjectChangeDetails.rs"]
mod __PHPersistentObjectChangeDetails;
#[path = "PHPhotoLibrary.rs"]
mod __PHPhotoLibrary;
#[path = "PHProject.rs"]
mod __PHProject;
#[path = "PHProjectChangeRequest.rs"]
mod __PHProjectChangeRequest;
#[path = "PhotosTypes.rs"]
mod __PhotosTypes;

#[cfg(feature = "PhotoKit_PHAdjustmentData")]
pub use self::__PHAdjustmentData::PHAdjustmentData;
#[cfg(feature = "PhotoKit_PHAsset")]
pub use self::__PHAsset::PHAsset;
#[cfg(feature = "PhotoKit_PHAssetChangeRequest")]
pub use self::__PHAssetChangeRequest::PHAssetChangeRequest;
pub use self::__PHAssetChangeRequest::PHContentEditingInputCancelledKey;
pub use self::__PHAssetChangeRequest::PHContentEditingInputErrorKey;
pub use self::__PHAssetChangeRequest::PHContentEditingInputRequestID;
#[cfg(feature = "PhotoKit_PHContentEditingInputRequestOptions")]
pub use self::__PHAssetChangeRequest::PHContentEditingInputRequestOptions;
pub use self::__PHAssetChangeRequest::PHContentEditingInputResultIsInCloudKey;
#[cfg(feature = "PhotoKit_PHAssetCollectionChangeRequest")]
pub use self::__PHAssetCollectionChangeRequest::PHAssetCollectionChangeRequest;
#[cfg(feature = "PhotoKit_PHAssetCreationRequest")]
pub use self::__PHAssetCreationRequest::PHAssetCreationRequest;
#[cfg(feature = "PhotoKit_PHAssetResourceCreationOptions")]
pub use self::__PHAssetCreationRequest::PHAssetResourceCreationOptions;
#[cfg(feature = "PhotoKit_PHAssetResource")]
pub use self::__PHAssetResource::PHAssetResource;
pub use self::__PHAssetResourceManager::PHAssetResourceDataRequestID;
#[cfg(feature = "PhotoKit_PHAssetResourceManager")]
pub use self::__PHAssetResourceManager::PHAssetResourceManager;
pub use self::__PHAssetResourceManager::PHAssetResourceProgressHandler;
#[cfg(feature = "PhotoKit_PHAssetResourceRequestOptions")]
pub use self::__PHAssetResourceManager::PHAssetResourceRequestOptions;
pub use self::__PHAssetResourceManager::PHInvalidAssetResourceDataRequestID;
#[cfg(feature = "PhotoKit_PHChange")]
pub use self::__PHChange::PHChange;
#[cfg(feature = "PhotoKit_PHFetchResultChangeDetails")]
pub use self::__PHChange::PHFetchResultChangeDetails;
#[cfg(feature = "PhotoKit_PHObjectChangeDetails")]
pub use self::__PHChange::PHObjectChangeDetails;
#[cfg(feature = "PhotoKit_PHChangeRequest")]
pub use self::__PHChangeRequest::PHChangeRequest;
#[cfg(feature = "PhotoKit_PHCloudIdentifier")]
pub use self::__PHCloudIdentifier::PHCloudIdentifier;
#[cfg(feature = "PhotoKit_PHCloudIdentifierMapping")]
pub use self::__PHCloudIdentifier::PHCloudIdentifierMapping;
#[cfg(feature = "PhotoKit_PHLocalIdentifierMapping")]
pub use self::__PHCloudIdentifier::PHLocalIdentifierMapping;
#[cfg(not(any(target_os = "ios", target_os = "tvos")))]
pub use self::__PHCloudIdentifier::PHLocalIdentifierNotFound;
#[cfg(feature = "PhotoKit_PHAssetCollection")]
pub use self::__PHCollection::PHAssetCollection;
#[cfg(feature = "PhotoKit_PHCollection")]
pub use self::__PHCollection::PHCollection;
#[cfg(feature = "PhotoKit_PHCollectionList")]
pub use self::__PHCollection::PHCollectionList;
#[cfg(feature = "PhotoKit_PHCollectionListChangeRequest")]
pub use self::__PHCollectionListChangeRequest::PHCollectionListChangeRequest;
#[cfg(feature = "PhotoKit_PHContentEditingInput")]
pub use self::__PHContentEditingInput::PHContentEditingInput;
#[cfg(feature = "PhotoKit_PHContentEditingOutput")]
pub use self::__PHContentEditingOutput::PHContentEditingOutput;
pub use self::__PHError::PHLocalIdentifiersErrorKey;
pub use self::__PHError::PHPhotosError;
pub use self::__PHError::PHPhotosErrorAccessRestricted;
pub use self::__PHError::PHPhotosErrorAccessUserDenied;
pub use self::__PHError::PHPhotosErrorChangeNotSupported;
pub use self::__PHError::PHPhotosErrorDomain;
pub use self::__PHError::PHPhotosErrorIdentifierNotFound;
pub use self::__PHError::PHPhotosErrorInternalError;
pub use self::__PHError::PHPhotosErrorInvalid;
pub use self::__PHError::PHPhotosErrorInvalidResource;
pub use self::__PHError::PHPhotosErrorLibraryInFileProviderSyncRoot;
pub use self::__PHError::PHPhotosErrorLibraryVolumeOffline;
pub use self::__PHError::PHPhotosErrorMissingResource;
pub use self::__PHError::PHPhotosErrorMultipleIdentifiersFound;
pub use self::__PHError::PHPhotosErrorNetworkAccessRequired;
pub use self::__PHError::PHPhotosErrorNetworkError;
pub use self::__PHError::PHPhotosErrorNotEnoughSpace;
pub use self::__PHError::PHPhotosErrorOperationInterrupted;
pub use self::__PHError::PHPhotosErrorPersistentChangeDetailsUnavailable;
pub use self::__PHError::PHPhotosErrorPersistentChangeTokenExpired;
pub use self::__PHError::PHPhotosErrorRelinquishingLibraryBundleToWriter;
pub use self::__PHError::PHPhotosErrorRequestNotSupportedForAsset;
pub use self::__PHError::PHPhotosErrorSwitchingSystemPhotoLibrary;
pub use self::__PHError::PHPhotosErrorUserCancelled;
#[cfg(feature = "PhotoKit_PHFetchOptions")]
pub use self::__PHFetchOptions::PHFetchOptions;
#[cfg(feature = "PhotoKit_PHFetchResult")]
pub use self::__PHFetchResult::PHFetchResult;
pub use self::__PHImageManager::PHAssetImageProgressHandler;
pub use self::__PHImageManager::PHAssetVideoProgressHandler;
#[cfg(feature = "PhotoKit_PHCachingImageManager")]
pub use self::__PHImageManager::PHCachingImageManager;
pub use self::__PHImageManager::PHImageCancelledKey;
pub use self::__PHImageManager::PHImageErrorKey;
#[cfg(feature = "PhotoKit_PHImageManager")]
pub use self::__PHImageManager::PHImageManager;
pub use self::__PHImageManager::PHImageManagerMaximumSize;
pub use self::__PHImageManager::PHImageRequestID;
#[cfg(feature = "PhotoKit_PHImageRequestOptions")]
pub use self::__PHImageManager::PHImageRequestOptions;
pub use self::__PHImageManager::PHImageRequestOptionsDeliveryMode;
pub use self::__PHImageManager::PHImageRequestOptionsDeliveryModeFastFormat;
pub use self::__PHImageManager::PHImageRequestOptionsDeliveryModeHighQualityFormat;
pub use self::__PHImageManager::PHImageRequestOptionsDeliveryModeOpportunistic;
pub use self::__PHImageManager::PHImageRequestOptionsResizeMode;
pub use self::__PHImageManager::PHImageRequestOptionsResizeModeExact;
pub use self::__PHImageManager::PHImageRequestOptionsResizeModeFast;
pub use self::__PHImageManager::PHImageRequestOptionsResizeModeNone;
pub use self::__PHImageManager::PHImageRequestOptionsVersion;
pub use self::__PHImageManager::PHImageRequestOptionsVersionCurrent;
pub use self::__PHImageManager::PHImageRequestOptionsVersionOriginal;
pub use self::__PHImageManager::PHImageRequestOptionsVersionUnadjusted;
pub use self::__PHImageManager::PHImageResultIsDegradedKey;
pub use self::__PHImageManager::PHImageResultIsInCloudKey;
pub use self::__PHImageManager::PHImageResultRequestIDKey;
pub use self::__PHImageManager::PHInvalidImageRequestID;
#[cfg(feature = "PhotoKit_PHLivePhotoRequestOptions")]
pub use self::__PHImageManager::PHLivePhotoRequestOptions;
#[cfg(feature = "PhotoKit_PHVideoRequestOptions")]
pub use self::__PHImageManager::PHVideoRequestOptions;
pub use self::__PHImageManager::PHVideoRequestOptionsDeliveryMode;
pub use self::__PHImageManager::PHVideoRequestOptionsDeliveryModeAutomatic;
pub use self::__PHImageManager::PHVideoRequestOptionsDeliveryModeFastFormat;
pub use self::__PHImageManager::PHVideoRequestOptionsDeliveryModeHighQualityFormat;
pub use self::__PHImageManager::PHVideoRequestOptionsDeliveryModeMediumQualityFormat;
pub use self::__PHImageManager::PHVideoRequestOptionsVersion;
pub use self::__PHImageManager::PHVideoRequestOptionsVersionCurrent;
pub use self::__PHImageManager::PHVideoRequestOptionsVersionOriginal;
#[cfg(feature = "PhotoKit_PHLivePhoto")]
pub use self::__PHLivePhoto::PHLivePhoto;
pub use self::__PHLivePhoto::PHLivePhotoInfoCancelledKey;
pub use self::__PHLivePhoto::PHLivePhotoInfoErrorKey;
pub use self::__PHLivePhoto::PHLivePhotoInfoIsDegradedKey;
pub use self::__PHLivePhoto::PHLivePhotoRequestID;
pub use self::__PHLivePhoto::PHLivePhotoRequestIDInvalid;
#[cfg(feature = "PhotoKit_PHLivePhotoEditingContext")]
pub use self::__PHLivePhotoEditingContext::PHLivePhotoEditingContext;
#[cfg(not(any(target_os = "ios", target_os = "tvos")))]
pub use self::__PHLivePhotoEditingContext::PHLivePhotoEditingErrorCode;
pub use self::__PHLivePhotoEditingContext::PHLivePhotoEditingErrorCodeAborted;
pub use self::__PHLivePhotoEditingContext::PHLivePhotoEditingErrorCodeUnknown;
#[cfg(not(any(target_os = "ios", target_os = "tvos")))]
pub use self::__PHLivePhotoEditingContext::PHLivePhotoEditingErrorDomain;
pub use self::__PHLivePhotoEditingContext::PHLivePhotoEditingOption;
pub use self::__PHLivePhotoEditingContext::PHLivePhotoFrame;
pub use self::__PHLivePhotoEditingContext::PHLivePhotoFrameType;
pub use self::__PHLivePhotoEditingContext::PHLivePhotoFrameTypePhoto;
pub use self::__PHLivePhotoEditingContext::PHLivePhotoFrameTypeVideo;
pub use self::__PHLivePhotoEditingContext::PHLivePhotoShouldRenderAtPlaybackTime;
#[cfg(feature = "PhotoKit_PHObject")]
pub use self::__PHObject::PHObject;
#[cfg(feature = "PhotoKit_PHObjectPlaceholder")]
pub use self::__PHObject::PHObjectPlaceholder;
#[cfg(feature = "PhotoKit_PHPersistentChange")]
pub use self::__PHPersistentChange::PHPersistentChange;
#[cfg(feature = "PhotoKit_PHPersistentChangeFetchResult")]
pub use self::__PHPersistentChangeFetchResult::PHPersistentChangeFetchResult;
#[cfg(feature = "PhotoKit_PHPersistentChangeToken")]
pub use self::__PHPersistentChangeToken::PHPersistentChangeToken;
#[cfg(feature = "PhotoKit_PHPersistentObjectChangeDetails")]
pub use self::__PHPersistentObjectChangeDetails::PHPersistentObjectChangeDetails;
pub use self::__PHPhotoLibrary::PHAccessLevel;
pub use self::__PHPhotoLibrary::PHAccessLevelAddOnly;
pub use self::__PHPhotoLibrary::PHAccessLevelReadWrite;
pub use self::__PHPhotoLibrary::PHAuthorizationStatus;
pub use self::__PHPhotoLibrary::PHAuthorizationStatusAuthorized;
pub use self::__PHPhotoLibrary::PHAuthorizationStatusDenied;
pub use self::__PHPhotoLibrary::PHAuthorizationStatusLimited;
pub use self::__PHPhotoLibrary::PHAuthorizationStatusNotDetermined;
pub use self::__PHPhotoLibrary::PHAuthorizationStatusRestricted;
#[cfg(feature = "PhotoKit_PHPhotoLibrary")]
pub use self::__PHPhotoLibrary::PHPhotoLibrary;
pub use self::__PHPhotoLibrary::PHPhotoLibraryAvailabilityObserver;
pub use self::__PHPhotoLibrary::PHPhotoLibraryChangeObserver;
#[cfg(feature = "PhotoKit_PHProject")]
#[cfg(not(any(target_os = "ios", target_os = "tvos")))]
pub use self::__PHProject::PHProject;
#[cfg(feature = "PhotoKit_PHProjectChangeRequest")]
#[cfg(not(any(target_os = "ios", target_os = "tvos")))]
pub use self::__PHProjectChangeRequest::PHProjectChangeRequest;
pub use self::__PhotosTypes::PHAssetBurstSelectionType;
pub use self::__PhotosTypes::PHAssetBurstSelectionTypeAutoPick;
pub use self::__PhotosTypes::PHAssetBurstSelectionTypeNone;
pub use self::__PhotosTypes::PHAssetBurstSelectionTypeUserPick;
pub use self::__PhotosTypes::PHAssetCollectionSubtype;
pub use self::__PhotosTypes::PHAssetCollectionSubtypeAlbumCloudShared;
pub use self::__PhotosTypes::PHAssetCollectionSubtypeAlbumImported;
pub use self::__PhotosTypes::PHAssetCollectionSubtypeAlbumMyPhotoStream;
pub use self::__PhotosTypes::PHAssetCollectionSubtypeAlbumRegular;
pub use self::__PhotosTypes::PHAssetCollectionSubtypeAlbumSyncedAlbum;
pub use self::__PhotosTypes::PHAssetCollectionSubtypeAlbumSyncedEvent;
pub use self::__PhotosTypes::PHAssetCollectionSubtypeAlbumSyncedFaces;
pub use self::__PhotosTypes::PHAssetCollectionSubtypeAny;
pub use self::__PhotosTypes::PHAssetCollectionSubtypeSmartAlbumAllHidden;
pub use self::__PhotosTypes::PHAssetCollectionSubtypeSmartAlbumAnimated;
pub use self::__PhotosTypes::PHAssetCollectionSubtypeSmartAlbumBursts;
pub use self::__PhotosTypes::PHAssetCollectionSubtypeSmartAlbumCinematic;
pub use self::__PhotosTypes::PHAssetCollectionSubtypeSmartAlbumDepthEffect;
pub use self::__PhotosTypes::PHAssetCollectionSubtypeSmartAlbumFavorites;
pub use self::__PhotosTypes::PHAssetCollectionSubtypeSmartAlbumGeneric;
pub use self::__PhotosTypes::PHAssetCollectionSubtypeSmartAlbumLivePhotos;
pub use self::__PhotosTypes::PHAssetCollectionSubtypeSmartAlbumLongExposures;
pub use self::__PhotosTypes::PHAssetCollectionSubtypeSmartAlbumPanoramas;
pub use self::__PhotosTypes::PHAssetCollectionSubtypeSmartAlbumRAW;
pub use self::__PhotosTypes::PHAssetCollectionSubtypeSmartAlbumRecentlyAdded;
pub use self::__PhotosTypes::PHAssetCollectionSubtypeSmartAlbumScreenshots;
pub use self::__PhotosTypes::PHAssetCollectionSubtypeSmartAlbumSelfPortraits;
pub use self::__PhotosTypes::PHAssetCollectionSubtypeSmartAlbumSlomoVideos;
pub use self::__PhotosTypes::PHAssetCollectionSubtypeSmartAlbumTimelapses;
pub use self::__PhotosTypes::PHAssetCollectionSubtypeSmartAlbumUnableToUpload;
pub use self::__PhotosTypes::PHAssetCollectionSubtypeSmartAlbumUserLibrary;
pub use self::__PhotosTypes::PHAssetCollectionSubtypeSmartAlbumVideos;
pub use self::__PhotosTypes::PHAssetCollectionType;
pub use self::__PhotosTypes::PHAssetCollectionTypeAlbum;
#[cfg(not(any(target_os = "macos")))]
pub use self::__PhotosTypes::PHAssetCollectionTypeMoment;
pub use self::__PhotosTypes::PHAssetCollectionTypeSmartAlbum;
pub use self::__PhotosTypes::PHAssetEditOperation;
pub use self::__PhotosTypes::PHAssetEditOperationContent;
pub use self::__PhotosTypes::PHAssetEditOperationDelete;
pub use self::__PhotosTypes::PHAssetEditOperationProperties;
pub use self::__PhotosTypes::PHAssetMediaSubtype;
pub use self::__PhotosTypes::PHAssetMediaSubtypeNone;
pub use self::__PhotosTypes::PHAssetMediaSubtypePhotoDepthEffect;
pub use self::__PhotosTypes::PHAssetMediaSubtypePhotoHDR;
pub use self::__PhotosTypes::PHAssetMediaSubtypePhotoLive;
pub use self::__PhotosTypes::PHAssetMediaSubtypePhotoPanorama;
pub use self::__PhotosTypes::PHAssetMediaSubtypePhotoScreenshot;
pub use self::__PhotosTypes::PHAssetMediaSubtypeVideoCinematic;
pub use self::__PhotosTypes::PHAssetMediaSubtypeVideoHighFrameRate;
pub use self::__PhotosTypes::PHAssetMediaSubtypeVideoStreamed;
pub use self::__PhotosTypes::PHAssetMediaSubtypeVideoTimelapse;
pub use self::__PhotosTypes::PHAssetMediaType;
pub use self::__PhotosTypes::PHAssetMediaTypeAudio;
pub use self::__PhotosTypes::PHAssetMediaTypeImage;
pub use self::__PhotosTypes::PHAssetMediaTypeUnknown;
pub use self::__PhotosTypes::PHAssetMediaTypeVideo;
pub use self::__PhotosTypes::PHAssetPlaybackStyle;
pub use self::__PhotosTypes::PHAssetPlaybackStyleImage;
pub use self::__PhotosTypes::PHAssetPlaybackStyleImageAnimated;
pub use self::__PhotosTypes::PHAssetPlaybackStyleLivePhoto;
pub use self::__PhotosTypes::PHAssetPlaybackStyleUnsupported;
pub use self::__PhotosTypes::PHAssetPlaybackStyleVideo;
pub use self::__PhotosTypes::PHAssetPlaybackStyleVideoLooping;
pub use self::__PhotosTypes::PHAssetResourceType;
pub use self::__PhotosTypes::PHAssetResourceTypeAdjustmentBasePairedVideo;
pub use self::__PhotosTypes::PHAssetResourceTypeAdjustmentBasePhoto;
pub use self::__PhotosTypes::PHAssetResourceTypeAdjustmentBaseVideo;
pub use self::__PhotosTypes::PHAssetResourceTypeAdjustmentData;
pub use self::__PhotosTypes::PHAssetResourceTypeAlternatePhoto;
pub use self::__PhotosTypes::PHAssetResourceTypeAudio;
pub use self::__PhotosTypes::PHAssetResourceTypeFullSizePairedVideo;
pub use self::__PhotosTypes::PHAssetResourceTypeFullSizePhoto;
pub use self::__PhotosTypes::PHAssetResourceTypeFullSizeVideo;
pub use self::__PhotosTypes::PHAssetResourceTypePairedVideo;
pub use self::__PhotosTypes::PHAssetResourceTypePhoto;
pub use self::__PhotosTypes::PHAssetResourceTypeVideo;
pub use self::__PhotosTypes::PHAssetSourceType;
pub use self::__PhotosTypes::PHAssetSourceTypeCloudShared;
pub use self::__PhotosTypes::PHAssetSourceTypeNone;
pub use self::__PhotosTypes::PHAssetSourceTypeUserLibrary;
pub use self::__PhotosTypes::PHAssetSourceTypeiTunesSynced;
pub use self::__PhotosTypes::PHCollectionEditOperation;
pub use self::__PhotosTypes::PHCollectionEditOperationAddContent;
pub use self::__PhotosTypes::PHCollectionEditOperationCreateContent;
pub use self::__PhotosTypes::PHCollectionEditOperationDelete;
pub use self::__PhotosTypes::PHCollectionEditOperationDeleteContent;
pub use self::__PhotosTypes::PHCollectionEditOperationRearrangeContent;
pub use self::__PhotosTypes::PHCollectionEditOperationRemoveContent;
pub use self::__PhotosTypes::PHCollectionEditOperationRename;
pub use self::__PhotosTypes::PHCollectionListSubtype;
pub use self::__PhotosTypes::PHCollectionListSubtypeAny;
#[cfg(not(any(target_os = "macos")))]
pub use self::__PhotosTypes::PHCollectionListSubtypeMomentListCluster;
#[cfg(not(any(target_os = "macos")))]
pub use self::__PhotosTypes::PHCollectionListSubtypeMomentListYear;
pub use self::__PhotosTypes::PHCollectionListSubtypeRegularFolder;
pub use self::__PhotosTypes::PHCollectionListSubtypeSmartFolderEvents;
pub use self::__PhotosTypes::PHCollectionListSubtypeSmartFolderFaces;
pub use self::__PhotosTypes::PHCollectionListType;
pub use self::__PhotosTypes::PHCollectionListTypeFolder;
#[cfg(not(any(target_os = "macos")))]
pub use self::__PhotosTypes::PHCollectionListTypeMomentList;
pub use self::__PhotosTypes::PHCollectionListTypeSmartFolder;
pub use self::__PhotosTypes::PHImageContentMode;
pub use self::__PhotosTypes::PHImageContentModeAspectFill;
pub use self::__PhotosTypes::PHImageContentModeAspectFit;
pub use self::__PhotosTypes::PHImageContentModeDefault;
pub use self::__PhotosTypes::PHObjectType;
pub use self::__PhotosTypes::PHObjectTypeAsset;
pub use self::__PhotosTypes::PHObjectTypeAssetCollection;
pub use self::__PhotosTypes::PHObjectTypeCollectionList;
