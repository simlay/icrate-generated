//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Contacts::*;
use crate::CoreLocation::*;
use crate::Foundation::*;

pub type CLLocationDegrees = c_double;

pub type CLLocationAccuracy = c_double;

pub type CLLocationSpeed = c_double;

pub type CLLocationSpeedAccuracy = c_double;

pub type CLLocationDirection = c_double;

pub type CLLocationDirectionAccuracy = c_double;

extern_struct!(
    /**
       CLLocationCoordinate2D

      Discussion:
        A structure that contains a geographical coordinate.

      Fields:
        latitude:
          The latitude in degrees.
        longitude:
          The longitude in degrees.
    */
    pub struct CLLocationCoordinate2D {
        pub latitude: CLLocationDegrees,
        pub longitude: CLLocationDegrees,
    }
);

pub type CLLocationDistance = c_double;

extern_static!(kCLDistanceFilterNone: CLLocationDistance);

extern_static!(kCLLocationAccuracyBestForNavigation: CLLocationAccuracy);

extern_static!(kCLLocationAccuracyBest: CLLocationAccuracy);

extern_static!(kCLLocationAccuracyNearestTenMeters: CLLocationAccuracy);

extern_static!(kCLLocationAccuracyHundredMeters: CLLocationAccuracy);

extern_static!(kCLLocationAccuracyKilometer: CLLocationAccuracy);

extern_static!(kCLLocationAccuracyThreeKilometers: CLLocationAccuracy);

extern_static!(kCLLocationAccuracyReduced: CLLocationAccuracy);

extern_static!(CLLocationDistanceMax: CLLocationDistance);

extern_static!(CLTimeIntervalMax: NSTimeInterval);

extern_static!(kCLLocationCoordinate2DInvalid: CLLocationCoordinate2D);

extern_fn!(
    pub unsafe fn CLLocationCoordinate2DIsValid(coord: CLLocationCoordinate2D) -> Bool;
);

extern_fn!(
    pub unsafe fn CLLocationCoordinate2DMake(
        latitude: CLLocationDegrees,
        longitude: CLLocationDegrees,
    ) -> CLLocationCoordinate2D;
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreLocation_CLFloor")]
    /**
       CLFloor

      Discussion:
        Encapsulates the information about a floor.
    */
    pub struct CLFloor;

    #[cfg(feature = "CoreLocation_CLFloor")]
    unsafe impl ClassType for CLFloor {
        type Super = NSObject;
    }
);

#[cfg(feature = "CoreLocation_CLFloor")]
/**
   CLFloor

  Discussion:
    Encapsulates the information about a floor.
*/
unsafe impl NSCoding for CLFloor {}

#[cfg(feature = "CoreLocation_CLFloor")]
/**
   CLFloor

  Discussion:
    Encapsulates the information about a floor.
*/
unsafe impl NSObjectProtocol for CLFloor {}

#[cfg(feature = "CoreLocation_CLFloor")]
/**
   CLFloor

  Discussion:
    Encapsulates the information about a floor.
*/
unsafe impl NSSecureCoding for CLFloor {}

