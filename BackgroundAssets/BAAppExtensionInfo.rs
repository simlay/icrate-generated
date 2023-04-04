//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::BackgroundAssets::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "BackgroundAssets_BAAppExtensionInfo")]
    pub struct BAAppExtensionInfo;

    #[cfg(feature = "BackgroundAssets_BAAppExtensionInfo")]
    unsafe impl ClassType for BAAppExtensionInfo {
        type Super = NSObject;
    }
);

#[cfg(feature = "BackgroundAssets_BAAppExtensionInfo")]
unsafe impl NSCoding for BAAppExtensionInfo {}

#[cfg(feature = "BackgroundAssets_BAAppExtensionInfo")]
unsafe impl NSObjectProtocol for BAAppExtensionInfo {}

#[cfg(feature = "BackgroundAssets_BAAppExtensionInfo")]
unsafe impl NSSecureCoding for BAAppExtensionInfo {}

extern_methods!(
    #[cfg(feature = "BackgroundAssets_BAAppExtensionInfo")]
    unsafe impl BAAppExtensionInfo {
        #[cfg(feature = "Foundation_NSNumber")]
        #[method_id(@__retain_semantics Other restrictedDownloadSizeRemaining)]
        pub unsafe fn restrictedDownloadSizeRemaining(&self) -> Option<Id<NSNumber>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
