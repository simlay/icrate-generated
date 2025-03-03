//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::HealthKit::*;
use crate::UniformTypeIdentifiers::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "HealthKit_HKAnchoredObjectQuery")]
    pub struct HKAnchoredObjectQuery;

    #[cfg(feature = "HealthKit_HKAnchoredObjectQuery")]
    unsafe impl ClassType for HKAnchoredObjectQuery {
        #[inherits(NSObject)]
        type Super = HKQuery;
    }
);

#[cfg(feature = "HealthKit_HKAnchoredObjectQuery")]
unsafe impl NSObjectProtocol for HKAnchoredObjectQuery {}

extern_methods!(
    #[cfg(feature = "HealthKit_HKAnchoredObjectQuery")]
    unsafe impl HKAnchoredObjectQuery {
        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "HealthKit_HKDeletedObject",
            feature = "HealthKit_HKQueryAnchor",
            feature = "HealthKit_HKSample"
        ))]
        #[method(updateHandler)]
        pub unsafe fn updateHandler(
            &self,
        ) -> *mut Block<
            (
                NonNull<HKAnchoredObjectQuery>,
                *mut NSArray<HKSample>,
                *mut NSArray<HKDeletedObject>,
                *mut HKQueryAnchor,
                *mut NSError,
            ),
            (),
        >;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "HealthKit_HKDeletedObject",
            feature = "HealthKit_HKQueryAnchor",
            feature = "HealthKit_HKSample"
        ))]
        #[method(setUpdateHandler:)]
        pub unsafe fn setUpdateHandler(
            &self,
            update_handler: Option<
                &Block<
                    (
                        NonNull<HKAnchoredObjectQuery>,
                        *mut NSArray<HKSample>,
                        *mut NSArray<HKDeletedObject>,
                        *mut HKQueryAnchor,
                        *mut NSError,
                    ),
                    (),
                >,
            >,
        );

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "Foundation_NSPredicate",
            feature = "HealthKit_HKDeletedObject",
            feature = "HealthKit_HKQueryAnchor",
            feature = "HealthKit_HKSample",
            feature = "HealthKit_HKSampleType"
        ))]
        #[method_id(@__retain_semantics Init initWithType:predicate:anchor:limit:resultsHandler:)]
        pub unsafe fn initWithType_predicate_anchor_limit_resultsHandler(
            this: Option<Allocated<Self>>,
            r#type: &HKSampleType,
            predicate: Option<&NSPredicate>,
            anchor: Option<&HKQueryAnchor>,
            limit: NSUInteger,
            handler: &Block<
                (
                    NonNull<HKAnchoredObjectQuery>,
                    *mut NSArray<HKSample>,
                    *mut NSArray<HKDeletedObject>,
                    *mut HKQueryAnchor,
                    *mut NSError,
                ),
                (),
            >,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "Foundation_NSPredicate",
            feature = "HealthKit_HKSample",
            feature = "HealthKit_HKSampleType"
        ))]
        #[deprecated]
        #[method_id(@__retain_semantics Init initWithType:predicate:anchor:limit:completionHandler:)]
        pub unsafe fn initWithType_predicate_anchor_limit_completionHandler(
            this: Option<Allocated<Self>>,
            r#type: &HKSampleType,
            predicate: Option<&NSPredicate>,
            anchor: NSUInteger,
            limit: NSUInteger,
            handler: &Block<
                (
                    NonNull<HKAnchoredObjectQuery>,
                    *mut NSArray<HKSample>,
                    NSUInteger,
                    *mut NSError,
                ),
                (),
            >,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "HealthKit_HKDeletedObject",
            feature = "HealthKit_HKQueryAnchor",
            feature = "HealthKit_HKQueryDescriptor",
            feature = "HealthKit_HKSample"
        ))]
        #[method_id(@__retain_semantics Init initWithQueryDescriptors:anchor:limit:resultsHandler:)]
        pub unsafe fn initWithQueryDescriptors_anchor_limit_resultsHandler(
            this: Option<Allocated<Self>>,
            query_descriptors: &NSArray<HKQueryDescriptor>,
            anchor: Option<&HKQueryAnchor>,
            limit: NSInteger,
            handler: &Block<
                (
                    NonNull<HKAnchoredObjectQuery>,
                    *mut NSArray<HKSample>,
                    *mut NSArray<HKDeletedObject>,
                    *mut HKQueryAnchor,
                    *mut NSError,
                ),
                (),
            >,
        ) -> Id<Self>;
    }
);
