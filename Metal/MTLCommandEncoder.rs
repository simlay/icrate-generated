//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Metal::*;

ns_options!(
    #[underlying(NSUInteger)]
    /**
      @brief Describes how a resource will be used by a shader through an argument buffer
    */
    pub enum MTLResourceUsage {
        MTLResourceUsageRead = 1 << 0,
        MTLResourceUsageWrite = 1 << 1,
        #[deprecated]
        MTLResourceUsageSample = 1 << 2,
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    /**
      @brief Describes the types of resources that the a barrier operates on
    */
    pub enum MTLBarrierScope {
        MTLBarrierScopeBuffers = 1 << 0,
        MTLBarrierScopeTextures = 1 << 1,
        MTLBarrierScopeRenderTargets = 1 << 2,
    }
);

extern_protocol!(
    /**
     @protocol MTLCommandEncoder
    @abstract MTLCommandEncoder is the common interface for objects that write commands into MTLCommandBuffers.
    */
    pub unsafe trait MTLCommandEncoder: NSObjectProtocol {
        /**
         @property device
        @abstract The device this resource was created against.
        */
        #[method_id(@__retain_semantics Other device)]
        unsafe fn device(&self) -> Id<ProtocolObject<dyn MTLDevice>>;

        #[cfg(feature = "Foundation_NSString")]
        /**
         @property label
        @abstract A string to help identify this object.
        */
        #[method_id(@__retain_semantics Other label)]
        fn label(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        /**
         @property label
        @abstract A string to help identify this object.
        */
        #[method(setLabel:)]
        fn setLabel(&self, label: Option<&NSString>);

        #[method(endEncoding)]
        fn endEncoding(&self);

        #[cfg(feature = "Foundation_NSString")]
        #[method(insertDebugSignpost:)]
        fn insertDebugSignpost(&self, string: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[method(pushDebugGroup:)]
        fn pushDebugGroup(&self, string: &NSString);

        #[method(popDebugGroup)]
        fn popDebugGroup(&self);
    }

    unsafe impl ProtocolType for dyn MTLCommandEncoder {}
);
