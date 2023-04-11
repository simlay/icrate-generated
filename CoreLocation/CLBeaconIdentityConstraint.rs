//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Contacts::*;
use crate::CoreLocation::*;
use crate::Foundation::*;

pub type CLBeaconMajorValue = u16;

pub type CLBeaconMinorValue = u16;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreLocation_CLBeaconIdentityConstraint")]
    /**
       CLBeaconIdentityConstraint

      Discussion:
        A constraint that describes the identity caracteristics of a beacon.

        A beacon identity is defined by UUID, major and minor values.
        UUID must be specified. If only UUID is specified, the major and
        minor values will be wildcarded and any beacons with the same
        UUID will satisfy the constraint. Similarly if only UUID and
        major value are specified, the minor value will be wildcarded
        and any beacons with the same UUID and major value will satisfy
        the constraint.

    */
    pub struct CLBeaconIdentityConstraint;

    #[cfg(feature = "CoreLocation_CLBeaconIdentityConstraint")]
    unsafe impl ClassType for CLBeaconIdentityConstraint {
        type Super = NSObject;
    }
);

#[cfg(feature = "CoreLocation_CLBeaconIdentityConstraint")]
/**
   CLBeaconIdentityConstraint

  Discussion:
    A constraint that describes the identity caracteristics of a beacon.

    A beacon identity is defined by UUID, major and minor values.
    UUID must be specified. If only UUID is specified, the major and
    minor values will be wildcarded and any beacons with the same
    UUID will satisfy the constraint. Similarly if only UUID and
    major value are specified, the minor value will be wildcarded
    and any beacons with the same UUID and major value will satisfy
    the constraint.

*/
unsafe impl NSCoding for CLBeaconIdentityConstraint {}

#[cfg(feature = "CoreLocation_CLBeaconIdentityConstraint")]
/**
   CLBeaconIdentityConstraint

  Discussion:
    A constraint that describes the identity caracteristics of a beacon.

    A beacon identity is defined by UUID, major and minor values.
    UUID must be specified. If only UUID is specified, the major and
    minor values will be wildcarded and any beacons with the same
    UUID will satisfy the constraint. Similarly if only UUID and
    major value are specified, the minor value will be wildcarded
    and any beacons with the same UUID and major value will satisfy
    the constraint.

*/
unsafe impl NSObjectProtocol for CLBeaconIdentityConstraint {}

#[cfg(feature = "CoreLocation_CLBeaconIdentityConstraint")]
/**
   CLBeaconIdentityConstraint

  Discussion:
    A constraint that describes the identity caracteristics of a beacon.

    A beacon identity is defined by UUID, major and minor values.
    UUID must be specified. If only UUID is specified, the major and
    minor values will be wildcarded and any beacons with the same
    UUID will satisfy the constraint. Similarly if only UUID and
    major value are specified, the minor value will be wildcarded
    and any beacons with the same UUID and major value will satisfy
    the constraint.

*/
unsafe impl NSSecureCoding for CLBeaconIdentityConstraint {}

extern_methods!(
    /**
       CLBeaconIdentityConstraint

      Discussion:
        A constraint that describes the identity caracteristics of a beacon.

        A beacon identity is defined by UUID, major and minor values.
        UUID must be specified. If only UUID is specified, the major and
        minor values will be wildcarded and any beacons with the same
        UUID will satisfy the constraint. Similarly if only UUID and
        major value are specified, the minor value will be wildcarded
        and any beacons with the same UUID and major value will satisfy
        the constraint.

    */
    #[cfg(feature = "CoreLocation_CLBeaconIdentityConstraint")]
    unsafe impl CLBeaconIdentityConstraint {
        #[cfg(feature = "Foundation_NSUUID")]
        /**
           UUID

          Discussion:
            UUID associated with the beacon.

        */
        #[method_id(@__retain_semantics Other UUID)]
        pub unsafe fn UUID(&self) -> Id<NSUUID>;

        #[cfg(feature = "Foundation_NSNumber")]
        /**
           major

          Discussion:
            Most significant value associated with the beacon.

        */
        #[method_id(@__retain_semantics Other major)]
        pub unsafe fn major(&self) -> Option<Id<NSNumber>>;

        #[cfg(feature = "Foundation_NSNumber")]
        /**
           minor

          Discussion:
            Least significant value associated with the beacon.

        */
        #[method_id(@__retain_semantics Other minor)]
        pub unsafe fn minor(&self) -> Option<Id<NSNumber>>;

        #[cfg(feature = "Foundation_NSUUID")]
        #[method_id(@__retain_semantics Init initWithUUID:)]
        pub unsafe fn initWithUUID(this: Option<Allocated<Self>>, uuid: &NSUUID) -> Id<Self>;

        #[cfg(feature = "Foundation_NSUUID")]
        #[method_id(@__retain_semantics Init initWithUUID:major:)]
        pub unsafe fn initWithUUID_major(
            this: Option<Allocated<Self>>,
            uuid: &NSUUID,
            major: CLBeaconMajorValue,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSUUID")]
        #[method_id(@__retain_semantics Init initWithUUID:major:minor:)]
        pub unsafe fn initWithUUID_major_minor(
            this: Option<Allocated<Self>>,
            uuid: &NSUUID,
            major: CLBeaconMajorValue,
            minor: CLBeaconMinorValue,
        ) -> Id<Self>;
    }
);
