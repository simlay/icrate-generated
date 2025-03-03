//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Metal::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum MTLBlendFactor {
        MTLBlendFactorZero = 0,
        MTLBlendFactorOne = 1,
        MTLBlendFactorSourceColor = 2,
        MTLBlendFactorOneMinusSourceColor = 3,
        MTLBlendFactorSourceAlpha = 4,
        MTLBlendFactorOneMinusSourceAlpha = 5,
        MTLBlendFactorDestinationColor = 6,
        MTLBlendFactorOneMinusDestinationColor = 7,
        MTLBlendFactorDestinationAlpha = 8,
        MTLBlendFactorOneMinusDestinationAlpha = 9,
        MTLBlendFactorSourceAlphaSaturated = 10,
        MTLBlendFactorBlendColor = 11,
        MTLBlendFactorOneMinusBlendColor = 12,
        MTLBlendFactorBlendAlpha = 13,
        MTLBlendFactorOneMinusBlendAlpha = 14,
        MTLBlendFactorSource1Color = 15,
        MTLBlendFactorOneMinusSource1Color = 16,
        MTLBlendFactorSource1Alpha = 17,
        MTLBlendFactorOneMinusSource1Alpha = 18,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum MTLBlendOperation {
        MTLBlendOperationAdd = 0,
        MTLBlendOperationSubtract = 1,
        MTLBlendOperationReverseSubtract = 2,
        MTLBlendOperationMin = 3,
        MTLBlendOperationMax = 4,
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum MTLColorWriteMask {
        MTLColorWriteMaskNone = 0,
        MTLColorWriteMaskRed = 0x1 << 3,
        MTLColorWriteMaskGreen = 0x1 << 2,
        MTLColorWriteMaskBlue = 0x1 << 1,
        MTLColorWriteMaskAlpha = 0x1 << 0,
        MTLColorWriteMaskAll = 0xf,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum MTLPrimitiveTopologyClass {
        MTLPrimitiveTopologyClassUnspecified = 0,
        MTLPrimitiveTopologyClassPoint = 1,
        MTLPrimitiveTopologyClassLine = 2,
        MTLPrimitiveTopologyClassTriangle = 3,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum MTLTessellationPartitionMode {
        MTLTessellationPartitionModePow2 = 0,
        MTLTessellationPartitionModeInteger = 1,
        MTLTessellationPartitionModeFractionalOdd = 2,
        MTLTessellationPartitionModeFractionalEven = 3,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum MTLTessellationFactorStepFunction {
        MTLTessellationFactorStepFunctionConstant = 0,
        MTLTessellationFactorStepFunctionPerPatch = 1,
        MTLTessellationFactorStepFunctionPerInstance = 2,
        MTLTessellationFactorStepFunctionPerPatchAndPerInstance = 3,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum MTLTessellationFactorFormat {
        MTLTessellationFactorFormatHalf = 0,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum MTLTessellationControlPointIndexType {
        MTLTessellationControlPointIndexTypeNone = 0,
        MTLTessellationControlPointIndexTypeUInt16 = 1,
        MTLTessellationControlPointIndexTypeUInt32 = 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLRenderPipelineColorAttachmentDescriptor")]
    pub struct MTLRenderPipelineColorAttachmentDescriptor;

    #[cfg(feature = "Metal_MTLRenderPipelineColorAttachmentDescriptor")]
    unsafe impl ClassType for MTLRenderPipelineColorAttachmentDescriptor {
        type Super = NSObject;
    }
);

#[cfg(feature = "Metal_MTLRenderPipelineColorAttachmentDescriptor")]
unsafe impl NSObjectProtocol for MTLRenderPipelineColorAttachmentDescriptor {}

extern_methods!(
    #[cfg(feature = "Metal_MTLRenderPipelineColorAttachmentDescriptor")]
    unsafe impl MTLRenderPipelineColorAttachmentDescriptor {
        #[method(pixelFormat)]
        pub fn pixelFormat(&self) -> MTLPixelFormat;

        #[method(setPixelFormat:)]
        pub fn setPixelFormat(&self, pixel_format: MTLPixelFormat);

        #[method(isBlendingEnabled)]
        pub fn isBlendingEnabled(&self) -> bool;

        #[method(setBlendingEnabled:)]
        pub fn setBlendingEnabled(&self, blending_enabled: bool);

        #[method(sourceRGBBlendFactor)]
        pub fn sourceRGBBlendFactor(&self) -> MTLBlendFactor;

        #[method(setSourceRGBBlendFactor:)]
        pub fn setSourceRGBBlendFactor(&self, source_rgb_blend_factor: MTLBlendFactor);

        #[method(destinationRGBBlendFactor)]
        pub fn destinationRGBBlendFactor(&self) -> MTLBlendFactor;

        #[method(setDestinationRGBBlendFactor:)]
        pub fn setDestinationRGBBlendFactor(&self, destination_rgb_blend_factor: MTLBlendFactor);

        #[method(rgbBlendOperation)]
        pub fn rgbBlendOperation(&self) -> MTLBlendOperation;

        #[method(setRgbBlendOperation:)]
        pub fn setRgbBlendOperation(&self, rgb_blend_operation: MTLBlendOperation);

        #[method(sourceAlphaBlendFactor)]
        pub fn sourceAlphaBlendFactor(&self) -> MTLBlendFactor;

        #[method(setSourceAlphaBlendFactor:)]
        pub fn setSourceAlphaBlendFactor(&self, source_alpha_blend_factor: MTLBlendFactor);

        #[method(destinationAlphaBlendFactor)]
        pub fn destinationAlphaBlendFactor(&self) -> MTLBlendFactor;

        #[method(setDestinationAlphaBlendFactor:)]
        pub fn setDestinationAlphaBlendFactor(
            &self,
            destination_alpha_blend_factor: MTLBlendFactor,
        );

        #[method(alphaBlendOperation)]
        pub fn alphaBlendOperation(&self) -> MTLBlendOperation;

        #[method(setAlphaBlendOperation:)]
        pub fn setAlphaBlendOperation(&self, alpha_blend_operation: MTLBlendOperation);

        #[method(writeMask)]
        pub fn writeMask(&self) -> MTLColorWriteMask;

        #[method(setWriteMask:)]
        pub fn setWriteMask(&self, write_mask: MTLColorWriteMask);
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLRenderPipelineReflection")]
    pub struct MTLRenderPipelineReflection;

    #[cfg(feature = "Metal_MTLRenderPipelineReflection")]
    unsafe impl ClassType for MTLRenderPipelineReflection {
        type Super = NSObject;
    }
);

#[cfg(feature = "Metal_MTLRenderPipelineReflection")]
unsafe impl NSObjectProtocol for MTLRenderPipelineReflection {}

extern_methods!(
    #[cfg(feature = "Metal_MTLRenderPipelineReflection")]
    unsafe impl MTLRenderPipelineReflection {
        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other vertexBindings)]
        pub unsafe fn vertexBindings(&self) -> Id<NSArray<ProtocolObject<dyn MTLBinding>>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other fragmentBindings)]
        pub unsafe fn fragmentBindings(&self) -> Id<NSArray<ProtocolObject<dyn MTLBinding>>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other tileBindings)]
        pub unsafe fn tileBindings(&self) -> Id<NSArray<ProtocolObject<dyn MTLBinding>>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other objectBindings)]
        pub unsafe fn objectBindings(&self) -> Id<NSArray<ProtocolObject<dyn MTLBinding>>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other meshBindings)]
        pub unsafe fn meshBindings(&self) -> Id<NSArray<ProtocolObject<dyn MTLBinding>>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Metal_MTLArgument"))]
        #[deprecated]
        #[method_id(@__retain_semantics Other vertexArguments)]
        pub fn vertexArguments(&self) -> Option<Id<NSArray<MTLArgument>>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Metal_MTLArgument"))]
        #[deprecated]
        #[method_id(@__retain_semantics Other fragmentArguments)]
        pub fn fragmentArguments(&self) -> Option<Id<NSArray<MTLArgument>>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Metal_MTLArgument"))]
        #[deprecated]
        #[method_id(@__retain_semantics Other tileArguments)]
        pub fn tileArguments(&self) -> Option<Id<NSArray<MTLArgument>>>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLRenderPipelineDescriptor")]
    pub struct MTLRenderPipelineDescriptor;

    #[cfg(feature = "Metal_MTLRenderPipelineDescriptor")]
    unsafe impl ClassType for MTLRenderPipelineDescriptor {
        type Super = NSObject;
    }
);

