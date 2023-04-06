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
    #[cfg(feature = "MapKit_MKShape")]
    #[cfg(not(any(target_os = "watchos")))]
    pub struct MKShape;

    #[cfg(not(any(target_os = "watchos")))]
    #[cfg(feature = "MapKit_MKShape")]
    unsafe impl ClassType for MKShape {
        type Super = NSObject;
    }
);

#[cfg(feature = "MapKit_MKShape")]
#[cfg(not(any(target_os = "watchos")))]
unsafe impl MKAnnotation for MKShape {}

#[cfg(feature = "MapKit_MKShape")]
#[cfg(not(any(target_os = "watchos")))]
unsafe impl NSObjectProtocol for MKShape {}

extern_methods!(
    #[cfg(feature = "MapKit_MKShape")]
    #[cfg(not(any(target_os = "watchos")))]
    unsafe impl MKShape {
        #[cfg(not(any(target_os = "watchos")))]
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Option<Id<NSString>>;

        #[cfg(not(any(target_os = "watchos")))]
        #[cfg(feature = "Foundation_NSString")]
        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: Option<&NSString>);

        #[cfg(not(any(target_os = "watchos")))]
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other subtitle)]
        pub unsafe fn subtitle(&self) -> Option<Id<NSString>>;

        #[cfg(not(any(target_os = "watchos")))]
        #[cfg(feature = "Foundation_NSString")]
        #[method(setSubtitle:)]
        pub unsafe fn setSubtitle(&self, subtitle: Option<&NSString>);
    }
);
