//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Contacts::*;
use crate::CoreLocation::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(c_int)]
    /**
       CLDeviceOrientation

      Discussion:
          Specifies a physical device orientation, equivalent to UIDeviceOrientation.

    */
    pub enum CLDeviceOrientation {
        CLDeviceOrientationUnknown = 0,
        CLDeviceOrientationPortrait = 1,
        CLDeviceOrientationPortraitUpsideDown = 2,
        CLDeviceOrientationLandscapeLeft = 3,
        CLDeviceOrientationLandscapeRight = 4,
        CLDeviceOrientationFaceUp = 5,
        CLDeviceOrientationFaceDown = 6,
    }
);

ns_enum!(
    #[underlying(c_int)]
    /**
       CLAuthorizationStatus

      Discussion:
          Represents the current authorization state of the application.

    */
    pub enum CLAuthorizationStatus {
        kCLAuthorizationStatusNotDetermined = 0,
        kCLAuthorizationStatusRestricted = 1,
        kCLAuthorizationStatusDenied = 2,
        kCLAuthorizationStatusAuthorizedAlways = 3,
        kCLAuthorizationStatusAuthorizedWhenInUse = 4,
        #[deprecated = "Use kCLAuthorizationStatusAuthorizedAlways"]
        kCLAuthorizationStatusAuthorized = kCLAuthorizationStatusAuthorizedAlways,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum CLAccuracyAuthorization {
        CLAccuracyAuthorizationFullAccuracy = 0,
        CLAccuracyAuthorizationReducedAccuracy = 1,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    /**
         CLActivityType

      Discussion:
            Expected user activity for the lifetime of the corresponding CLLocationManager instance.
            Used to optimize the positioning experience delivered to this CLLocationManager instance
            for the expected activity.
    */
    pub enum CLActivityType {
        CLActivityTypeOther = 1,
        CLActivityTypeAutomotiveNavigation = 2,
        CLActivityTypeFitness = 3,
        CLActivityTypeOtherNavigation = 4,
        CLActivityTypeAirborne = 5,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreLocation_CLLocationManager")]
    /**
       CLLocationManager

      Discussion:
        The CLLocationManager object is your entry point to the location service.
    */
    pub struct CLLocationManager;

    #[cfg(feature = "CoreLocation_CLLocationManager")]
    unsafe impl ClassType for CLLocationManager {
        type Super = NSObject;
    }
);

#[cfg(feature = "CoreLocation_CLLocationManager")]
/**
   CLLocationManager

  Discussion:
    The CLLocationManager object is your entry point to the location service.
*/
unsafe impl NSObjectProtocol for CLLocationManager {}

extern_methods!(
    /**
       CLLocationManager

      Discussion:
        The CLLocationManager object is your entry point to the location service.
    */
    #[cfg(feature = "CoreLocation_CLLocationManager")]
    unsafe impl CLLocationManager {
        #[method(locationServicesEnabled)]
        pub unsafe fn locationServicesEnabled_class() -> bool;

        #[method(headingAvailable)]
        pub unsafe fn headingAvailable_class() -> bool;

        #[method(significantLocationChangeMonitoringAvailable)]
        pub unsafe fn significantLocationChangeMonitoringAvailable() -> bool;

        #[method(isMonitoringAvailableForClass:)]
        pub unsafe fn isMonitoringAvailableForClass(region_class: &Class) -> bool;

        #[deprecated]
        #[method(regionMonitoringAvailable)]
        pub unsafe fn regionMonitoringAvailable() -> bool;

        #[deprecated = "Use +isMonitoringAvailableForClass: and -authorizationStatus instead"]
        #[method(regionMonitoringEnabled)]
        pub unsafe fn regionMonitoringEnabled() -> bool;

        #[method(isRangingAvailable)]
        pub unsafe fn isRangingAvailable() -> bool;

        /**
           authorizationStatus

          Discussion:
              Returns the current authorization status of the calling application.
        */
        #[method(authorizationStatus)]
        pub unsafe fn authorizationStatus(&self) -> CLAuthorizationStatus;

        #[deprecated]
        #[method(authorizationStatus)]
        pub unsafe fn authorizationStatus_class() -> CLAuthorizationStatus;

        /**
           accuracyAuthorization

          Discussion:
              Returns information about the calling application's access to accurate location.  See the
              documentation for CLAccuracyAuthorization for information about how to interpret the return value.

              Note that this property should generally be interpreted in the context of the authorizationStatus
              property.  For example, if authorizationStatus == kCLAuthorizationStatusNotDetermined then accurate
              location information cannot be received even when accuracyAuthorization is
              CLAccuracyAuthorizationFullAccuracy.
        */
        #[method(accuracyAuthorization)]
        pub unsafe fn accuracyAuthorization(&self) -> CLAccuracyAuthorization;

        /**
           authorizedForWidgetUpdates

          Discussion:
              Returns true if widgets of the calling application may be eligible to receive location updates.

              If the calling application has authorizationStatus == kCLAuthorizationStatusAuthorizedWhenInUse,
              and  authorizedForWidgetUpdates == YES, then widgets will be able to get location updates if called upon
              to refresh within a few minutes of having been seen.

              If the calling application has authorizationStatus == kCLAuthorizationStatusAuthorizedAlways,
              then authorizedForWidgetUpdates will always be YES.
        */
        #[method(isAuthorizedForWidgetUpdates)]
        pub unsafe fn isAuthorizedForWidgetUpdates(&self) -> bool;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<ProtocolObject<dyn CLLocationManagerDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn CLLocationManagerDelegate>>,
        );

        /**
           locationServicesEnabled

          Discussion:
              Deprecated. Use +locationServicesEnabled instead.
        */
        #[deprecated]
        #[method(locationServicesEnabled)]
        pub unsafe fn locationServicesEnabled(&self) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        /**
           purpose

          Discussion:
              Allows the application to specify what location will be used for in their app. This
              will be displayed along with the standard Location permissions dialogs. This property will need to be
              set prior to calling startUpdatingLocation.

              Deprecated.  Set the purpose string in Info.plist using key NSLocationUsageDescription.
        */
        #[deprecated = "Set the purpose string in Info.plist using key NSLocationUsageDescription"]
        #[method_id(@__retain_semantics Other purpose)]
        pub unsafe fn purpose(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        /**
           purpose

          Discussion:
              Allows the application to specify what location will be used for in their app. This
              will be displayed along with the standard Location permissions dialogs. This property will need to be
              set prior to calling startUpdatingLocation.

              Deprecated.  Set the purpose string in Info.plist using key NSLocationUsageDescription.
        */
        #[deprecated = "Set the purpose string in Info.plist using key NSLocationUsageDescription"]
        #[method(setPurpose:)]
        pub unsafe fn setPurpose(&self, purpose: Option<&NSString>);

        /**
             activityType

          Discussion:
                Specifies the type of user activity. Currently affects behavior such as
                the determination of when location updates may be automatically paused.
                By default, CLActivityTypeOther is used.
        */
        #[method(activityType)]
        pub unsafe fn activityType(&self) -> CLActivityType;

        /**
             activityType

          Discussion:
                Specifies the type of user activity. Currently affects behavior such as
                the determination of when location updates may be automatically paused.
                By default, CLActivityTypeOther is used.
        */
        #[method(setActivityType:)]
        pub unsafe fn setActivityType(&self, activity_type: CLActivityType);

        /**
           distanceFilter

          Discussion:
              Specifies the minimum update distance in meters. Client will not be notified of movements of less
              than the stated value, unless the accuracy has improved. Pass in kCLDistanceFilterNone to be
              notified of all movements. By default, kCLDistanceFilterNone is used.
        */
        #[method(distanceFilter)]
        pub unsafe fn distanceFilter(&self) -> CLLocationDistance;

        /**
           distanceFilter

          Discussion:
              Specifies the minimum update distance in meters. Client will not be notified of movements of less
              than the stated value, unless the accuracy has improved. Pass in kCLDistanceFilterNone to be
              notified of all movements. By default, kCLDistanceFilterNone is used.
        */
        #[method(setDistanceFilter:)]
        pub unsafe fn setDistanceFilter(&self, distance_filter: CLLocationDistance);

        /**
           desiredAccuracy

          Discussion:
              The desired location accuracy. The location service will try its best to achieve
              your desired accuracy. However, it is not guaranteed. To optimize
              power performance, be sure to specify an appropriate accuracy for your usage scenario (eg,
              use a large accuracy value when only a coarse location is needed). Use kCLLocationAccuracyBest to
              achieve the best possible accuracy. Use kCLLocationAccuracyBestForNavigation for navigation.
              The default value varies by platform.
        */
        #[method(desiredAccuracy)]
        pub unsafe fn desiredAccuracy(&self) -> CLLocationAccuracy;

        /**
           desiredAccuracy

          Discussion:
              The desired location accuracy. The location service will try its best to achieve
              your desired accuracy. However, it is not guaranteed. To optimize
              power performance, be sure to specify an appropriate accuracy for your usage scenario (eg,
              use a large accuracy value when only a coarse location is needed). Use kCLLocationAccuracyBest to
              achieve the best possible accuracy. Use kCLLocationAccuracyBestForNavigation for navigation.
              The default value varies by platform.
        */
        #[method(setDesiredAccuracy:)]
        pub unsafe fn setDesiredAccuracy(&self, desired_accuracy: CLLocationAccuracy);

        /**
             pausesLocationUpdatesAutomatically

          Discussion:
                Specifies that location updates may automatically be paused when possible.
                By default, this is YES for applications linked against iOS 6.0 or later.
        */
        #[method(pausesLocationUpdatesAutomatically)]
        pub unsafe fn pausesLocationUpdatesAutomatically(&self) -> bool;

        /**
             pausesLocationUpdatesAutomatically

          Discussion:
                Specifies that location updates may automatically be paused when possible.
                By default, this is YES for applications linked against iOS 6.0 or later.
        */
        #[method(setPausesLocationUpdatesAutomatically:)]
        pub unsafe fn setPausesLocationUpdatesAutomatically(
            &self,
            pauses_location_updates_automatically: bool,
        );

        /**
           allowsBackgroundLocationUpdates

          Discussion:
              By default, this is NO for applications linked against iOS 9.0 or later,
              regardless of minimum deployment target.

              With UIBackgroundModes set to include "location" in Info.plist, you must
              also set this property to YES at runtime whenever calling
              -startUpdatingLocation with the intent to continue in the background.

              Setting this property to YES when UIBackgroundModes does not include
              "location" is a fatal error.

              Resetting this property to NO is equivalent to omitting "location" from
              the UIBackgroundModes value.  Access to location is still permitted
              whenever the application is running (ie not suspended), and has
              sufficient authorization (ie it has WhenInUse authorization and is in
              use, or it has Always authorization).  However, the app will still be
              subject to the usual task suspension rules.

              See -requestWhenInUseAuthorization and -requestAlwaysAuthorization for
              more details on possible authorization values.
        */
        #[method(allowsBackgroundLocationUpdates)]
        pub unsafe fn allowsBackgroundLocationUpdates(&self) -> bool;

        /**
           allowsBackgroundLocationUpdates

          Discussion:
              By default, this is NO for applications linked against iOS 9.0 or later,
              regardless of minimum deployment target.

              With UIBackgroundModes set to include "location" in Info.plist, you must
              also set this property to YES at runtime whenever calling
              -startUpdatingLocation with the intent to continue in the background.

              Setting this property to YES when UIBackgroundModes does not include
              "location" is a fatal error.

              Resetting this property to NO is equivalent to omitting "location" from
              the UIBackgroundModes value.  Access to location is still permitted
              whenever the application is running (ie not suspended), and has
              sufficient authorization (ie it has WhenInUse authorization and is in
              use, or it has Always authorization).  However, the app will still be
              subject to the usual task suspension rules.

              See -requestWhenInUseAuthorization and -requestAlwaysAuthorization for
              more details on possible authorization values.
        */
        #[method(setAllowsBackgroundLocationUpdates:)]
        pub unsafe fn setAllowsBackgroundLocationUpdates(
            &self,
            allows_background_location_updates: bool,
        );

        /**
           showsBackgroundLocationIndicator

          Discussion:
              Specifies that an indicator be shown when the app makes use of continuous
              background location updates.  Starting continuous background location
              updates requires the app to set UIBackgroundModes to include "location"
              and to set the property allowsBackgroundLocationUpdates to YES before
              calling -startUpdatingLocation with the intent to continue in the
              background.

              Note that this property only applies to apps with Always authorization.
              For apps with WhenInUse authorization, the indicator is always shown when
              using continuous background location updates in order to maintain user
              visibility and that the app is still in use.

              The default value of this property is NO.
        */
        #[method(showsBackgroundLocationIndicator)]
        pub unsafe fn showsBackgroundLocationIndicator(&self) -> bool;

        /**
           showsBackgroundLocationIndicator

          Discussion:
              Specifies that an indicator be shown when the app makes use of continuous
              background location updates.  Starting continuous background location
              updates requires the app to set UIBackgroundModes to include "location"
              and to set the property allowsBackgroundLocationUpdates to YES before
              calling -startUpdatingLocation with the intent to continue in the
              background.

              Note that this property only applies to apps with Always authorization.
              For apps with WhenInUse authorization, the indicator is always shown when
              using continuous background location updates in order to maintain user
              visibility and that the app is still in use.

              The default value of this property is NO.
        */
        #[method(setShowsBackgroundLocationIndicator:)]
        pub unsafe fn setShowsBackgroundLocationIndicator(
            &self,
            shows_background_location_indicator: bool,
        );

        #[cfg(feature = "CoreLocation_CLLocation")]
        /**
           location

          Discussion:
              The last location received. Will be nil until a location has been received.
        */
        #[method_id(@__retain_semantics Other location)]
        pub unsafe fn location(&self) -> Option<Id<CLLocation>>;

        /**
           headingAvailable

          Discussion:
              Deprecated. Use +headingAvailable instead.
        */
        #[deprecated]
        #[method(headingAvailable)]
        pub unsafe fn headingAvailable(&self) -> bool;

        /**
           headingFilter

          Discussion:
              Specifies the minimum amount of change in degrees needed for a heading service update. Client will not
              be notified of updates less than the stated filter value. Pass in kCLHeadingFilterNone to be
              notified of all updates. By default, 1 degree is used.
        */
        #[method(headingFilter)]
        pub unsafe fn headingFilter(&self) -> CLLocationDegrees;

        /**
           headingFilter

          Discussion:
              Specifies the minimum amount of change in degrees needed for a heading service update. Client will not
              be notified of updates less than the stated filter value. Pass in kCLHeadingFilterNone to be
              notified of all updates. By default, 1 degree is used.
        */
        #[method(setHeadingFilter:)]
        pub unsafe fn setHeadingFilter(&self, heading_filter: CLLocationDegrees);

        /**
           headingOrientation

          Discussion:
              Specifies a physical device orientation from which heading calculation should be referenced. By default,
              CLDeviceOrientationPortrait is used. CLDeviceOrientationUnknown, CLDeviceOrientationFaceUp, and
              CLDeviceOrientationFaceDown are ignored.

        */
        #[method(headingOrientation)]
        pub unsafe fn headingOrientation(&self) -> CLDeviceOrientation;

        /**
           headingOrientation

          Discussion:
              Specifies a physical device orientation from which heading calculation should be referenced. By default,
              CLDeviceOrientationPortrait is used. CLDeviceOrientationUnknown, CLDeviceOrientationFaceUp, and
              CLDeviceOrientationFaceDown are ignored.

        */
        #[method(setHeadingOrientation:)]
        pub unsafe fn setHeadingOrientation(&self, heading_orientation: CLDeviceOrientation);

        #[cfg(feature = "CoreLocation_CLHeading")]
        /**
           heading

          Discussion:
              Returns the latest heading update received, or nil if none is available.
        */
        #[method_id(@__retain_semantics Other heading)]
        pub unsafe fn heading(&self) -> Option<Id<CLHeading>>;

        /**
           maximumRegionMonitoringDistance

          Discussion:
               the maximum region size, in terms of a distance from a central point, that the framework can support.
               Attempts to register a region larger than this will generate a kCLErrorRegionMonitoringFailure.
               This value may vary based on the hardware features of the device, as well as on dynamically changing resource constraints.
        */
        #[method(maximumRegionMonitoringDistance)]
        pub unsafe fn maximumRegionMonitoringDistance(&self) -> CLLocationDistance;

        #[cfg(all(feature = "CoreLocation_CLRegion", feature = "Foundation_NSSet"))]
        /**
           monitoredRegions

          Discussion:
               Retrieve a set of objects for the regions that are currently being monitored.  If any location manager
               has been instructed to monitor a region, during this or previous launches of your application, it will
               be present in this set.
        */
        #[method_id(@__retain_semantics Other monitoredRegions)]
        pub unsafe fn monitoredRegions(&self) -> Id<NSSet<CLRegion>>;

        #[cfg(all(feature = "CoreLocation_CLRegion", feature = "Foundation_NSSet"))]
        /**
           rangedRegions

          Discussion:
               Retrieve a set of objects representing the regions for which this location manager is actively providing ranging.
        */
        #[deprecated = "Use -rangedBeaconConstraints"]
        #[method_id(@__retain_semantics Other rangedRegions)]
        pub unsafe fn rangedRegions(&self) -> Id<NSSet<CLRegion>>;

        #[cfg(all(
            feature = "CoreLocation_CLBeaconIdentityConstraint",
            feature = "Foundation_NSSet"
        ))]
        /**
           rangedBeaconConstraints

          Discussion:
              Retrieve a set of beacon constraints for which this location manager is actively providing ranging.
        */
        #[method_id(@__retain_semantics Other rangedBeaconConstraints)]
        pub unsafe fn rangedBeaconConstraints(&self) -> Id<NSSet<CLBeaconIdentityConstraint>>;

        #[method(requestWhenInUseAuthorization)]
        pub unsafe fn requestWhenInUseAuthorization(&self);

        #[method(requestAlwaysAuthorization)]
        pub unsafe fn requestAlwaysAuthorization(&self);

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[method(requestTemporaryFullAccuracyAuthorizationWithPurposeKey:completion:)]
        pub unsafe fn requestTemporaryFullAccuracyAuthorizationWithPurposeKey_completion(
            &self,
            purpose_key: &NSString,
            completion: Option<&Block<(*mut NSError,), ()>>,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method(requestTemporaryFullAccuracyAuthorizationWithPurposeKey:)]
        pub unsafe fn requestTemporaryFullAccuracyAuthorizationWithPurposeKey(
            &self,
            purpose_key: &NSString,
        );

        #[method(startUpdatingLocation)]
        pub unsafe fn startUpdatingLocation(&self);

        #[method(stopUpdatingLocation)]
        pub unsafe fn stopUpdatingLocation(&self);

        #[method(requestLocation)]
        pub unsafe fn requestLocation(&self);

        #[method(startUpdatingHeading)]
        pub unsafe fn startUpdatingHeading(&self);

        #[method(stopUpdatingHeading)]
        pub unsafe fn stopUpdatingHeading(&self);

        #[method(dismissHeadingCalibrationDisplay)]
        pub unsafe fn dismissHeadingCalibrationDisplay(&self);

        #[method(startMonitoringSignificantLocationChanges)]
        pub unsafe fn startMonitoringSignificantLocationChanges(&self);

        #[method(stopMonitoringSignificantLocationChanges)]
        pub unsafe fn stopMonitoringSignificantLocationChanges(&self);

        #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSError"))]
        #[method(startMonitoringLocationPushesWithCompletion:)]
        pub unsafe fn startMonitoringLocationPushesWithCompletion(
            &self,
            completion: Option<&Block<(*mut NSData, *mut NSError), ()>>,
        );

        #[method(stopMonitoringLocationPushes)]
        pub unsafe fn stopMonitoringLocationPushes(&self);

        #[cfg(feature = "CoreLocation_CLRegion")]
        #[deprecated]
        #[method(startMonitoringForRegion:desiredAccuracy:)]
        pub unsafe fn startMonitoringForRegion_desiredAccuracy(
            &self,
            region: &CLRegion,
            accuracy: CLLocationAccuracy,
        );

        #[cfg(feature = "CoreLocation_CLRegion")]
        #[method(stopMonitoringForRegion:)]
        pub unsafe fn stopMonitoringForRegion(&self, region: &CLRegion);

        #[cfg(feature = "CoreLocation_CLRegion")]
        #[method(startMonitoringForRegion:)]
        pub unsafe fn startMonitoringForRegion(&self, region: &CLRegion);

        #[cfg(feature = "CoreLocation_CLRegion")]
        #[method(requestStateForRegion:)]
        pub unsafe fn requestStateForRegion(&self, region: &CLRegion);

        #[cfg(feature = "CoreLocation_CLBeaconRegion")]
        #[deprecated = "Use -startRangingBeaconsSatisfyingConstraint:"]
        #[method(startRangingBeaconsInRegion:)]
        pub unsafe fn startRangingBeaconsInRegion(&self, region: &CLBeaconRegion);

        #[cfg(feature = "CoreLocation_CLBeaconRegion")]
        #[deprecated = "Use -stopRangingBeaconsSatisfyingConstraint:"]
        #[method(stopRangingBeaconsInRegion:)]
        pub unsafe fn stopRangingBeaconsInRegion(&self, region: &CLBeaconRegion);

        #[cfg(feature = "CoreLocation_CLBeaconIdentityConstraint")]
        #[method(startRangingBeaconsSatisfyingConstraint:)]
        pub unsafe fn startRangingBeaconsSatisfyingConstraint(
            &self,
            constraint: &CLBeaconIdentityConstraint,
        );

        #[cfg(feature = "CoreLocation_CLBeaconIdentityConstraint")]
        #[method(stopRangingBeaconsSatisfyingConstraint:)]
        pub unsafe fn stopRangingBeaconsSatisfyingConstraint(
            &self,
            constraint: &CLBeaconIdentityConstraint,
        );

        #[deprecated = "You can remove calls to this method"]
        #[method(allowDeferredLocationUpdatesUntilTraveled:timeout:)]
        pub unsafe fn allowDeferredLocationUpdatesUntilTraveled_timeout(
            &self,
            distance: CLLocationDistance,
            timeout: NSTimeInterval,
        );

        #[deprecated = "You can remove calls to this method"]
        #[method(disallowDeferredLocationUpdates)]
        pub unsafe fn disallowDeferredLocationUpdates(&self);

        #[deprecated = "You can remove calls to this method"]
        #[method(deferredLocationUpdatesAvailable)]
        pub unsafe fn deferredLocationUpdatesAvailable() -> bool;

        #[cfg(all(
            feature = "CoreLocation_CLLocation",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString"
        ))]
        #[method(requestHistoricalLocationsWithPurposeKey:sampleCount:completionHandler:)]
        pub unsafe fn requestHistoricalLocationsWithPurposeKey_sampleCount_completionHandler(
            &self,
            purpose_key: &NSString,
            sample_count: NSInteger,
            handler: &Block<(NonNull<NSArray<CLLocation>>, *mut NSError), ()>,
        );
    }
);
