//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::PhotoKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "PhotoKit_PHAssetResourceCreationOptions")]
    pub struct PHAssetResourceCreationOptions;

    #[cfg(feature = "PhotoKit_PHAssetResourceCreationOptions")]
    unsafe impl ClassType for PHAssetResourceCreationOptions {
        type Super = NSObject;
    }
);

#[cfg(feature = "PhotoKit_PHAssetResourceCreationOptions")]
unsafe impl NSObjectProtocol for PHAssetResourceCreationOptions {}

extern_methods!(
    #[cfg(feature = "PhotoKit_PHAssetResourceCreationOptions")]
    unsafe impl PHAssetResourceCreationOptions {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other originalFilename)]
        pub unsafe fn originalFilename(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setOriginalFilename:)]
        pub unsafe fn setOriginalFilename(&self, original_filename: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other uniformTypeIdentifier)]
        pub unsafe fn uniformTypeIdentifier(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setUniformTypeIdentifier:)]
        pub unsafe fn setUniformTypeIdentifier(&self, uniform_type_identifier: Option<&NSString>);

        #[method(shouldMoveFile)]
        pub unsafe fn shouldMoveFile(&self) -> bool;

        #[method(setShouldMoveFile:)]
        pub unsafe fn setShouldMoveFile(&self, should_move_file: bool);
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "PhotoKit_PHAssetCreationRequest")]
    pub struct PHAssetCreationRequest;

    #[cfg(feature = "PhotoKit_PHAssetCreationRequest")]
    unsafe impl ClassType for PHAssetCreationRequest {
        #[inherits(PHChangeRequest, NSObject)]
        type Super = PHAssetChangeRequest;
    }
);

#[cfg(feature = "PhotoKit_PHAssetCreationRequest")]
unsafe impl NSObjectProtocol for PHAssetCreationRequest {}

extern_methods!(
    #[cfg(feature = "PhotoKit_PHAssetCreationRequest")]
    unsafe impl PHAssetCreationRequest {
        #[method_id(@__retain_semantics Other creationRequestForAsset)]
        pub unsafe fn creationRequestForAsset() -> Id<Self>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSNumber"))]
        #[method(supportsAssetResourceTypes:)]
        pub unsafe fn supportsAssetResourceTypes(types: &NSArray<NSNumber>) -> bool;

        #[cfg(all(
            feature = "Foundation_NSURL",
            feature = "PhotoKit_PHAssetResourceCreationOptions"
        ))]
        #[method(addResourceWithType:fileURL:options:)]
        pub unsafe fn addResourceWithType_fileURL_options(
            &self,
            r#type: PHAssetResourceType,
            file_url: &NSURL,
            options: Option<&PHAssetResourceCreationOptions>,
        );

        #[cfg(all(
            feature = "Foundation_NSData",
            feature = "PhotoKit_PHAssetResourceCreationOptions"
        ))]
        #[method(addResourceWithType:data:options:)]
        pub unsafe fn addResourceWithType_data_options(
            &self,
            r#type: PHAssetResourceType,
            data: &NSData,
            options: Option<&PHAssetResourceCreationOptions>,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `PHAssetChangeRequest`
    #[cfg(feature = "PhotoKit_PHAssetCreationRequest")]
    unsafe impl PHAssetCreationRequest {
        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other creationRequestForAssetFromImage:)]
        pub unsafe fn creationRequestForAssetFromImage(image: &NSImage) -> Id<Self>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other creationRequestForAssetFromImageAtFileURL:)]
        pub unsafe fn creationRequestForAssetFromImageAtFileURL(
            file_url: &NSURL,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other creationRequestForAssetFromVideoAtFileURL:)]
        pub unsafe fn creationRequestForAssetFromVideoAtFileURL(
            file_url: &NSURL,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "PhotoKit_PHAsset")]
        #[method_id(@__retain_semantics Other changeRequestForAsset:)]
        pub unsafe fn changeRequestForAsset(asset: &PHAsset) -> Id<Self>;
    }
);
