//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Metal::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLRasterizationRateSampleArray")]
    pub struct MTLRasterizationRateSampleArray;

    #[cfg(feature = "Metal_MTLRasterizationRateSampleArray")]
    unsafe impl ClassType for MTLRasterizationRateSampleArray {
        type Super = NSObject;
    }
);

#[cfg(feature = "Metal_MTLRasterizationRateSampleArray")]
unsafe impl NSObjectProtocol for MTLRasterizationRateSampleArray {}

extern_methods!(
    #[cfg(feature = "Metal_MTLRasterizationRateSampleArray")]
    unsafe impl MTLRasterizationRateSampleArray {
        #[cfg(feature = "Foundation_NSNumber")]
        #[method_id(@__retain_semantics Other objectAtIndexedSubscript:)]
        pub unsafe fn objectAtIndexedSubscript(&self, index: NSUInteger) -> Id<NSNumber>;

        #[cfg(feature = "Foundation_NSNumber")]
        #[method(setObject:atIndexedSubscript:)]
        pub unsafe fn setObject_atIndexedSubscript(&self, value: &NSNumber, index: NSUInteger);
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLRasterizationRateLayerDescriptor")]
    pub struct MTLRasterizationRateLayerDescriptor;

    #[cfg(feature = "Metal_MTLRasterizationRateLayerDescriptor")]
    unsafe impl ClassType for MTLRasterizationRateLayerDescriptor {
        type Super = NSObject;
    }
);

#[cfg(feature = "Metal_MTLRasterizationRateLayerDescriptor")]
unsafe impl NSObjectProtocol for MTLRasterizationRateLayerDescriptor {}

extern_methods!(
    #[cfg(feature = "Metal_MTLRasterizationRateLayerDescriptor")]
    unsafe impl MTLRasterizationRateLayerDescriptor {
        #[cfg(not(any(target_os = "ios", target_os = "macos")))]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithSampleCount:)]
        pub unsafe fn initWithSampleCount(
            this: Option<Allocated<Self>>,
            sample_count: MTLSize,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithSampleCount:horizontal:vertical:)]
        pub unsafe fn initWithSampleCount_horizontal_vertical(
            this: Option<Allocated<Self>>,
            sample_count: MTLSize,
            horizontal: NonNull<c_float>,
            vertical: NonNull<c_float>,
        ) -> Id<Self>;

        #[method(maxSampleCount)]
        pub unsafe fn maxSampleCount(&self) -> MTLSize;

        #[method(horizontalSampleStorage)]
        pub unsafe fn horizontalSampleStorage(&self) -> NonNull<c_float>;

        #[method(verticalSampleStorage)]
        pub unsafe fn verticalSampleStorage(&self) -> NonNull<c_float>;

        #[cfg(feature = "Metal_MTLRasterizationRateSampleArray")]
        #[method_id(@__retain_semantics Other horizontal)]
        pub unsafe fn horizontal(&self) -> Id<MTLRasterizationRateSampleArray>;

        #[cfg(feature = "Metal_MTLRasterizationRateSampleArray")]
        #[method_id(@__retain_semantics Other vertical)]
        pub unsafe fn vertical(&self) -> Id<MTLRasterizationRateSampleArray>;
    }
);

