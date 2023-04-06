//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::MediaPlayer::*;

pub type MPMediaEntityPersistentID = u64;

#[cfg(not(any(target_os = "watchos")))]
extern_static!(MPMediaEntityPropertyPersistentID: &'static NSString);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MediaPlayer_MPMediaEntity")]
    #[cfg(not(any(target_os = "macos", target_os = "watchos")))]
    pub struct MPMediaEntity;

    #[cfg(not(any(target_os = "macos", target_os = "watchos")))]
    #[cfg(feature = "MediaPlayer_MPMediaEntity")]
    unsafe impl ClassType for MPMediaEntity {
        type Super = NSObject;
    }
);

#[cfg(feature = "MediaPlayer_MPMediaEntity")]
#[cfg(not(any(target_os = "macos", target_os = "watchos")))]
unsafe impl NSCoding for MPMediaEntity {}

#[cfg(feature = "MediaPlayer_MPMediaEntity")]
#[cfg(not(any(target_os = "macos", target_os = "watchos")))]
unsafe impl NSObjectProtocol for MPMediaEntity {}

#[cfg(feature = "MediaPlayer_MPMediaEntity")]
#[cfg(not(any(target_os = "macos", target_os = "watchos")))]
unsafe impl NSSecureCoding for MPMediaEntity {}

extern_methods!(
    #[cfg(feature = "MediaPlayer_MPMediaEntity")]
    #[cfg(not(any(target_os = "macos", target_os = "watchos")))]
    unsafe impl MPMediaEntity {
        #[cfg(not(any(target_os = "macos", target_os = "watchos")))]
        #[cfg(feature = "Foundation_NSString")]
        #[method(canFilterByProperty:)]
        pub unsafe fn canFilterByProperty(property: &NSString) -> bool;

        #[cfg(not(any(target_os = "macos", target_os = "watchos")))]
        #[cfg(all(feature = "Foundation_NSSet", feature = "Foundation_NSString"))]
        #[cfg(not(any(target_os = "macos", target_os = "tvos", target_os = "watchos")))]
        #[method(enumerateValuesForProperties:usingBlock:)]
        pub unsafe fn enumerateValuesForProperties_usingBlock(
            &self,
            properties: &NSSet<NSString>,
            block: &Block<(NonNull<NSString>, NonNull<Object>, NonNull<Bool>), ()>,
        );

        #[cfg(not(any(target_os = "macos", target_os = "watchos")))]
        #[cfg(not(any(target_os = "macos", target_os = "watchos")))]
        #[method_id(@__retain_semantics Other objectForKeyedSubscript:)]
        pub unsafe fn objectForKeyedSubscript(&self, key: &Object) -> Option<Id<Object>>;

        #[cfg(not(any(target_os = "macos", target_os = "watchos")))]
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other valueForProperty:)]
        pub unsafe fn valueForProperty(&self, property: &NSString) -> Option<Id<Object>>;

        #[cfg(not(any(target_os = "macos", target_os = "watchos")))]
        #[cfg(not(any(target_os = "macos", target_os = "watchos")))]
        #[method(persistentID)]
        pub unsafe fn persistentID(&self) -> MPMediaEntityPersistentID;
    }
);