#[cfg(feature = "Metal_MTLRenderPipelineDescriptor")]
unsafe impl NSObjectProtocol for MTLRenderPipelineDescriptor {}

extern_methods!(
    #[cfg(feature = "Metal_MTLRenderPipelineDescriptor")]
    unsafe impl MTLRenderPipelineDescriptor {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other label)]
        pub fn label(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setLabel:)]
        pub fn setLabel(&self, label: Option<&NSString>);

        #[method_id(@__retain_semantics Other vertexFunction)]
        pub fn vertexFunction(&self) -> Option<Id<ProtocolObject<dyn MTLFunction>>>;

        #[method(setVertexFunction:)]
        pub fn setVertexFunction(&self, vertex_function: Option<&ProtocolObject<dyn MTLFunction>>);

        #[method_id(@__retain_semantics Other fragmentFunction)]
        pub fn fragmentFunction(&self) -> Option<Id<ProtocolObject<dyn MTLFunction>>>;

        #[method(setFragmentFunction:)]
        pub fn setFragmentFunction(
            &self,
            fragment_function: Option<&ProtocolObject<dyn MTLFunction>>,
        );

        #[cfg(feature = "Metal_MTLVertexDescriptor")]
        #[method_id(@__retain_semantics Other vertexDescriptor)]
        pub fn vertexDescriptor(&self) -> Option<Id<MTLVertexDescriptor>>;

        #[cfg(feature = "Metal_MTLVertexDescriptor")]
        #[method(setVertexDescriptor:)]
        pub fn setVertexDescriptor(&self, vertex_descriptor: Option<&MTLVertexDescriptor>);

        #[deprecated]
        #[method(sampleCount)]
        pub fn sampleCount(&self) -> NSUInteger;

        #[deprecated]
        #[method(setSampleCount:)]
        pub fn setSampleCount(&self, sample_count: NSUInteger);

        #[method(rasterSampleCount)]
        pub fn rasterSampleCount(&self) -> NSUInteger;

        #[method(setRasterSampleCount:)]
        pub fn setRasterSampleCount(&self, raster_sample_count: NSUInteger);

        #[method(isAlphaToCoverageEnabled)]
        pub fn isAlphaToCoverageEnabled(&self) -> bool;

        #[method(setAlphaToCoverageEnabled:)]
        pub fn setAlphaToCoverageEnabled(&self, alpha_to_coverage_enabled: bool);

        #[method(isAlphaToOneEnabled)]
        pub fn isAlphaToOneEnabled(&self) -> bool;

        #[method(setAlphaToOneEnabled:)]
        pub fn setAlphaToOneEnabled(&self, alpha_to_one_enabled: bool);

        #[method(isRasterizationEnabled)]
        pub fn isRasterizationEnabled(&self) -> bool;

        #[method(setRasterizationEnabled:)]
        pub fn setRasterizationEnabled(&self, rasterization_enabled: bool);

        #[method(maxVertexAmplificationCount)]
        pub fn maxVertexAmplificationCount(&self) -> NSUInteger;

        #[method(setMaxVertexAmplificationCount:)]
        pub unsafe fn setMaxVertexAmplificationCount(
            &self,
            max_vertex_amplification_count: NSUInteger,
        );

        #[cfg(feature = "Metal_MTLRenderPipelineColorAttachmentDescriptorArray")]
        #[method_id(@__retain_semantics Other colorAttachments)]
        pub fn colorAttachments(&self) -> Id<MTLRenderPipelineColorAttachmentDescriptorArray>;

        #[method(depthAttachmentPixelFormat)]
        pub fn depthAttachmentPixelFormat(&self) -> MTLPixelFormat;

        #[method(setDepthAttachmentPixelFormat:)]
        pub fn setDepthAttachmentPixelFormat(&self, depth_attachment_pixel_format: MTLPixelFormat);

        #[method(stencilAttachmentPixelFormat)]
        pub fn stencilAttachmentPixelFormat(&self) -> MTLPixelFormat;

        #[method(setStencilAttachmentPixelFormat:)]
        pub fn setStencilAttachmentPixelFormat(
            &self,
            stencil_attachment_pixel_format: MTLPixelFormat,
        );

        #[method(inputPrimitiveTopology)]
        pub fn inputPrimitiveTopology(&self) -> MTLPrimitiveTopologyClass;

        #[method(setInputPrimitiveTopology:)]
        pub unsafe fn setInputPrimitiveTopology(
            &self,
            input_primitive_topology: MTLPrimitiveTopologyClass,
        );

        #[method(tessellationPartitionMode)]
        pub fn tessellationPartitionMode(&self) -> MTLTessellationPartitionMode;

        #[method(setTessellationPartitionMode:)]
        pub unsafe fn setTessellationPartitionMode(
            &self,
            tessellation_partition_mode: MTLTessellationPartitionMode,
        );

        #[method(maxTessellationFactor)]
        pub fn maxTessellationFactor(&self) -> NSUInteger;

        #[method(setMaxTessellationFactor:)]
        pub unsafe fn setMaxTessellationFactor(&self, max_tessellation_factor: NSUInteger);

        #[method(isTessellationFactorScaleEnabled)]
        pub fn isTessellationFactorScaleEnabled(&self) -> bool;

        #[method(setTessellationFactorScaleEnabled:)]
        pub fn setTessellationFactorScaleEnabled(&self, tessellation_factor_scale_enabled: bool);

        #[method(tessellationFactorFormat)]
        pub fn tessellationFactorFormat(&self) -> MTLTessellationFactorFormat;

        #[method(setTessellationFactorFormat:)]
        pub fn setTessellationFactorFormat(
            &self,
            tessellation_factor_format: MTLTessellationFactorFormat,
        );

        #[method(tessellationControlPointIndexType)]
        pub fn tessellationControlPointIndexType(&self) -> MTLTessellationControlPointIndexType;

        #[method(setTessellationControlPointIndexType:)]
        pub unsafe fn setTessellationControlPointIndexType(
            &self,
            tessellation_control_point_index_type: MTLTessellationControlPointIndexType,
        );

        #[method(tessellationFactorStepFunction)]
        pub fn tessellationFactorStepFunction(&self) -> MTLTessellationFactorStepFunction;

        #[method(setTessellationFactorStepFunction:)]
        pub fn setTessellationFactorStepFunction(
            &self,
            tessellation_factor_step_function: MTLTessellationFactorStepFunction,
        );

        #[method(tessellationOutputWindingOrder)]
        pub fn tessellationOutputWindingOrder(&self) -> MTLWinding;

        #[method(setTessellationOutputWindingOrder:)]
        pub fn setTessellationOutputWindingOrder(
            &self,
            tessellation_output_winding_order: MTLWinding,
        );

        #[cfg(feature = "Metal_MTLPipelineBufferDescriptorArray")]
        #[method_id(@__retain_semantics Other vertexBuffers)]
        pub fn vertexBuffers(&self) -> Id<MTLPipelineBufferDescriptorArray>;

        #[cfg(feature = "Metal_MTLPipelineBufferDescriptorArray")]
        #[method_id(@__retain_semantics Other fragmentBuffers)]
        pub fn fragmentBuffers(&self) -> Id<MTLPipelineBufferDescriptorArray>;

        #[method(supportIndirectCommandBuffers)]
        pub fn supportIndirectCommandBuffers(&self) -> bool;

        #[method(setSupportIndirectCommandBuffers:)]
        pub fn setSupportIndirectCommandBuffers(&self, support_indirect_command_buffers: bool);

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other binaryArchives)]
        pub fn binaryArchives(&self) -> Option<Id<NSArray<ProtocolObject<dyn MTLBinaryArchive>>>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setBinaryArchives:)]
        pub fn setBinaryArchives(
            &self,
            binary_archives: Option<&NSArray<ProtocolObject<dyn MTLBinaryArchive>>>,
        );

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other vertexPreloadedLibraries)]
        pub fn vertexPreloadedLibraries(
            &self,
        ) -> Id<NSArray<ProtocolObject<dyn MTLDynamicLibrary>>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setVertexPreloadedLibraries:)]
        pub fn setVertexPreloadedLibraries(
            &self,
            vertex_preloaded_libraries: &NSArray<ProtocolObject<dyn MTLDynamicLibrary>>,
        );

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other fragmentPreloadedLibraries)]
        pub fn fragmentPreloadedLibraries(
            &self,
        ) -> Id<NSArray<ProtocolObject<dyn MTLDynamicLibrary>>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setFragmentPreloadedLibraries:)]
        pub fn setFragmentPreloadedLibraries(
            &self,
            fragment_preloaded_libraries: &NSArray<ProtocolObject<dyn MTLDynamicLibrary>>,
        );

        #[cfg(feature = "Metal_MTLLinkedFunctions")]
        #[method_id(@__retain_semantics Other vertexLinkedFunctions)]
        pub fn vertexLinkedFunctions(&self) -> Id<MTLLinkedFunctions>;

        #[cfg(feature = "Metal_MTLLinkedFunctions")]
        #[method(setVertexLinkedFunctions:)]
        pub fn setVertexLinkedFunctions(
            &self,
            vertex_linked_functions: Option<&MTLLinkedFunctions>,
        );

        #[cfg(feature = "Metal_MTLLinkedFunctions")]
        #[method_id(@__retain_semantics Other fragmentLinkedFunctions)]
        pub fn fragmentLinkedFunctions(&self) -> Id<MTLLinkedFunctions>;

        #[cfg(feature = "Metal_MTLLinkedFunctions")]
        #[method(setFragmentLinkedFunctions:)]
        pub fn setFragmentLinkedFunctions(
            &self,
            fragment_linked_functions: Option<&MTLLinkedFunctions>,
        );

        #[method(supportAddingVertexBinaryFunctions)]
        pub fn supportAddingVertexBinaryFunctions(&self) -> bool;

        #[method(setSupportAddingVertexBinaryFunctions:)]
        pub fn setSupportAddingVertexBinaryFunctions(
            &self,
            support_adding_vertex_binary_functions: bool,
        );

        #[method(supportAddingFragmentBinaryFunctions)]
        pub fn supportAddingFragmentBinaryFunctions(&self) -> bool;

        #[method(setSupportAddingFragmentBinaryFunctions:)]
        pub fn setSupportAddingFragmentBinaryFunctions(
            &self,
            support_adding_fragment_binary_functions: bool,
        );

        #[method(maxVertexCallStackDepth)]
        pub fn maxVertexCallStackDepth(&self) -> NSUInteger;

        #[method(setMaxVertexCallStackDepth:)]
        pub fn setMaxVertexCallStackDepth(&self, max_vertex_call_stack_depth: NSUInteger);

        #[method(maxFragmentCallStackDepth)]
        pub fn maxFragmentCallStackDepth(&self) -> NSUInteger;

        #[method(setMaxFragmentCallStackDepth:)]
        pub fn setMaxFragmentCallStackDepth(&self, max_fragment_call_stack_depth: NSUInteger);

        #[method(reset)]
        pub fn reset(&self);
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLRenderPipelineFunctionsDescriptor")]
    pub struct MTLRenderPipelineFunctionsDescriptor;

    #[cfg(feature = "Metal_MTLRenderPipelineFunctionsDescriptor")]
    unsafe impl ClassType for MTLRenderPipelineFunctionsDescriptor {
        type Super = NSObject;
    }
);

