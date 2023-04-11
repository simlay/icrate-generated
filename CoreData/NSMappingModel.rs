//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreData_NSMappingModel")]
    pub struct NSMappingModel;

    #[cfg(feature = "CoreData_NSMappingModel")]
    unsafe impl ClassType for NSMappingModel {
        type Super = NSObject;
    }
);

#[cfg(feature = "CoreData_NSMappingModel")]
unsafe impl NSObjectProtocol for NSMappingModel {}

extern_methods!(
    #[cfg(feature = "CoreData_NSMappingModel")]
    unsafe impl NSMappingModel {
        #[cfg(all(
            feature = "CoreData_NSManagedObjectModel",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSBundle"
        ))]
        #[method_id(@__retain_semantics Other mappingModelFromBundles:forSourceModel:destinationModel:)]
        pub unsafe fn mappingModelFromBundles_forSourceModel_destinationModel(
            bundles: Option<&NSArray<NSBundle>>,
            source_model: Option<&NSManagedObjectModel>,
            destination_model: Option<&NSManagedObjectModel>,
        ) -> Option<Id<NSMappingModel>>;

        #[cfg(all(
            feature = "CoreData_NSManagedObjectModel",
            feature = "Foundation_NSError"
        ))]
        #[method_id(@__retain_semantics Other inferredMappingModelForSourceModel:destinationModel:error:_)]
        pub unsafe fn inferredMappingModelForSourceModel_destinationModel_error(
            source_model: &NSManagedObjectModel,
            destination_model: &NSManagedObjectModel,
        ) -> Result<Id<NSMappingModel>, Id<NSError>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Init initWithContentsOfURL:)]
        pub unsafe fn initWithContentsOfURL(
            this: Option<Allocated<Self>>,
            url: Option<&NSURL>,
        ) -> Option<Id<Self>>;

        #[cfg(all(feature = "CoreData_NSEntityMapping", feature = "Foundation_NSArray"))]
        /**
          Returns/sets the collection of entity mappings for the model.  The order of the mappings dictates the order in which they will be processed during migration.
        */
        #[method_id(@__retain_semantics Other entityMappings)]
        pub unsafe fn entityMappings(&self) -> Option<Id<NSArray<NSEntityMapping>>>;

        #[cfg(all(feature = "CoreData_NSEntityMapping", feature = "Foundation_NSArray"))]
        /**
          Returns/sets the collection of entity mappings for the model.  The order of the mappings dictates the order in which they will be processed during migration.
        */
        #[method(setEntityMappings:)]
        pub unsafe fn setEntityMappings(&self, entity_mappings: Option<&NSArray<NSEntityMapping>>);

        #[cfg(all(
            feature = "CoreData_NSEntityMapping",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString"
        ))]
        /**
          Returns a dictionary of the entity mappings for the model, keyed by their respective name.  (This API is provided for quick access to a mapping by name, rather than iterating the ordered entityMapping array.)
        */
        #[method_id(@__retain_semantics Other entityMappingsByName)]
        pub unsafe fn entityMappingsByName(&self) -> Id<NSDictionary<NSString, NSEntityMapping>>;
    }
);
