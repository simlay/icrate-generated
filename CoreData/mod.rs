//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#![allow(unused_imports)]
#![allow(deprecated)]
#[path = "CoreDataDefines.rs"]
mod __CoreDataDefines;
#[path = "CoreDataErrors.rs"]
mod __CoreDataErrors;
#[path = "NSAtomicStore.rs"]
mod __NSAtomicStore;
#[path = "NSAtomicStoreCacheNode.rs"]
mod __NSAtomicStoreCacheNode;
#[path = "NSAttributeDescription.rs"]
mod __NSAttributeDescription;
#[path = "NSBatchDeleteRequest.rs"]
mod __NSBatchDeleteRequest;
#[path = "NSBatchInsertRequest.rs"]
mod __NSBatchInsertRequest;
#[path = "NSBatchUpdateRequest.rs"]
mod __NSBatchUpdateRequest;
#[path = "NSCoreDataCoreSpotlightDelegate.rs"]
mod __NSCoreDataCoreSpotlightDelegate;
#[path = "NSDerivedAttributeDescription.rs"]
mod __NSDerivedAttributeDescription;
#[path = "NSEntityDescription.rs"]
mod __NSEntityDescription;
#[path = "NSEntityMapping.rs"]
mod __NSEntityMapping;
#[path = "NSEntityMigrationPolicy.rs"]
mod __NSEntityMigrationPolicy;
#[path = "NSExpressionDescription.rs"]
mod __NSExpressionDescription;
#[path = "NSFetchIndexDescription.rs"]
mod __NSFetchIndexDescription;
#[path = "NSFetchIndexElementDescription.rs"]
mod __NSFetchIndexElementDescription;
#[path = "NSFetchRequest.rs"]
mod __NSFetchRequest;
#[path = "NSFetchRequestExpression.rs"]
mod __NSFetchRequestExpression;
#[path = "NSFetchedPropertyDescription.rs"]
mod __NSFetchedPropertyDescription;
#[path = "NSFetchedResultsController.rs"]
mod __NSFetchedResultsController;
#[path = "NSIncrementalStore.rs"]
mod __NSIncrementalStore;
#[path = "NSIncrementalStoreNode.rs"]
mod __NSIncrementalStoreNode;
#[path = "NSManagedObject.rs"]
mod __NSManagedObject;
#[path = "NSManagedObjectContext.rs"]
mod __NSManagedObjectContext;
#[path = "NSManagedObjectID.rs"]
mod __NSManagedObjectID;
#[path = "NSManagedObjectModel.rs"]
mod __NSManagedObjectModel;
#[path = "NSMappingModel.rs"]
mod __NSMappingModel;
#[path = "NSMergePolicy.rs"]
mod __NSMergePolicy;
#[path = "NSMigrationManager.rs"]
mod __NSMigrationManager;
#[path = "NSPersistentCloudKitContainer.rs"]
mod __NSPersistentCloudKitContainer;
#[path = "NSPersistentCloudKitContainerEvent.rs"]
mod __NSPersistentCloudKitContainerEvent;
#[path = "NSPersistentCloudKitContainerEventRequest.rs"]
mod __NSPersistentCloudKitContainerEventRequest;
#[path = "NSPersistentCloudKitContainerOptions.rs"]
mod __NSPersistentCloudKitContainerOptions;
#[path = "NSPersistentContainer.rs"]
mod __NSPersistentContainer;
#[path = "NSPersistentHistoryChange.rs"]
mod __NSPersistentHistoryChange;
#[path = "NSPersistentHistoryChangeRequest.rs"]
mod __NSPersistentHistoryChangeRequest;
#[path = "NSPersistentHistoryToken.rs"]
mod __NSPersistentHistoryToken;
#[path = "NSPersistentHistoryTransaction.rs"]
mod __NSPersistentHistoryTransaction;
#[path = "NSPersistentStore.rs"]
mod __NSPersistentStore;
#[path = "NSPersistentStoreCoordinator.rs"]
mod __NSPersistentStoreCoordinator;
#[path = "NSPersistentStoreDescription.rs"]
mod __NSPersistentStoreDescription;
#[path = "NSPersistentStoreRequest.rs"]
mod __NSPersistentStoreRequest;
#[path = "NSPersistentStoreResult.rs"]
mod __NSPersistentStoreResult;
#[path = "NSPropertyDescription.rs"]
mod __NSPropertyDescription;
#[path = "NSPropertyMapping.rs"]
mod __NSPropertyMapping;
#[path = "NSQueryGenerationToken.rs"]
mod __NSQueryGenerationToken;
#[path = "NSRelationshipDescription.rs"]
mod __NSRelationshipDescription;
#[path = "NSSaveChangesRequest.rs"]
mod __NSSaveChangesRequest;