#[cfg(feature = "Metal_MTLRenderPipelineFunctionsDescriptor")]
unsafe impl NSObjectProtocol for MTLRenderPipelineFunctionsDescriptor {}

extern_methods!(
    #[cfg(feature = "Metal_MTLRenderPipelineFunctionsDescriptor")]
    unsafe impl MTLRenderPipelineFunctionsDescriptor {
        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other vertexAdditionalBinaryFunctions)]
        pub unsafe fn vertexAdditionalBinaryFunctions(
            &self,
        ) -> Option<Id<NSArray<ProtocolObject<dyn MTLFunction>>>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setVertexAdditionalBinaryFunctions:)]
        pub unsafe fn setVertexAdditionalBinaryFunctions(
            &self,
            vertex_additional_binary_functions: Option<&NSArray<ProtocolObject<dyn MTLFunction>>>,
        );

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other fragmentAdditionalBinaryFunctions)]
        pub unsafe fn fragmentAdditionalBinaryFunctions(
            &self,
        ) -> Option<Id<NSArray<ProtocolObject<dyn MTLFunction>>>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setFragmentAdditionalBinaryFunctions:)]
        pub unsafe fn setFragmentAdditionalBinaryFunctions(
            &self,
            fragment_additional_binary_functions: Option<&NSArray<ProtocolObject<dyn MTLFunction>>>,
        );

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other tileAdditionalBinaryFunctions)]
        pub unsafe fn tileAdditionalBinaryFunctions(
            &self,
        ) -> Option<Id<NSArray<ProtocolObject<dyn MTLFunction>>>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setTileAdditionalBinaryFunctions:)]
        pub unsafe fn setTileAdditionalBinaryFunctions(
            &self,
            tile_additional_binary_functions: Option<&NSArray<ProtocolObject<dyn MTLFunction>>>,
        );
    }
);

