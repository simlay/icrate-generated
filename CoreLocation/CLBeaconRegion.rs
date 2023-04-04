//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Contacts::*;
use crate::CoreLocation::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreLocation_CLBeaconRegion")]
    #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
    pub struct CLBeaconRegion;

    #[cfg(feature = "CoreLocation_CLBeaconRegion")]
    unsafe impl ClassType for CLBeaconRegion {
        #[inherits(NSObject)]
        type Super = CLRegion;
    }
);

#[cfg(feature = "CoreLocation_CLBeaconRegion")]
unsafe impl NSCoding for CLBeaconRegion {}

#[cfg(feature = "CoreLocation_CLBeaconRegion")]
unsafe impl NSObjectProtocol for CLBeaconRegion {}

#[cfg(feature = "CoreLocation_CLBeaconRegion")]
unsafe impl NSSecureCoding for CLBeaconRegion {}

extern_methods!(
    #[cfg(feature = "CoreLocation_CLBeaconRegion")]
    unsafe impl CLBeaconRegion {
        #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSUUID"))]
        #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
        #[method_id(@__retain_semantics Init initWithUUID:identifier:)]
        pub unsafe fn initWithUUID_identifier(
            this: Option<Allocated<Self>>,
            uuid: &NSUUID,
            identifier: &NSString,
        ) -> Id<Self>;

        #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSUUID"))]
        #[deprecated]
        #[method_id(@__retain_semantics Init initWithProximityUUID:identifier:)]
        pub unsafe fn initWithProximityUUID_identifier(
            this: Option<Allocated<Self>>,
            proximity_uuid: &NSUUID,
            identifier: &NSString,
        ) -> Id<Self>;

        #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSUUID"))]
        #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
        #[method_id(@__retain_semantics Init initWithUUID:major:identifier:)]
        pub unsafe fn initWithUUID_major_identifier(
            this: Option<Allocated<Self>>,
            uuid: &NSUUID,
            major: CLBeaconMajorValue,
            identifier: &NSString,
        ) -> Id<Self>;

        #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSUUID"))]
        #[deprecated]
        #[method_id(@__retain_semantics Init initWithProximityUUID:major:identifier:)]
        pub unsafe fn initWithProximityUUID_major_identifier(
            this: Option<Allocated<Self>>,
            proximity_uuid: &NSUUID,
            major: CLBeaconMajorValue,
            identifier: &NSString,
        ) -> Id<Self>;

        #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSUUID"))]
        #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
        #[method_id(@__retain_semantics Init initWithUUID:major:minor:identifier:)]
        pub unsafe fn initWithUUID_major_minor_identifier(
            this: Option<Allocated<Self>>,
            uuid: &NSUUID,
            major: CLBeaconMajorValue,
            minor: CLBeaconMinorValue,
            identifier: &NSString,
        ) -> Id<Self>;

        #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSUUID"))]
        #[deprecated]
        #[method_id(@__retain_semantics Init initWithProximityUUID:major:minor:identifier:)]
        pub unsafe fn initWithProximityUUID_major_minor_identifier(
            this: Option<Allocated<Self>>,
            proximity_uuid: &NSUUID,
            major: CLBeaconMajorValue,
            minor: CLBeaconMinorValue,
            identifier: &NSString,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "CoreLocation_CLBeaconIdentityConstraint",
            feature = "Foundation_NSString"
        ))]
        #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
        #[method_id(@__retain_semantics Init initWithBeaconIdentityConstraint:identifier:)]
        pub unsafe fn initWithBeaconIdentityConstraint_identifier(
            this: Option<Allocated<Self>>,
            beacon_identity_constraint: &CLBeaconIdentityConstraint,
            identifier: &NSString,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "Foundation_NSMutableDictionary",
            feature = "Foundation_NSNumber",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other peripheralDataWithMeasuredPower:)]
        pub unsafe fn peripheralDataWithMeasuredPower(
            &self,
            measured_power: Option<&NSNumber>,
        ) -> Id<NSMutableDictionary<NSString, Object>, Owned>;

        #[cfg(feature = "CoreLocation_CLBeaconIdentityConstraint")]
        #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
        #[method_id(@__retain_semantics Other beaconIdentityConstraint)]
        pub unsafe fn beaconIdentityConstraint(&self) -> Id<CLBeaconIdentityConstraint>;

        #[cfg(feature = "Foundation_NSUUID")]
        #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
        #[method_id(@__retain_semantics Other UUID)]
        pub unsafe fn UUID(&self) -> Id<NSUUID>;

        #[cfg(feature = "Foundation_NSUUID")]
        #[deprecated]
        #[method_id(@__retain_semantics Other proximityUUID)]
        pub unsafe fn proximityUUID(&self) -> Id<NSUUID>;

        #[cfg(feature = "Foundation_NSNumber")]
        #[method_id(@__retain_semantics Other major)]
        pub unsafe fn major(&self) -> Option<Id<NSNumber>>;

        #[cfg(feature = "Foundation_NSNumber")]
        #[method_id(@__retain_semantics Other minor)]
        pub unsafe fn minor(&self) -> Option<Id<NSNumber>>;

        #[method(notifyEntryStateOnDisplay)]
        pub unsafe fn notifyEntryStateOnDisplay(&self) -> bool;

        #[method(setNotifyEntryStateOnDisplay:)]
        pub unsafe fn setNotifyEntryStateOnDisplay(&self, notify_entry_state_on_display: bool);
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreLocation_CLBeacon")]
    #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
    pub struct CLBeacon;

    #[cfg(feature = "CoreLocation_CLBeacon")]
    unsafe impl ClassType for CLBeacon {
        type Super = NSObject;
    }
);

