//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Metal::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum MTLCommandBufferStatus {
        MTLCommandBufferStatusNotEnqueued = 0,
        MTLCommandBufferStatusEnqueued = 1,
        MTLCommandBufferStatusCommitted = 2,
        MTLCommandBufferStatusScheduled = 3,
        MTLCommandBufferStatusCompleted = 4,
        MTLCommandBufferStatusError = 5,
    }
);

extern_static!(MTLCommandBufferErrorDomain: &'static NSErrorDomain);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum MTLCommandBufferError {
        MTLCommandBufferErrorNone = 0,
        MTLCommandBufferErrorInternal = 1,
        MTLCommandBufferErrorTimeout = 2,
        MTLCommandBufferErrorPageFault = 3,
        #[deprecated]
        MTLCommandBufferErrorBlacklisted = 4,
        MTLCommandBufferErrorAccessRevoked = 4,
        MTLCommandBufferErrorNotPermitted = 7,
        MTLCommandBufferErrorOutOfMemory = 8,
        MTLCommandBufferErrorInvalidResource = 9,
        MTLCommandBufferErrorMemoryless = 10,
        MTLCommandBufferErrorDeviceRemoved = 11,
        MTLCommandBufferErrorStackOverflow = 12,
    }
);

extern_static!(MTLCommandBufferEncoderInfoErrorKey: &'static NSErrorUserInfoKey);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum MTLCommandBufferErrorOption {
        MTLCommandBufferErrorOptionNone = 0,
        MTLCommandBufferErrorOptionEncoderExecutionStatus = 1 << 0,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum MTLCommandEncoderErrorState {
        MTLCommandEncoderErrorStateUnknown = 0,
        MTLCommandEncoderErrorStateCompleted = 1,
        MTLCommandEncoderErrorStateAffected = 2,
        MTLCommandEncoderErrorStatePending = 3,
        MTLCommandEncoderErrorStateFaulted = 4,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLCommandBufferDescriptor")]
    pub struct MTLCommandBufferDescriptor;

    #[cfg(feature = "Metal_MTLCommandBufferDescriptor")]
    unsafe impl ClassType for MTLCommandBufferDescriptor {
        type Super = NSObject;
    }
);

#[cfg(feature = "Metal_MTLCommandBufferDescriptor")]
unsafe impl NSObjectProtocol for MTLCommandBufferDescriptor {}

extern_methods!(
    #[cfg(feature = "Metal_MTLCommandBufferDescriptor")]
    unsafe impl MTLCommandBufferDescriptor {
        #[method(retainedReferences)]
        pub unsafe fn retainedReferences(&self) -> bool;

        #[method(setRetainedReferences:)]
        pub unsafe fn setRetainedReferences(&self, retained_references: bool);

        #[method(errorOptions)]
        pub unsafe fn errorOptions(&self) -> MTLCommandBufferErrorOption;

        #[method(setErrorOptions:)]
        pub unsafe fn setErrorOptions(&self, error_options: MTLCommandBufferErrorOption);
    }
);

extern_protocol!(
    pub unsafe trait MTLCommandBufferEncoderInfo: NSObjectProtocol {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other label)]
        unsafe fn label(&self) -> Id<NSString>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other debugSignposts)]
        unsafe fn debugSignposts(&self) -> Id<NSArray<NSString>>;

        #[method(errorState)]
        unsafe fn errorState(&self) -> MTLCommandEncoderErrorState;
    }

    unsafe impl ProtocolType for dyn MTLCommandBufferEncoderInfo {}
);

pub type MTLCommandBufferHandler = *mut Block<(NonNull<ProtocolObject<dyn MTLCommandBuffer>>,), ()>;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum MTLDispatchType {
        MTLDispatchTypeSerial = 0,
        MTLDispatchTypeConcurrent = 1,
    }
);

