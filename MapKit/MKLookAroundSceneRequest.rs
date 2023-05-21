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
    #[cfg(feature = "MapKit_MKLookAroundSceneRequest")]
    #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
    pub struct MKLookAroundSceneRequest;

    #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
    #[cfg(feature = "MapKit_MKLookAroundSceneRequest")]
    unsafe impl ClassType for MKLookAroundSceneRequest {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MapKit_MKLookAroundSceneRequest")]
#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
unsafe impl NSObjectProtocol for MKLookAroundSceneRequest {}

#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
extern_methods!(
    #[cfg(feature = "MapKit_MKLookAroundSceneRequest")]
    #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
    unsafe impl MKLookAroundSceneRequest {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithCoordinate:)]
        pub unsafe fn initWithCoordinate(
            this: Option<Allocated<Self>>,
            coordinate: CLLocationCoordinate2D,
        ) -> Id<Self>;

        #[cfg(feature = "MapKit_MKMapItem")]
        #[method_id(@__retain_semantics Init initWithMapItem:)]
        pub unsafe fn initWithMapItem(
            this: Option<Allocated<Self>>,
            map_item: &MKMapItem,
        ) -> Id<Self>;

        #[method(coordinate)]
        pub unsafe fn coordinate(&self) -> CLLocationCoordinate2D;

        #[cfg(feature = "MapKit_MKMapItem")]
        #[method_id(@__retain_semantics Other mapItem)]
        pub unsafe fn mapItem(&self) -> Option<Id<MKMapItem>>;

        #[method(isCancelled)]
        pub unsafe fn isCancelled(&self) -> bool;

        #[method(isLoading)]
        pub unsafe fn isLoading(&self) -> bool;

        #[cfg(all(feature = "Foundation_NSError", feature = "MapKit_MKLookAroundScene"))]
        #[method(getSceneWithCompletionHandler:)]
        pub unsafe fn getSceneWithCompletionHandler(
            &self,
            completion_handler: &Block<(*mut MKLookAroundScene, *mut NSError), ()>,
        );

        #[method(cancel)]
        pub unsafe fn cancel(&self);
    }
);
