//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Metal::*;
use crate::MetalFX::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum MTLFXSpatialScalerColorProcessingMode {
        MTLFXSpatialScalerColorProcessingModePerceptual = 0,
        MTLFXSpatialScalerColorProcessingModeLinear = 1,
        MTLFXSpatialScalerColorProcessingModeHDR = 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MetalFX_MTLFXSpatialScalerDescriptor")]
    pub struct MTLFXSpatialScalerDescriptor;

    #[cfg(feature = "MetalFX_MTLFXSpatialScalerDescriptor")]
    unsafe impl ClassType for MTLFXSpatialScalerDescriptor {
        type Super = NSObject;
    }
);

extern_methods!(
    #[cfg(feature = "MetalFX_MTLFXSpatialScalerDescriptor")]
    unsafe impl MTLFXSpatialScalerDescriptor {
        #[method(colorTextureFormat)]
        pub unsafe fn colorTextureFormat(&self) -> MTLPixelFormat;

        #[method(setColorTextureFormat:)]
        pub unsafe fn setColorTextureFormat(&self, color_texture_format: MTLPixelFormat);

        #[method(outputTextureFormat)]
        pub unsafe fn outputTextureFormat(&self) -> MTLPixelFormat;

        #[method(setOutputTextureFormat:)]
        pub unsafe fn setOutputTextureFormat(&self, output_texture_format: MTLPixelFormat);

        #[method(inputWidth)]
        pub unsafe fn inputWidth(&self) -> NSUInteger;

        #[method(setInputWidth:)]
        pub unsafe fn setInputWidth(&self, input_width: NSUInteger);

        #[method(inputHeight)]
        pub unsafe fn inputHeight(&self) -> NSUInteger;

        #[method(setInputHeight:)]
        pub unsafe fn setInputHeight(&self, input_height: NSUInteger);

        #[method(outputWidth)]
        pub unsafe fn outputWidth(&self) -> NSUInteger;

        #[method(setOutputWidth:)]
        pub unsafe fn setOutputWidth(&self, output_width: NSUInteger);

        #[method(outputHeight)]
        pub unsafe fn outputHeight(&self) -> NSUInteger;

        #[method(setOutputHeight:)]
        pub unsafe fn setOutputHeight(&self, output_height: NSUInteger);

        #[method(colorProcessingMode)]
        pub unsafe fn colorProcessingMode(&self) -> MTLFXSpatialScalerColorProcessingMode;

        #[method(setColorProcessingMode:)]
        pub unsafe fn setColorProcessingMode(
            &self,
            color_processing_mode: MTLFXSpatialScalerColorProcessingMode,
        );

        #[method_id(@__retain_semantics New newSpatialScalerWithDevice:)]
        pub unsafe fn newSpatialScalerWithDevice(
            &self,
            device: &MTLDevice,
        ) -> Option<Id<MTLFXSpatialScaler, Shared>>;

        #[method(supportsDevice:)]
        pub unsafe fn supportsDevice(device: &MTLDevice) -> bool;
    }
);

extern_protocol!(
    pub struct MTLFXSpatialScaler;

    unsafe impl ProtocolType for MTLFXSpatialScaler {
        #[method(colorTextureUsage)]
        pub unsafe fn colorTextureUsage(&self) -> MTLTextureUsage;

        #[method(outputTextureUsage)]
        pub unsafe fn outputTextureUsage(&self) -> MTLTextureUsage;

        #[method(inputContentWidth)]
        pub unsafe fn inputContentWidth(&self) -> NSUInteger;

        #[method(setInputContentWidth:)]
        pub unsafe fn setInputContentWidth(&self, input_content_width: NSUInteger);

        #[method(inputContentHeight)]
        pub unsafe fn inputContentHeight(&self) -> NSUInteger;

        #[method(setInputContentHeight:)]
        pub unsafe fn setInputContentHeight(&self, input_content_height: NSUInteger);

        #[method_id(@__retain_semantics Other colorTexture)]
        pub unsafe fn colorTexture(&self) -> Option<Id<MTLTexture, Shared>>;

        #[method(setColorTexture:)]
        pub unsafe fn setColorTexture(&self, color_texture: Option<&MTLTexture>);

        #[method_id(@__retain_semantics Other outputTexture)]
        pub unsafe fn outputTexture(&self) -> Option<Id<MTLTexture, Shared>>;

        #[method(setOutputTexture:)]
        pub unsafe fn setOutputTexture(&self, output_texture: Option<&MTLTexture>);

        #[method(colorTextureFormat)]
        pub unsafe fn colorTextureFormat(&self) -> MTLPixelFormat;

        #[method(outputTextureFormat)]
        pub unsafe fn outputTextureFormat(&self) -> MTLPixelFormat;

        #[method(inputWidth)]
        pub unsafe fn inputWidth(&self) -> NSUInteger;

        #[method(inputHeight)]
        pub unsafe fn inputHeight(&self) -> NSUInteger;

        #[method(outputWidth)]
        pub unsafe fn outputWidth(&self) -> NSUInteger;

        #[method(outputHeight)]
        pub unsafe fn outputHeight(&self) -> NSUInteger;

        #[method(colorProcessingMode)]
        pub unsafe fn colorProcessingMode(&self) -> MTLFXSpatialScalerColorProcessingMode;

        #[method_id(@__retain_semantics Other fence)]
        pub unsafe fn fence(&self) -> Option<Id<MTLFence, Shared>>;

        #[method(setFence:)]
        pub unsafe fn setFence(&self, fence: Option<&MTLFence>);

        #[method(encodeToCommandBuffer:)]
        pub unsafe fn encodeToCommandBuffer(&self, command_buffer: &MTLCommandBuffer);
    }
);
