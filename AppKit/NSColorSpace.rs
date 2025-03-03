//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSColorSpaceModel {
        NSColorSpaceModelUnknown = -1,
        NSColorSpaceModelGray = 0,
        NSColorSpaceModelRGB = 1,
        NSColorSpaceModelCMYK = 2,
        NSColorSpaceModelLAB = 3,
        NSColorSpaceModelDeviceN = 4,
        NSColorSpaceModelIndexed = 5,
        NSColorSpaceModelPatterned = 6,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSColorSpace")]
    pub struct NSColorSpace;

    #[cfg(feature = "AppKit_NSColorSpace")]
    unsafe impl ClassType for NSColorSpace {
        type Super = NSObject;
    }
);

#[cfg(feature = "AppKit_NSColorSpace")]
unsafe impl NSCoding for NSColorSpace {}

#[cfg(feature = "AppKit_NSColorSpace")]
unsafe impl NSObjectProtocol for NSColorSpace {}

#[cfg(feature = "AppKit_NSColorSpace")]
unsafe impl NSSecureCoding for NSColorSpace {}

extern_methods!(
    #[cfg(feature = "AppKit_NSColorSpace")]
    unsafe impl NSColorSpace {
        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Init initWithICCProfileData:)]
        pub unsafe fn initWithICCProfileData(
            this: Option<Allocated<Self>>,
            icc_data: &NSData,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other ICCProfileData)]
        pub unsafe fn ICCProfileData(&self) -> Option<Id<NSData>>;

        #[method_id(@__retain_semantics Init initWithColorSyncProfile:)]
        pub unsafe fn initWithColorSyncProfile(
            this: Option<Allocated<Self>>,
            prof: NonNull<c_void>,
        ) -> Option<Id<Self>>;

        #[method(colorSyncProfile)]
        pub unsafe fn colorSyncProfile(&self) -> *mut c_void;

        #[method(numberOfColorComponents)]
        pub unsafe fn numberOfColorComponents(&self) -> NSInteger;

        #[method(colorSpaceModel)]
        pub unsafe fn colorSpaceModel(&self) -> NSColorSpaceModel;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other localizedName)]
        pub unsafe fn localizedName(&self) -> Option<Id<NSString>>;

        #[method_id(@__retain_semantics Other sRGBColorSpace)]
        pub unsafe fn sRGBColorSpace() -> Id<NSColorSpace>;

        #[method_id(@__retain_semantics Other genericGamma22GrayColorSpace)]
        pub unsafe fn genericGamma22GrayColorSpace() -> Id<NSColorSpace>;

        #[method_id(@__retain_semantics Other extendedSRGBColorSpace)]
        pub unsafe fn extendedSRGBColorSpace() -> Id<NSColorSpace>;

        #[method_id(@__retain_semantics Other extendedGenericGamma22GrayColorSpace)]
        pub unsafe fn extendedGenericGamma22GrayColorSpace() -> Id<NSColorSpace>;

        #[method_id(@__retain_semantics Other displayP3ColorSpace)]
        pub unsafe fn displayP3ColorSpace() -> Id<NSColorSpace>;

        #[method_id(@__retain_semantics Other adobeRGB1998ColorSpace)]
        pub unsafe fn adobeRGB1998ColorSpace() -> Id<NSColorSpace>;

        #[method_id(@__retain_semantics Other genericRGBColorSpace)]
        pub unsafe fn genericRGBColorSpace() -> Id<NSColorSpace>;

        #[method_id(@__retain_semantics Other genericGrayColorSpace)]
        pub unsafe fn genericGrayColorSpace() -> Id<NSColorSpace>;

        #[method_id(@__retain_semantics Other genericCMYKColorSpace)]
        pub unsafe fn genericCMYKColorSpace() -> Id<NSColorSpace>;

        #[method_id(@__retain_semantics Other deviceRGBColorSpace)]
        pub unsafe fn deviceRGBColorSpace() -> Id<NSColorSpace>;

        #[method_id(@__retain_semantics Other deviceGrayColorSpace)]
        pub unsafe fn deviceGrayColorSpace() -> Id<NSColorSpace>;

        #[method_id(@__retain_semantics Other deviceCMYKColorSpace)]
        pub unsafe fn deviceCMYKColorSpace() -> Id<NSColorSpace>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other availableColorSpacesWithModel:)]
        pub unsafe fn availableColorSpacesWithModel(
            model: NSColorSpaceModel,
        ) -> Id<NSArray<NSColorSpace>>;
    }
);

extern_static!(NSUnknownColorSpaceModel: NSColorSpaceModel = NSColorSpaceModelUnknown);

extern_static!(NSGrayColorSpaceModel: NSColorSpaceModel = NSColorSpaceModelGray);

extern_static!(NSRGBColorSpaceModel: NSColorSpaceModel = NSColorSpaceModelRGB);

extern_static!(NSCMYKColorSpaceModel: NSColorSpaceModel = NSColorSpaceModelCMYK);

extern_static!(NSLABColorSpaceModel: NSColorSpaceModel = NSColorSpaceModelLAB);

extern_static!(NSDeviceNColorSpaceModel: NSColorSpaceModel = NSColorSpaceModelDeviceN);

extern_static!(NSIndexedColorSpaceModel: NSColorSpaceModel = NSColorSpaceModelIndexed);

extern_static!(NSPatternColorSpaceModel: NSColorSpaceModel = NSColorSpaceModelPatterned);