extern_methods!(
    /**
       CLFloor

      Discussion:
        Encapsulates the information about a floor.
    */
    #[cfg(feature = "CoreLocation_CLFloor")]
    unsafe impl CLFloor {
        /**
           level

          Discussion:
            This is a logical representation that will vary on definition from building-to-building.
            Floor 0 will always represent the floor designated as "ground".
            This number may be negative to designate floors below the ground floor
            and positive to indicate floors above the ground floor.
            It is not intended to match any numbering that might actually be used in the building.
            It is erroneous to use as an estimate of altitude.
        */
        #[method(level)]
        pub unsafe fn level(&self) -> NSInteger;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreLocation_CLLocationSourceInformation")]
    /**
      CLLocationSourceInformation

     Discussion:
         For a CLLocation, represents information about the source of the location, including whether it came from a simulator
    */
    pub struct CLLocationSourceInformation;

    #[cfg(feature = "CoreLocation_CLLocationSourceInformation")]
    unsafe impl ClassType for CLLocationSourceInformation {
        type Super = NSObject;
    }
);

#[cfg(feature = "CoreLocation_CLLocationSourceInformation")]
/**
  CLLocationSourceInformation

 Discussion:
     For a CLLocation, represents information about the source of the location, including whether it came from a simulator
*/
unsafe impl NSCoding for CLLocationSourceInformation {}

#[cfg(feature = "CoreLocation_CLLocationSourceInformation")]
/**
  CLLocationSourceInformation

 Discussion:
     For a CLLocation, represents information about the source of the location, including whether it came from a simulator
*/
unsafe impl NSObjectProtocol for CLLocationSourceInformation {}

#[cfg(feature = "CoreLocation_CLLocationSourceInformation")]
/**
  CLLocationSourceInformation

 Discussion:
     For a CLLocation, represents information about the source of the location, including whether it came from a simulator
*/
unsafe impl NSSecureCoding for CLLocationSourceInformation {}

extern_methods!(
    /**
      CLLocationSourceInformation

     Discussion:
         For a CLLocation, represents information about the source of the location, including whether it came from a simulator
    */
    #[cfg(feature = "CoreLocation_CLLocationSourceInformation")]
    unsafe impl CLLocationSourceInformation {
        #[method_id(@__retain_semantics Init initWithSoftwareSimulationState:andExternalAccessoryState:)]
        pub unsafe fn initWithSoftwareSimulationState_andExternalAccessoryState(
            this: Option<Allocated<Self>>,
            is_software: bool,
            is_accessory: bool,
        ) -> Id<Self>;

        /**
          isSimulatedBySoftware

         Discussion:
            Set to YES if this location was detected as being generated by a software simulator, such as Xcode
        */
        #[method(isSimulatedBySoftware)]
        pub unsafe fn isSimulatedBySoftware(&self) -> bool;

        /**
          isProducedByAccessory

         Discussion:
             Set to YES if this location was generated from an external accessory, such as CarPlay or an MFi accessory
        */
        #[method(isProducedByAccessory)]
        pub unsafe fn isProducedByAccessory(&self) -> bool;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreLocation_CLLocation")]
    /**
       CLLocation

      Discussion:
        Represents a geographical coordinate along with accuracy and timestamp information.
    */
    pub struct CLLocation;

    #[cfg(feature = "CoreLocation_CLLocation")]
    unsafe impl ClassType for CLLocation {
        type Super = NSObject;
    }
);

#[cfg(feature = "CoreLocation_CLLocation")]
/**
   CLLocation

  Discussion:
    Represents a geographical coordinate along with accuracy and timestamp information.
*/
unsafe impl NSCoding for CLLocation {}

#[cfg(feature = "CoreLocation_CLLocation")]
/**
   CLLocation

  Discussion:
    Represents a geographical coordinate along with accuracy and timestamp information.
*/
unsafe impl NSObjectProtocol for CLLocation {}

#[cfg(feature = "CoreLocation_CLLocation")]
/**
   CLLocation

  Discussion:
    Represents a geographical coordinate along with accuracy and timestamp information.
*/
unsafe impl NSSecureCoding for CLLocation {}

extern_methods!(
    /**
       CLLocation

      Discussion:
        Represents a geographical coordinate along with accuracy and timestamp information.
    */
    #[cfg(feature = "CoreLocation_CLLocation")]
    unsafe impl CLLocation {
        #[method_id(@__retain_semantics Init initWithLatitude:longitude:)]
        pub unsafe fn initWithLatitude_longitude(
            this: Option<Allocated<Self>>,
            latitude: CLLocationDegrees,
            longitude: CLLocationDegrees,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Init initWithCoordinate:altitude:horizontalAccuracy:verticalAccuracy:timestamp:)]
        pub unsafe fn initWithCoordinate_altitude_horizontalAccuracy_verticalAccuracy_timestamp(
            this: Option<Allocated<Self>>,
            coordinate: CLLocationCoordinate2D,
            altitude: CLLocationDistance,
            h_accuracy: CLLocationAccuracy,
            v_accuracy: CLLocationAccuracy,
            timestamp: &NSDate,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Init initWithCoordinate:altitude:horizontalAccuracy:verticalAccuracy:course:speed:timestamp:)]
        pub unsafe fn initWithCoordinate_altitude_horizontalAccuracy_verticalAccuracy_course_speed_timestamp(
            this: Option<Allocated<Self>>,
            coordinate: CLLocationCoordinate2D,
            altitude: CLLocationDistance,
            h_accuracy: CLLocationAccuracy,
            v_accuracy: CLLocationAccuracy,
            course: CLLocationDirection,
            speed: CLLocationSpeed,
            timestamp: &NSDate,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Init initWithCoordinate:altitude:horizontalAccuracy:verticalAccuracy:course:courseAccuracy:speed:speedAccuracy:timestamp:)]
        pub unsafe fn initWithCoordinate_altitude_horizontalAccuracy_verticalAccuracy_course_courseAccuracy_speed_speedAccuracy_timestamp(
            this: Option<Allocated<Self>>,
            coordinate: CLLocationCoordinate2D,
            altitude: CLLocationDistance,
            h_accuracy: CLLocationAccuracy,
            v_accuracy: CLLocationAccuracy,
            course: CLLocationDirection,
            course_accuracy: CLLocationDirectionAccuracy,
            speed: CLLocationSpeed,
            speed_accuracy: CLLocationSpeedAccuracy,
            timestamp: &NSDate,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "CoreLocation_CLLocationSourceInformation",
            feature = "Foundation_NSDate"
        ))]
        #[method_id(@__retain_semantics Init initWithCoordinate:altitude:horizontalAccuracy:verticalAccuracy:course:courseAccuracy:speed:speedAccuracy:timestamp:sourceInfo:)]
        pub unsafe fn initWithCoordinate_altitude_horizontalAccuracy_verticalAccuracy_course_courseAccuracy_speed_speedAccuracy_timestamp_sourceInfo(
            this: Option<Allocated<Self>>,
            coordinate: CLLocationCoordinate2D,
            altitude: CLLocationDistance,
            h_accuracy: CLLocationAccuracy,
            v_accuracy: CLLocationAccuracy,
            course: CLLocationDirection,
            course_accuracy: CLLocationDirectionAccuracy,
            speed: CLLocationSpeed,
            speed_accuracy: CLLocationSpeedAccuracy,
            timestamp: &NSDate,
            source_info: &CLLocationSourceInformation,
        ) -> Id<Self>;

        /**
           coordinate

          Discussion:
            Returns the coordinate of the current location.
        */
        #[method(coordinate)]
        pub unsafe fn coordinate(&self) -> CLLocationCoordinate2D;

        /**
           altitude

          Discussion:
            Returns the altitude of the location. Can be positive (above sea level) or negative (below sea level).
        */
        #[method(altitude)]
        pub unsafe fn altitude(&self) -> CLLocationDistance;

        /**
           ellipsoidalAltitude

          Discussion:
            Returns the ellipsoidal altitude of the location under the WGS 84 reference frame.
            Can be positive or negative.
        */
        #[method(ellipsoidalAltitude)]
        pub unsafe fn ellipsoidalAltitude(&self) -> CLLocationDistance;

        /**
           horizontalAccuracy

          Discussion:
            Returns the horizontal accuracy of the location. Negative if the lateral location is invalid.
        */
        #[method(horizontalAccuracy)]
        pub unsafe fn horizontalAccuracy(&self) -> CLLocationAccuracy;

        /**
           verticalAccuracy

          Discussion:
            Returns the vertical accuracy of the location. Negative if the altitude is invalid.
        */
        #[method(verticalAccuracy)]
        pub unsafe fn verticalAccuracy(&self) -> CLLocationAccuracy;

        /**
           course

          Discussion:
            Returns the course of the location in degrees true North. Negative if course is invalid.

          Range:
            0.0 - 359.9 degrees, 0 being true North
        */
        #[method(course)]
        pub unsafe fn course(&self) -> CLLocationDirection;

        /**
           courseAccuracy

          Discussion:
            Returns the course accuracy of the location in degrees.  Returns negative if course is invalid.
        */
        #[method(courseAccuracy)]
        pub unsafe fn courseAccuracy(&self) -> CLLocationDirectionAccuracy;

        /**
           speed

          Discussion:
            Returns the speed of the location in m/s. Negative if speed is invalid.
        */
        #[method(speed)]
        pub unsafe fn speed(&self) -> CLLocationSpeed;

        /**
           speedAccuracy

          Discussion:
            Returns the speed accuracy of the location in m/s. Returns -1 if invalid.
        */
        #[method(speedAccuracy)]
        pub unsafe fn speedAccuracy(&self) -> CLLocationSpeedAccuracy;

        #[cfg(feature = "Foundation_NSDate")]
        /**
           timestamp

          Discussion:
            Returns the timestamp when this location was determined.
        */
        #[method_id(@__retain_semantics Other timestamp)]
        pub unsafe fn timestamp(&self) -> Id<NSDate>;

        #[cfg(feature = "CoreLocation_CLFloor")]
        /**
           floor

          Discussion:
            Contains information about the logical floor that you are on
            in the current building if you are inside a supported venue.
            This will be nil if the floor is unavailable.
        */
        #[method_id(@__retain_semantics Other floor)]
        pub unsafe fn floor(&self) -> Option<Id<CLFloor>>;

        #[cfg(feature = "CoreLocation_CLLocationSourceInformation")]
        /**
           sourceInformation

          Discussion:
            Contains information about the source of this location.
        */
        #[method_id(@__retain_semantics Other sourceInformation)]
        pub unsafe fn sourceInformation(&self) -> Option<Id<CLLocationSourceInformation>>;
    }
);
