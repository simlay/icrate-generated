//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Metal::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum MTLHeapType {
        MTLHeapTypeAutomatic = 0,
        MTLHeapTypePlacement = 1,
        MTLHeapTypeSparse = 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLHeapDescriptor")]
    pub struct MTLHeapDescriptor;

    #[cfg(feature = "Metal_MTLHeapDescriptor")]
    unsafe impl ClassType for MTLHeapDescriptor {
        type Super = NSObject;
    }
);

#[cfg(feature = "Metal_MTLHeapDescriptor")]
unsafe impl NSObjectProtocol for MTLHeapDescriptor {}

extern_methods!(
    #[cfg(feature = "Metal_MTLHeapDescriptor")]
    unsafe impl MTLHeapDescriptor {
        #[method(size)]
        pub fn size(&self) -> NSUInteger;

        #[method(setSize:)]
        pub fn setSize(&self, size: NSUInteger);

        #[method(storageMode)]
        pub fn storageMode(&self) -> MTLStorageMode;

        #[method(setStorageMode:)]
        pub fn setStorageMode(&self, storage_mode: MTLStorageMode);

        #[method(cpuCacheMode)]
        pub fn cpuCacheMode(&self) -> MTLCPUCacheMode;

        #[method(setCpuCacheMode:)]
        pub fn setCpuCacheMode(&self, cpu_cache_mode: MTLCPUCacheMode);

        #[method(sparsePageSize)]
        pub unsafe fn sparsePageSize(&self) -> MTLSparsePageSize;

        #[method(setSparsePageSize:)]
        pub unsafe fn setSparsePageSize(&self, sparse_page_size: MTLSparsePageSize);

        #[method(hazardTrackingMode)]
        pub fn hazardTrackingMode(&self) -> MTLHazardTrackingMode;

        #[method(setHazardTrackingMode:)]
        pub fn setHazardTrackingMode(&self, hazard_tracking_mode: MTLHazardTrackingMode);

        #[method(resourceOptions)]
        pub fn resourceOptions(&self) -> MTLResourceOptions;

        #[method(setResourceOptions:)]
        pub fn setResourceOptions(&self, resource_options: MTLResourceOptions);

        #[method(type)]
        pub unsafe fn r#type(&self) -> MTLHeapType;

        #[method(setType:)]
        pub fn setType(&self, r#type: MTLHeapType);
    }
);

extern_protocol!(
    pub unsafe trait MTLHeap: NSObjectProtocol {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other label)]
        fn label(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setLabel:)]
        fn setLabel(&self, label: Option<&NSString>);

        #[method_id(@__retain_semantics Other device)]
        fn device(&self) -> Id<ProtocolObject<dyn MTLDevice>>;

        #[method(storageMode)]
        fn storageMode(&self) -> MTLStorageMode;

        #[method(cpuCacheMode)]
        fn cpuCacheMode(&self) -> MTLCPUCacheMode;

        #[method(hazardTrackingMode)]
        fn hazardTrackingMode(&self) -> MTLHazardTrackingMode;

        #[method(resourceOptions)]
        fn resourceOptions(&self) -> MTLResourceOptions;

        #[method(size)]
        fn size(&self) -> NSUInteger;

        #[method(usedSize)]
        fn usedSize(&self) -> NSUInteger;

        #[method(currentAllocatedSize)]
        fn currentAllocatedSize(&self) -> NSUInteger;

        #[method(maxAvailableSizeWithAlignment:)]
        fn maxAvailableSizeWithAlignment(&self, alignment: NSUInteger) -> NSUInteger;

        #[method_id(@__retain_semantics New newBufferWithLength:options:)]
        fn newBufferWithLength_options(
            &self,
            length: NSUInteger,
            options: MTLResourceOptions,
        ) -> Option<Id<ProtocolObject<dyn MTLBuffer>>>;

        #[cfg(feature = "Metal_MTLTextureDescriptor")]
        #[method_id(@__retain_semantics New newTextureWithDescriptor:)]
        fn newTextureWithDescriptor(
            &self,
            desc: &MTLTextureDescriptor,
        ) -> Option<Id<ProtocolObject<dyn MTLTexture>>>;

        #[method(setPurgeableState:)]
        fn setPurgeableState(&self, state: MTLPurgeableState) -> MTLPurgeableState;

        #[method(type)]
        unsafe fn r#type(&self) -> MTLHeapType;

        #[method_id(@__retain_semantics New newBufferWithLength:options:offset:)]
        unsafe fn newBufferWithLength_options_offset(
            &self,
            length: NSUInteger,
            options: MTLResourceOptions,
            offset: NSUInteger,
        ) -> Option<Id<ProtocolObject<dyn MTLBuffer>>>;

        #[cfg(feature = "Metal_MTLTextureDescriptor")]
        #[method_id(@__retain_semantics New newTextureWithDescriptor:offset:)]
        unsafe fn newTextureWithDescriptor_offset(
            &self,
            descriptor: &MTLTextureDescriptor,
            offset: NSUInteger,
        ) -> Option<Id<ProtocolObject<dyn MTLTexture>>>;

        #[method_id(@__retain_semantics New newAccelerationStructureWithSize:)]
        unsafe fn newAccelerationStructureWithSize(
            &self,
            size: NSUInteger,
        ) -> Option<Id<ProtocolObject<dyn MTLAccelerationStructure>>>;

        #[cfg(feature = "Metal_MTLAccelerationStructureDescriptor")]
        #[method_id(@__retain_semantics New newAccelerationStructureWithDescriptor:)]
        unsafe fn newAccelerationStructureWithDescriptor(
            &self,
            descriptor: &MTLAccelerationStructureDescriptor,
        ) -> Option<Id<ProtocolObject<dyn MTLAccelerationStructure>>>;

        #[method_id(@__retain_semantics New newAccelerationStructureWithSize:offset:)]
        unsafe fn newAccelerationStructureWithSize_offset(
            &self,
            size: NSUInteger,
            offset: NSUInteger,
        ) -> Option<Id<ProtocolObject<dyn MTLAccelerationStructure>>>;

        #[cfg(feature = "Metal_MTLAccelerationStructureDescriptor")]
        #[method_id(@__retain_semantics New newAccelerationStructureWithDescriptor:offset:)]
        unsafe fn newAccelerationStructureWithDescriptor_offset(
            &self,
            descriptor: &MTLAccelerationStructureDescriptor,
            offset: NSUInteger,
        ) -> Option<Id<ProtocolObject<dyn MTLAccelerationStructure>>>;
    }

    unsafe impl ProtocolType for dyn MTLHeap {}
);
