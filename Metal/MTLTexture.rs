//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Metal::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum MTLTextureType {
        MTLTextureType1D = 0,
        MTLTextureType1DArray = 1,
        MTLTextureType2D = 2,
        MTLTextureType2DArray = 3,
        MTLTextureType2DMultisample = 4,
        MTLTextureTypeCube = 5,
        MTLTextureTypeCubeArray = 6,
        MTLTextureType3D = 7,
        MTLTextureType2DMultisampleArray = 8,
        MTLTextureTypeTextureBuffer = 9,
    }
);

ns_enum!(
    #[underlying(u8)]
    pub enum MTLTextureSwizzle {
        MTLTextureSwizzleZero = 0,
        MTLTextureSwizzleOne = 1,
        MTLTextureSwizzleRed = 2,
        MTLTextureSwizzleGreen = 3,
        MTLTextureSwizzleBlue = 4,
        MTLTextureSwizzleAlpha = 5,
    }
);

extern_struct!(
    #[encoding_name("?")]
    pub struct MTLTextureSwizzleChannels {
        pub red: MTLTextureSwizzle,
        pub green: MTLTextureSwizzle,
        pub blue: MTLTextureSwizzle,
        pub alpha: MTLTextureSwizzle,
    }
);

inline_fn!(
    pub unsafe fn MTLTextureSwizzleChannelsMake(
        r: MTLTextureSwizzle,
        g: MTLTextureSwizzle,
        b: MTLTextureSwizzle,
        a: MTLTextureSwizzle,
    ) -> MTLTextureSwizzleChannels {
        todo!()
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLSharedTextureHandle")]
    pub struct MTLSharedTextureHandle;

    #[cfg(feature = "Metal_MTLSharedTextureHandle")]
    unsafe impl ClassType for MTLSharedTextureHandle {
        type Super = NSObject;
    }
);

#[cfg(feature = "Metal_MTLSharedTextureHandle")]
unsafe impl NSCoding for MTLSharedTextureHandle {}

#[cfg(feature = "Metal_MTLSharedTextureHandle")]
unsafe impl NSObjectProtocol for MTLSharedTextureHandle {}

#[cfg(feature = "Metal_MTLSharedTextureHandle")]
unsafe impl NSSecureCoding for MTLSharedTextureHandle {}

extern_methods!(
    #[cfg(feature = "Metal_MTLSharedTextureHandle")]
    unsafe impl MTLSharedTextureHandle {
        #[method_id(@__retain_semantics Other device)]
        pub fn device(&self) -> Id<ProtocolObject<dyn MTLDevice>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other label)]
        pub fn label(&self) -> Option<Id<NSString>>;
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum MTLTextureUsage {
        MTLTextureUsageUnknown = 0x0000,
        MTLTextureUsageShaderRead = 0x0001,
        MTLTextureUsageShaderWrite = 0x0002,
        MTLTextureUsageRenderTarget = 0x0004,
        MTLTextureUsagePixelFormatView = 0x0010,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum MTLTextureCompressionType {
        MTLTextureCompressionTypeLossless = 0,
        MTLTextureCompressionTypeLossy = 1,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLTextureDescriptor")]
    pub struct MTLTextureDescriptor;

    #[cfg(feature = "Metal_MTLTextureDescriptor")]
    unsafe impl ClassType for MTLTextureDescriptor {
        type Super = NSObject;
    }
);

#[cfg(feature = "Metal_MTLTextureDescriptor")]
unsafe impl NSObjectProtocol for MTLTextureDescriptor {}

extern_methods!(
    #[cfg(feature = "Metal_MTLTextureDescriptor")]
    unsafe impl MTLTextureDescriptor {
        #[method_id(@__retain_semantics Other texture2DDescriptorWithPixelFormat:width:height:mipmapped:)]
        pub unsafe fn texture2DDescriptorWithPixelFormat_width_height_mipmapped(
            pixel_format: MTLPixelFormat,
            width: NSUInteger,
            height: NSUInteger,
            mipmapped: bool,
        ) -> Id<MTLTextureDescriptor>;

        #[method_id(@__retain_semantics Other textureCubeDescriptorWithPixelFormat:size:mipmapped:)]
        pub unsafe fn textureCubeDescriptorWithPixelFormat_size_mipmapped(
            pixel_format: MTLPixelFormat,
            size: NSUInteger,
            mipmapped: bool,
        ) -> Id<MTLTextureDescriptor>;

        #[method_id(@__retain_semantics Other textureBufferDescriptorWithPixelFormat:width:resourceOptions:usage:)]
        pub unsafe fn textureBufferDescriptorWithPixelFormat_width_resourceOptions_usage(
            pixel_format: MTLPixelFormat,
            width: NSUInteger,
            resource_options: MTLResourceOptions,
            usage: MTLTextureUsage,
        ) -> Id<MTLTextureDescriptor>;

        #[method(textureType)]
        pub fn textureType(&self) -> MTLTextureType;

        #[method(setTextureType:)]
        pub fn setTextureType(&self, texture_type: MTLTextureType);

        #[method(pixelFormat)]
        pub fn pixelFormat(&self) -> MTLPixelFormat;

        #[method(setPixelFormat:)]
        pub fn setPixelFormat(&self, pixel_format: MTLPixelFormat);

        #[method(width)]
        pub fn width(&self) -> NSUInteger;

        #[method(setWidth:)]
        pub unsafe fn setWidth(&self, width: NSUInteger);

        #[method(height)]
        pub fn height(&self) -> NSUInteger;

        #[method(setHeight:)]
        pub unsafe fn setHeight(&self, height: NSUInteger);

        #[method(depth)]
        pub fn depth(&self) -> NSUInteger;

        #[method(setDepth:)]
        pub unsafe fn setDepth(&self, depth: NSUInteger);

        #[method(mipmapLevelCount)]
        pub fn mipmapLevelCount(&self) -> NSUInteger;

        #[method(setMipmapLevelCount:)]
        pub unsafe fn setMipmapLevelCount(&self, mipmap_level_count: NSUInteger);

        #[method(sampleCount)]
        pub fn sampleCount(&self) -> NSUInteger;

        #[method(setSampleCount:)]
        pub unsafe fn setSampleCount(&self, sample_count: NSUInteger);

        #[method(arrayLength)]
        pub fn arrayLength(&self) -> NSUInteger;

        #[method(setArrayLength:)]
        pub unsafe fn setArrayLength(&self, array_length: NSUInteger);

        #[method(resourceOptions)]
        pub fn resourceOptions(&self) -> MTLResourceOptions;

        #[method(setResourceOptions:)]
        pub fn setResourceOptions(&self, resource_options: MTLResourceOptions);

        #[method(cpuCacheMode)]
        pub fn cpuCacheMode(&self) -> MTLCPUCacheMode;

        #[method(setCpuCacheMode:)]
        pub fn setCpuCacheMode(&self, cpu_cache_mode: MTLCPUCacheMode);

        #[method(storageMode)]
        pub fn storageMode(&self) -> MTLStorageMode;

        #[method(setStorageMode:)]
        pub fn setStorageMode(&self, storage_mode: MTLStorageMode);

        #[method(hazardTrackingMode)]
        pub fn hazardTrackingMode(&self) -> MTLHazardTrackingMode;

        #[method(setHazardTrackingMode:)]
        pub fn setHazardTrackingMode(&self, hazard_tracking_mode: MTLHazardTrackingMode);

        #[method(usage)]
        pub fn usage(&self) -> MTLTextureUsage;

        #[method(setUsage:)]
        pub fn setUsage(&self, usage: MTLTextureUsage);

        #[method(allowGPUOptimizedContents)]
        pub fn allowGPUOptimizedContents(&self) -> bool;

        #[method(setAllowGPUOptimizedContents:)]
        pub fn setAllowGPUOptimizedContents(&self, allow_gpu_optimized_contents: bool);

        #[method(compressionType)]
        pub unsafe fn compressionType(&self) -> MTLTextureCompressionType;

        #[method(setCompressionType:)]
        pub unsafe fn setCompressionType(&self, compression_type: MTLTextureCompressionType);

        #[method(swizzle)]
        pub fn swizzle(&self) -> MTLTextureSwizzleChannels;

        #[method(setSwizzle:)]
        pub fn setSwizzle(&self, swizzle: MTLTextureSwizzleChannels);
    }
);

extern_protocol!(
    pub unsafe trait MTLTexture: MTLResource {
        #[deprecated = "Use parentTexture or buffer instead"]
        #[method_id(@__retain_semantics Other rootResource)]
        fn rootResource(&self) -> Option<Id<ProtocolObject<dyn MTLResource>>>;

        #[method_id(@__retain_semantics Other parentTexture)]
        fn parentTexture(&self) -> Option<Id<ProtocolObject<dyn MTLTexture>>>;

        #[method(parentRelativeLevel)]
        fn parentRelativeLevel(&self) -> NSUInteger;

        #[method(parentRelativeSlice)]
        fn parentRelativeSlice(&self) -> NSUInteger;

        #[method_id(@__retain_semantics Other buffer)]
        fn buffer(&self) -> Option<Id<ProtocolObject<dyn MTLBuffer>>>;

        #[method(bufferOffset)]
        fn bufferOffset(&self) -> NSUInteger;

        #[method(bufferBytesPerRow)]
        fn bufferBytesPerRow(&self) -> NSUInteger;

        #[method(iosurfacePlane)]
        fn iosurfacePlane(&self) -> NSUInteger;

        #[method(textureType)]
        fn textureType(&self) -> MTLTextureType;

        #[method(pixelFormat)]
        fn pixelFormat(&self) -> MTLPixelFormat;

        #[method(width)]
        fn width(&self) -> NSUInteger;

        #[method(height)]
        fn height(&self) -> NSUInteger;

        #[method(depth)]
        fn depth(&self) -> NSUInteger;

        #[method(mipmapLevelCount)]
        fn mipmapLevelCount(&self) -> NSUInteger;

        #[method(sampleCount)]
        fn sampleCount(&self) -> NSUInteger;

        #[method(arrayLength)]
        fn arrayLength(&self) -> NSUInteger;

        #[method(usage)]
        fn usage(&self) -> MTLTextureUsage;

        #[method(isShareable)]
        fn isShareable(&self) -> bool;

        #[method(isFramebufferOnly)]
        fn isFramebufferOnly(&self) -> bool;

        #[optional]
        #[method(firstMipmapInTail)]
        fn firstMipmapInTail(&self) -> NSUInteger;

        #[optional]
        #[method(tailSizeInBytes)]
        fn tailSizeInBytes(&self) -> NSUInteger;

        #[optional]
        #[method(isSparse)]
        fn isSparse(&self) -> bool;

        #[method(allowGPUOptimizedContents)]
        fn allowGPUOptimizedContents(&self) -> bool;

        #[method(compressionType)]
        unsafe fn compressionType(&self) -> MTLTextureCompressionType;

        #[method(gpuResourceID)]
        unsafe fn gpuResourceID(&self) -> MTLResourceID;

        #[method(getBytes:bytesPerRow:bytesPerImage:fromRegion:mipmapLevel:slice:)]
        unsafe fn getBytes_bytesPerRow_bytesPerImage_fromRegion_mipmapLevel_slice(
            &self,
            pixel_bytes: NonNull<c_void>,
            bytes_per_row: NSUInteger,
            bytes_per_image: NSUInteger,
            region: MTLRegion,
            level: NSUInteger,
            slice: NSUInteger,
        );

        #[method(replaceRegion:mipmapLevel:slice:withBytes:bytesPerRow:bytesPerImage:)]
        unsafe fn replaceRegion_mipmapLevel_slice_withBytes_bytesPerRow_bytesPerImage(
            &self,
            region: MTLRegion,
            level: NSUInteger,
            slice: NSUInteger,
            pixel_bytes: NonNull<c_void>,
            bytes_per_row: NSUInteger,
            bytes_per_image: NSUInteger,
        );

        #[method(getBytes:bytesPerRow:fromRegion:mipmapLevel:)]
        unsafe fn getBytes_bytesPerRow_fromRegion_mipmapLevel(
            &self,
            pixel_bytes: NonNull<c_void>,
            bytes_per_row: NSUInteger,
            region: MTLRegion,
            level: NSUInteger,
        );

        #[method(replaceRegion:mipmapLevel:withBytes:bytesPerRow:)]
        unsafe fn replaceRegion_mipmapLevel_withBytes_bytesPerRow(
            &self,
            region: MTLRegion,
            level: NSUInteger,
            pixel_bytes: NonNull<c_void>,
            bytes_per_row: NSUInteger,
        );

        #[method_id(@__retain_semantics New newTextureViewWithPixelFormat:)]
        fn newTextureViewWithPixelFormat(
            &self,
            pixel_format: MTLPixelFormat,
        ) -> Option<Id<ProtocolObject<dyn MTLTexture>>>;

        #[method_id(@__retain_semantics New newTextureViewWithPixelFormat:textureType:levels:slices:)]
        unsafe fn newTextureViewWithPixelFormat_textureType_levels_slices(
            &self,
            pixel_format: MTLPixelFormat,
            texture_type: MTLTextureType,
            level_range: NSRange,
            slice_range: NSRange,
        ) -> Option<Id<ProtocolObject<dyn MTLTexture>>>;

        #[cfg(feature = "Metal_MTLSharedTextureHandle")]
        #[method_id(@__retain_semantics New newSharedTextureHandle)]
        fn newSharedTextureHandle(&self) -> Option<Id<MTLSharedTextureHandle>>;

        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other remoteStorageTexture)]
        fn remoteStorageTexture(&self) -> Option<Id<ProtocolObject<dyn MTLTexture>>>;

        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics New newRemoteTextureViewForDevice:)]
        unsafe fn newRemoteTextureViewForDevice(
            &self,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Option<Id<ProtocolObject<dyn MTLTexture>>>;

        #[method(swizzle)]
        fn swizzle(&self) -> MTLTextureSwizzleChannels;

        #[method_id(@__retain_semantics New newTextureViewWithPixelFormat:textureType:levels:slices:swizzle:)]
        unsafe fn newTextureViewWithPixelFormat_textureType_levels_slices_swizzle(
            &self,
            pixel_format: MTLPixelFormat,
            texture_type: MTLTextureType,
            level_range: NSRange,
            slice_range: NSRange,
            swizzle: MTLTextureSwizzleChannels,
        ) -> Option<Id<ProtocolObject<dyn MTLTexture>>>;
    }

    unsafe impl ProtocolType for dyn MTLTexture {}
);