extern_protocol!(
    pub unsafe trait MTLRenderPipelineState: NSObjectProtocol {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other label)]
        unsafe fn label(&self) -> Option<Id<NSString>>;

        #[method_id(@__retain_semantics Other device)]
        unsafe fn device(&self) -> Id<ProtocolObject<dyn MTLDevice>>;

        #[method(maxTotalThreadsPerThreadgroup)]
        unsafe fn maxTotalThreadsPerThreadgroup(&self) -> NSUInteger;

        #[method(threadgroupSizeMatchesTileSize)]
        unsafe fn threadgroupSizeMatchesTileSize(&self) -> bool;

        #[method(imageblockSampleLength)]
        unsafe fn imageblockSampleLength(&self) -> NSUInteger;

        #[method(imageblockMemoryLengthForDimensions:)]
        unsafe fn imageblockMemoryLengthForDimensions(
            &self,
            imageblock_dimensions: MTLSize,
        ) -> NSUInteger;

        #[method(supportIndirectCommandBuffers)]
        unsafe fn supportIndirectCommandBuffers(&self) -> bool;

        #[method(maxTotalThreadsPerObjectThreadgroup)]
        unsafe fn maxTotalThreadsPerObjectThreadgroup(&self) -> NSUInteger;

        #[method(maxTotalThreadsPerMeshThreadgroup)]
        unsafe fn maxTotalThreadsPerMeshThreadgroup(&self) -> NSUInteger;

        #[method(objectThreadExecutionWidth)]
        unsafe fn objectThreadExecutionWidth(&self) -> NSUInteger;

        #[method(meshThreadExecutionWidth)]
        unsafe fn meshThreadExecutionWidth(&self) -> NSUInteger;

        #[method(maxTotalThreadgroupsPerMeshGrid)]
        unsafe fn maxTotalThreadgroupsPerMeshGrid(&self) -> NSUInteger;

        #[method(gpuResourceID)]
        unsafe fn gpuResourceID(&self) -> MTLResourceID;

        #[method_id(@__retain_semantics Other functionHandleWithFunction:stage:)]
        unsafe fn functionHandleWithFunction_stage(
            &self,
            function: &ProtocolObject<dyn MTLFunction>,
            stage: MTLRenderStages,
        ) -> Option<Id<ProtocolObject<dyn MTLFunctionHandle>>>;

        #[cfg(feature = "Metal_MTLVisibleFunctionTableDescriptor")]
        #[method_id(@__retain_semantics New newVisibleFunctionTableWithDescriptor:stage:)]
        unsafe fn newVisibleFunctionTableWithDescriptor_stage(
            &self,
            descriptor: &MTLVisibleFunctionTableDescriptor,
            stage: MTLRenderStages,
        ) -> Option<Id<ProtocolObject<dyn MTLVisibleFunctionTable>>>;

        #[cfg(feature = "Metal_MTLIntersectionFunctionTableDescriptor")]
        #[method_id(@__retain_semantics New newIntersectionFunctionTableWithDescriptor:stage:)]
        unsafe fn newIntersectionFunctionTableWithDescriptor_stage(
            &self,
            descriptor: &MTLIntersectionFunctionTableDescriptor,
            stage: MTLRenderStages,
        ) -> Option<Id<ProtocolObject<dyn MTLIntersectionFunctionTable>>>;

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Metal_MTLRenderPipelineFunctionsDescriptor"
        ))]
        #[method_id(@__retain_semantics New newRenderPipelineStateWithAdditionalBinaryFunctions:error:_)]
        unsafe fn newRenderPipelineStateWithAdditionalBinaryFunctions_error(
            &self,
            additional_binary_functions: &MTLRenderPipelineFunctionsDescriptor,
        ) -> Result<Id<ProtocolObject<dyn MTLRenderPipelineState>>, Id<NSError>>;
    }

    unsafe impl ProtocolType for dyn MTLRenderPipelineState {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLRenderPipelineColorAttachmentDescriptorArray")]
    pub struct MTLRenderPipelineColorAttachmentDescriptorArray;

    #[cfg(feature = "Metal_MTLRenderPipelineColorAttachmentDescriptorArray")]
    unsafe impl ClassType for MTLRenderPipelineColorAttachmentDescriptorArray {
        type Super = NSObject;
    }
);

