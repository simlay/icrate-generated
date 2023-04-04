//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#![allow(unused_imports)]
#![allow(deprecated)]
#[path = "NSFileProviderActions.rs"]
mod __NSFileProviderActions;
#[path = "NSFileProviderDefines.rs"]
mod __NSFileProviderDefines;
#[path = "NSFileProviderDomain.rs"]
mod __NSFileProviderDomain;
#[path = "NSFileProviderEnumerating.rs"]
mod __NSFileProviderEnumerating;
#[path = "NSFileProviderError.rs"]
mod __NSFileProviderError;
#[path = "NSFileProviderExtension.rs"]
mod __NSFileProviderExtension;
#[path = "NSFileProviderItem.rs"]
mod __NSFileProviderItem;
#[path = "NSFileProviderItemDecoration.rs"]
mod __NSFileProviderItemDecoration;
#[path = "NSFileProviderManager.rs"]
mod __NSFileProviderManager;
#[path = "NSFileProviderModifyItemOptions.rs"]
mod __NSFileProviderModifyItemOptions;
#[path = "NSFileProviderReplicatedExtension.rs"]
mod __NSFileProviderReplicatedExtension;
#[path = "NSFileProviderRequest.rs"]
mod __NSFileProviderRequest;
#[path = "NSFileProviderService.rs"]
mod __NSFileProviderService;
#[path = "NSFileProviderTesting.rs"]
mod __NSFileProviderTesting;
#[path = "NSFileProviderThumbnailing.rs"]
mod __NSFileProviderThumbnailing;

pub use self::__NSFileProviderActions::NSFileProviderExtensionActionIdentifier;

pub use self::__NSFileProviderDomain::NSFileProviderDomainIdentifier;
#[cfg(feature = "FileProvider_NSFileProviderDomainVersion")]
pub use self::__NSFileProviderDomain::NSFileProviderDomainVersion;

pub use self::__NSFileProviderDomain::NSFileProviderDomainTestingModes;

pub use self::__NSFileProviderDomain::NSFileProviderDomainTestingModeAlwaysEnabled;

#[cfg(feature = "FileProvider_NSFileProviderDomain")]
pub use self::__NSFileProviderDomain::NSFileProviderDomain;
pub use self::__NSFileProviderDomain::NSFileProviderDomainTestingModeInteractive;

pub use self::__NSFileProviderDomain::NSFileProviderDomainDidChange;

pub use self::__NSFileProviderEnumerating::NSFileProviderSyncAnchor;

pub use self::__NSFileProviderEnumerating::NSFileProviderPage;

pub use self::__NSFileProviderEnumerating::NSFileProviderInitialPageSortedByDate;

pub use self::__NSFileProviderEnumerating::NSFileProviderInitialPageSortedByName;

pub use self::__NSFileProviderEnumerating::NSFileProviderEnumerationObserver;

pub use self::__NSFileProviderEnumerating::NSFileProviderChangeObserver;

pub use self::__NSFileProviderEnumerating::NSFileProviderEnumerator;

#[cfg(not(any(target_os = "macos")))]
pub use self::__NSFileProviderError::NSFileProviderErrorCollidingItemKey;
pub use self::__NSFileProviderError::NSFileProviderErrorDomain;

pub use self::__NSFileProviderError::NSFileProviderErrorItemKey;

pub use self::__NSFileProviderError::NSFileProviderErrorNonExistentItemIdentifierKey;

pub use self::__NSFileProviderError::NSFileProviderErrorCode;

pub use self::__NSFileProviderError::NSFileProviderErrorNotAuthenticated;

pub use self::__NSFileProviderError::NSFileProviderErrorFilenameCollision;

pub use self::__NSFileProviderError::NSFileProviderErrorSyncAnchorExpired;

pub use self::__NSFileProviderError::NSFileProviderErrorPageExpired;

pub use self::__NSFileProviderError::NSFileProviderErrorInsufficientQuota;

pub use self::__NSFileProviderError::NSFileProviderErrorServerUnreachable;

pub use self::__NSFileProviderError::NSFileProviderErrorNoSuchItem;

pub use self::__NSFileProviderError::NSFileProviderErrorDeletionRejected;

pub use self::__NSFileProviderError::NSFileProviderErrorDirectoryNotEmpty;

pub use self::__NSFileProviderError::NSFileProviderErrorProviderNotFound;

pub use self::__NSFileProviderError::NSFileProviderErrorProviderTranslocated;

pub use self::__NSFileProviderError::NSFileProviderErrorOlderExtensionVersionRunning;

pub use self::__NSFileProviderError::NSFileProviderErrorNewerExtensionVersionFound;

pub use self::__NSFileProviderError::NSFileProviderErrorCannotSynchronize;

pub use self::__NSFileProviderError::NSFileProviderErrorNonEvictableChildren;

pub use self::__NSFileProviderError::NSFileProviderErrorUnsyncedEdits;

pub use self::__NSFileProviderError::NSFileProviderErrorNonEvictable;

pub use self::__NSFileProviderError::NSFileProviderErrorVersionNoLongerAvailable;

