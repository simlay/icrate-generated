//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameController::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameController_GCGamepadSnapshot")]
    /**
     A GCGamepadSnapshot snapshot is a concrete GCGamepad implementation. It can be used directly in an
    application to implement controller input replays. It is also returned as the result API_DEPRECATED("GCGamepad has been deprecated, use GCExtendedGamepad instead", macos(10.9, 10.15), ios(7.0, 13.0), tvos(7.0, 13.0))of polling
    a controller.

    The current snapshotData is readily available to access as NSData. A developer can serialize this to any
    destination necessary using the NSData API.

    The data contains some version of a GCGamepadSnapShotData structure.

    @see -[GCGamepad saveSnapshot]
    */
    #[deprecated = "GCGamepad has been deprecated, use GCExtendedGamepad instead"]
    pub struct GCGamepadSnapshot;

    #[cfg(feature = "GameController_GCGamepadSnapshot")]
    unsafe impl ClassType for GCGamepadSnapshot {
        #[inherits(GCPhysicalInputProfile, NSObject)]
        type Super = GCGamepad;
    }
);

#[cfg(feature = "GameController_GCGamepadSnapshot")]
/**
 A GCGamepadSnapshot snapshot is a concrete GCGamepad implementation. It can be used directly in an
application to implement controller input replays. It is also returned as the result API_DEPRECATED("GCGamepad has been deprecated, use GCExtendedGamepad instead", macos(10.9, 10.15), ios(7.0, 13.0), tvos(7.0, 13.0))of polling
a controller.

The current snapshotData is readily available to access as NSData. A developer can serialize this to any
destination necessary using the NSData API.

The data contains some version of a GCGamepadSnapShotData structure.

@see -[GCGamepad saveSnapshot]
*/
unsafe impl NSObjectProtocol for GCGamepadSnapshot {}

extern_methods!(
    /**
     A GCGamepadSnapshot snapshot is a concrete GCGamepad implementation. It can be used directly in an
    application to implement controller input replays. It is also returned as the result API_DEPRECATED("GCGamepad has been deprecated, use GCExtendedGamepad instead", macos(10.9, 10.15), ios(7.0, 13.0), tvos(7.0, 13.0))of polling
    a controller.

    The current snapshotData is readily available to access as NSData. A developer can serialize this to any
    destination necessary using the NSData API.

    The data contains some version of a GCGamepadSnapShotData structure.

    @see -[GCGamepad saveSnapshot]
    */
    #[cfg(feature = "GameController_GCGamepadSnapshot")]
    unsafe impl GCGamepadSnapshot {
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

extern_struct!(
    #[encoding_name("?")]
    #[deprecated = "GCGamepad has been deprecated, use GCExtendedGamepad instead"]
    pub struct GCGamepadSnapShotDataV100 {
        pub version: u16,
        pub size: u16,
        pub dpadX: c_float,
        pub dpadY: c_float,
        pub buttonA: c_float,
        pub buttonB: c_float,
        pub buttonX: c_float,
        pub buttonY: c_float,
        pub leftShoulder: c_float,
        pub rightShoulder: c_float,
    }
);

extern_fn!(
    #[cfg(feature = "Foundation_NSData")]
    #[deprecated = "GCGamepad has been deprecated, use GCExtendedGamepad instead"]
    pub unsafe fn GCGamepadSnapShotDataV100FromNSData(
        snapshot_data: *mut GCGamepadSnapShotDataV100,
        data: Option<&NSData>,
    ) -> Bool;
);

extern_fn!(
    #[cfg(feature = "Foundation_NSData")]
    #[deprecated = "GCGamepad has been deprecated, use GCExtendedGamepad instead"]
    pub unsafe fn NSDataFromGCGamepadSnapShotDataV100(
        snapshot_data: *mut GCGamepadSnapShotDataV100,
    ) -> *mut NSData;
);