#[cfg(feature = "Metal_MTLRenderPipelineColorAttachmentDescriptorArray")]
unsafe impl NSObjectProtocol for MTLRenderPipelineColorAttachmentDescriptorArray {}

extern_methods!(
    #[cfg(feature = "Metal_MTLRenderPipelineColorAttachmentDescriptorArray")]
    unsafe impl MTLRenderPipelineColorAttachmentDescriptorArray {
        #[cfg(feature = "Metal_MTLRenderPipelineColorAttachmentDescriptor")]
        #[method_id(@__retain_semantics Other objectAtIndexedSubscript:)]
        pub unsafe fn objectAtIndexedSubscript(
            &self,
            attachment_index: NSUInteger,
        ) -> Id<MTLRenderPipelineColorAttachmentDescriptor>;

        #[cfg(feature = "Metal_MTLRenderPipelineColorAttachmentDescriptor")]
        #[method(setObject:atIndexedSubscript:)]
        pub unsafe fn setObject_atIndexedSubscript(
            &self,
            attachment: Option<&MTLRenderPipelineColorAttachmentDescriptor>,
            attachment_index: NSUInteger,
        );
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLTileRenderPipelineColorAttachmentDescriptor")]
    pub struct MTLTileRenderPipelineColorAttachmentDescriptor;

    #[cfg(feature = "Metal_MTLTileRenderPipelineColorAttachmentDescriptor")]
    unsafe impl ClassType for MTLTileRenderPipelineColorAttachmentDescriptor {
        type Super = NSObject;
    }
);

#[cfg(feature = "Metal_MTLTileRenderPipelineColorAttachmentDescriptor")]
unsafe impl NSObjectProtocol for MTLTileRenderPipelineColorAttachmentDescriptor {}

extern_methods!(
    #[cfg(feature = "Metal_MTLTileRenderPipelineColorAttachmentDescriptor")]
    unsafe impl MTLTileRenderPipelineColorAttachmentDescriptor {
        #[method(pixelFormat)]
        pub unsafe fn pixelFormat(&self) -> MTLPixelFormat;

        #[method(setPixelFormat:)]
        pub unsafe fn setPixelFormat(&self, pixel_format: MTLPixelFormat);
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLTileRenderPipelineColorAttachmentDescriptorArray")]
    pub struct MTLTileRenderPipelineColorAttachmentDescriptorArray;

    #[cfg(feature = "Metal_MTLTileRenderPipelineColorAttachmentDescriptorArray")]
    unsafe impl ClassType for MTLTileRenderPipelineColorAttachmentDescriptorArray {
        type Super = NSObject;
    }
);

#[cfg(feature = "Metal_MTLTileRenderPipelineColorAttachmentDescriptorArray")]
unsafe impl NSObjectProtocol for MTLTileRenderPipelineColorAttachmentDescriptorArray {}

extern_methods!(
    #[cfg(feature = "Metal_MTLTileRenderPipelineColorAttachmentDescriptorArray")]
    unsafe impl MTLTileRenderPipelineColorAttachmentDescriptorArray {
        #[cfg(feature = "Metal_MTLTileRenderPipelineColorAttachmentDescriptor")]
        #[method_id(@__retain_semantics Other objectAtIndexedSubscript:)]
        pub unsafe fn objectAtIndexedSubscript(
            &self,
            attachment_index: NSUInteger,
        ) -> Id<MTLTileRenderPipelineColorAttachmentDescriptor>;

        #[cfg(feature = "Metal_MTLTileRenderPipelineColorAttachmentDescriptor")]
        #[method(setObject:atIndexedSubscript:)]
        pub unsafe fn setObject_atIndexedSubscript(
            &self,
            attachment: &MTLTileRenderPipelineColorAttachmentDescriptor,
            attachment_index: NSUInteger,
        );
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLTileRenderPipelineDescriptor")]
    pub struct MTLTileRenderPipelineDescriptor;

    #[cfg(feature = "Metal_MTLTileRenderPipelineDescriptor")]
    unsafe impl ClassType for MTLTileRenderPipelineDescriptor {
        type Super = NSObject;
    }
);

#[cfg(feature = "Metal_MTLTileRenderPipelineDescriptor")]
unsafe impl NSObjectProtocol for MTLTileRenderPipelineDescriptor {}

extern_methods!(
    #[cfg(feature = "Metal_MTLTileRenderPipelineDescriptor")]
    unsafe impl MTLTileRenderPipelineDescriptor {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other label)]
        pub unsafe fn label(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setLabel:)]
        pub unsafe fn setLabel(&self, label: Option<&NSString>);

        #[method_id(@__retain_semantics Other tileFunction)]
        pub unsafe fn tileFunction(&self) -> Id<ProtocolObject<dyn MTLFunction>>;

        #[method(setTileFunction:)]
        pub unsafe fn setTileFunction(&self, tile_function: &ProtocolObject<dyn MTLFunction>);

        #[method(rasterSampleCount)]
        pub unsafe fn rasterSampleCount(&self) -> NSUInteger;

        #[method(setRasterSampleCount:)]
        pub unsafe fn setRasterSampleCount(&self, raster_sample_count: NSUInteger);

        #[cfg(feature = "Metal_MTLTileRenderPipelineColorAttachmentDescriptorArray")]
        #[method_id(@__retain_semantics Other colorAttachments)]
        pub unsafe fn colorAttachments(
            &self,
        ) -> Id<MTLTileRenderPipelineColorAttachmentDescriptorArray>;

        #[method(threadgroupSizeMatchesTileSize)]
        pub unsafe fn threadgroupSizeMatchesTileSize(&self) -> bool;

        #[method(setThreadgroupSizeMatchesTileSize:)]
        pub unsafe fn setThreadgroupSizeMatchesTileSize(
            &self,
            threadgroup_size_matches_tile_size: bool,
        );

        #[cfg(feature = "Metal_MTLPipelineBufferDescriptorArray")]
        #[method_id(@__retain_semantics Other tileBuffers)]
        pub unsafe fn tileBuffers(&self) -> Id<MTLPipelineBufferDescriptorArray>;

        #[method(maxTotalThreadsPerThreadgroup)]
        pub unsafe fn maxTotalThreadsPerThreadgroup(&self) -> NSUInteger;

        #[method(setMaxTotalThreadsPerThreadgroup:)]
        pub unsafe fn setMaxTotalThreadsPerThreadgroup(
            &self,
            max_total_threads_per_threadgroup: NSUInteger,
        );

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other binaryArchives)]
        pub unsafe fn binaryArchives(
            &self,
        ) -> Option<Id<NSArray<ProtocolObject<dyn MTLBinaryArchive>>>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setBinaryArchives:)]
        pub unsafe fn setBinaryArchives(
            &self,
            binary_archives: Option<&NSArray<ProtocolObject<dyn MTLBinaryArchive>>>,
        );

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other preloadedLibraries)]
        pub unsafe fn preloadedLibraries(
            &self,
        ) -> Id<NSArray<ProtocolObject<dyn MTLDynamicLibrary>>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setPreloadedLibraries:)]
        pub unsafe fn setPreloadedLibraries(
            &self,
            preloaded_libraries: &NSArray<ProtocolObject<dyn MTLDynamicLibrary>>,
        );

        #[cfg(feature = "Metal_MTLLinkedFunctions")]
        #[method_id(@__retain_semantics Other linkedFunctions)]
        pub unsafe fn linkedFunctions(&self) -> Id<MTLLinkedFunctions>;

        #[cfg(feature = "Metal_MTLLinkedFunctions")]
        #[method(setLinkedFunctions:)]
        pub unsafe fn setLinkedFunctions(&self, linked_functions: Option<&MTLLinkedFunctions>);

        #[method(supportAddingBinaryFunctions)]
        pub unsafe fn supportAddingBinaryFunctions(&self) -> bool;

        #[method(setSupportAddingBinaryFunctions:)]
        pub unsafe fn setSupportAddingBinaryFunctions(&self, support_adding_binary_functions: bool);

        #[method(maxCallStackDepth)]
        pub unsafe fn maxCallStackDepth(&self) -> NSUInteger;

        #[method(setMaxCallStackDepth:)]
        pub unsafe fn setMaxCallStackDepth(&self, max_call_stack_depth: NSUInteger);

        #[method(reset)]
        pub unsafe fn reset(&self);
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLMeshRenderPipelineDescriptor")]
    pub struct MTLMeshRenderPipelineDescriptor;

    #[cfg(feature = "Metal_MTLMeshRenderPipelineDescriptor")]
    unsafe impl ClassType for MTLMeshRenderPipelineDescriptor {
        type Super = NSObject;
    }
);

