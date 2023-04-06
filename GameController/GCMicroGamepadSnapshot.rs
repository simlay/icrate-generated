//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameController::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameController_GCMicroGamepadSnapshot")]
    #[deprecated = "GCMicroGamepadSnapshot has been deprecated, use [GCController controllerWithMicroGamepad] instead"]
    pub struct GCMicroGamepadSnapshot;

    #[deprecated = "GCMicroGamepadSnapshot has been deprecated, use [GCController controllerWithMicroGamepad] instead"]
    #[cfg(feature = "GameController_GCMicroGamepadSnapshot")]
    unsafe impl ClassType for GCMicroGamepadSnapshot {
        #[inherits(GCPhysicalInputProfile, NSObject)]
        type Super = GCMicroGamepad;
    }
);

#[cfg(feature = "GameController_GCMicroGamepadSnapshot")]
unsafe impl NSObjectProtocol for GCMicroGamepadSnapshot {}

extern_methods!(
    #[cfg(feature = "GameController_GCMicroGamepadSnapshot")]
    unsafe impl GCMicroGamepadSnapshot {
        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other snapshotData)]
        pub unsafe fn snapshotData(&self) -> Id<NSData>;

        #[cfg(feature = "Foundation_NSData")]
        #[method(setSnapshotData:)]
        pub unsafe fn setSnapshotData(&self, snapshot_data: &NSData);

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Init initWithSnapshotData:)]
        pub unsafe fn initWithSnapshotData(
            this: Option<Allocated<Self>>,
            data: &NSData,
        ) -> Id<Self>;

        #[cfg(all(feature = "Foundation_NSData", feature = "GameController_GCController"))]
        #[method_id(@__retain_semantics Init initWithController:snapshotData:)]
        pub unsafe fn initWithController_snapshotData(
            this: Option<Allocated<Self>>,
            controller: &GCController,
            data: &NSData,
        ) -> Id<Self>;
    }
);

#[deprecated = "GCMicroGamepadSnapshot has been deprecated, use [GCController controllerWithMicroGamepad] instead"]
ns_enum!(
    #[underlying(NSInteger)]
    pub enum GCMicroGamepadSnapshotDataVersion {
        #[deprecated = "GCMicroGamepadSnapshot has been deprecated, use [GCController controllerWithMicroGamepad] instead"]
        GCMicroGamepadSnapshotDataVersion1 = 0x0100,
    }
);

#[deprecated = "GCMicroGamepadSnapshot has been deprecated, use [GCController controllerWithMicroGamepad] instead"]
extern_static!(GCCurrentMicroGamepadSnapshotDataVersion: GCMicroGamepadSnapshotDataVersion);

extern_struct!(
    #[encoding_name("?")]
    #[deprecated = "GCMicroGamepadSnapshot has been deprecated, use [GCController controllerWithMicroGamepad] instead"]
    pub struct GCMicroGamepadSnapshotData {
        pub version: u16,
        pub size: u16,
        pub dpadX: c_float,
        pub dpadY: c_float,
        pub buttonA: c_float,
        pub buttonX: c_float,
    }
);

extern_fn!(
    #[cfg(feature = "Foundation_NSData")]
    #[deprecated = "GCMicroGamepadSnapshot has been deprecated, use [GCController controllerWithMicroGamepad] instead"]
    pub unsafe fn GCMicroGamepadSnapshotDataFromNSData(
        snapshot_data: *mut GCMicroGamepadSnapshotData,
        data: Option<&NSData>,
    ) -> Bool;
);

extern_fn!(
    #[cfg(feature = "Foundation_NSData")]
    #[deprecated = "GCMicroGamepadSnapshot has been deprecated, use [GCController controllerWithMicroGamepad] instead"]
    pub unsafe fn NSDataFromGCMicroGamepadSnapshotData(
        snapshot_data: *mut GCMicroGamepadSnapshotData,
    ) -> *mut NSData;
);

extern_struct!(
    #[encoding_name("?")]
    #[deprecated = "GCMicroGamepadSnapshot has been deprecated, use [GCController controllerWithMicroGamepad] instead"]
    pub struct GCMicroGamepadSnapShotDataV100 {
        pub version: u16,
        pub size: u16,
        pub dpadX: c_float,
        pub dpadY: c_float,
        pub buttonA: c_float,
        pub buttonX: c_float,
    }
);

extern_fn!(
    #[cfg(feature = "Foundation_NSData")]
    #[deprecated = "GCMicroGamepadSnapshot has been deprecated, use [GCController controllerWithMicroGamepad] instead"]
    pub unsafe fn GCMicroGamepadSnapShotDataV100FromNSData(
        snapshot_data: *mut GCMicroGamepadSnapShotDataV100,
        data: Option<&NSData>,
    ) -> Bool;
);

extern_fn!(
    #[cfg(feature = "Foundation_NSData")]
    #[deprecated = "GCMicroGamepadSnapshot has been deprecated, use [GCController controllerWithMicroGamepad] instead"]
    pub unsafe fn NSDataFromGCMicroGamepadSnapShotDataV100(
        snapshot_data: *mut GCMicroGamepadSnapShotDataV100,
    ) -> *mut NSData;
);
