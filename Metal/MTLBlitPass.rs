//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Metal::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLBlitPassSampleBufferAttachmentDescriptor")]
    pub struct MTLBlitPassSampleBufferAttachmentDescriptor;

    #[cfg(feature = "Metal_MTLBlitPassSampleBufferAttachmentDescriptor")]
    unsafe impl ClassType for MTLBlitPassSampleBufferAttachmentDescriptor {
        type Super = NSObject;
    }
);

#[cfg(feature = "Metal_MTLBlitPassSampleBufferAttachmentDescriptor")]
unsafe impl NSObjectProtocol for MTLBlitPassSampleBufferAttachmentDescriptor {}

extern_methods!(
    #[cfg(feature = "Metal_MTLBlitPassSampleBufferAttachmentDescriptor")]
    unsafe impl MTLBlitPassSampleBufferAttachmentDescriptor {
        /**
         @property sampleBuffer
        @abstract The sample buffer to store samples for the blit-pass defined samples.
        If sampleBuffer is non-nil, the sample indices will be used to store samples into
        the sample buffer.  If no sample buffer is provided, no samples will be taken.
        If any of the sample indices are specified as MTLCounterDontSample, no sample
        will be taken for that action.
        */
        #[method_id(@__retain_semantics Other sampleBuffer)]
        pub unsafe fn sampleBuffer(&self)
            -> Option<Id<ProtocolObject<dyn MTLCounterSampleBuffer>>>;

        /**
         @property sampleBuffer
        @abstract The sample buffer to store samples for the blit-pass defined samples.
        If sampleBuffer is non-nil, the sample indices will be used to store samples into
        the sample buffer.  If no sample buffer is provided, no samples will be taken.
        If any of the sample indices are specified as MTLCounterDontSample, no sample
        will be taken for that action.
        */
        #[method(setSampleBuffer:)]
        pub unsafe fn setSampleBuffer(
            &self,
            sample_buffer: Option<&ProtocolObject<dyn MTLCounterSampleBuffer>>,
        );

        /**
         @property startOfEncoderSampleIndex
        @abstract The sample index to use to store the sample taken at the start of
        command encoder processing.  Setting the value to MTLCounterDontSample will cause
        this sample to be omitted.
        @discussion On devices where MTLCounterSamplingPointAtStageBoundary is unsupported,
        this sample index is invalid and must be set to MTLCounterDontSample or creation of a blit pass will fail.
        */
        #[method(startOfEncoderSampleIndex)]
        pub unsafe fn startOfEncoderSampleIndex(&self) -> NSUInteger;

        /**
         @property startOfEncoderSampleIndex
        @abstract The sample index to use to store the sample taken at the start of
        command encoder processing.  Setting the value to MTLCounterDontSample will cause
        this sample to be omitted.
        @discussion On devices where MTLCounterSamplingPointAtStageBoundary is unsupported,
        this sample index is invalid and must be set to MTLCounterDontSample or creation of a blit pass will fail.
        */
        #[method(setStartOfEncoderSampleIndex:)]
        pub unsafe fn setStartOfEncoderSampleIndex(
            &self,
            start_of_encoder_sample_index: NSUInteger,
        );

        /**
         @property endOfEncoderSampleIndex
        @abstract The sample index to use to store the sample taken at the end of
        Command encoder processing.  Setting the value to MTLCounterDontSample will cause
        this sample to be omitted.
        @discussion On devices where MTLCounterSamplingPointAtStageBoundary is unsupported,
        this sample index is invalid and must be set to MTLCounterDontSample or creation of a blit pass will fail.
        */
        #[method(endOfEncoderSampleIndex)]
        pub unsafe fn endOfEncoderSampleIndex(&self) -> NSUInteger;

        /**
         @property endOfEncoderSampleIndex
        @abstract The sample index to use to store the sample taken at the end of
        Command encoder processing.  Setting the value to MTLCounterDontSample will cause
        this sample to be omitted.
        @discussion On devices where MTLCounterSamplingPointAtStageBoundary is unsupported,
        this sample index is invalid and must be set to MTLCounterDontSample or creation of a blit pass will fail.
        */
        #[method(setEndOfEncoderSampleIndex:)]
        pub unsafe fn setEndOfEncoderSampleIndex(&self, end_of_encoder_sample_index: NSUInteger);
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLBlitPassSampleBufferAttachmentDescriptorArray")]
    pub struct MTLBlitPassSampleBufferAttachmentDescriptorArray;

    #[cfg(feature = "Metal_MTLBlitPassSampleBufferAttachmentDescriptorArray")]
    unsafe impl ClassType for MTLBlitPassSampleBufferAttachmentDescriptorArray {
        type Super = NSObject;
    }
);

#[cfg(feature = "Metal_MTLBlitPassSampleBufferAttachmentDescriptorArray")]
unsafe impl NSObjectProtocol for MTLBlitPassSampleBufferAttachmentDescriptorArray {}

extern_methods!(
    #[cfg(feature = "Metal_MTLBlitPassSampleBufferAttachmentDescriptorArray")]
    unsafe impl MTLBlitPassSampleBufferAttachmentDescriptorArray {
        #[cfg(feature = "Metal_MTLBlitPassSampleBufferAttachmentDescriptor")]
        #[method_id(@__retain_semantics Other objectAtIndexedSubscript:)]
        pub unsafe fn objectAtIndexedSubscript(
            &self,
            attachment_index: NSUInteger,
        ) -> Id<MTLBlitPassSampleBufferAttachmentDescriptor>;

        #[cfg(feature = "Metal_MTLBlitPassSampleBufferAttachmentDescriptor")]
        #[method(setObject:atIndexedSubscript:)]
        pub unsafe fn setObject_atIndexedSubscript(
            &self,
            attachment: Option<&MTLBlitPassSampleBufferAttachmentDescriptor>,
            attachment_index: NSUInteger,
        );
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLBlitPassDescriptor")]
    /**
     @class MTLBlitPassDescriptor
    @abstract MTLBlitPassDescriptor represents a collection of attachments to be used to create a concrete blit command encoder
    */
    pub struct MTLBlitPassDescriptor;

    #[cfg(feature = "Metal_MTLBlitPassDescriptor")]
    unsafe impl ClassType for MTLBlitPassDescriptor {
        type Super = NSObject;
    }
);

#[cfg(feature = "Metal_MTLBlitPassDescriptor")]
/**
 @class MTLBlitPassDescriptor
@abstract MTLBlitPassDescriptor represents a collection of attachments to be used to create a concrete blit command encoder
*/
unsafe impl NSObjectProtocol for MTLBlitPassDescriptor {}

extern_methods!(
    /**
     @class MTLBlitPassDescriptor
    @abstract MTLBlitPassDescriptor represents a collection of attachments to be used to create a concrete blit command encoder
    */
    #[cfg(feature = "Metal_MTLBlitPassDescriptor")]
    unsafe impl MTLBlitPassDescriptor {
        #[method_id(@__retain_semantics Other blitPassDescriptor)]
        pub unsafe fn blitPassDescriptor() -> Id<MTLBlitPassDescriptor>;

        #[cfg(feature = "Metal_MTLBlitPassSampleBufferAttachmentDescriptorArray")]
        /**
         @property sampleBufferAttachments
        @abstract An array of sample buffers and associated sample indices.
        */
        #[method_id(@__retain_semantics Other sampleBufferAttachments)]
        pub unsafe fn sampleBufferAttachments(
            &self,
        ) -> Id<MTLBlitPassSampleBufferAttachmentDescriptorArray>;
    }
);