#[cfg(feature = "Metal_MTLMeshRenderPipelineDescriptor")]
unsafe impl NSObjectProtocol for MTLMeshRenderPipelineDescriptor {}

extern_methods!(
    #[cfg(feature = "Metal_MTLMeshRenderPipelineDescriptor")]
    unsafe impl MTLMeshRenderPipelineDescriptor {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other label)]
        pub unsafe fn label(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setLabel:)]
        pub unsafe fn setLabel(&self, label: Option<&NSString>);

        #[method_id(@__retain_semantics Other objectFunction)]
        pub unsafe fn objectFunction(&self) -> Option<Id<ProtocolObject<dyn MTLFunction>>>;

        #[method(setObjectFunction:)]
        pub unsafe fn setObjectFunction(
            &self,
            object_function: Option<&ProtocolObject<dyn MTLFunction>>,
        );

        #[method_id(@__retain_semantics Other meshFunction)]
        pub unsafe fn meshFunction(&self) -> Option<Id<ProtocolObject<dyn MTLFunction>>>;

        #[method(setMeshFunction:)]
        pub unsafe fn setMeshFunction(
            &self,
            mesh_function: Option<&ProtocolObject<dyn MTLFunction>>,
        );

        #[method_id(@__retain_semantics Other fragmentFunction)]
        pub unsafe fn fragmentFunction(&self) -> Option<Id<ProtocolObject<dyn MTLFunction>>>;

        #[method(setFragmentFunction:)]
        pub unsafe fn setFragmentFunction(
            &self,
            fragment_function: Option<&ProtocolObject<dyn MTLFunction>>,
        );

        #[method(maxTotalThreadsPerObjectThreadgroup)]
        pub unsafe fn maxTotalThreadsPerObjectThreadgroup(&self) -> NSUInteger;

        #[method(setMaxTotalThreadsPerObjectThreadgroup:)]
        pub unsafe fn setMaxTotalThreadsPerObjectThreadgroup(
            &self,
            max_total_threads_per_object_threadgroup: NSUInteger,
        );

        #[method(maxTotalThreadsPerMeshThreadgroup)]
        pub unsafe fn maxTotalThreadsPerMeshThreadgroup(&self) -> NSUInteger;

        #[method(setMaxTotalThreadsPerMeshThreadgroup:)]
        pub unsafe fn setMaxTotalThreadsPerMeshThreadgroup(
            &self,
            max_total_threads_per_mesh_threadgroup: NSUInteger,
        );

        #[method(objectThreadgroupSizeIsMultipleOfThreadExecutionWidth)]
        pub unsafe fn objectThreadgroupSizeIsMultipleOfThreadExecutionWidth(&self) -> bool;

        #[method(setObjectThreadgroupSizeIsMultipleOfThreadExecutionWidth:)]
        pub unsafe fn setObjectThreadgroupSizeIsMultipleOfThreadExecutionWidth(
            &self,
            object_threadgroup_size_is_multiple_of_thread_execution_width: bool,
        );

        #[method(meshThreadgroupSizeIsMultipleOfThreadExecutionWidth)]
        pub unsafe fn meshThreadgroupSizeIsMultipleOfThreadExecutionWidth(&self) -> bool;

        #[method(setMeshThreadgroupSizeIsMultipleOfThreadExecutionWidth:)]
        pub unsafe fn setMeshThreadgroupSizeIsMultipleOfThreadExecutionWidth(
            &self,
            mesh_threadgroup_size_is_multiple_of_thread_execution_width: bool,
        );

        #[method(payloadMemoryLength)]
        pub unsafe fn payloadMemoryLength(&self) -> NSUInteger;

        #[method(setPayloadMemoryLength:)]
        pub unsafe fn setPayloadMemoryLength(&self, payload_memory_length: NSUInteger);

        #[method(maxTotalThreadgroupsPerMeshGrid)]
        pub unsafe fn maxTotalThreadgroupsPerMeshGrid(&self) -> NSUInteger;

        #[method(setMaxTotalThreadgroupsPerMeshGrid:)]
        pub unsafe fn setMaxTotalThreadgroupsPerMeshGrid(
            &self,
            max_total_threadgroups_per_mesh_grid: NSUInteger,
        );

        #[cfg(feature = "Metal_MTLPipelineBufferDescriptorArray")]
        #[method_id(@__retain_semantics Other objectBuffers)]
        pub unsafe fn objectBuffers(&self) -> Id<MTLPipelineBufferDescriptorArray>;

        #[cfg(feature = "Metal_MTLPipelineBufferDescriptorArray")]
        #[method_id(@__retain_semantics Other meshBuffers)]
        pub unsafe fn meshBuffers(&self) -> Id<MTLPipelineBufferDescriptorArray>;

        #[cfg(feature = "Metal_MTLPipelineBufferDescriptorArray")]
        #[method_id(@__retain_semantics Other fragmentBuffers)]
        pub unsafe fn fragmentBuffers(&self) -> Id<MTLPipelineBufferDescriptorArray>;

        #[method(rasterSampleCount)]
        pub unsafe fn rasterSampleCount(&self) -> NSUInteger;

        #[method(setRasterSampleCount:)]
        pub unsafe fn setRasterSampleCount(&self, raster_sample_count: NSUInteger);

        #[method(isAlphaToCoverageEnabled)]
        pub unsafe fn isAlphaToCoverageEnabled(&self) -> bool;

        #[method(setAlphaToCoverageEnabled:)]
        pub unsafe fn setAlphaToCoverageEnabled(&self, alpha_to_coverage_enabled: bool);

        #[method(isAlphaToOneEnabled)]
        pub unsafe fn isAlphaToOneEnabled(&self) -> bool;

        #[method(setAlphaToOneEnabled:)]
        pub unsafe fn setAlphaToOneEnabled(&self, alpha_to_one_enabled: bool);

        #[method(isRasterizationEnabled)]
        pub unsafe fn isRasterizationEnabled(&self) -> bool;

        #[method(setRasterizationEnabled:)]
        pub unsafe fn setRasterizationEnabled(&self, rasterization_enabled: bool);

        #[method(maxVertexAmplificationCount)]
        pub unsafe fn maxVertexAmplificationCount(&self) -> NSUInteger;

        #[method(setMaxVertexAmplificationCount:)]
        pub unsafe fn setMaxVertexAmplificationCount(
            &self,
            max_vertex_amplification_count: NSUInteger,
        );

        #[cfg(feature = "Metal_MTLRenderPipelineColorAttachmentDescriptorArray")]
        #[method_id(@__retain_semantics Other colorAttachments)]
        pub unsafe fn colorAttachments(
            &self,
        ) -> Id<MTLRenderPipelineColorAttachmentDescriptorArray>;

        #[method(depthAttachmentPixelFormat)]
        pub unsafe fn depthAttachmentPixelFormat(&self) -> MTLPixelFormat;

        #[method(setDepthAttachmentPixelFormat:)]
        pub unsafe fn setDepthAttachmentPixelFormat(
            &self,
            depth_attachment_pixel_format: MTLPixelFormat,
        );

        #[method(stencilAttachmentPixelFormat)]
        pub unsafe fn stencilAttachmentPixelFormat(&self) -> MTLPixelFormat;

        #[method(setStencilAttachmentPixelFormat:)]
        pub unsafe fn setStencilAttachmentPixelFormat(
            &self,
            stencil_attachment_pixel_format: MTLPixelFormat,
        );

        #[method(reset)]
        pub unsafe fn reset(&self);
    }
);
