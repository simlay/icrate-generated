//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Contacts::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::MapKit::*;

pub type MKLocalSearchCompletionHandler =
    *mut Block<(*mut MKLocalSearchResponse, *mut NSError), ()>;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MapKit_MKLocalSearch")]
    #[cfg(not(any(target_os = "watchos")))]
    pub struct MKLocalSearch;

    #[cfg(not(any(target_os = "watchos")))]
    #[cfg(feature = "MapKit_MKLocalSearch")]
    unsafe impl ClassType for MKLocalSearch {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MapKit_MKLocalSearch")]
#[cfg(not(any(target_os = "watchos")))]
unsafe impl NSObjectProtocol for MKLocalSearch {}

#[cfg(not(any(target_os = "watchos")))]
extern_methods!(
    #[cfg(feature = "MapKit_MKLocalSearch")]
    #[cfg(not(any(target_os = "watchos")))]
    unsafe impl MKLocalSearch {
        #[cfg(feature = "MapKit_MKLocalSearchRequest")]
        #[method_id(@__retain_semantics Init initWithRequest:)]
        pub unsafe fn initWithRequest(
            this: Option<Allocated<Self>>,
            request: &MKLocalSearchRequest,
        ) -> Id<Self>;

        #[cfg(feature = "MapKit_MKLocalPointsOfInterestRequest")]
        #[cfg(not(any(target_os = "watchos")))]
        #[method_id(@__retain_semantics Init initWithPointsOfInterestRequest:)]
        pub unsafe fn initWithPointsOfInterestRequest(
            this: Option<Allocated<Self>>,
            request: &MKLocalPointsOfInterestRequest,
        ) -> Id<Self>;

        #[method(startWithCompletionHandler:)]
        pub unsafe fn startWithCompletionHandler(
            &self,
            completion_handler: MKLocalSearchCompletionHandler,
        );

        #[method(cancel)]
        pub unsafe fn cancel(&self);

        #[method(isSearching)]
        pub unsafe fn isSearching(&self) -> bool;
    }
);

#[cfg(not(any(target_os = "watchos")))]
extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MapKit_MKLocalSearch")]
    #[cfg(not(any(target_os = "watchos")))]
    unsafe impl MKLocalSearch {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
