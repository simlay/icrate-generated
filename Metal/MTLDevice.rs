//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Metal::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum MTLIOCompressionMethod {
        MTLIOCompressionMethodZlib = 0,
        MTLIOCompressionMethodLZFSE = 1,
        MTLIOCompressionMethodLZ4 = 2,
        MTLIOCompressionMethodLZMA = 3,
        MTLIOCompressionMethodLZBitmap = 4,
    }
);

extern_fn!(
    pub unsafe fn MTLCreateSystemDefaultDevice() -> *mut ProtocolObject<dyn MTLDevice>;
);

extern_fn!(
    #[cfg(feature = "Foundation_NSArray")]
    #[cfg(not(any(target_os = "ios")))]
    pub unsafe fn MTLCopyAllDevices() -> NonNull<NSArray<ProtocolObject<dyn MTLDevice>>>;
);

typed_enum!(
    pub type MTLDeviceNotificationName = NSString;
);

extern_static!(MTLDeviceWasAddedNotification: &'static MTLDeviceNotificationName);

extern_static!(MTLDeviceRemovalRequestedNotification: &'static MTLDeviceNotificationName);

extern_static!(MTLDeviceWasRemovedNotification: &'static MTLDeviceNotificationName);

pub type MTLDeviceNotificationHandler = *mut Block<
    (
        NonNull<ProtocolObject<dyn MTLDevice>>,
        NonNull<MTLDeviceNotificationName>,
    ),
    (),
>;

extern_fn!(
    #[cfg(not(any(target_os = "ios")))]
    pub unsafe fn MTLRemoveDeviceObserver(observer: &NSObject);
);

ns_enum!(
    #[underlying(NSUInteger)]
    #[deprecated = "Use MTLGPUFamily instead"]
    pub enum MTLFeatureSet {
        #[cfg(not(any(target_os = "macos")))]
        MTLFeatureSet_iOS_GPUFamily1_v1 = 0,
        #[cfg(not(any(target_os = "macos")))]
        MTLFeatureSet_iOS_GPUFamily2_v1 = 1,
        #[cfg(not(any(target_os = "macos")))]
        MTLFeatureSet_iOS_GPUFamily1_v2 = 2,
        #[cfg(not(any(target_os = "macos")))]
        MTLFeatureSet_iOS_GPUFamily2_v2 = 3,
        #[cfg(not(any(target_os = "macos")))]
        MTLFeatureSet_iOS_GPUFamily3_v1 = 4,
        #[cfg(not(any(target_os = "macos")))]
        MTLFeatureSet_iOS_GPUFamily1_v3 = 5,
        #[cfg(not(any(target_os = "macos")))]
        MTLFeatureSet_iOS_GPUFamily2_v3 = 6,
        #[cfg(not(any(target_os = "macos")))]
        MTLFeatureSet_iOS_GPUFamily3_v2 = 7,
        #[cfg(not(any(target_os = "macos")))]
        MTLFeatureSet_iOS_GPUFamily1_v4 = 8,
        #[cfg(not(any(target_os = "macos")))]
        MTLFeatureSet_iOS_GPUFamily2_v4 = 9,
        #[cfg(not(any(target_os = "macos")))]
        MTLFeatureSet_iOS_GPUFamily3_v3 = 10,
        #[cfg(not(any(target_os = "macos")))]
        MTLFeatureSet_iOS_GPUFamily4_v1 = 11,
        #[cfg(not(any(target_os = "macos")))]
        MTLFeatureSet_iOS_GPUFamily1_v5 = 12,
        #[cfg(not(any(target_os = "macos")))]
        MTLFeatureSet_iOS_GPUFamily2_v5 = 13,
        #[cfg(not(any(target_os = "macos")))]
        MTLFeatureSet_iOS_GPUFamily3_v4 = 14,
        #[cfg(not(any(target_os = "macos")))]
        MTLFeatureSet_iOS_GPUFamily4_v2 = 15,
        #[cfg(not(any(target_os = "macos")))]
        MTLFeatureSet_iOS_GPUFamily5_v1 = 16,
        #[cfg(not(any(target_os = "ios")))]
        MTLFeatureSet_macOS_GPUFamily1_v1 = 10000,
        #[cfg(not(any(target_os = "ios")))]
        MTLFeatureSet_OSX_GPUFamily1_v1 = MTLFeatureSet_macOS_GPUFamily1_v1,
        #[cfg(not(any(target_os = "ios")))]
        MTLFeatureSet_macOS_GPUFamily1_v2 = 10001,
        #[cfg(not(any(target_os = "ios")))]
        MTLFeatureSet_OSX_GPUFamily1_v2 = MTLFeatureSet_macOS_GPUFamily1_v2,
        #[cfg(not(any(target_os = "ios")))]
        MTLFeatureSet_macOS_ReadWriteTextureTier2 = 10002,
        #[cfg(not(any(target_os = "ios")))]
        MTLFeatureSet_OSX_ReadWriteTextureTier2 = MTLFeatureSet_macOS_ReadWriteTextureTier2,
        #[cfg(not(any(target_os = "ios")))]
        MTLFeatureSet_macOS_GPUFamily1_v3 = 10003,
        #[cfg(not(any(target_os = "ios")))]
        MTLFeatureSet_macOS_GPUFamily1_v4 = 10004,
        #[cfg(not(any(target_os = "ios")))]
        MTLFeatureSet_macOS_GPUFamily2_v1 = 10005,
        #[cfg(not(any(target_os = "ios", target_os = "macos")))]
        MTLFeatureSet_tvOS_GPUFamily1_v1 = 30000,
        #[cfg(not(any(target_os = "ios", target_os = "macos")))]
        MTLFeatureSet_TVOS_GPUFamily1_v1 = MTLFeatureSet_tvOS_GPUFamily1_v1,
        #[cfg(not(any(target_os = "ios", target_os = "macos")))]
        MTLFeatureSet_tvOS_GPUFamily1_v2 = 30001,
        #[cfg(not(any(target_os = "ios", target_os = "macos")))]
        MTLFeatureSet_tvOS_GPUFamily1_v3 = 30002,
        #[cfg(not(any(target_os = "ios", target_os = "macos")))]
        MTLFeatureSet_tvOS_GPUFamily2_v1 = 30003,
        #[cfg(not(any(target_os = "ios", target_os = "macos")))]
        MTLFeatureSet_tvOS_GPUFamily1_v4 = 30004,
        #[cfg(not(any(target_os = "ios", target_os = "macos")))]
        MTLFeatureSet_tvOS_GPUFamily2_v2 = 30005,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum MTLGPUFamily {
        MTLGPUFamilyApple1 = 1001,
        MTLGPUFamilyApple2 = 1002,
        MTLGPUFamilyApple3 = 1003,
        MTLGPUFamilyApple4 = 1004,
        MTLGPUFamilyApple5 = 1005,
        MTLGPUFamilyApple6 = 1006,
        MTLGPUFamilyApple7 = 1007,
        MTLGPUFamilyApple8 = 1008,
        #[deprecated]
        MTLGPUFamilyMac1 = 2001,
        MTLGPUFamilyMac2 = 2002,
        MTLGPUFamilyCommon1 = 3001,
        MTLGPUFamilyCommon2 = 3002,
        MTLGPUFamilyCommon3 = 3003,
        #[deprecated]
        MTLGPUFamilyMacCatalyst1 = 4001,
        #[deprecated]
        MTLGPUFamilyMacCatalyst2 = 4002,
        MTLGPUFamilyMetal3 = 5001,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    #[cfg(not(any(target_os = "ios")))]
    pub enum MTLDeviceLocation {
        #[cfg(not(any(target_os = "ios")))]
        MTLDeviceLocationBuiltIn = 0,
        #[cfg(not(any(target_os = "ios")))]
        MTLDeviceLocationSlot = 1,
        #[cfg(not(any(target_os = "ios")))]
        MTLDeviceLocationExternal = 2,
        #[cfg(not(any(target_os = "ios")))]
        MTLDeviceLocationUnspecified = NSUIntegerMax as _,
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum MTLPipelineOption {
        MTLPipelineOptionNone = 0,
        MTLPipelineOptionArgumentInfo = 1 << 0,
        MTLPipelineOptionBufferTypeInfo = 1 << 1,
        MTLPipelineOptionFailOnBinaryArchiveMiss = 1 << 2,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum MTLReadWriteTextureTier {
        MTLReadWriteTextureTierNone = 0,
        MTLReadWriteTextureTier1 = 1,
        MTLReadWriteTextureTier2 = 2,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum MTLArgumentBuffersTier {
        MTLArgumentBuffersTier1 = 0,
        MTLArgumentBuffersTier2 = 1,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum MTLSparseTextureRegionAlignmentMode {
        MTLSparseTextureRegionAlignmentModeOutward = 0,
        MTLSparseTextureRegionAlignmentModeInward = 1,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum MTLSparsePageSize {
        MTLSparsePageSize16 = 101,
        MTLSparsePageSize64 = 102,
        MTLSparsePageSize256 = 103,
    }
);

extern_struct!(
    #[encoding_name("?")]
    pub struct MTLAccelerationStructureSizes {
        pub accelerationStructureSize: NSUInteger,
        pub buildScratchBufferSize: NSUInteger,
        pub refitScratchBufferSize: NSUInteger,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum MTLCounterSamplingPoint {
        MTLCounterSamplingPointAtStageBoundary = 0,
        MTLCounterSamplingPointAtDrawBoundary = 1,
        MTLCounterSamplingPointAtDispatchBoundary = 2,
        MTLCounterSamplingPointAtTileDispatchBoundary = 3,
        MTLCounterSamplingPointAtBlitBoundary = 4,
    }
);

extern_struct!(
    #[encoding_name("?")]
    pub struct MTLSizeAndAlign {
        pub size: NSUInteger,
        pub align: NSUInteger,
    }
);

pub type MTLNewLibraryCompletionHandler =
    *mut Block<(*mut ProtocolObject<dyn MTLLibrary>, *mut NSError), ()>;

pub type MTLNewRenderPipelineStateCompletionHandler = *mut Block<
    (
        *mut ProtocolObject<dyn MTLRenderPipelineState>,
        *mut NSError,
    ),
    (),
>;

pub type MTLNewRenderPipelineStateWithReflectionCompletionHandler = *mut Block<
    (
        *mut ProtocolObject<dyn MTLRenderPipelineState>,
        *mut MTLRenderPipelineReflection,
        *mut NSError,
    ),
    (),
>;

pub type MTLNewComputePipelineStateCompletionHandler = *mut Block<
    (
        *mut ProtocolObject<dyn MTLComputePipelineState>,
        *mut NSError,
    ),
    (),
>;

pub type MTLNewComputePipelineStateWithReflectionCompletionHandler = *mut Block<
    (
        *mut ProtocolObject<dyn MTLComputePipelineState>,
        *mut MTLComputePipelineReflection,
        *mut NSError,
    ),
    (),
>;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLArgumentDescriptor")]
    pub struct MTLArgumentDescriptor;

    #[cfg(feature = "Metal_MTLArgumentDescriptor")]
    unsafe impl ClassType for MTLArgumentDescriptor {
        type Super = NSObject;
    }
);

#[cfg(feature = "Metal_MTLArgumentDescriptor")]
unsafe impl NSObjectProtocol for MTLArgumentDescriptor {}

extern_methods!(
    #[cfg(feature = "Metal_MTLArgumentDescriptor")]
    unsafe impl MTLArgumentDescriptor {
        #[method_id(@__retain_semantics Other argumentDescriptor)]
        pub fn argumentDescriptor() -> Id<MTLArgumentDescriptor>;

        #[method(dataType)]
        pub unsafe fn dataType(&self) -> MTLDataType;

        #[method(setDataType:)]
        pub fn setDataType(&self, data_type: MTLDataType);

        #[method(index)]
        pub unsafe fn index(&self) -> NSUInteger;

        #[method(setIndex:)]
        pub fn setIndex(&self, index: NSUInteger);

        #[method(arrayLength)]
        pub unsafe fn arrayLength(&self) -> NSUInteger;

        #[method(setArrayLength:)]
        pub unsafe fn setArrayLength(&self, array_length: NSUInteger);

        #[method(access)]
        pub unsafe fn access(&self) -> MTLArgumentAccess;

        #[method(setAccess:)]
        pub fn setAccess(&self, access: MTLArgumentAccess);

        #[method(textureType)]
        pub unsafe fn textureType(&self) -> MTLTextureType;

        #[method(setTextureType:)]
        pub fn setTextureType(&self, texture_type: MTLTextureType);

        #[method(constantBlockAlignment)]
        pub unsafe fn constantBlockAlignment(&self) -> NSUInteger;

        #[method(setConstantBlockAlignment:)]
        pub unsafe fn setConstantBlockAlignment(&self, constant_block_alignment: NSUInteger);
    }
);

pub type MTLTimestamp = u64;

extern_protocol!(
    pub unsafe trait MTLDevice: NSObjectProtocol {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other name)]
        fn name(&self) -> Id<NSString>;

        #[method(registryID)]
        fn registryID(&self) -> u64;

        #[method(maxThreadsPerThreadgroup)]
        fn maxThreadsPerThreadgroup(&self) -> MTLSize;

        #[cfg(not(any(target_os = "ios")))]
        #[method(isLowPower)]
        fn isLowPower(&self) -> bool;

        #[cfg(not(any(target_os = "ios")))]
        #[method(isHeadless)]
        fn isHeadless(&self) -> bool;

        #[cfg(not(any(target_os = "ios")))]
        #[method(isRemovable)]
        fn isRemovable(&self) -> bool;

        #[method(hasUnifiedMemory)]
        fn hasUnifiedMemory(&self) -> bool;

        #[cfg(not(any(target_os = "ios")))]
        #[method(recommendedMaxWorkingSetSize)]
        fn recommendedMaxWorkingSetSize(&self) -> u64;

        #[cfg(not(any(target_os = "ios")))]
        #[method(location)]
        fn location(&self) -> MTLDeviceLocation;

        #[cfg(not(any(target_os = "ios")))]
        #[method(locationNumber)]
        fn locationNumber(&self) -> NSUInteger;

        #[cfg(not(any(target_os = "ios")))]
        #[method(maxTransferRate)]
        fn maxTransferRate(&self) -> u64;

        #[cfg(not(any(target_os = "ios")))]
        #[method(isDepth24Stencil8PixelFormatSupported)]
        fn isDepth24Stencil8PixelFormatSupported(&self) -> bool;

        #[method(readWriteTextureSupport)]
        fn readWriteTextureSupport(&self) -> MTLReadWriteTextureTier;

        #[method(argumentBuffersSupport)]
        fn argumentBuffersSupport(&self) -> MTLArgumentBuffersTier;

        #[method(areRasterOrderGroupsSupported)]
        unsafe fn areRasterOrderGroupsSupported(&self) -> bool;

        #[method(supports32BitFloatFiltering)]
        fn supports32BitFloatFiltering(&self) -> bool;

        #[method(supports32BitMSAA)]
        fn supports32BitMSAA(&self) -> bool;

        #[method(supportsQueryTextureLOD)]
        fn supportsQueryTextureLOD(&self) -> bool;

        #[cfg(not(any(target_os = "ios")))]
        #[method(supportsBCTextureCompression)]
        fn supportsBCTextureCompression(&self) -> bool;

        #[method(supportsPullModelInterpolation)]
        fn supportsPullModelInterpolation(&self) -> bool;

        #[deprecated]
        #[method(areBarycentricCoordsSupported)]
        unsafe fn areBarycentricCoordsSupported(&self) -> bool;

        #[method(supportsShaderBarycentricCoordinates)]
        fn supportsShaderBarycentricCoordinates(&self) -> bool;

        #[method(currentAllocatedSize)]
        fn currentAllocatedSize(&self) -> NSUInteger;

        #[method_id(@__retain_semantics New newCommandQueue)]
        fn newCommandQueue(&self) -> Option<Id<ProtocolObject<dyn MTLCommandQueue>>>;

        #[method_id(@__retain_semantics New newCommandQueueWithMaxCommandBufferCount:)]
        fn newCommandQueueWithMaxCommandBufferCount(
            &self,
            max_command_buffer_count: NSUInteger,
        ) -> Option<Id<ProtocolObject<dyn MTLCommandQueue>>>;

        #[cfg(feature = "Metal_MTLTextureDescriptor")]
        #[method(heapTextureSizeAndAlignWithDescriptor:)]
        fn heapTextureSizeAndAlignWithDescriptor(
            &self,
            desc: &MTLTextureDescriptor,
        ) -> MTLSizeAndAlign;

        #[method(heapBufferSizeAndAlignWithLength:options:)]
        fn heapBufferSizeAndAlignWithLength_options(
            &self,
            length: NSUInteger,
            options: MTLResourceOptions,
        ) -> MTLSizeAndAlign;

        #[cfg(feature = "Metal_MTLHeapDescriptor")]
        #[method_id(@__retain_semantics New newHeapWithDescriptor:)]
        fn newHeapWithDescriptor(
            &self,
            descriptor: &MTLHeapDescriptor,
        ) -> Option<Id<ProtocolObject<dyn MTLHeap>>>;

        #[method_id(@__retain_semantics New newBufferWithLength:options:)]
        fn newBufferWithLength_options(
            &self,
            length: NSUInteger,
            options: MTLResourceOptions,
        ) -> Option<Id<ProtocolObject<dyn MTLBuffer>>>;

        #[method_id(@__retain_semantics New newBufferWithBytes:length:options:)]
        unsafe fn newBufferWithBytes_length_options(
            &self,
            pointer: NonNull<c_void>,
            length: NSUInteger,
            options: MTLResourceOptions,
        ) -> Option<Id<ProtocolObject<dyn MTLBuffer>>>;

        #[method_id(@__retain_semantics New newBufferWithBytesNoCopy:length:options:deallocator:)]
        unsafe fn newBufferWithBytesNoCopy_length_options_deallocator(
            &self,
            pointer: NonNull<c_void>,
            length: NSUInteger,
            options: MTLResourceOptions,
            deallocator: Option<&Block<(NonNull<c_void>, NSUInteger), ()>>,
        ) -> Option<Id<ProtocolObject<dyn MTLBuffer>>>;

        #[cfg(feature = "Metal_MTLDepthStencilDescriptor")]
        #[method_id(@__retain_semantics New newDepthStencilStateWithDescriptor:)]
        fn newDepthStencilStateWithDescriptor(
            &self,
            descriptor: &MTLDepthStencilDescriptor,
        ) -> Option<Id<ProtocolObject<dyn MTLDepthStencilState>>>;

        #[cfg(feature = "Metal_MTLTextureDescriptor")]
        #[method_id(@__retain_semantics New newTextureWithDescriptor:)]
        fn newTextureWithDescriptor(
            &self,
            descriptor: &MTLTextureDescriptor,
        ) -> Option<Id<ProtocolObject<dyn MTLTexture>>>;

        #[cfg(feature = "Metal_MTLTextureDescriptor")]
        #[method_id(@__retain_semantics New newSharedTextureWithDescriptor:)]
        unsafe fn newSharedTextureWithDescriptor(
            &self,
            descriptor: &MTLTextureDescriptor,
        ) -> Option<Id<ProtocolObject<dyn MTLTexture>>>;

        #[cfg(feature = "Metal_MTLSharedTextureHandle")]
        #[method_id(@__retain_semantics New newSharedTextureWithHandle:)]
        unsafe fn newSharedTextureWithHandle(
            &self,
            shared_handle: &MTLSharedTextureHandle,
        ) -> Option<Id<ProtocolObject<dyn MTLTexture>>>;

        #[cfg(feature = "Metal_MTLSamplerDescriptor")]
        #[method_id(@__retain_semantics New newSamplerStateWithDescriptor:)]
        fn newSamplerStateWithDescriptor(
            &self,
            descriptor: &MTLSamplerDescriptor,
        ) -> Option<Id<ProtocolObject<dyn MTLSamplerState>>>;

        #[method_id(@__retain_semantics New newDefaultLibrary)]
        fn newDefaultLibrary(&self) -> Option<Id<ProtocolObject<dyn MTLLibrary>>>;

        #[cfg(all(feature = "Foundation_NSBundle", feature = "Foundation_NSError"))]
        #[method_id(@__retain_semantics New newDefaultLibraryWithBundle:error:_)]
        unsafe fn newDefaultLibraryWithBundle_error(
            &self,
            bundle: &NSBundle,
        ) -> Result<Id<ProtocolObject<dyn MTLLibrary>>, Id<NSError>>;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[deprecated = "Use -newLibraryWithURL:error: instead"]
        #[method_id(@__retain_semantics New newLibraryWithFile:error:_)]
        fn newLibraryWithFile_error(
            &self,
            filepath: &NSString,
        ) -> Result<Id<ProtocolObject<dyn MTLLibrary>>, Id<NSError>>;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
        #[method_id(@__retain_semantics New newLibraryWithURL:error:_)]
        unsafe fn newLibraryWithURL_error(
            &self,
            url: &NSURL,
        ) -> Result<Id<ProtocolObject<dyn MTLLibrary>>, Id<NSError>>;

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "Metal_MTLCompileOptions"
        ))]
        #[method_id(@__retain_semantics New newLibraryWithSource:options:error:_)]
        fn newLibraryWithSource_options_error(
            &self,
            source: &NSString,
            options: Option<&MTLCompileOptions>,
        ) -> Result<Id<ProtocolObject<dyn MTLLibrary>>, Id<NSError>>;

        #[cfg(all(feature = "Foundation_NSString", feature = "Metal_MTLCompileOptions"))]
        #[method(newLibraryWithSource:options:completionHandler:)]
        unsafe fn newLibraryWithSource_options_completionHandler(
            &self,
            source: &NSString,
            options: Option<&MTLCompileOptions>,
            completion_handler: MTLNewLibraryCompletionHandler,
        );

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Metal_MTLStitchedLibraryDescriptor"
        ))]
        #[method_id(@__retain_semantics New newLibraryWithStitchedDescriptor:error:_)]
        unsafe fn newLibraryWithStitchedDescriptor_error(
            &self,
            descriptor: &MTLStitchedLibraryDescriptor,
        ) -> Result<Id<ProtocolObject<dyn MTLLibrary>>, Id<NSError>>;

        #[cfg(feature = "Metal_MTLStitchedLibraryDescriptor")]
        #[method(newLibraryWithStitchedDescriptor:completionHandler:)]
        unsafe fn newLibraryWithStitchedDescriptor_completionHandler(
            &self,
            descriptor: &MTLStitchedLibraryDescriptor,
            completion_handler: MTLNewLibraryCompletionHandler,
        );

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Metal_MTLRenderPipelineDescriptor"
        ))]
        #[method_id(@__retain_semantics New newRenderPipelineStateWithDescriptor:error:_)]
        fn newRenderPipelineStateWithDescriptor_error(
            &self,
            descriptor: &MTLRenderPipelineDescriptor,
        ) -> Result<Id<ProtocolObject<dyn MTLRenderPipelineState>>, Id<NSError>>;

        #[cfg(feature = "Metal_MTLRenderPipelineDescriptor")]
        #[method(newRenderPipelineStateWithDescriptor:completionHandler:)]
        unsafe fn newRenderPipelineStateWithDescriptor_completionHandler(
            &self,
            descriptor: &MTLRenderPipelineDescriptor,
            completion_handler: MTLNewRenderPipelineStateCompletionHandler,
        );

        #[cfg(feature = "Metal_MTLRenderPipelineDescriptor")]
        #[method(newRenderPipelineStateWithDescriptor:options:completionHandler:)]
        unsafe fn newRenderPipelineStateWithDescriptor_options_completionHandler(
            &self,
            descriptor: &MTLRenderPipelineDescriptor,
            options: MTLPipelineOption,
            completion_handler: MTLNewRenderPipelineStateWithReflectionCompletionHandler,
        );

        #[cfg(feature = "Foundation_NSError")]
        #[method_id(@__retain_semantics New newComputePipelineStateWithFunction:error:_)]
        fn newComputePipelineStateWithFunction_error(
            &self,
            compute_function: &ProtocolObject<dyn MTLFunction>,
        ) -> Result<Id<ProtocolObject<dyn MTLComputePipelineState>>, Id<NSError>>;

        #[method(newComputePipelineStateWithFunction:completionHandler:)]
        unsafe fn newComputePipelineStateWithFunction_completionHandler(
            &self,
            compute_function: &ProtocolObject<dyn MTLFunction>,
            completion_handler: MTLNewComputePipelineStateCompletionHandler,
        );

        #[method(newComputePipelineStateWithFunction:options:completionHandler:)]
        unsafe fn newComputePipelineStateWithFunction_options_completionHandler(
            &self,
            compute_function: &ProtocolObject<dyn MTLFunction>,
            options: MTLPipelineOption,
            completion_handler: MTLNewComputePipelineStateWithReflectionCompletionHandler,
        );

        #[cfg(feature = "Metal_MTLComputePipelineDescriptor")]
        #[method(newComputePipelineStateWithDescriptor:options:completionHandler:)]
        unsafe fn newComputePipelineStateWithDescriptor_options_completionHandler(
            &self,
            descriptor: &MTLComputePipelineDescriptor,
            options: MTLPipelineOption,
            completion_handler: MTLNewComputePipelineStateWithReflectionCompletionHandler,
        );

        #[method_id(@__retain_semantics New newFence)]
        fn newFence(&self) -> Option<Id<ProtocolObject<dyn MTLFence>>>;

        #[deprecated = "Use supportsFamily instead"]
        #[method(supportsFeatureSet:)]
        fn supportsFeatureSet(&self, feature_set: MTLFeatureSet) -> bool;

        #[method(supportsFamily:)]
        fn supportsFamily(&self, gpu_family: MTLGPUFamily) -> bool;

        #[method(supportsTextureSampleCount:)]
        fn supportsTextureSampleCount(&self, sample_count: NSUInteger) -> bool;

        #[method(minimumLinearTextureAlignmentForPixelFormat:)]
        fn minimumLinearTextureAlignmentForPixelFormat(&self, format: MTLPixelFormat)
            -> NSUInteger;

        #[method(minimumTextureBufferAlignmentForPixelFormat:)]
        fn minimumTextureBufferAlignmentForPixelFormat(&self, format: MTLPixelFormat)
            -> NSUInteger;

        #[cfg(feature = "Metal_MTLTileRenderPipelineDescriptor")]
        #[method(newRenderPipelineStateWithTileDescriptor:options:completionHandler:)]
        unsafe fn newRenderPipelineStateWithTileDescriptor_options_completionHandler(
            &self,
            descriptor: &MTLTileRenderPipelineDescriptor,
            options: MTLPipelineOption,
            completion_handler: MTLNewRenderPipelineStateWithReflectionCompletionHandler,
        );

        #[cfg(feature = "Metal_MTLMeshRenderPipelineDescriptor")]
        #[method(newRenderPipelineStateWithMeshDescriptor:options:completionHandler:)]
        unsafe fn newRenderPipelineStateWithMeshDescriptor_options_completionHandler(
            &self,
            descriptor: &MTLMeshRenderPipelineDescriptor,
            options: MTLPipelineOption,
            completion_handler: MTLNewRenderPipelineStateWithReflectionCompletionHandler,
        );

        #[method(maxThreadgroupMemoryLength)]
        fn maxThreadgroupMemoryLength(&self) -> NSUInteger;

        #[method(maxArgumentBufferSamplerCount)]
        fn maxArgumentBufferSamplerCount(&self) -> NSUInteger;

        #[method(areProgrammableSamplePositionsSupported)]
        unsafe fn areProgrammableSamplePositionsSupported(&self) -> bool;

        #[method(getDefaultSamplePositions:count:)]
        unsafe fn getDefaultSamplePositions_count(
            &self,
            positions: NonNull<MTLSamplePosition>,
            count: NSUInteger,
        );

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Metal_MTLArgumentDescriptor"
        ))]
        #[method_id(@__retain_semantics New newArgumentEncoderWithArguments:)]
        fn newArgumentEncoderWithArguments(
            &self,
            arguments: &NSArray<MTLArgumentDescriptor>,
        ) -> Option<Id<ProtocolObject<dyn MTLArgumentEncoder>>>;

        #[method(supportsRasterizationRateMapWithLayerCount:)]
        unsafe fn supportsRasterizationRateMapWithLayerCount(
            &self,
            layer_count: NSUInteger,
        ) -> bool;

        #[cfg(feature = "Metal_MTLRasterizationRateMapDescriptor")]
        #[method_id(@__retain_semantics New newRasterizationRateMapWithDescriptor:)]
        unsafe fn newRasterizationRateMapWithDescriptor(
            &self,
            descriptor: &MTLRasterizationRateMapDescriptor,
        ) -> Option<Id<ProtocolObject<dyn MTLRasterizationRateMap>>>;

        #[cfg(feature = "Metal_MTLIndirectCommandBufferDescriptor")]
        #[method_id(@__retain_semantics New newIndirectCommandBufferWithDescriptor:maxCommandCount:options:)]
        unsafe fn newIndirectCommandBufferWithDescriptor_maxCommandCount_options(
            &self,
            descriptor: &MTLIndirectCommandBufferDescriptor,
            max_count: NSUInteger,
            options: MTLResourceOptions,
        ) -> Option<Id<ProtocolObject<dyn MTLIndirectCommandBuffer>>>;

        #[method_id(@__retain_semantics New newEvent)]
        fn newEvent(&self) -> Option<Id<ProtocolObject<dyn MTLEvent>>>;

        #[method_id(@__retain_semantics New newSharedEvent)]
        fn newSharedEvent(&self) -> Option<Id<ProtocolObject<dyn MTLSharedEvent>>>;

        #[cfg(feature = "Metal_MTLSharedEventHandle")]
        #[method_id(@__retain_semantics New newSharedEventWithHandle:)]
        unsafe fn newSharedEventWithHandle(
            &self,
            shared_event_handle: &MTLSharedEventHandle,
        ) -> Option<Id<ProtocolObject<dyn MTLSharedEvent>>>;

        #[cfg(not(any(target_os = "ios")))]
        #[method(peerGroupID)]
        unsafe fn peerGroupID(&self) -> u64;

        #[cfg(not(any(target_os = "ios")))]
        #[method(peerIndex)]
        unsafe fn peerIndex(&self) -> u32;

        #[cfg(not(any(target_os = "ios")))]
        #[method(peerCount)]
        unsafe fn peerCount(&self) -> u32;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
        #[method_id(@__retain_semantics New newIOHandleWithURL:error:_)]
        unsafe fn newIOHandleWithURL_error(
            &self,
            url: &NSURL,
        ) -> Result<Id<ProtocolObject<dyn MTLIOFileHandle>>, Id<NSError>>;

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Metal_MTLIOCommandQueueDescriptor"
        ))]
        #[method_id(@__retain_semantics New newIOCommandQueueWithDescriptor:error:_)]
        unsafe fn newIOCommandQueueWithDescriptor_error(
            &self,
            descriptor: &MTLIOCommandQueueDescriptor,
        ) -> Result<Id<ProtocolObject<dyn MTLIOCommandQueue>>, Id<NSError>>;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
        #[method_id(@__retain_semantics New newIOHandleWithURL:compressionMethod:error:_)]
        unsafe fn newIOHandleWithURL_compressionMethod_error(
            &self,
            url: &NSURL,
            compression_method: MTLIOCompressionMethod,
        ) -> Result<Id<ProtocolObject<dyn MTLIOFileHandle>>, Id<NSError>>;

        #[method(sparseTileSizeWithTextureType:pixelFormat:sampleCount:)]
        unsafe fn sparseTileSizeWithTextureType_pixelFormat_sampleCount(
            &self,
            texture_type: MTLTextureType,
            pixel_format: MTLPixelFormat,
            sample_count: NSUInteger,
        ) -> MTLSize;

        #[method(sparseTileSizeInBytes)]
        unsafe fn sparseTileSizeInBytes(&self) -> NSUInteger;

        #[optional]
        #[method(convertSparsePixelRegions:toTileRegions:withTileSize:alignmentMode:numRegions:)]
        unsafe fn convertSparsePixelRegions_toTileRegions_withTileSize_alignmentMode_numRegions(
            &self,
            pixel_regions: NonNull<MTLRegion>,
            tile_regions: NonNull<MTLRegion>,
            tile_size: MTLSize,
            mode: MTLSparseTextureRegionAlignmentMode,
            num_regions: NSUInteger,
        );

        #[optional]
        #[method(convertSparseTileRegions:toPixelRegions:withTileSize:numRegions:)]
        unsafe fn convertSparseTileRegions_toPixelRegions_withTileSize_numRegions(
            &self,
            tile_regions: NonNull<MTLRegion>,
            pixel_regions: NonNull<MTLRegion>,
            tile_size: MTLSize,
            num_regions: NSUInteger,
        );

        #[method(sparseTileSizeInBytesForSparsePageSize:)]
        unsafe fn sparseTileSizeInBytesForSparsePageSize(
            &self,
            sparse_page_size: MTLSparsePageSize,
        ) -> NSUInteger;

        #[method(sparseTileSizeWithTextureType:pixelFormat:sampleCount:sparsePageSize:)]
        unsafe fn sparseTileSizeWithTextureType_pixelFormat_sampleCount_sparsePageSize(
            &self,
            texture_type: MTLTextureType,
            pixel_format: MTLPixelFormat,
            sample_count: NSUInteger,
            sparse_page_size: MTLSparsePageSize,
        ) -> MTLSize;

        #[method(maxBufferLength)]
        fn maxBufferLength(&self) -> NSUInteger;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other counterSets)]
        unsafe fn counterSets(&self) -> Option<Id<NSArray<ProtocolObject<dyn MTLCounterSet>>>>;

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Metal_MTLCounterSampleBufferDescriptor"
        ))]
        #[method_id(@__retain_semantics New newCounterSampleBufferWithDescriptor:error:_)]
        unsafe fn newCounterSampleBufferWithDescriptor_error(
            &self,
            descriptor: &MTLCounterSampleBufferDescriptor,
        ) -> Result<Id<ProtocolObject<dyn MTLCounterSampleBuffer>>, Id<NSError>>;

        #[method(sampleTimestamps:gpuTimestamp:)]
        unsafe fn sampleTimestamps_gpuTimestamp(
            &self,
            cpu_timestamp: NonNull<MTLTimestamp>,
            gpu_timestamp: NonNull<MTLTimestamp>,
        );

        #[method_id(@__retain_semantics New newArgumentEncoderWithBufferBinding:)]
        unsafe fn newArgumentEncoderWithBufferBinding(
            &self,
            buffer_binding: &ProtocolObject<dyn MTLBufferBinding>,
        ) -> Id<ProtocolObject<dyn MTLArgumentEncoder>>;

        #[method(supportsCounterSampling:)]
        fn supportsCounterSampling(&self, sampling_point: MTLCounterSamplingPoint) -> bool;

        #[method(supportsVertexAmplificationCount:)]
        fn supportsVertexAmplificationCount(&self, count: NSUInteger) -> bool;

        #[method(supportsDynamicLibraries)]
        fn supportsDynamicLibraries(&self) -> bool;

        #[method(supportsRenderDynamicLibraries)]
        unsafe fn supportsRenderDynamicLibraries(&self) -> bool;

        #[cfg(feature = "Foundation_NSError")]
        #[method_id(@__retain_semantics New newDynamicLibrary:error:_)]
        fn newDynamicLibrary_error(
            &self,
            library: &ProtocolObject<dyn MTLLibrary>,
        ) -> Result<Id<ProtocolObject<dyn MTLDynamicLibrary>>, Id<NSError>>;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
        #[method_id(@__retain_semantics New newDynamicLibraryWithURL:error:_)]
        fn newDynamicLibraryWithURL_error(
            &self,
            url: &NSURL,
        ) -> Result<Id<ProtocolObject<dyn MTLDynamicLibrary>>, Id<NSError>>;

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Metal_MTLBinaryArchiveDescriptor"
        ))]
        #[method_id(@__retain_semantics New newBinaryArchiveWithDescriptor:error:_)]
        fn newBinaryArchiveWithDescriptor_error(
            &self,
            descriptor: &MTLBinaryArchiveDescriptor,
        ) -> Result<Id<ProtocolObject<dyn MTLBinaryArchive>>, Id<NSError>>;

        #[method(supportsRaytracing)]
        fn supportsRaytracing(&self) -> bool;

        #[cfg(feature = "Metal_MTLAccelerationStructureDescriptor")]
        #[method(accelerationStructureSizesWithDescriptor:)]
        fn accelerationStructureSizesWithDescriptor(
            &self,
            descriptor: &MTLAccelerationStructureDescriptor,
        ) -> MTLAccelerationStructureSizes;

        #[method_id(@__retain_semantics New newAccelerationStructureWithSize:)]
        fn newAccelerationStructureWithSize(
            &self,
            size: NSUInteger,
        ) -> Option<Id<ProtocolObject<dyn MTLAccelerationStructure>>>;

        #[cfg(feature = "Metal_MTLAccelerationStructureDescriptor")]
        #[method_id(@__retain_semantics New newAccelerationStructureWithDescriptor:)]
        unsafe fn newAccelerationStructureWithDescriptor(
            &self,
            descriptor: &MTLAccelerationStructureDescriptor,
        ) -> Option<Id<ProtocolObject<dyn MTLAccelerationStructure>>>;

        #[method(heapAccelerationStructureSizeAndAlignWithSize:)]
        unsafe fn heapAccelerationStructureSizeAndAlignWithSize(
            &self,
            size: NSUInteger,
        ) -> MTLSizeAndAlign;

        #[cfg(feature = "Metal_MTLAccelerationStructureDescriptor")]
        #[method(heapAccelerationStructureSizeAndAlignWithDescriptor:)]
        unsafe fn heapAccelerationStructureSizeAndAlignWithDescriptor(
            &self,
            descriptor: &MTLAccelerationStructureDescriptor,
        ) -> MTLSizeAndAlign;

        #[method(supportsFunctionPointers)]
        fn supportsFunctionPointers(&self) -> bool;

        #[method(supportsFunctionPointersFromRender)]
        unsafe fn supportsFunctionPointersFromRender(&self) -> bool;

        #[method(supportsRaytracingFromRender)]
        unsafe fn supportsRaytracingFromRender(&self) -> bool;

        #[method(supportsPrimitiveMotionBlur)]
        unsafe fn supportsPrimitiveMotionBlur(&self) -> bool;
    }

    unsafe impl ProtocolType for dyn MTLDevice {}
);