extern_protocol!(
    pub unsafe trait MTLCommandBuffer: NSObjectProtocol {
        #[method_id(@__retain_semantics Other device)]
        unsafe fn device(&self) -> Id<ProtocolObject<dyn MTLDevice>>;

        #[method_id(@__retain_semantics Other commandQueue)]
        unsafe fn commandQueue(&self) -> Id<ProtocolObject<dyn MTLCommandQueue>>;

        #[method(retainedReferences)]
        unsafe fn retainedReferences(&self) -> bool;

        #[method(errorOptions)]
        unsafe fn errorOptions(&self) -> MTLCommandBufferErrorOption;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other label)]
        fn label(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setLabel:)]
        fn setLabel(&self, label: Option<&NSString>);

        #[method(kernelStartTime)]
        unsafe fn kernelStartTime(&self) -> CFTimeInterval;

        #[method(kernelEndTime)]
        unsafe fn kernelEndTime(&self) -> CFTimeInterval;

        #[method_id(@__retain_semantics Other logs)]
        unsafe fn logs(&self) -> Id<ProtocolObject<dyn MTLLogContainer>>;

        #[method(GPUStartTime)]
        unsafe fn GPUStartTime(&self) -> CFTimeInterval;

        #[method(GPUEndTime)]
        unsafe fn GPUEndTime(&self) -> CFTimeInterval;

        #[method(enqueue)]
        fn enqueue(&self);

        #[method(commit)]
        fn commit(&self);

        #[method(addScheduledHandler:)]
        unsafe fn addScheduledHandler(&self, block: MTLCommandBufferHandler);

        #[method(presentDrawable:)]
        fn presentDrawable(&self, drawable: &ProtocolObject<dyn MTLDrawable>);

        #[method(presentDrawable:atTime:)]
        unsafe fn presentDrawable_atTime(
            &self,
            drawable: &ProtocolObject<dyn MTLDrawable>,
            presentation_time: CFTimeInterval,
        );

        #[method(presentDrawable:afterMinimumDuration:)]
        unsafe fn presentDrawable_afterMinimumDuration(
            &self,
            drawable: &ProtocolObject<dyn MTLDrawable>,
            duration: CFTimeInterval,
        );

        #[method(waitUntilScheduled)]
        fn waitUntilScheduled(&self);

        #[method(addCompletedHandler:)]
        unsafe fn addCompletedHandler(&self, block: MTLCommandBufferHandler);

        #[method(waitUntilCompleted)]
        unsafe fn waitUntilCompleted(&self);

        #[method(status)]
        fn status(&self) -> MTLCommandBufferStatus;

        #[cfg(feature = "Foundation_NSError")]
        #[method_id(@__retain_semantics Other error)]
        unsafe fn error(&self) -> Option<Id<NSError>>;

        #[method_id(@__retain_semantics Other blitCommandEncoder)]
        fn blitCommandEncoder(&self) -> Option<Id<ProtocolObject<dyn MTLBlitCommandEncoder>>>;

        #[cfg(feature = "Metal_MTLRenderPassDescriptor")]
        #[method_id(@__retain_semantics Other renderCommandEncoderWithDescriptor:)]
        fn renderCommandEncoderWithDescriptor(
            &self,
            render_pass_descriptor: &MTLRenderPassDescriptor,
        ) -> Option<Id<ProtocolObject<dyn MTLRenderCommandEncoder>>>;

        #[cfg(feature = "Metal_MTLComputePassDescriptor")]
        #[method_id(@__retain_semantics Other computeCommandEncoderWithDescriptor:)]
        unsafe fn computeCommandEncoderWithDescriptor(
            &self,
            compute_pass_descriptor: &MTLComputePassDescriptor,
        ) -> Option<Id<ProtocolObject<dyn MTLComputeCommandEncoder>>>;

        #[cfg(feature = "Metal_MTLBlitPassDescriptor")]
        #[method_id(@__retain_semantics Other blitCommandEncoderWithDescriptor:)]
        unsafe fn blitCommandEncoderWithDescriptor(
            &self,
            blit_pass_descriptor: &MTLBlitPassDescriptor,
        ) -> Option<Id<ProtocolObject<dyn MTLBlitCommandEncoder>>>;

        #[method_id(@__retain_semantics Other computeCommandEncoder)]
        fn computeCommandEncoder(&self)
            -> Option<Id<ProtocolObject<dyn MTLComputeCommandEncoder>>>;

        #[method_id(@__retain_semantics Other computeCommandEncoderWithDispatchType:)]
        fn computeCommandEncoderWithDispatchType(
            &self,
            dispatch_type: MTLDispatchType,
        ) -> Option<Id<ProtocolObject<dyn MTLComputeCommandEncoder>>>;

        #[method(encodeWaitForEvent:value:)]
        fn encodeWaitForEvent_value(&self, event: &ProtocolObject<dyn MTLEvent>, value: u64);

        #[method(encodeSignalEvent:value:)]
        fn encodeSignalEvent_value(&self, event: &ProtocolObject<dyn MTLEvent>, value: u64);

        #[cfg(feature = "Metal_MTLRenderPassDescriptor")]
        #[method_id(@__retain_semantics Other parallelRenderCommandEncoderWithDescriptor:)]
        fn parallelRenderCommandEncoderWithDescriptor(
            &self,
            render_pass_descriptor: &MTLRenderPassDescriptor,
        ) -> Option<Id<ProtocolObject<dyn MTLParallelRenderCommandEncoder>>>;

        #[method_id(@__retain_semantics Other resourceStateCommandEncoder)]
        unsafe fn resourceStateCommandEncoder(
            &self,
        ) -> Option<Id<ProtocolObject<dyn MTLResourceStateCommandEncoder>>>;

        #[cfg(feature = "Metal_MTLResourceStatePassDescriptor")]
        #[method_id(@__retain_semantics Other resourceStateCommandEncoderWithDescriptor:)]
        unsafe fn resourceStateCommandEncoderWithDescriptor(
            &self,
            resource_state_pass_descriptor: &MTLResourceStatePassDescriptor,
        ) -> Option<Id<ProtocolObject<dyn MTLResourceStateCommandEncoder>>>;

        #[method_id(@__retain_semantics Other accelerationStructureCommandEncoder)]
        fn accelerationStructureCommandEncoder(
            &self,
        ) -> Option<Id<ProtocolObject<dyn MTLAccelerationStructureCommandEncoder>>>;

        #[cfg(feature = "Metal_MTLAccelerationStructurePassDescriptor")]
        #[method_id(@__retain_semantics Other accelerationStructureCommandEncoderWithDescriptor:)]
        unsafe fn accelerationStructureCommandEncoderWithDescriptor(
            &self,
            descriptor: &MTLAccelerationStructurePassDescriptor,
        ) -> Id<ProtocolObject<dyn MTLAccelerationStructureCommandEncoder>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(pushDebugGroup:)]
        fn pushDebugGroup(&self, string: &NSString);

        #[method(popDebugGroup)]
        fn popDebugGroup(&self);
    }

    unsafe impl ProtocolType for dyn MTLCommandBuffer {}
);
