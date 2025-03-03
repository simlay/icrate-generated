//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Metal::*;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum MTLAccelerationStructureRefitOptions {
        MTLAccelerationStructureRefitOptionVertexData = 1 << 0,
        MTLAccelerationStructureRefitOptionPerPrimitiveData = 1 << 1,
    }
);

extern_protocol!(
    pub unsafe trait MTLAccelerationStructureCommandEncoder: MTLCommandEncoder {
        #[cfg(feature = "Metal_MTLAccelerationStructureDescriptor")]
        #[method(buildAccelerationStructure:descriptor:scratchBuffer:scratchBufferOffset:)]
        fn buildAccelerationStructure_descriptor_scratchBuffer_scratchBufferOffset(
            &self,
            acceleration_structure: &ProtocolObject<dyn MTLAccelerationStructure>,
            descriptor: &MTLAccelerationStructureDescriptor,
            scratch_buffer: &ProtocolObject<dyn MTLBuffer>,
            scratch_buffer_offset: NSUInteger,
        );

        #[cfg(feature = "Metal_MTLAccelerationStructureDescriptor")]
        #[method(refitAccelerationStructure:descriptor:destination:scratchBuffer:scratchBufferOffset:)]
        unsafe fn refitAccelerationStructure_descriptor_destination_scratchBuffer_scratchBufferOffset(
            &self,
            source_acceleration_structure: &ProtocolObject<dyn MTLAccelerationStructure>,
            descriptor: &MTLAccelerationStructureDescriptor,
            destination_acceleration_structure: Option<
                &ProtocolObject<dyn MTLAccelerationStructure>,
            >,
            scratch_buffer: &ProtocolObject<dyn MTLBuffer>,
            scratch_buffer_offset: NSUInteger,
        );

        #[cfg(feature = "Metal_MTLAccelerationStructureDescriptor")]
        #[method(refitAccelerationStructure:descriptor:destination:scratchBuffer:scratchBufferOffset:options:)]
        unsafe fn refitAccelerationStructure_descriptor_destination_scratchBuffer_scratchBufferOffset_options(
            &self,
            source_acceleration_structure: &ProtocolObject<dyn MTLAccelerationStructure>,
            descriptor: &MTLAccelerationStructureDescriptor,
            destination_acceleration_structure: Option<
                &ProtocolObject<dyn MTLAccelerationStructure>,
            >,
            scratch_buffer: &ProtocolObject<dyn MTLBuffer>,
            scratch_buffer_offset: NSUInteger,
            options: MTLAccelerationStructureRefitOptions,
        );

        #[method(copyAccelerationStructure:toAccelerationStructure:)]
        unsafe fn copyAccelerationStructure_toAccelerationStructure(
            &self,
            source_acceleration_structure: &ProtocolObject<dyn MTLAccelerationStructure>,
            destination_acceleration_structure: &ProtocolObject<dyn MTLAccelerationStructure>,
        );

        #[method(writeCompactedAccelerationStructureSize:toBuffer:offset:)]
        fn writeCompactedAccelerationStructureSize_toBuffer_offset(
            &self,
            acceleration_structure: &ProtocolObject<dyn MTLAccelerationStructure>,
            buffer: &ProtocolObject<dyn MTLBuffer>,
            offset: NSUInteger,
        );

        #[method(writeCompactedAccelerationStructureSize:toBuffer:offset:sizeDataType:)]
        unsafe fn writeCompactedAccelerationStructureSize_toBuffer_offset_sizeDataType(
            &self,
            acceleration_structure: &ProtocolObject<dyn MTLAccelerationStructure>,
            buffer: &ProtocolObject<dyn MTLBuffer>,
            offset: NSUInteger,
            size_data_type: MTLDataType,
        );

        #[method(copyAndCompactAccelerationStructure:toAccelerationStructure:)]
        fn copyAndCompactAccelerationStructure_toAccelerationStructure(
            &self,
            source_acceleration_structure: &ProtocolObject<dyn MTLAccelerationStructure>,
            destination_acceleration_structure: &ProtocolObject<dyn MTLAccelerationStructure>,
        );

        #[method(updateFence:)]
        unsafe fn updateFence(&self, fence: &ProtocolObject<dyn MTLFence>);

        #[method(waitForFence:)]
        unsafe fn waitForFence(&self, fence: &ProtocolObject<dyn MTLFence>);

        #[method(useResource:usage:)]
        unsafe fn useResource_usage(
            &self,
            resource: &ProtocolObject<dyn MTLResource>,
            usage: MTLResourceUsage,
        );

        #[method(useResources:count:usage:)]
        unsafe fn useResources_count_usage(
            &self,
            resources: NonNull<NonNull<ProtocolObject<dyn MTLResource>>>,
            count: NSUInteger,
            usage: MTLResourceUsage,
        );

        #[method(useHeap:)]
        unsafe fn useHeap(&self, heap: &ProtocolObject<dyn MTLHeap>);

        #[method(useHeaps:count:)]
        unsafe fn useHeaps_count(
            &self,
            heaps: NonNull<NonNull<ProtocolObject<dyn MTLHeap>>>,
            count: NSUInteger,
        );

        #[method(sampleCountersInBuffer:atSampleIndex:withBarrier:)]
        unsafe fn sampleCountersInBuffer_atSampleIndex_withBarrier(
            &self,
            sample_buffer: &ProtocolObject<dyn MTLCounterSampleBuffer>,
            sample_index: NSUInteger,
            barrier: bool,
        );
    }

    unsafe impl ProtocolType for dyn MTLAccelerationStructureCommandEncoder {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLAccelerationStructurePassSampleBufferAttachmentDescriptor")]
    pub struct MTLAccelerationStructurePassSampleBufferAttachmentDescriptor;

    #[cfg(feature = "Metal_MTLAccelerationStructurePassSampleBufferAttachmentDescriptor")]
    unsafe impl ClassType for MTLAccelerationStructurePassSampleBufferAttachmentDescriptor {
        type Super = NSObject;
    }
);

#[cfg(feature = "Metal_MTLAccelerationStructurePassSampleBufferAttachmentDescriptor")]
unsafe impl NSObjectProtocol for MTLAccelerationStructurePassSampleBufferAttachmentDescriptor {}

extern_methods!(
    #[cfg(feature = "Metal_MTLAccelerationStructurePassSampleBufferAttachmentDescriptor")]
    unsafe impl MTLAccelerationStructurePassSampleBufferAttachmentDescriptor {
        #[method_id(@__retain_semantics Other sampleBuffer)]
        pub unsafe fn sampleBuffer(&self)
            -> Option<Id<ProtocolObject<dyn MTLCounterSampleBuffer>>>;

        #[method(setSampleBuffer:)]
        pub unsafe fn setSampleBuffer(
            &self,
            sample_buffer: Option<&ProtocolObject<dyn MTLCounterSampleBuffer>>,
        );

        #[method(startOfEncoderSampleIndex)]
        pub unsafe fn startOfEncoderSampleIndex(&self) -> NSUInteger;

        #[method(setStartOfEncoderSampleIndex:)]
        pub unsafe fn setStartOfEncoderSampleIndex(
            &self,
            start_of_encoder_sample_index: NSUInteger,
        );

        #[method(endOfEncoderSampleIndex)]
        pub unsafe fn endOfEncoderSampleIndex(&self) -> NSUInteger;

        #[method(setEndOfEncoderSampleIndex:)]
        pub unsafe fn setEndOfEncoderSampleIndex(&self, end_of_encoder_sample_index: NSUInteger);
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLAccelerationStructurePassSampleBufferAttachmentDescriptorArray")]
    pub struct MTLAccelerationStructurePassSampleBufferAttachmentDescriptorArray;

    #[cfg(feature = "Metal_MTLAccelerationStructurePassSampleBufferAttachmentDescriptorArray")]
    unsafe impl ClassType for MTLAccelerationStructurePassSampleBufferAttachmentDescriptorArray {
        type Super = NSObject;
    }
);

#[cfg(feature = "Metal_MTLAccelerationStructurePassSampleBufferAttachmentDescriptorArray")]
unsafe impl NSObjectProtocol for MTLAccelerationStructurePassSampleBufferAttachmentDescriptorArray {}

extern_methods!(
    #[cfg(feature = "Metal_MTLAccelerationStructurePassSampleBufferAttachmentDescriptorArray")]
    unsafe impl MTLAccelerationStructurePassSampleBufferAttachmentDescriptorArray {
        #[cfg(feature = "Metal_MTLAccelerationStructurePassSampleBufferAttachmentDescriptor")]
        #[method_id(@__retain_semantics Other objectAtIndexedSubscript:)]
        pub unsafe fn objectAtIndexedSubscript(
            &self,
            attachment_index: NSUInteger,
        ) -> Id<MTLAccelerationStructurePassSampleBufferAttachmentDescriptor>;

        #[cfg(feature = "Metal_MTLAccelerationStructurePassSampleBufferAttachmentDescriptor")]
        #[method(setObject:atIndexedSubscript:)]
        pub unsafe fn setObject_atIndexedSubscript(
            &self,
            attachment: Option<&MTLAccelerationStructurePassSampleBufferAttachmentDescriptor>,
            attachment_index: NSUInteger,
        );
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLAccelerationStructurePassDescriptor")]
    pub struct MTLAccelerationStructurePassDescriptor;

    #[cfg(feature = "Metal_MTLAccelerationStructurePassDescriptor")]
    unsafe impl ClassType for MTLAccelerationStructurePassDescriptor {
        type Super = NSObject;
    }
);

#[cfg(feature = "Metal_MTLAccelerationStructurePassDescriptor")]
unsafe impl NSObjectProtocol for MTLAccelerationStructurePassDescriptor {}

extern_methods!(
    #[cfg(feature = "Metal_MTLAccelerationStructurePassDescriptor")]
    unsafe impl MTLAccelerationStructurePassDescriptor {
        #[method_id(@__retain_semantics Other accelerationStructurePassDescriptor)]
        pub unsafe fn accelerationStructurePassDescriptor(
        ) -> Id<MTLAccelerationStructurePassDescriptor>;

        #[cfg(feature = "Metal_MTLAccelerationStructurePassSampleBufferAttachmentDescriptorArray")]
        #[method_id(@__retain_semantics Other sampleBufferAttachments)]
        pub unsafe fn sampleBufferAttachments(
            &self,
        ) -> Id<MTLAccelerationStructurePassSampleBufferAttachmentDescriptorArray>;
    }
);
