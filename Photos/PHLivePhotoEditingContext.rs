//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::PhotoKit::*;

typed_enum!(
    pub type PHLivePhotoEditingOption = NSString;
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "PhotoKit_PHLivePhotoEditingContext")]
    pub struct PHLivePhotoEditingContext;

    #[cfg(feature = "PhotoKit_PHLivePhotoEditingContext")]
    unsafe impl ClassType for PHLivePhotoEditingContext {
        type Super = NSObject;
    }
);

#[cfg(feature = "PhotoKit_PHLivePhotoEditingContext")]
unsafe impl NSObjectProtocol for PHLivePhotoEditingContext {}

extern_methods!(
    #[cfg(feature = "PhotoKit_PHLivePhotoEditingContext")]
    unsafe impl PHLivePhotoEditingContext {
        #[cfg(feature = "PhotoKit_PHContentEditingInput")]
        #[method_id(@__retain_semantics Init initWithLivePhotoEditingInput:)]
        pub unsafe fn initWithLivePhotoEditingInput(
            this: Option<Allocated<Self>>,
            live_photo_input: &PHContentEditingInput,
        ) -> Option<Id<Self>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[cfg(feature = "CoreImage_CIImage")]
        #[method_id(@__retain_semantics Other fullSizeImage)]
        pub unsafe fn fullSizeImage(&self) -> Id<CIImage>;

        #[method(audioVolume)]
        pub unsafe fn audioVolume(&self) -> c_float;

        #[method(setAudioVolume:)]
        pub unsafe fn setAudioVolume(&self, audio_volume: c_float);

        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "PhotoKit_PHLivePhoto"
        ))]
        #[method(prepareLivePhotoForPlaybackWithTargetSize:options:completionHandler:)]
        pub unsafe fn prepareLivePhotoForPlaybackWithTargetSize_options_completionHandler(
            &self,
            target_size: CGSize,
            options: Option<&NSDictionary<NSString, Object>>,
            handler: &Block<(*mut PHLivePhoto, *mut NSError), ()>,
        );

        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "PhotoKit_PHContentEditingOutput"
        ))]
        #[method(saveLivePhotoToOutput:options:completionHandler:)]
        pub unsafe fn saveLivePhotoToOutput_options_completionHandler(
            &self,
            output: &PHContentEditingOutput,
            options: Option<&NSDictionary<NSString, Object>>,
            handler: &Block<(Bool, *mut NSError), ()>,
        );

        #[method(cancel)]
        pub unsafe fn cancel(&self);
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum PHLivePhotoFrameType {
        PHLivePhotoFrameTypePhoto = 0,
        PHLivePhotoFrameTypeVideo = 1,
    }
);

extern_protocol!(
    pub unsafe trait PHLivePhotoFrame {
        #[cfg(feature = "CoreImage_CIImage")]
        #[method_id(@__retain_semantics Other image)]
        unsafe fn image(&self) -> Id<CIImage>;

        #[method(type)]
        unsafe fn r#type(&self) -> PHLivePhotoFrameType;

        #[method(renderScale)]
        unsafe fn renderScale(&self) -> CGFloat;
    }

    unsafe impl ProtocolType for dyn PHLivePhotoFrame {}
);

extern_static!(PHLivePhotoShouldRenderAtPlaybackTime: &'static PHLivePhotoEditingOption);

#[cfg(not(any(target_os = "ios", target_os = "tvos")))]
extern_static!(PHLivePhotoEditingErrorDomain: &'static NSString);

#[cfg(not(any(target_os = "ios", target_os = "tvos")))]
ns_enum!(
    #[underlying(NSInteger)]
    pub enum PHLivePhotoEditingErrorCode {
        #[deprecated]
        PHLivePhotoEditingErrorCodeUnknown = 0,
        #[deprecated]
        PHLivePhotoEditingErrorCodeAborted = 1,
    }
);