pub use self::__NSFileProviderError::NSFileProviderErrorExcludedFromSync;

pub use self::__NSFileProviderError::NSFileProviderErrorDomainDisabled;
#[cfg(feature = "FileProvider_NSFileProviderExtension")]
#[cfg(not(any(target_os = "macos")))]
pub use self::__NSFileProviderExtension::NSFileProviderExtension;

pub use self::__NSFileProviderItem::NSFileProviderItemIdentifier;

pub use self::__NSFileProviderItem::NSFileProviderRootContainerItemIdentifier;

pub use self::__NSFileProviderItem::NSFileProviderWorkingSetContainerItemIdentifier;

#[cfg(feature = "FileProvider_NSFileProviderItemVersion")]
pub use self::__NSFileProviderItem::NSFileProviderItemVersion;
pub use self::__NSFileProviderItem::NSFileProviderTrashContainerItemIdentifier;

pub use self::__NSFileProviderItem::NSFileProviderFavoriteRankUnranked;

pub use self::__NSFileProviderItem::NSFileProviderItemCapabilities;

pub use self::__NSFileProviderItem::NSFileProviderItemCapabilitiesAllowsReading;

pub use self::__NSFileProviderItem::NSFileProviderItemCapabilitiesAllowsWriting;

pub use self::__NSFileProviderItem::NSFileProviderItemCapabilitiesAllowsReparenting;

pub use self::__NSFileProviderItem::NSFileProviderItemCapabilitiesAllowsRenaming;

pub use self::__NSFileProviderItem::NSFileProviderItemCapabilitiesAllowsTrashing;

pub use self::__NSFileProviderItem::NSFileProviderItemCapabilitiesAllowsDeleting;

pub use self::__NSFileProviderItem::NSFileProviderItemCapabilitiesAllowsEvicting;

pub use self::__NSFileProviderItem::NSFileProviderItemCapabilitiesAllowsExcludingFromSync;

pub use self::__NSFileProviderItem::NSFileProviderItemCapabilitiesAllowsAddingSubItems;

pub use self::__NSFileProviderItem::NSFileProviderItemCapabilitiesAllowsContentEnumerating;

pub use self::__NSFileProviderItem::NSFileProviderItemCapabilitiesAllowsAll;

pub use self::__NSFileProviderItem::NSFileProviderItemFields;

pub use self::__NSFileProviderItem::NSFileProviderItemContents;

pub use self::__NSFileProviderItem::NSFileProviderItemFilename;

pub use self::__NSFileProviderItem::NSFileProviderItemParentItemIdentifier;

pub use self::__NSFileProviderItem::NSFileProviderItemLastUsedDate;

pub use self::__NSFileProviderItem::NSFileProviderItemTagData;

pub use self::__NSFileProviderItem::NSFileProviderItemFavoriteRank;

pub use self::__NSFileProviderItem::NSFileProviderItemCreationDate;

pub use self::__NSFileProviderItem::NSFileProviderItemContentModificationDate;

pub use self::__NSFileProviderItem::NSFileProviderItemFileSystemFlags;

pub use self::__NSFileProviderItem::NSFileProviderItemExtendedAttributes;

pub use self::__NSFileProviderItem::NSFileProviderItemTypeAndCreator;

pub use self::__NSFileProviderItem::NSFileProviderFileSystemFlags;

pub use self::__NSFileProviderItem::NSFileProviderFileSystemUserExecutable;

pub use self::__NSFileProviderItem::NSFileProviderFileSystemUserReadable;

pub use self::__NSFileProviderItem::NSFileProviderFileSystemUserWritable;

pub use self::__NSFileProviderItem::NSFileProviderFileSystemHidden;

pub use self::__NSFileProviderItem::NSFileProviderFileSystemPathExtensionHidden;

pub use self::__NSFileProviderItem::NSFileProviderTypeAndCreator;

pub use self::__NSFileProviderItem::NSFileProviderContentPolicy;

pub use self::__NSFileProviderItem::NSFileProviderContentPolicyInherited;

pub use self::__NSFileProviderItem::NSFileProviderContentPolicyDownloadLazily;

pub use self::__NSFileProviderItem::NSFileProviderContentPolicyDownloadLazilyAndEvictOnRemoteUpdate;

pub use self::__NSFileProviderItem::NSFileProviderContentPolicyDownloadEagerlyAndKeepDownloaded;

pub use self::__NSFileProviderItem::NSFileProviderItemProtocol;

pub use self::__NSFileProviderItem::NSFileProviderItem;

pub use self::__NSFileProviderItemDecoration::NSFileProviderItemDecorationIdentifier;

pub use self::__NSFileProviderItemDecoration::NSFileProviderItemDecorating;

pub use self::__NSFileProviderManager::NSFileProviderDomainRemovalMode;

pub use self::__NSFileProviderManager::NSFileProviderDomainRemovalModeRemoveAll;

pub use self::__NSFileProviderManager::NSFileProviderDomainRemovalModePreserveDirtyUserData;

pub use self::__NSFileProviderManager::NSFileProviderDomainRemovalModePreserveDownloadedUserData;
#[cfg(feature = "FileProvider_NSFileProviderManager")]
pub use self::__NSFileProviderManager::NSFileProviderManager;