#[cfg(feature = "CoreLocation_CLBeacon")]
unsafe impl NSCoding for CLBeacon {}

#[cfg(feature = "CoreLocation_CLBeacon")]
unsafe impl NSObjectProtocol for CLBeacon {}

#[cfg(feature = "CoreLocation_CLBeacon")]
unsafe impl NSSecureCoding for CLBeacon {}

extern_methods!(
    #[cfg(feature = "CoreLocation_CLBeacon")]
    unsafe impl CLBeacon {
        #[cfg(feature = "Foundation_NSDate")]
        #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
        #[method_id(@__retain_semantics Other timestamp)]
        pub unsafe fn timestamp(&self) -> Id<NSDate>;

        #[cfg(feature = "Foundation_NSUUID")]
        #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
        #[method_id(@__retain_semantics Other UUID)]
        pub unsafe fn UUID(&self) -> Id<NSUUID>;

        #[cfg(feature = "Foundation_NSUUID")]
        #[deprecated]
        #[method_id(@__retain_semantics Other proximityUUID)]
        pub unsafe fn proximityUUID(&self) -> Id<NSUUID>;

        #[cfg(feature = "Foundation_NSNumber")]
        #[method_id(@__retain_semantics Other major)]
        pub unsafe fn major(&self) -> Id<NSNumber>;

        #[cfg(feature = "Foundation_NSNumber")]
        #[method_id(@__retain_semantics Other minor)]
        pub unsafe fn minor(&self) -> Id<NSNumber>;

        #[method(proximity)]
        pub unsafe fn proximity(&self) -> CLProximity;

        #[method(accuracy)]
        pub unsafe fn accuracy(&self) -> CLLocationAccuracy;

        #[method(rssi)]
        pub unsafe fn rssi(&self) -> NSInteger;
    }
);

extern_methods!(
    /// Methods declared on superclass `CLRegion`
    #[cfg(feature = "CoreLocation_CLBeaconRegion")]
    unsafe impl CLBeaconRegion {
        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Please see CLCircularRegion"]
        #[cfg(not(any(target_os = "tvos")))]
        #[method_id(@__retain_semantics Init initCircularRegionWithCenter:radius:identifier:)]
        pub unsafe fn initCircularRegionWithCenter_radius_identifier(
            this: Option<Allocated<Self>>,
            center: CLLocationCoordinate2D,
            radius: CLLocationDistance,
            identifier: &NSString,
        ) -> Id<Self>;
    }
);
