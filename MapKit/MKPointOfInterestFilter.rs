//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Contacts::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::MapKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MapKit_MKPointOfInterestFilter")]
    #[cfg(not(any(target_os = "watchos")))]
    pub struct MKPointOfInterestFilter;

    #[cfg(not(any(target_os = "watchos")))]
    #[cfg(feature = "MapKit_MKPointOfInterestFilter")]
    unsafe impl ClassType for MKPointOfInterestFilter {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MapKit_MKPointOfInterestFilter")]
#[cfg(not(any(target_os = "watchos")))]
unsafe impl NSCoding for MKPointOfInterestFilter {}

#[cfg(feature = "MapKit_MKPointOfInterestFilter")]
#[cfg(not(any(target_os = "watchos")))]
unsafe impl NSCopying for MKPointOfInterestFilter {}

#[cfg(feature = "MapKit_MKPointOfInterestFilter")]
#[cfg(not(any(target_os = "watchos")))]
unsafe impl NSObjectProtocol for MKPointOfInterestFilter {}

#[cfg(feature = "MapKit_MKPointOfInterestFilter")]
#[cfg(not(any(target_os = "watchos")))]
unsafe impl NSSecureCoding for MKPointOfInterestFilter {}

#[cfg(not(any(target_os = "watchos")))]
extern_methods!(
    #[cfg(feature = "MapKit_MKPointOfInterestFilter")]
    #[cfg(not(any(target_os = "watchos")))]
    unsafe impl MKPointOfInterestFilter {
        #[method_id(@__retain_semantics Other filterIncludingAllCategories)]
        pub unsafe fn filterIncludingAllCategories() -> Id<MKPointOfInterestFilter>;

        #[method_id(@__retain_semantics Other filterExcludingAllCategories)]
        pub unsafe fn filterExcludingAllCategories() -> Id<MKPointOfInterestFilter>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Init initIncludingCategories:)]
        pub unsafe fn initIncludingCategories(
            this: Option<Allocated<Self>>,
            categories: &NSArray<MKPointOfInterestCategory>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Init initExcludingCategories:)]
        pub unsafe fn initExcludingCategories(
            this: Option<Allocated<Self>>,
            categories: &NSArray<MKPointOfInterestCategory>,
        ) -> Id<Self>;

        #[method(includesCategory:)]
        pub unsafe fn includesCategory(&self, category: &MKPointOfInterestCategory) -> bool;

        #[method(excludesCategory:)]
        pub unsafe fn excludesCategory(&self, category: &MKPointOfInterestCategory) -> bool;
    }
);

#[cfg(not(any(target_os = "watchos")))]
extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MapKit_MKPointOfInterestFilter")]
    #[cfg(not(any(target_os = "watchos")))]
    unsafe impl MKPointOfInterestFilter {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