pub use self::__NSFileProviderManager::NSFileProviderMaterializedSetDidChange;

pub use self::__NSFileProviderManager::NSFileProviderPendingSetDidChange;

#[cfg(not(any(target_os = "ios")))]
pub use self::__NSFileProviderManager::NSFileProviderManagerDisconnectionOptions;
#[cfg(not(any(target_os = "ios")))]
pub use self::__NSFileProviderManager::NSFileProviderManagerDisconnectionOptionsTemporary;
pub use self::__NSFileProviderManager::NSFileProviderPendingSetEnumerator;

pub use self::__NSFileProviderModifyItemOptions::NSFileProviderModifyItemOptions;

pub use self::__NSFileProviderModifyItemOptions::NSFileProviderModifyItemMayAlreadyExist;

pub use self::__NSFileProviderReplicatedExtension::NSFileProviderCreateItemOptions;

pub use self::__NSFileProviderReplicatedExtension::NSFileProviderCreateItemMayAlreadyExist;

pub use self::__NSFileProviderReplicatedExtension::NSFileProviderCreateItemDeletionConflicted;

pub use self::__NSFileProviderReplicatedExtension::NSFileProviderDeleteItemOptions;

pub use self::__NSFileProviderReplicatedExtension::NSFileProviderDeleteItemRecursive;
#[cfg(not(any(target_os = "ios")))]
pub use self::__NSFileProviderReplicatedExtension::NSFileProviderFetchContentsOptions;
#[cfg(not(any(target_os = "ios")))]
pub use self::__NSFileProviderReplicatedExtension::NSFileProviderFetchContentsOptionsStrictVersioning;
#[cfg(not(any(target_os = "ios")))]
pub use self::__NSFileProviderReplicatedExtension::NSFileProviderMaterializationFlags;
#[cfg(not(any(target_os = "ios")))]
pub use self::__NSFileProviderReplicatedExtension::NSFileProviderMaterializationFlagsKnownSparseRanges;

pub use self::__NSFileProviderReplicatedExtension::NSFileProviderEnumerating;

pub use self::__NSFileProviderReplicatedExtension::NSFileProviderReplicatedExtension;

pub use self::__NSFileProviderReplicatedExtension::NSFileProviderIncrementalContentFetching;

pub use self::__NSFileProviderReplicatedExtension::NSFileProviderServicing;

pub use self::__NSFileProviderReplicatedExtension::NSFileProviderThumbnailing;

pub use self::__NSFileProviderReplicatedExtension::NSFileProviderCustomAction;
#[cfg(not(any(target_os = "ios")))]
pub use self::__NSFileProviderReplicatedExtension::NSFileProviderUserInteractionSuppressing;

pub use self::__NSFileProviderReplicatedExtension::NSFileProviderDomainState;
#[cfg(not(any(target_os = "ios")))]
pub use self::__NSFileProviderReplicatedExtension::NSFileProviderPartialContentFetching;
#[cfg(feature = "FileProvider_NSFileProviderRequest")]
pub use self::__NSFileProviderRequest::NSFileProviderRequest;

pub use self::__NSFileProviderService::NSFileProviderServiceSource;

pub use self::__NSFileProviderTesting::NSFileProviderTestingOperationType;

pub use self::__NSFileProviderTesting::NSFileProviderTestingOperationTypeIngestion;

pub use self::__NSFileProviderTesting::NSFileProviderTestingOperationTypeLookup;

pub use self::__NSFileProviderTesting::NSFileProviderTestingOperationTypeCreation;

pub use self::__NSFileProviderTesting::NSFileProviderTestingOperationTypeModification;

pub use self::__NSFileProviderTesting::NSFileProviderTestingOperationTypeDeletion;

pub use self::__NSFileProviderTesting::NSFileProviderTestingOperationTypeContentFetch;

pub use self::__NSFileProviderTesting::NSFileProviderTestingOperationTypeChildrenEnumeration;

pub use self::__NSFileProviderTesting::NSFileProviderTestingOperationTypeCollisionResolution;

pub use self::__NSFileProviderTesting::NSFileProviderTestingOperation;

pub use self::__NSFileProviderTesting::NSFileProviderTestingOperationSide;

pub use self::__NSFileProviderTesting::NSFileProviderTestingOperationSideDisk;

pub use self::__NSFileProviderTesting::NSFileProviderTestingOperationSideFileProvider;

pub use self::__NSFileProviderTesting::NSFileProviderTestingIngestion;

pub use self::__NSFileProviderTesting::NSFileProviderTestingLookup;

pub use self::__NSFileProviderTesting::NSFileProviderTestingCreation;

pub use self::__NSFileProviderTesting::NSFileProviderTestingModification;

pub use self::__NSFileProviderTesting::NSFileProviderTestingDeletion;

pub use self::__NSFileProviderTesting::NSFileProviderTestingContentFetch;

pub use self::__NSFileProviderTesting::NSFileProviderTestingChildrenEnumeration;

pub use self::__NSFileProviderTesting::NSFileProviderTestingCollisionResolution;
