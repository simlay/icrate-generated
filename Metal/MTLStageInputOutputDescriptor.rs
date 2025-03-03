//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Metal::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum MTLAttributeFormat {
        MTLAttributeFormatInvalid = 0,
        MTLAttributeFormatUChar2 = 1,
        MTLAttributeFormatUChar3 = 2,
        MTLAttributeFormatUChar4 = 3,
        MTLAttributeFormatChar2 = 4,
        MTLAttributeFormatChar3 = 5,
        MTLAttributeFormatChar4 = 6,
        MTLAttributeFormatUChar2Normalized = 7,
        MTLAttributeFormatUChar3Normalized = 8,
        MTLAttributeFormatUChar4Normalized = 9,
        MTLAttributeFormatChar2Normalized = 10,
        MTLAttributeFormatChar3Normalized = 11,
        MTLAttributeFormatChar4Normalized = 12,
        MTLAttributeFormatUShort2 = 13,
        MTLAttributeFormatUShort3 = 14,
        MTLAttributeFormatUShort4 = 15,
        MTLAttributeFormatShort2 = 16,
        MTLAttributeFormatShort3 = 17,
        MTLAttributeFormatShort4 = 18,
        MTLAttributeFormatUShort2Normalized = 19,
        MTLAttributeFormatUShort3Normalized = 20,
        MTLAttributeFormatUShort4Normalized = 21,
        MTLAttributeFormatShort2Normalized = 22,
        MTLAttributeFormatShort3Normalized = 23,
        MTLAttributeFormatShort4Normalized = 24,
        MTLAttributeFormatHalf2 = 25,
        MTLAttributeFormatHalf3 = 26,
        MTLAttributeFormatHalf4 = 27,
        MTLAttributeFormatFloat = 28,
        MTLAttributeFormatFloat2 = 29,
        MTLAttributeFormatFloat3 = 30,
        MTLAttributeFormatFloat4 = 31,
        MTLAttributeFormatInt = 32,
        MTLAttributeFormatInt2 = 33,
        MTLAttributeFormatInt3 = 34,
        MTLAttributeFormatInt4 = 35,
        MTLAttributeFormatUInt = 36,
        MTLAttributeFormatUInt2 = 37,
        MTLAttributeFormatUInt3 = 38,
        MTLAttributeFormatUInt4 = 39,
        MTLAttributeFormatInt1010102Normalized = 40,
        MTLAttributeFormatUInt1010102Normalized = 41,
        MTLAttributeFormatUChar4Normalized_BGRA = 42,
        MTLAttributeFormatUChar = 45,
        MTLAttributeFormatChar = 46,
        MTLAttributeFormatUCharNormalized = 47,
        MTLAttributeFormatCharNormalized = 48,
        MTLAttributeFormatUShort = 49,
        MTLAttributeFormatShort = 50,
        MTLAttributeFormatUShortNormalized = 51,
        MTLAttributeFormatShortNormalized = 52,
        MTLAttributeFormatHalf = 53,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum MTLIndexType {
        MTLIndexTypeUInt16 = 0,
        MTLIndexTypeUInt32 = 1,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum MTLStepFunction {
        MTLStepFunctionConstant = 0,
        MTLStepFunctionPerVertex = 1,
        MTLStepFunctionPerInstance = 2,
        MTLStepFunctionPerPatch = 3,
        MTLStepFunctionPerPatchControlPoint = 4,
        MTLStepFunctionThreadPositionInGridX = 5,
        MTLStepFunctionThreadPositionInGridY = 6,
        MTLStepFunctionThreadPositionInGridXIndexed = 7,
        MTLStepFunctionThreadPositionInGridYIndexed = 8,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLBufferLayoutDescriptor")]
    pub struct MTLBufferLayoutDescriptor;

    #[cfg(feature = "Metal_MTLBufferLayoutDescriptor")]
    unsafe impl ClassType for MTLBufferLayoutDescriptor {
        type Super = NSObject;
    }
);

#[cfg(feature = "Metal_MTLBufferLayoutDescriptor")]
unsafe impl NSObjectProtocol for MTLBufferLayoutDescriptor {}

extern_methods!(
    #[cfg(feature = "Metal_MTLBufferLayoutDescriptor")]
    unsafe impl MTLBufferLayoutDescriptor {
        #[method(stride)]
        pub fn stride(&self) -> NSUInteger;

        #[method(setStride:)]
        pub fn setStride(&self, stride: NSUInteger);

        #[method(stepFunction)]
        pub fn stepFunction(&self) -> MTLStepFunction;

        #[method(setStepFunction:)]
        pub fn setStepFunction(&self, step_function: MTLStepFunction);

        #[method(stepRate)]
        pub fn stepRate(&self) -> NSUInteger;

        #[method(setStepRate:)]
        pub fn setStepRate(&self, step_rate: NSUInteger);
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLBufferLayoutDescriptorArray")]
    pub struct MTLBufferLayoutDescriptorArray;

    #[cfg(feature = "Metal_MTLBufferLayoutDescriptorArray")]
    unsafe impl ClassType for MTLBufferLayoutDescriptorArray {
        type Super = NSObject;
    }
);

#[cfg(feature = "Metal_MTLBufferLayoutDescriptorArray")]
unsafe impl NSObjectProtocol for MTLBufferLayoutDescriptorArray {}

extern_methods!(
    #[cfg(feature = "Metal_MTLBufferLayoutDescriptorArray")]
    unsafe impl MTLBufferLayoutDescriptorArray {
        #[cfg(feature = "Metal_MTLBufferLayoutDescriptor")]
        #[method_id(@__retain_semantics Other objectAtIndexedSubscript:)]
        pub unsafe fn objectAtIndexedSubscript(
            &self,
            index: NSUInteger,
        ) -> Id<MTLBufferLayoutDescriptor>;

        #[cfg(feature = "Metal_MTLBufferLayoutDescriptor")]
        #[method(setObject:atIndexedSubscript:)]
        pub unsafe fn setObject_atIndexedSubscript(
            &self,
            buffer_desc: Option<&MTLBufferLayoutDescriptor>,
            index: NSUInteger,
        );
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLAttributeDescriptor")]
    pub struct MTLAttributeDescriptor;

    #[cfg(feature = "Metal_MTLAttributeDescriptor")]
    unsafe impl ClassType for MTLAttributeDescriptor {
        type Super = NSObject;
    }
);

#[cfg(feature = "Metal_MTLAttributeDescriptor")]
unsafe impl NSObjectProtocol for MTLAttributeDescriptor {}

extern_methods!(
    #[cfg(feature = "Metal_MTLAttributeDescriptor")]
    unsafe impl MTLAttributeDescriptor {
        #[method(format)]
        pub fn format(&self) -> MTLAttributeFormat;

        #[method(setFormat:)]
        pub fn setFormat(&self, format: MTLAttributeFormat);

        #[method(offset)]
        pub fn offset(&self) -> NSUInteger;

        #[method(setOffset:)]
        pub fn setOffset(&self, offset: NSUInteger);

        #[method(bufferIndex)]
        pub fn bufferIndex(&self) -> NSUInteger;

        #[method(setBufferIndex:)]
        pub unsafe fn setBufferIndex(&self, buffer_index: NSUInteger);
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLAttributeDescriptorArray")]
    pub struct MTLAttributeDescriptorArray;

    #[cfg(feature = "Metal_MTLAttributeDescriptorArray")]
    unsafe impl ClassType for MTLAttributeDescriptorArray {
        type Super = NSObject;
    }
);

#[cfg(feature = "Metal_MTLAttributeDescriptorArray")]
unsafe impl NSObjectProtocol for MTLAttributeDescriptorArray {}

extern_methods!(
    #[cfg(feature = "Metal_MTLAttributeDescriptorArray")]
    unsafe impl MTLAttributeDescriptorArray {
        #[cfg(feature = "Metal_MTLAttributeDescriptor")]
        #[method_id(@__retain_semantics Other objectAtIndexedSubscript:)]
        pub unsafe fn objectAtIndexedSubscript(
            &self,
            index: NSUInteger,
        ) -> Id<MTLAttributeDescriptor>;

        #[cfg(feature = "Metal_MTLAttributeDescriptor")]
        #[method(setObject:atIndexedSubscript:)]
        pub unsafe fn setObject_atIndexedSubscript(
            &self,
            attribute_desc: Option<&MTLAttributeDescriptor>,
            index: NSUInteger,
        );
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLStageInputOutputDescriptor")]
    pub struct MTLStageInputOutputDescriptor;

    #[cfg(feature = "Metal_MTLStageInputOutputDescriptor")]
    unsafe impl ClassType for MTLStageInputOutputDescriptor {
        type Super = NSObject;
    }
);

#[cfg(feature = "Metal_MTLStageInputOutputDescriptor")]
unsafe impl NSObjectProtocol for MTLStageInputOutputDescriptor {}

extern_methods!(
    #[cfg(feature = "Metal_MTLStageInputOutputDescriptor")]
    unsafe impl MTLStageInputOutputDescriptor {
        #[method_id(@__retain_semantics Other stageInputOutputDescriptor)]
        pub fn stageInputOutputDescriptor() -> Id<MTLStageInputOutputDescriptor>;

        #[cfg(feature = "Metal_MTLBufferLayoutDescriptorArray")]
        #[method_id(@__retain_semantics Other layouts)]
        pub fn layouts(&self) -> Id<MTLBufferLayoutDescriptorArray>;

        #[cfg(feature = "Metal_MTLAttributeDescriptorArray")]
        #[method_id(@__retain_semantics Other attributes)]
        pub fn attributes(&self) -> Id<MTLAttributeDescriptorArray>;

        #[method(indexType)]
        pub fn indexType(&self) -> MTLIndexType;

        #[method(setIndexType:)]
        pub unsafe fn setIndexType(&self, index_type: MTLIndexType);

        #[method(indexBufferIndex)]
        pub fn indexBufferIndex(&self) -> NSUInteger;

        #[method(setIndexBufferIndex:)]
        pub unsafe fn setIndexBufferIndex(&self, index_buffer_index: NSUInteger);

        #[method(reset)]
        pub fn reset(&self);
    }
);
