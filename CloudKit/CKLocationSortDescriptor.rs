//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CloudKit::*;
use crate::CoreLocation::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CloudKit_CKLocationSortDescriptor")]
    pub struct CKLocationSortDescriptor;

    #[cfg(feature = "CloudKit_CKLocationSortDescriptor")]
    unsafe impl ClassType for CKLocationSortDescriptor {
        #[inherits(NSObject)]
        type Super = NSSortDescriptor;
    }
);

#[cfg(feature = "CloudKit_CKLocationSortDescriptor")]
unsafe impl NSCoding for CKLocationSortDescriptor {}

#[cfg(feature = "CloudKit_CKLocationSortDescriptor")]
unsafe impl NSObjectProtocol for CKLocationSortDescriptor {}

#[cfg(feature = "CloudKit_CKLocationSortDescriptor")]
unsafe impl NSSecureCoding for CKLocationSortDescriptor {}

extern_methods!(
    #[cfg(feature = "CloudKit_CKLocationSortDescriptor")]
    unsafe impl CKLocationSortDescriptor {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[cfg(all(feature = "CoreLocation_CLLocation", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Init initWithKey:relativeLocation:)]
        pub unsafe fn initWithKey_relativeLocation(
            this: Option<Allocated<Self>>,
            key: &NSString,
            relative_location: &CLLocation,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Option<Allocated<Self>>, a_decoder: &NSCoder)
            -> Id<Self>;

        #[cfg(feature = "CoreLocation_CLLocation")]
        #[method_id(@__retain_semantics Other relativeLocation)]
        pub unsafe fn relativeLocation(&self) -> Id<CLLocation>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSSortDescriptor`
    #[cfg(feature = "CloudKit_CKLocationSortDescriptor")]
    unsafe impl CKLocationSortDescriptor {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other sortDescriptorWithKey:ascending:)]
        pub unsafe fn sortDescriptorWithKey_ascending(
            key: Option<&NSString>,
            ascending: bool,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other sortDescriptorWithKey:ascending:selector:)]
        pub unsafe fn sortDescriptorWithKey_ascending_selector(
            key: Option<&NSString>,
            ascending: bool,
            selector: Option<Sel>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithKey:ascending:)]
        pub unsafe fn initWithKey_ascending(
            this: Option<Allocated<Self>>,
            key: Option<&NSString>,
            ascending: bool,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithKey:ascending:selector:)]
        pub unsafe fn initWithKey_ascending_selector(
            this: Option<Allocated<Self>>,
            key: Option<&NSString>,
            ascending: bool,
            selector: Option<Sel>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other sortDescriptorWithKey:ascending:comparator:)]
        pub unsafe fn sortDescriptorWithKey_ascending_comparator(
            key: Option<&NSString>,
            ascending: bool,
            cmptr: NSComparator,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithKey:ascending:comparator:)]
        pub unsafe fn initWithKey_ascending_comparator(
            this: Option<Allocated<Self>>,
            key: Option<&NSString>,
            ascending: bool,
            cmptr: NSComparator,
        ) -> Id<Self>;
    }
);
