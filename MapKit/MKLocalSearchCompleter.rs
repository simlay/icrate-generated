//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Contacts::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::MapKit::*;

#[cfg(not(any(target_os = "watchos")))]
ns_enum!(
    #[underlying(NSInteger)]
    pub enum MKSearchCompletionFilterType {
        #[deprecated = "Use MKLocalSearchCompleterResultType"]
        #[cfg(not(any(target_os = "watchos")))]
        MKSearchCompletionFilterTypeLocationsAndQueries = 0,
        #[deprecated = "Use MKLocalSearchCompleterResultType"]
        #[cfg(not(any(target_os = "watchos")))]
        MKSearchCompletionFilterTypeLocationsOnly = 1,
    }
);

#[cfg(not(any(target_os = "watchos")))]
ns_options!(
    #[underlying(NSUInteger)]
    pub enum MKLocalSearchCompleterResultType {
        #[cfg(not(any(target_os = "watchos")))]
        MKLocalSearchCompleterResultTypeAddress = 1 << 0,
        #[cfg(not(any(target_os = "watchos")))]
        MKLocalSearchCompleterResultTypePointOfInterest = 1 << 1,
        #[cfg(not(any(target_os = "watchos")))]
        MKLocalSearchCompleterResultTypeQuery = 1 << 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MapKit_MKLocalSearchCompleter")]
    #[cfg(not(any(target_os = "watchos")))]
    pub struct MKLocalSearchCompleter;

    #[cfg(not(any(target_os = "watchos")))]
    #[cfg(feature = "MapKit_MKLocalSearchCompleter")]
    unsafe impl ClassType for MKLocalSearchCompleter {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MapKit_MKLocalSearchCompleter")]
#[cfg(not(any(target_os = "watchos")))]
unsafe impl NSObjectProtocol for MKLocalSearchCompleter {}

#[cfg(not(any(target_os = "watchos")))]
extern_methods!(
    #[cfg(feature = "MapKit_MKLocalSearchCompleter")]
    #[cfg(not(any(target_os = "watchos")))]
    unsafe impl MKLocalSearchCompleter {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other queryFragment)]
        pub unsafe fn queryFragment(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setQueryFragment:)]
        pub unsafe fn setQueryFragment(&self, query_fragment: &NSString);

        #[method(region)]
        pub unsafe fn region(&self) -> MKCoordinateRegion;

        #[method(setRegion:)]
        pub unsafe fn setRegion(&self, region: MKCoordinateRegion);

        #[deprecated = "Use resultTypes"]
        #[cfg(not(any(target_os = "watchos")))]
        #[method(filterType)]
        pub unsafe fn filterType(&self) -> MKSearchCompletionFilterType;

        #[deprecated = "Use resultTypes"]
        #[cfg(not(any(target_os = "watchos")))]
        #[method(setFilterType:)]
        pub unsafe fn setFilterType(&self, filter_type: MKSearchCompletionFilterType);

        #[cfg(not(any(target_os = "watchos")))]
        #[method(resultTypes)]
        pub unsafe fn resultTypes(&self) -> MKLocalSearchCompleterResultType;

        #[cfg(not(any(target_os = "watchos")))]
        #[method(setResultTypes:)]
        pub unsafe fn setResultTypes(&self, result_types: MKLocalSearchCompleterResultType);

        #[cfg(feature = "MapKit_MKPointOfInterestFilter")]
        #[cfg(not(any(target_os = "watchos")))]
        #[method_id(@__retain_semantics Other pointOfInterestFilter)]
        pub unsafe fn pointOfInterestFilter(&self) -> Option<Id<MKPointOfInterestFilter>>;

        #[cfg(feature = "MapKit_MKPointOfInterestFilter")]
        #[cfg(not(any(target_os = "watchos")))]
        #[method(setPointOfInterestFilter:)]
        pub unsafe fn setPointOfInterestFilter(
            &self,
            point_of_interest_filter: Option<&MKPointOfInterestFilter>,
        );

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Id<ProtocolObject<dyn MKLocalSearchCompleterDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn MKLocalSearchCompleterDelegate>>,
        );

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "MapKit_MKLocalSearchCompletion"
        ))]
        #[method_id(@__retain_semantics Other results)]
        pub unsafe fn results(&self) -> Id<NSArray<MKLocalSearchCompletion>>;

        #[method(isSearching)]
        pub unsafe fn isSearching(&self) -> bool;

        #[method(cancel)]
        pub unsafe fn cancel(&self);
    }
);

#[cfg(not(any(target_os = "watchos")))]
extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MapKit_MKLocalSearchCompleter")]
    #[cfg(not(any(target_os = "watchos")))]
    unsafe impl MKLocalSearchCompleter {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

#[cfg(not(any(target_os = "watchos")))]
extern_protocol!(
    pub unsafe trait MKLocalSearchCompleterDelegate: NSObjectProtocol {
        #[cfg(feature = "MapKit_MKLocalSearchCompleter")]
        #[cfg(not(any(target_os = "watchos")))]
        #[optional]
        #[method(completerDidUpdateResults:)]
        unsafe fn completerDidUpdateResults(&self, completer: &MKLocalSearchCompleter);

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "MapKit_MKLocalSearchCompleter"
        ))]
        #[cfg(not(any(target_os = "watchos")))]
        #[optional]
        #[method(completer:didFailWithError:)]
        unsafe fn completer_didFailWithError(
            &self,
            completer: &MKLocalSearchCompleter,
            error: &NSError,
        );
    }

    unsafe impl ProtocolType for dyn MKLocalSearchCompleterDelegate {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MapKit_MKLocalSearchCompletion")]
    #[cfg(not(any(target_os = "watchos")))]
    pub struct MKLocalSearchCompletion;

    #[cfg(not(any(target_os = "watchos")))]
    #[cfg(feature = "MapKit_MKLocalSearchCompletion")]
    unsafe impl ClassType for MKLocalSearchCompletion {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MapKit_MKLocalSearchCompletion")]
#[cfg(not(any(target_os = "watchos")))]
unsafe impl NSObjectProtocol for MKLocalSearchCompletion {}

#[cfg(not(any(target_os = "watchos")))]
extern_methods!(
    #[cfg(feature = "MapKit_MKLocalSearchCompletion")]
    #[cfg(not(any(target_os = "watchos")))]
    unsafe impl MKLocalSearchCompletion {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Id<NSString>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSValue"))]
        #[method_id(@__retain_semantics Other titleHighlightRanges)]
        pub unsafe fn titleHighlightRanges(&self) -> Id<NSArray<NSValue>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other subtitle)]
        pub unsafe fn subtitle(&self) -> Id<NSString>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSValue"))]
        #[method_id(@__retain_semantics Other subtitleHighlightRanges)]
        pub unsafe fn subtitleHighlightRanges(&self) -> Id<NSArray<NSValue>>;
    }
);

#[cfg(not(any(target_os = "watchos")))]
extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MapKit_MKLocalSearchCompletion")]
    #[cfg(not(any(target_os = "watchos")))]
    unsafe impl MKLocalSearchCompletion {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    #[cfg(feature = "MapKit_MKLocalSearchRequest")]
    unsafe impl MKLocalSearchRequest {
        #[cfg(feature = "MapKit_MKLocalSearchCompletion")]
        #[cfg(not(any(target_os = "watchos")))]
        #[method_id(@__retain_semantics Init initWithCompletion:)]
        pub unsafe fn initWithCompletion(
            this: Option<Allocated<Self>>,
            completion: &MKLocalSearchCompletion,
        ) -> Id<Self>;
    }
);
