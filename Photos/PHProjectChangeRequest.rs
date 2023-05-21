//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::PhotoKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "PhotoKit_PHProjectChangeRequest")]
    #[cfg(not(any(target_os = "ios", target_os = "tvos")))]
    pub struct PHProjectChangeRequest;

    #[cfg(not(any(target_os = "ios", target_os = "tvos")))]
    #[cfg(feature = "PhotoKit_PHProjectChangeRequest")]
    unsafe impl ClassType for PHProjectChangeRequest {
        #[inherits(NSObject)]
        type Super = PHChangeRequest;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "PhotoKit_PHProjectChangeRequest")]
#[cfg(not(any(target_os = "ios", target_os = "tvos")))]
unsafe impl NSObjectProtocol for PHProjectChangeRequest {}

#[cfg(not(any(target_os = "ios", target_os = "tvos")))]
extern_methods!(
    #[cfg(feature = "PhotoKit_PHProjectChangeRequest")]
    #[cfg(not(any(target_os = "ios", target_os = "tvos")))]
    unsafe impl PHProjectChangeRequest {
        #[cfg(feature = "PhotoKit_PHProject")]
        #[cfg(not(any(target_os = "ios", target_os = "tvos")))]
        #[method_id(@__retain_semantics Init initWithProject:)]
        pub unsafe fn initWithProject(
            this: Option<Allocated<Self>>,
            project: &PHProject,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[cfg(not(any(target_os = "ios", target_os = "tvos")))]
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[cfg(not(any(target_os = "ios", target_os = "tvos")))]
        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: &NSString);

        #[cfg(feature = "Foundation_NSData")]
        #[cfg(not(any(target_os = "ios", target_os = "tvos")))]
        #[method_id(@__retain_semantics Other projectExtensionData)]
        pub unsafe fn projectExtensionData(&self) -> Id<NSData>;

        #[cfg(feature = "Foundation_NSData")]
        #[cfg(not(any(target_os = "ios", target_os = "tvos")))]
        #[method(setProjectExtensionData:)]
        pub unsafe fn setProjectExtensionData(&self, project_extension_data: &NSData);

        #[cfg(feature = "PhotoKit_PHAsset")]
        #[deprecated]
        #[cfg(not(any(target_os = "ios", target_os = "tvos")))]
        #[method(setKeyAsset:)]
        pub unsafe fn setKeyAsset(&self, key_asset: Option<&PHAsset>);

        #[cfg(feature = "AppKit_NSImage")]
        #[cfg(not(any(target_os = "ios", target_os = "tvos")))]
        #[method(setProjectPreviewImage:)]
        pub unsafe fn setProjectPreviewImage(&self, preview_image: &NSImage);

        #[cfg(not(any(target_os = "ios", target_os = "tvos")))]
        #[method(removeAssets:)]
        pub unsafe fn removeAssets(&self, assets: &ProtocolObject<dyn NSFastEnumeration>);
    }
);

#[cfg(not(any(target_os = "ios", target_os = "tvos")))]
extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "PhotoKit_PHProjectChangeRequest")]
    #[cfg(not(any(target_os = "ios", target_os = "tvos")))]
    unsafe impl PHProjectChangeRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