extern_methods!(
    #[cfg(feature = "Metal_MTLRasterizationRateLayerDescriptor")]
    unsafe impl MTLRasterizationRateLayerDescriptor {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLRasterizationRateLayerArray")]
    pub struct MTLRasterizationRateLayerArray;

    #[cfg(feature = "Metal_MTLRasterizationRateLayerArray")]
    unsafe impl ClassType for MTLRasterizationRateLayerArray {
        type Super = NSObject;
    }
);

#[cfg(feature = "Metal_MTLRasterizationRateLayerArray")]
unsafe impl NSObjectProtocol for MTLRasterizationRateLayerArray {}

extern_methods!(
    #[cfg(feature = "Metal_MTLRasterizationRateLayerArray")]
    unsafe impl MTLRasterizationRateLayerArray {
        #[cfg(feature = "Metal_MTLRasterizationRateLayerDescriptor")]
        #[method_id(@__retain_semantics Other objectAtIndexedSubscript:)]
        pub unsafe fn objectAtIndexedSubscript(
            &self,
            layer_index: NSUInteger,
        ) -> Option<Id<MTLRasterizationRateLayerDescriptor>>;

        #[cfg(feature = "Metal_MTLRasterizationRateLayerDescriptor")]
        #[method(setObject:atIndexedSubscript:)]
        pub unsafe fn setObject_atIndexedSubscript(
            &self,
            layer: Option<&MTLRasterizationRateLayerDescriptor>,
            layer_index: NSUInteger,
        );
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Metal_MTLRasterizationRateMapDescriptor")]
    pub struct MTLRasterizationRateMapDescriptor;

    #[cfg(feature = "Metal_MTLRasterizationRateMapDescriptor")]
    unsafe impl ClassType for MTLRasterizationRateMapDescriptor {
        type Super = NSObject;
    }
);

#[cfg(feature = "Metal_MTLRasterizationRateMapDescriptor")]
unsafe impl NSObjectProtocol for MTLRasterizationRateMapDescriptor {}

extern_methods!(
    #[cfg(feature = "Metal_MTLRasterizationRateMapDescriptor")]
    unsafe impl MTLRasterizationRateMapDescriptor {
        #[method_id(@__retain_semantics Other rasterizationRateMapDescriptorWithScreenSize:)]
        pub unsafe fn rasterizationRateMapDescriptorWithScreenSize(
            screen_size: MTLSize,
        ) -> Id<MTLRasterizationRateMapDescriptor>;

        #[cfg(feature = "Metal_MTLRasterizationRateLayerDescriptor")]
        #[method_id(@__retain_semantics Other rasterizationRateMapDescriptorWithScreenSize:layer:)]
        pub unsafe fn rasterizationRateMapDescriptorWithScreenSize_layer(
            screen_size: MTLSize,
            layer: &MTLRasterizationRateLayerDescriptor,
        ) -> Id<MTLRasterizationRateMapDescriptor>;

        #[cfg(feature = "Metal_MTLRasterizationRateLayerDescriptor")]
        #[method_id(@__retain_semantics Other rasterizationRateMapDescriptorWithScreenSize:layerCount:layers:)]
        pub unsafe fn rasterizationRateMapDescriptorWithScreenSize_layerCount_layers(
            screen_size: MTLSize,
            layer_count: NSUInteger,
            layers: &mut Id<MTLRasterizationRateLayerDescriptor>,
        ) -> Id<MTLRasterizationRateMapDescriptor>;

        #[cfg(feature = "Metal_MTLRasterizationRateLayerDescriptor")]
        #[method_id(@__retain_semantics Other layerAtIndex:)]
        pub unsafe fn layerAtIndex(
            &self,
            layer_index: NSUInteger,
        ) -> Option<Id<MTLRasterizationRateLayerDescriptor>>;

        #[cfg(feature = "Metal_MTLRasterizationRateLayerDescriptor")]
        #[method(setLayer:atIndex:)]
        pub unsafe fn setLayer_atIndex(
            &self,
            layer: Option<&MTLRasterizationRateLayerDescriptor>,
            layer_index: NSUInteger,
        );

        #[cfg(feature = "Metal_MTLRasterizationRateLayerArray")]
        #[method_id(@__retain_semantics Other layers)]
        pub unsafe fn layers(&self) -> Id<MTLRasterizationRateLayerArray>;

        #[method(screenSize)]
        pub unsafe fn screenSize(&self) -> MTLSize;

        #[method(setScreenSize:)]
        pub unsafe fn setScreenSize(&self, screen_size: MTLSize);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other label)]
        pub unsafe fn label(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setLabel:)]
        pub unsafe fn setLabel(&self, label: Option<&NSString>);

        #[method(layerCount)]
        pub unsafe fn layerCount(&self) -> NSUInteger;
    }
);

extern_protocol!(
    pub unsafe trait MTLRasterizationRateMap: NSObjectProtocol {
        #[method_id(@__retain_semantics Other device)]
        unsafe fn device(&self) -> Id<ProtocolObject<dyn MTLDevice>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other label)]
        unsafe fn label(&self) -> Option<Id<NSString>>;

        #[method(screenSize)]
        unsafe fn screenSize(&self) -> MTLSize;

        #[method(physicalGranularity)]
        unsafe fn physicalGranularity(&self) -> MTLSize;

        #[method(layerCount)]
        unsafe fn layerCount(&self) -> NSUInteger;

        #[method(parameterBufferSizeAndAlign)]
        unsafe fn parameterBufferSizeAndAlign(&self) -> MTLSizeAndAlign;

        #[method(copyParameterDataToBuffer:offset:)]
        unsafe fn copyParameterDataToBuffer_offset(
            &self,
            buffer: &ProtocolObject<dyn MTLBuffer>,
            offset: NSUInteger,
        );

        #[method(physicalSizeForLayer:)]
        unsafe fn physicalSizeForLayer(&self, layer_index: NSUInteger) -> MTLSize;

        #[method(mapScreenToPhysicalCoordinates:forLayer:)]
        unsafe fn mapScreenToPhysicalCoordinates_forLayer(
            &self,
            screen_coordinates: MTLCoordinate2D,
            layer_index: NSUInteger,
        ) -> MTLCoordinate2D;

        #[method(mapPhysicalToScreenCoordinates:forLayer:)]
        unsafe fn mapPhysicalToScreenCoordinates_forLayer(
            &self,
            physical_coordinates: MTLCoordinate2D,
            layer_index: NSUInteger,
        ) -> MTLCoordinate2D;
    }

    unsafe impl ProtocolType for dyn MTLRasterizationRateMap {}
);