pub use self::__CoreDataDefines::NSCoreDataVersionNumber;
pub use self::__CoreDataErrors::NSAffectedObjectsErrorKey;
pub use self::__CoreDataErrors::NSAffectedStoresErrorKey;
pub use self::__CoreDataErrors::NSCoreDataError;
pub use self::__CoreDataErrors::NSDetailedErrorsKey;
pub use self::__CoreDataErrors::NSEntityMigrationPolicyError;
pub use self::__CoreDataErrors::NSExternalRecordImportError;
pub use self::__CoreDataErrors::NSInferredMappingModelError;
pub use self::__CoreDataErrors::NSManagedObjectConstraintMergeError;
pub use self::__CoreDataErrors::NSManagedObjectConstraintValidationError;
pub use self::__CoreDataErrors::NSManagedObjectContextLockingError;
pub use self::__CoreDataErrors::NSManagedObjectExternalRelationshipError;
pub use self::__CoreDataErrors::NSManagedObjectMergeError;
pub use self::__CoreDataErrors::NSManagedObjectReferentialIntegrityError;
pub use self::__CoreDataErrors::NSManagedObjectValidationError;
pub use self::__CoreDataErrors::NSMigrationCancelledError;
pub use self::__CoreDataErrors::NSMigrationConstraintViolationError;
pub use self::__CoreDataErrors::NSMigrationError;
pub use self::__CoreDataErrors::NSMigrationManagerDestinationStoreError;
pub use self::__CoreDataErrors::NSMigrationManagerSourceStoreError;
pub use self::__CoreDataErrors::NSMigrationMissingMappingModelError;
pub use self::__CoreDataErrors::NSMigrationMissingSourceModelError;
pub use self::__CoreDataErrors::NSPersistentHistoryTokenExpiredError;
pub use self::__CoreDataErrors::NSPersistentStoreCoordinatorLockingError;
pub use self::__CoreDataErrors::NSPersistentStoreIncompatibleSchemaError;
pub use self::__CoreDataErrors::NSPersistentStoreIncompatibleVersionHashError;
pub use self::__CoreDataErrors::NSPersistentStoreIncompleteSaveError;
pub use self::__CoreDataErrors::NSPersistentStoreInvalidTypeError;
pub use self::__CoreDataErrors::NSPersistentStoreOpenError;
pub use self::__CoreDataErrors::NSPersistentStoreOperationError;
pub use self::__CoreDataErrors::NSPersistentStoreSaveConflictsError;
pub use self::__CoreDataErrors::NSPersistentStoreSaveConflictsErrorKey;
pub use self::__CoreDataErrors::NSPersistentStoreSaveError;
pub use self::__CoreDataErrors::NSPersistentStoreTimeoutError;
pub use self::__CoreDataErrors::NSPersistentStoreTypeMismatchError;
pub use self::__CoreDataErrors::NSPersistentStoreUnsupportedRequestTypeError;
pub use self::__CoreDataErrors::NSSQLiteError;
pub use self::__CoreDataErrors::NSSQLiteErrorDomain;
pub use self::__CoreDataErrors::NSValidationDateTooLateError;
pub use self::__CoreDataErrors::NSValidationDateTooSoonError;
pub use self::__CoreDataErrors::NSValidationInvalidDateError;
pub use self::__CoreDataErrors::NSValidationInvalidURIError;
pub use self::__CoreDataErrors::NSValidationKeyErrorKey;
pub use self::__CoreDataErrors::NSValidationMissingMandatoryPropertyError;
pub use self::__CoreDataErrors::NSValidationMultipleErrorsError;
pub use self::__CoreDataErrors::NSValidationNumberTooLargeError;
pub use self::__CoreDataErrors::NSValidationNumberTooSmallError;
pub use self::__CoreDataErrors::NSValidationObjectErrorKey;
pub use self::__CoreDataErrors::NSValidationPredicateErrorKey;
pub use self::__CoreDataErrors::NSValidationRelationshipDeniedDeleteError;
pub use self::__CoreDataErrors::NSValidationRelationshipExceedsMaximumCountError;
pub use self::__CoreDataErrors::NSValidationRelationshipLacksMinimumCountError;
pub use self::__CoreDataErrors::NSValidationStringPatternMatchingError;
pub use self::__CoreDataErrors::NSValidationStringTooLongError;
pub use self::__CoreDataErrors::NSValidationStringTooShortError;
pub use self::__CoreDataErrors::NSValidationValueErrorKey;
#[cfg(feature = "CoreData_NSAtomicStore")]
pub use self::__NSAtomicStore::NSAtomicStore;
#[cfg(feature = "CoreData_NSAtomicStoreCacheNode")]
pub use self::__NSAtomicStoreCacheNode::NSAtomicStoreCacheNode;
#[cfg(feature = "CoreData_NSAttributeDescription")]
pub use self::__NSAttributeDescription::NSAttributeDescription;
pub use self::__NSAttributeDescription::NSAttributeType;
pub use self::__NSAttributeDescription::NSBinaryDataAttributeType;
pub use self::__NSAttributeDescription::NSBooleanAttributeType;
pub use self::__NSAttributeDescription::NSDateAttributeType;
pub use self::__NSAttributeDescription::NSDecimalAttributeType;
pub use self::__NSAttributeDescription::NSDoubleAttributeType;
pub use self::__NSAttributeDescription::NSFloatAttributeType;
pub use self::__NSAttributeDescription::NSInteger16AttributeType;
pub use self::__NSAttributeDescription::NSInteger32AttributeType;
pub use self::__NSAttributeDescription::NSInteger64AttributeType;
pub use self::__NSAttributeDescription::NSObjectIDAttributeType;
pub use self::__NSAttributeDescription::NSStringAttributeType;
pub use self::__NSAttributeDescription::NSTransformableAttributeType;
pub use self::__NSAttributeDescription::NSURIAttributeType;
pub use self::__NSAttributeDescription::NSUUIDAttributeType;
pub use self::__NSAttributeDescription::NSUndefinedAttributeType;
#[cfg(feature = "CoreData_NSBatchDeleteRequest")]
pub use self::__NSBatchDeleteRequest::NSBatchDeleteRequest;
#[cfg(feature = "CoreData_NSBatchInsertRequest")]
pub use self::__NSBatchInsertRequest::NSBatchInsertRequest;
#[cfg(feature = "CoreData_NSBatchUpdateRequest")]
pub use self::__NSBatchUpdateRequest::NSBatchUpdateRequest;
#[cfg(feature = "CoreData_NSCoreDataCoreSpotlightDelegate")]
#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
pub use self::__NSCoreDataCoreSpotlightDelegate::NSCoreDataCoreSpotlightDelegate;
#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
pub use self::__NSCoreDataCoreSpotlightDelegate::NSCoreDataCoreSpotlightDelegateIndexDidUpdateNotification;
#[cfg(feature = "CoreData_NSDerivedAttributeDescription")]
pub use self::__NSDerivedAttributeDescription::NSDerivedAttributeDescription;
#[cfg(feature = "CoreData_NSEntityDescription")]
pub use self::__NSEntityDescription::NSEntityDescription;
pub use self::__NSEntityMapping::NSAddEntityMappingType;
pub use self::__NSEntityMapping::NSCopyEntityMappingType;
pub use self::__NSEntityMapping::NSCustomEntityMappingType;
#[cfg(feature = "CoreData_NSEntityMapping")]
pub use self::__NSEntityMapping::NSEntityMapping;
pub use self::__NSEntityMapping::NSEntityMappingType;
pub use self::__NSEntityMapping::NSRemoveEntityMappingType;
pub use self::__NSEntityMapping::NSTransformEntityMappingType;
pub use self::__NSEntityMapping::NSUndefinedEntityMappingType;
#[cfg(feature = "CoreData_NSEntityMigrationPolicy")]
pub use self::__NSEntityMigrationPolicy::NSEntityMigrationPolicy;
pub use self::__NSEntityMigrationPolicy::NSMigrationDestinationObjectKey;
pub use self::__NSEntityMigrationPolicy::NSMigrationEntityMappingKey;
pub use self::__NSEntityMigrationPolicy::NSMigrationEntityPolicyKey;
pub use self::__NSEntityMigrationPolicy::NSMigrationManagerKey;
pub use self::__NSEntityMigrationPolicy::NSMigrationPropertyMappingKey;
pub use self::__NSEntityMigrationPolicy::NSMigrationSourceObjectKey;
#[cfg(feature = "CoreData_NSExpressionDescription")]
pub use self::__NSExpressionDescription::NSExpressionDescription;
#[cfg(feature = "CoreData_NSFetchIndexDescription")]
pub use self::__NSFetchIndexDescription::NSFetchIndexDescription;
#[cfg(feature = "CoreData_NSFetchIndexElementDescription")]
pub use self::__NSFetchIndexElementDescription::NSFetchIndexElementDescription;
pub use self::__NSFetchIndexElementDescription::NSFetchIndexElementType;
pub use self::__NSFetchIndexElementDescription::NSFetchIndexElementTypeBinary;
pub use self::__NSFetchIndexElementDescription::NSFetchIndexElementTypeRTree;
#[cfg(feature = "CoreData_NSAsynchronousFetchRequest")]
pub use self::__NSFetchRequest::NSAsynchronousFetchRequest;
pub use self::__NSFetchRequest::NSCountResultType;
pub use self::__NSFetchRequest::NSDictionaryResultType;
#[cfg(feature = "CoreData_NSFetchRequest")]
pub use self::__NSFetchRequest::NSFetchRequest;
pub use self::__NSFetchRequest::NSFetchRequestResult;
pub use self::__NSFetchRequest::NSFetchRequestResultType;
pub use self::__NSFetchRequest::NSManagedObjectIDResultType;
pub use self::__NSFetchRequest::NSManagedObjectResultType;
pub use self::__NSFetchRequest::NSPersistentStoreAsynchronousFetchResultCompletionBlock;
#[cfg(feature = "CoreData_NSFetchRequestExpression")]
pub use self::__NSFetchRequestExpression::NSFetchRequestExpression;
pub use self::__NSFetchRequestExpression::NSFetchRequestExpressionType;
#[cfg(feature = "CoreData_NSFetchedPropertyDescription")]
pub use self::__NSFetchedPropertyDescription::NSFetchedPropertyDescription;
pub use self::__NSFetchedResultsController::NSFetchedResultsChangeDelete;
pub use self::__NSFetchedResultsController::NSFetchedResultsChangeInsert;
pub use self::__NSFetchedResultsController::NSFetchedResultsChangeMove;
pub use self::__NSFetchedResultsController::NSFetchedResultsChangeType;
pub use self::__NSFetchedResultsController::NSFetchedResultsChangeUpdate;
#[cfg(feature = "CoreData_NSFetchedResultsController")]
pub use self::__NSFetchedResultsController::NSFetchedResultsController;
pub use self::__NSFetchedResultsController::NSFetchedResultsControllerDelegate;
pub use self::__NSFetchedResultsController::NSFetchedResultsSectionInfo;
#[cfg(feature = "CoreData_NSIncrementalStore")]
pub use self::__NSIncrementalStore::NSIncrementalStore;
#[cfg(feature = "CoreData_NSIncrementalStoreNode")]
pub use self::__NSIncrementalStoreNode::NSIncrementalStoreNode;
#[cfg(feature = "CoreData_NSManagedObject")]
pub use self::__NSManagedObject::NSManagedObject;
pub use self::__NSManagedObject::NSSnapshotEventMergePolicy;
pub use self::__NSManagedObject::NSSnapshotEventRefresh;
pub use self::__NSManagedObject::NSSnapshotEventRollback;
pub use self::__NSManagedObject::NSSnapshotEventType;
pub use self::__NSManagedObject::NSSnapshotEventUndoDeletion;
pub use self::__NSManagedObject::NSSnapshotEventUndoInsertion;
pub use self::__NSManagedObject::NSSnapshotEventUndoUpdate;
pub use self::__NSManagedObjectContext::NSConfinementConcurrencyType;
pub use self::__NSManagedObjectContext::NSDeletedObjectIDsKey;
pub use self::__NSManagedObjectContext::NSDeletedObjectsKey;
pub use self::__NSManagedObjectContext::NSInsertedObjectIDsKey;
pub use self::__NSManagedObjectContext::NSInsertedObjectsKey;
pub use self::__NSManagedObjectContext::NSInvalidatedAllObjectsKey;
pub use self::__NSManagedObjectContext::NSInvalidatedObjectIDsKey;
pub use self::__NSManagedObjectContext::NSInvalidatedObjectsKey;
pub use self::__NSManagedObjectContext::NSMainQueueConcurrencyType;
#[cfg(feature = "CoreData_NSManagedObjectContext")]
pub use self::__NSManagedObjectContext::NSManagedObjectContext;
pub use self::__NSManagedObjectContext::NSManagedObjectContextConcurrencyType;
pub use self::__NSManagedObjectContext::NSManagedObjectContextDidMergeChangesObjectIDsNotification;
pub use self::__NSManagedObjectContext::NSManagedObjectContextDidSaveNotification;
pub use self::__NSManagedObjectContext::NSManagedObjectContextDidSaveObjectIDsNotification;
pub use self::__NSManagedObjectContext::NSManagedObjectContextObjectsDidChangeNotification;
pub use self::__NSManagedObjectContext::NSManagedObjectContextQueryGenerationKey;
pub use self::__NSManagedObjectContext::NSManagedObjectContextWillSaveNotification;
pub use self::__NSManagedObjectContext::NSPrivateQueueConcurrencyType;
pub use self::__NSManagedObjectContext::NSRefreshedObjectIDsKey;
pub use self::__NSManagedObjectContext::NSRefreshedObjectsKey;
pub use self::__NSManagedObjectContext::NSUpdatedObjectIDsKey;
pub use self::__NSManagedObjectContext::NSUpdatedObjectsKey;
#[cfg(feature = "CoreData_NSManagedObjectID")]
pub use self::__NSManagedObjectID::NSManagedObjectID;
#[cfg(feature = "CoreData_NSManagedObjectModel")]
pub use self::__NSManagedObjectModel::NSManagedObjectModel;
#[cfg(feature = "CoreData_NSMappingModel")]
pub use self::__NSMappingModel::NSMappingModel;
#[cfg(feature = "CoreData_NSConstraintConflict")]
pub use self::__NSMergePolicy::NSConstraintConflict;
pub use self::__NSMergePolicy::NSErrorMergePolicyType;
pub use self::__NSMergePolicy::NSMergeByPropertyObjectTrumpMergePolicyType;
pub use self::__NSMergePolicy::NSMergeByPropertyStoreTrumpMergePolicyType;
#[cfg(feature = "CoreData_NSMergeConflict")]
pub use self::__NSMergePolicy::NSMergeConflict;
#[cfg(feature = "CoreData_NSMergePolicy")]
pub use self::__NSMergePolicy::NSMergePolicy;
pub use self::__NSMergePolicy::NSMergePolicyType;
pub use self::__NSMergePolicy::NSOverwriteMergePolicyType;
pub use self::__NSMergePolicy::NSRollbackMergePolicyType;
#[cfg(feature = "CoreData_NSMigrationManager")]
pub use self::__NSMigrationManager::NSMigrationManager;
#[cfg(feature = "CoreData_NSPersistentCloudKitContainer")]
pub use self::__NSPersistentCloudKitContainer::NSPersistentCloudKitContainer;
pub use self::__NSPersistentCloudKitContainer::NSPersistentCloudKitContainerSchemaInitializationOptions;
pub use self::__NSPersistentCloudKitContainer::NSPersistentCloudKitContainerSchemaInitializationOptionsDryRun;
pub use self::__NSPersistentCloudKitContainer::NSPersistentCloudKitContainerSchemaInitializationOptionsNone;
pub use self::__NSPersistentCloudKitContainer::NSPersistentCloudKitContainerSchemaInitializationOptionsPrintSchema;
#[cfg(feature = "CoreData_NSPersistentCloudKitContainerEvent")]
pub use self::__NSPersistentCloudKitContainerEvent::NSPersistentCloudKitContainerEvent;
pub use self::__NSPersistentCloudKitContainerEvent::NSPersistentCloudKitContainerEventChangedNotification;
pub use self::__NSPersistentCloudKitContainerEvent::NSPersistentCloudKitContainerEventType;
pub use self::__NSPersistentCloudKitContainerEvent::NSPersistentCloudKitContainerEventTypeExport;
pub use self::__NSPersistentCloudKitContainerEvent::NSPersistentCloudKitContainerEventTypeImport;
pub use self::__NSPersistentCloudKitContainerEvent::NSPersistentCloudKitContainerEventTypeSetup;
pub use self::__NSPersistentCloudKitContainerEvent::NSPersistentCloudKitContainerEventUserInfoKey;
#[cfg(feature = "CoreData_NSPersistentCloudKitContainerEventRequest")]
pub use self::__NSPersistentCloudKitContainerEventRequest::NSPersistentCloudKitContainerEventRequest;
#[cfg(feature = "CoreData_NSPersistentCloudKitContainerOptions")]
pub use self::__NSPersistentCloudKitContainerOptions::NSPersistentCloudKitContainerOptions;
#[cfg(feature = "CoreData_NSPersistentContainer")]
pub use self::__NSPersistentContainer::NSPersistentContainer;
#[cfg(feature = "CoreData_NSPersistentHistoryChange")]
pub use self::__NSPersistentHistoryChange::NSPersistentHistoryChange;
pub use self::__NSPersistentHistoryChange::NSPersistentHistoryChangeType;
pub use self::__NSPersistentHistoryChange::NSPersistentHistoryChangeTypeDelete;
pub use self::__NSPersistentHistoryChange::NSPersistentHistoryChangeTypeInsert;
pub use self::__NSPersistentHistoryChange::NSPersistentHistoryChangeTypeUpdate;
#[cfg(feature = "CoreData_NSPersistentHistoryChangeRequest")]
pub use self::__NSPersistentHistoryChangeRequest::NSPersistentHistoryChangeRequest;
#[cfg(feature = "CoreData_NSPersistentHistoryToken")]
pub use self::__NSPersistentHistoryToken::NSPersistentHistoryToken;
#[cfg(feature = "CoreData_NSPersistentHistoryTransaction")]
pub use self::__NSPersistentHistoryTransaction::NSPersistentHistoryTransaction;
#[cfg(feature = "CoreData_NSPersistentStore")]
pub use self::__NSPersistentStore::NSPersistentStore;
pub use self::__NSPersistentStoreCoordinator::NSAddedPersistentStoresKey;
#[cfg(not(any(target_os = "ios")))]
pub use self::__NSPersistentStoreCoordinator::NSBinaryExternalRecordType;
pub use self::__NSPersistentStoreCoordinator::NSBinaryStoreInsecureDecodingCompatibilityOption;
pub use self::__NSPersistentStoreCoordinator::NSBinaryStoreSecureDecodingClasses;
pub use self::__NSPersistentStoreCoordinator::NSBinaryStoreType;
#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
pub use self::__NSPersistentStoreCoordinator::NSCoreDataCoreSpotlightExporter;
#[cfg(not(any(target_os = "ios")))]
pub use self::__NSPersistentStoreCoordinator::NSEntityNameInPathKey;
#[cfg(not(any(target_os = "ios")))]
pub use self::__NSPersistentStoreCoordinator::NSExternalRecordExtensionOption;
#[cfg(not(any(target_os = "ios")))]
pub use self::__NSPersistentStoreCoordinator::NSExternalRecordsDirectoryOption;
#[cfg(not(any(target_os = "ios")))]
pub use self::__NSPersistentStoreCoordinator::NSExternalRecordsFileFormatOption;
pub use self::__NSPersistentStoreCoordinator::NSIgnorePersistentStoreVersioningOption;
pub use self::__NSPersistentStoreCoordinator::NSInMemoryStoreType;
pub use self::__NSPersistentStoreCoordinator::NSInferMappingModelAutomaticallyOption;
pub use self::__NSPersistentStoreCoordinator::NSMigratePersistentStoresAutomaticallyOption;
#[cfg(not(any(target_os = "ios")))]
pub use self::__NSPersistentStoreCoordinator::NSModelPathKey;
#[cfg(not(any(target_os = "ios")))]
pub use self::__NSPersistentStoreCoordinator::NSObjectURIKey;
pub use self::__NSPersistentStoreCoordinator::NSPersistentHistoryTokenKey;
pub use self::__NSPersistentStoreCoordinator::NSPersistentHistoryTrackingKey;
pub use self::__NSPersistentStoreCoordinator::NSPersistentStoreConnectionPoolMaxSizeKey;
#[cfg(feature = "CoreData_NSPersistentStoreCoordinator")]
pub use self::__NSPersistentStoreCoordinator::NSPersistentStoreCoordinator;
pub use self::__NSPersistentStoreCoordinator::NSPersistentStoreCoordinatorStoresDidChangeNotification;
pub use self::__NSPersistentStoreCoordinator::NSPersistentStoreCoordinatorStoresWillChangeNotification;
pub use self::__NSPersistentStoreCoordinator::NSPersistentStoreCoordinatorWillRemoveStoreNotification;
pub use self::__NSPersistentStoreCoordinator::NSPersistentStoreDidImportUbiquitousContentChangesNotification;
#[cfg(not(any(target_os = "macos")))]
pub use self::__NSPersistentStoreCoordinator::NSPersistentStoreFileProtectionKey;
pub use self::__NSPersistentStoreCoordinator::NSPersistentStoreForceDestroyOption;
pub use self::__NSPersistentStoreCoordinator::NSPersistentStoreOSCompatibility;
pub use self::__NSPersistentStoreCoordinator::NSPersistentStoreRebuildFromUbiquitousContentOption;
pub use self::__NSPersistentStoreCoordinator::NSPersistentStoreRemoteChangeNotification;
pub use self::__NSPersistentStoreCoordinator::NSPersistentStoreRemoteChangeNotificationPostOptionKey;
pub use self::__NSPersistentStoreCoordinator::NSPersistentStoreRemoveUbiquitousMetadataOption;
pub use self::__NSPersistentStoreCoordinator::NSPersistentStoreTimeoutOption;
pub use self::__NSPersistentStoreCoordinator::NSPersistentStoreURLKey;
pub use self::__NSPersistentStoreCoordinator::NSPersistentStoreUbiquitousContainerIdentifierKey;
pub use self::__NSPersistentStoreCoordinator::NSPersistentStoreUbiquitousContentNameKey;
pub use self::__NSPersistentStoreCoordinator::NSPersistentStoreUbiquitousContentURLKey;
pub use self::__NSPersistentStoreCoordinator::NSPersistentStoreUbiquitousPeerTokenOption;
pub use self::__NSPersistentStoreCoordinator::NSPersistentStoreUbiquitousTransitionType;
pub use self::__NSPersistentStoreCoordinator::NSPersistentStoreUbiquitousTransitionTypeAccountAdded;
pub use self::__NSPersistentStoreCoordinator::NSPersistentStoreUbiquitousTransitionTypeAccountRemoved;
pub use self::__NSPersistentStoreCoordinator::NSPersistentStoreUbiquitousTransitionTypeContentRemoved;
pub use self::__NSPersistentStoreCoordinator::NSPersistentStoreUbiquitousTransitionTypeInitialImportCompleted;
pub use self::__NSPersistentStoreCoordinator::NSPersistentStoreUbiquitousTransitionTypeKey;
pub use self::__NSPersistentStoreCoordinator::NSReadOnlyPersistentStoreOption;
pub use self::__NSPersistentStoreCoordinator::NSRemovedPersistentStoresKey;
pub use self::__NSPersistentStoreCoordinator::NSSQLiteAnalyzeOption;
pub use self::__NSPersistentStoreCoordinator::NSSQLiteManualVacuumOption;
pub use self::__NSPersistentStoreCoordinator::NSSQLitePragmasOption;
pub use self::__NSPersistentStoreCoordinator::NSSQLiteStoreType;
pub use self::__NSPersistentStoreCoordinator::NSStoreModelVersionHashesKey;
pub use self::__NSPersistentStoreCoordinator::NSStoreModelVersionIdentifiersKey;
#[cfg(not(any(target_os = "ios")))]
pub use self::__NSPersistentStoreCoordinator::NSStorePathKey;
pub use self::__NSPersistentStoreCoordinator::NSStoreTypeKey;
#[cfg(not(any(target_os = "ios")))]
pub use self::__NSPersistentStoreCoordinator::NSStoreUUIDInPathKey;
pub use self::__NSPersistentStoreCoordinator::NSStoreUUIDKey;
pub use self::__NSPersistentStoreCoordinator::NSUUIDChangedPersistentStoresKey;
#[cfg(not(any(target_os = "ios")))]
pub use self::__NSPersistentStoreCoordinator::NSValidateXMLStoreOption;
#[cfg(not(any(target_os = "ios")))]
pub use self::__NSPersistentStoreCoordinator::NSXMLExternalRecordType;
#[cfg(not(any(target_os = "ios")))]
pub use self::__NSPersistentStoreCoordinator::NSXMLStoreType;
#[cfg(feature = "CoreData_NSPersistentStoreDescription")]
pub use self::__NSPersistentStoreDescription::NSPersistentStoreDescription;
pub use self::__NSPersistentStoreRequest::NSBatchDeleteRequestType;
pub use self::__NSPersistentStoreRequest::NSBatchInsertRequestType;
pub use self::__NSPersistentStoreRequest::NSBatchUpdateRequestType;
pub use self::__NSPersistentStoreRequest::NSFetchRequestType;
#[cfg(feature = "CoreData_NSPersistentStoreRequest")]
pub use self::__NSPersistentStoreRequest::NSPersistentStoreRequest;
pub use self::__NSPersistentStoreRequest::NSPersistentStoreRequestType;
pub use self::__NSPersistentStoreRequest::NSSaveRequestType;
#[cfg(feature = "CoreData_NSAsynchronousFetchResult")]
pub use self::__NSPersistentStoreResult::NSAsynchronousFetchResult;
pub use self::__NSPersistentStoreResult::NSBatchDeleteRequestResultType;
#[cfg(feature = "CoreData_NSBatchDeleteResult")]
pub use self::__NSPersistentStoreResult::NSBatchDeleteResult;
pub use self::__NSPersistentStoreResult::NSBatchDeleteResultTypeCount;
pub use self::__NSPersistentStoreResult::NSBatchDeleteResultTypeObjectIDs;
pub use self::__NSPersistentStoreResult::NSBatchDeleteResultTypeStatusOnly;
pub use self::__NSPersistentStoreResult::NSBatchInsertRequestResultType;
pub use self::__NSPersistentStoreResult::NSBatchInsertRequestResultTypeCount;
pub use self::__NSPersistentStoreResult::NSBatchInsertRequestResultTypeObjectIDs;
pub use self::__NSPersistentStoreResult::NSBatchInsertRequestResultTypeStatusOnly;
#[cfg(feature = "CoreData_NSBatchInsertResult")]
pub use self::__NSPersistentStoreResult::NSBatchInsertResult;
pub use self::__NSPersistentStoreResult::NSBatchUpdateRequestResultType;
#[cfg(feature = "CoreData_NSBatchUpdateResult")]
pub use self::__NSPersistentStoreResult::NSBatchUpdateResult;
#[cfg(feature = "CoreData_NSPersistentCloudKitContainerEventResult")]
pub use self::__NSPersistentStoreResult::NSPersistentCloudKitContainerEventResult;
pub use self::__NSPersistentStoreResult::NSPersistentCloudKitContainerEventResultType;
pub use self::__NSPersistentStoreResult::NSPersistentCloudKitContainerEventResultTypeCountEvents;
pub use self::__NSPersistentStoreResult::NSPersistentCloudKitContainerEventResultTypeEvents;
#[cfg(feature = "CoreData_NSPersistentHistoryResult")]
pub use self::__NSPersistentStoreResult::NSPersistentHistoryResult;
pub use self::__NSPersistentStoreResult::NSPersistentHistoryResultType;
pub use self::__NSPersistentStoreResult::NSPersistentHistoryResultTypeChangesOnly;
pub use self::__NSPersistentStoreResult::NSPersistentHistoryResultTypeCount;
pub use self::__NSPersistentStoreResult::NSPersistentHistoryResultTypeObjectIDs;
pub use self::__NSPersistentStoreResult::NSPersistentHistoryResultTypeStatusOnly;
pub use self::__NSPersistentStoreResult::NSPersistentHistoryResultTypeTransactionsAndChanges;
pub use self::__NSPersistentStoreResult::NSPersistentHistoryResultTypeTransactionsOnly;
#[cfg(feature = "CoreData_NSPersistentStoreAsynchronousResult")]
pub use self::__NSPersistentStoreResult::NSPersistentStoreAsynchronousResult;
#[cfg(feature = "CoreData_NSPersistentStoreResult")]
pub use self::__NSPersistentStoreResult::NSPersistentStoreResult;
pub use self::__NSPersistentStoreResult::NSStatusOnlyResultType;
pub use self::__NSPersistentStoreResult::NSUpdatedObjectIDsResultType;
pub use self::__NSPersistentStoreResult::NSUpdatedObjectsCountResultType;
#[cfg(feature = "CoreData_NSPropertyDescription")]
pub use self::__NSPropertyDescription::NSPropertyDescription;
#[cfg(feature = "CoreData_NSPropertyMapping")]
pub use self::__NSPropertyMapping::NSPropertyMapping;
#[cfg(feature = "CoreData_NSQueryGenerationToken")]
pub use self::__NSQueryGenerationToken::NSQueryGenerationToken;
pub use self::__NSRelationshipDescription::NSCascadeDeleteRule;
pub use self::__NSRelationshipDescription::NSDeleteRule;
pub use self::__NSRelationshipDescription::NSDenyDeleteRule;
pub use self::__NSRelationshipDescription::NSNoActionDeleteRule;
pub use self::__NSRelationshipDescription::NSNullifyDeleteRule;
#[cfg(feature = "CoreData_NSRelationshipDescription")]
pub use self::__NSRelationshipDescription::NSRelationshipDescription;
#[cfg(feature = "CoreData_NSSaveChangesRequest")]
pub use self::__NSSaveChangesRequest::NSSaveChangesRequest;
