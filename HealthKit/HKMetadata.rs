//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::HealthKit::*;
use crate::UniformTypeIdentifiers::*;

extern_static!(HKMetadataKeyDeviceSerialNumber: &'static NSString);

extern_static!(HKMetadataKeyBodyTemperatureSensorLocation: &'static NSString);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum HKBodyTemperatureSensorLocation {
        HKBodyTemperatureSensorLocationOther = 0,
        HKBodyTemperatureSensorLocationArmpit = 1,
        HKBodyTemperatureSensorLocationBody = 2,
        HKBodyTemperatureSensorLocationEar = 3,
        HKBodyTemperatureSensorLocationFinger = 4,
        HKBodyTemperatureSensorLocationGastroIntestinal = 5,
        HKBodyTemperatureSensorLocationMouth = 6,
        HKBodyTemperatureSensorLocationRectum = 7,
        HKBodyTemperatureSensorLocationToe = 8,
        HKBodyTemperatureSensorLocationEarDrum = 9,
        HKBodyTemperatureSensorLocationTemporalArtery = 10,
        HKBodyTemperatureSensorLocationForehead = 11,
    }
);

extern_static!(HKMetadataKeyHeartRateSensorLocation: &'static NSString);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum HKHeartRateSensorLocation {
        HKHeartRateSensorLocationOther = 0,
        HKHeartRateSensorLocationChest = 1,
        HKHeartRateSensorLocationWrist = 2,
        HKHeartRateSensorLocationFinger = 3,
        HKHeartRateSensorLocationHand = 4,
        HKHeartRateSensorLocationEarLobe = 5,
        HKHeartRateSensorLocationFoot = 6,
    }
);

extern_static!(HKMetadataKeyHeartRateMotionContext: &'static NSString);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum HKHeartRateMotionContext {
        HKHeartRateMotionContextNotSet = 0,
        HKHeartRateMotionContextSedentary = 1,
        HKHeartRateMotionContextActive = 2,
    }
);

extern_static!(HKMetadataKeyUserMotionContext: &'static NSString);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum HKUserMotionContext {
        HKUserMotionContextNotSet = 0,
        HKUserMotionContextStationary = 1,
        HKUserMotionContextActive = 2,
    }
);

extern_static!(HKMetadataKeySessionEstimate: &'static NSString);

ns_enum!(
    #[underlying(NSInteger)]
    /**
     @enum          HKHeartRateRecoveryTestType
    @abstract      Represents the test used to determine a Heart Rate Recovery value

    @constant      HKHeartRateRecoveryTestTypeMaxExercise                  The user was exerted to their physical limit to evaluate actual Heart Rate Recovery.
    @constant      HKHeartRateRecoveryTestTypePredictionSubMaxExercise     A specific test protocol was used to calculate and correlate a predicted Heart Rate Recovery.
    @constant      HKHeartRateRecoveryTestTypePredictionNonExercise        A non-exercise equation was used based on user metrics to calculate a predicted Heart Rate Recovery.
    */
    pub enum HKHeartRateRecoveryTestType {
        HKHeartRateRecoveryTestTypeMaxExercise = 1,
        HKHeartRateRecoveryTestTypePredictionSubMaxExercise = 2,
        HKHeartRateRecoveryTestTypePredictionNonExercise = 3,
    }
);

extern_static!(HKMetadataKeyHeartRateRecoveryTestType: &'static NSString);

extern_static!(HKMetadataKeyHeartRateRecoveryActivityType: &'static NSString);

extern_static!(HKMetadataKeyHeartRateRecoveryActivityDuration: &'static NSString);

extern_static!(HKMetadataKeyHeartRateRecoveryMaxObservedRecoveryHeartRate: &'static NSString);

extern_static!(HKMetadataKeyFoodType: &'static NSString);

extern_static!(HKMetadataKeyUDIDeviceIdentifier: &'static NSString);

extern_static!(HKMetadataKeyUDIProductionIdentifier: &'static NSString);

extern_static!(HKMetadataKeyDigitalSignature: &'static NSString);

extern_static!(HKMetadataKeyExternalUUID: &'static NSString);

extern_static!(HKMetadataKeySyncIdentifier: &'static NSString);

extern_static!(HKMetadataKeySyncVersion: &'static NSString);

extern_static!(HKMetadataKeyTimeZone: &'static NSString);

extern_static!(HKMetadataKeyDeviceName: &'static NSString);

extern_static!(HKMetadataKeyDeviceManufacturerName: &'static NSString);

extern_static!(HKMetadataKeyWasTakenInLab: &'static NSString);

extern_static!(HKMetadataKeyReferenceRangeLowerLimit: &'static NSString);

extern_static!(HKMetadataKeyReferenceRangeUpperLimit: &'static NSString);

extern_static!(HKMetadataKeyWasUserEntered: &'static NSString);

extern_static!(HKMetadataKeyWorkoutBrandName: &'static NSString);

extern_static!(HKMetadataKeyGroupFitness: &'static NSString);

extern_static!(HKMetadataKeyIndoorWorkout: &'static NSString);

extern_static!(HKMetadataKeyCoachedWorkout: &'static NSString);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum HKWeatherCondition {
        HKWeatherConditionNone = 0,
        HKWeatherConditionClear = 1,
        HKWeatherConditionFair = 2,
        HKWeatherConditionPartlyCloudy = 3,
        HKWeatherConditionMostlyCloudy = 4,
        HKWeatherConditionCloudy = 5,
        HKWeatherConditionFoggy = 6,
        HKWeatherConditionHaze = 7,
        HKWeatherConditionWindy = 8,
        HKWeatherConditionBlustery = 9,
        HKWeatherConditionSmoky = 10,
        HKWeatherConditionDust = 11,
        HKWeatherConditionSnow = 12,
        HKWeatherConditionHail = 13,
        HKWeatherConditionSleet = 14,
        HKWeatherConditionFreezingDrizzle = 15,
        HKWeatherConditionFreezingRain = 16,
        HKWeatherConditionMixedRainAndHail = 17,
        HKWeatherConditionMixedRainAndSnow = 18,
        HKWeatherConditionMixedRainAndSleet = 19,
        HKWeatherConditionMixedSnowAndSleet = 20,
        HKWeatherConditionDrizzle = 21,
        HKWeatherConditionScatteredShowers = 22,
        HKWeatherConditionShowers = 23,
        HKWeatherConditionThunderstorms = 24,
        HKWeatherConditionTropicalStorm = 25,
        HKWeatherConditionHurricane = 26,
        HKWeatherConditionTornado = 27,
    }
);

extern_static!(HKMetadataKeyWeatherCondition: &'static NSString);

extern_static!(HKMetadataKeyWeatherTemperature: &'static NSString);

extern_static!(HKMetadataKeyWeatherHumidity: &'static NSString);

extern_static!(HKMetadataKeySexualActivityProtectionUsed: &'static NSString);

extern_static!(HKMetadataKeyMenstrualCycleStart: &'static NSString);

extern_static!(HKMetadataKeyLapLength: &'static NSString);

ns_enum!(
    #[underlying(NSInteger)]
    /**
     @enum          HKWorkoutSwimmingLocationType
    @abstract      This enumerated type is used to represent the location type of a swimming workout.
    @discussion    This value indicates whether a swimming workout was performed in a pool or open water.
    */
    pub enum HKWorkoutSwimmingLocationType {
        HKWorkoutSwimmingLocationTypeUnknown = 0,
        HKWorkoutSwimmingLocationTypePool = 1,
        HKWorkoutSwimmingLocationTypeOpenWater = 2,
    }
);

extern_static!(HKMetadataKeySwimmingLocationType: &'static NSString);

ns_enum!(
    #[underlying(NSInteger)]
    /**
     @enum          HKSwimmingStrokeStyle
    @abstract      Represents a style of stroke used during a swimming workout.
    */
    pub enum HKSwimmingStrokeStyle {
        HKSwimmingStrokeStyleUnknown = 0,
        HKSwimmingStrokeStyleMixed = 1,
        HKSwimmingStrokeStyleFreestyle = 2,
        HKSwimmingStrokeStyleBackstroke = 3,
        HKSwimmingStrokeStyleBreaststroke = 4,
        HKSwimmingStrokeStyleButterfly = 5,
        HKSwimmingStrokeStyleKickboard = 6,
    }
);

extern_static!(HKMetadataKeySwimmingStrokeStyle: &'static NSString);

ns_enum!(
    #[underlying(NSInteger)]
    /**
     @enum          HKInsulinDeliveryReason
    @abstract      Represents a medical reason for the delivery of insulin

    @constant      HKInsulinDeliveryReasonBasal  Delivery for the base metabolic needs of the individual, often
    administered as a continuous rate from an insulin pump, or a periodic
    injection of slow-acting insulin.
    @constant      HKInsulinDeliveryReasonBolus  Delivery for the episodic needs of the individual, such as a meal or
    glucose level correction.
    */
    pub enum HKInsulinDeliveryReason {
        HKInsulinDeliveryReasonBasal = 1,
        HKInsulinDeliveryReasonBolus = 2,
    }
);

extern_static!(HKMetadataKeyInsulinDeliveryReason: &'static NSString);

ns_enum!(
    #[underlying(NSInteger)]
    /**
     @enum          HKBloodGlucoseMealTime
    @abstract      Indicates how your blood glucose reading relates to a meal.

    @constant      HKBloodGlucoseMealTimePreprandial   A glucose value measured at the time just before a meal.
    @constant      HKBloodGlucoseMealTimePostprandial  A glucose value measured after a meal.
    */
    pub enum HKBloodGlucoseMealTime {
        HKBloodGlucoseMealTimePreprandial = 1,
        HKBloodGlucoseMealTimePostprandial = 2,
    }
);

extern_static!(HKMetadataKeyBloodGlucoseMealTime: &'static NSString);

ns_enum!(
    #[underlying(NSInteger)]
    /**
     @enum          HKVO2MaxTestType
    @abstract      Represents the test used to create a VO2 Max Sample.

    @constant      HKVO2MaxTestTypeMaxExercise                      The user was exerted to their physical limit to evaluate and measure actual VO2Max.
    @constant      HKVO2MaxTestTypePredictionSubMaxExercise         A specific test protocol was used to calculate and correlate a predicted VO2Max.
    @constant      HKVO2MaxTestTypePredictionNonExercise            A non-exercise equation was used based on user metrics to calculate a predicted VO2Max.
    */
    pub enum HKVO2MaxTestType {
        HKVO2MaxTestTypeMaxExercise = 1,
        HKVO2MaxTestTypePredictionSubMaxExercise = 2,
        HKVO2MaxTestTypePredictionNonExercise = 3,
    }
);

extern_static!(HKMetadataKeyVO2MaxTestType: &'static NSString);

extern_static!(HKMetadataKeyAverageSpeed: &'static NSString);

extern_static!(HKMetadataKeyMaximumSpeed: &'static NSString);

extern_static!(HKMetadataKeyAlpineSlopeGrade: &'static NSString);

extern_static!(HKMetadataKeyElevationAscended: &'static NSString);

extern_static!(HKMetadataKeyElevationDescended: &'static NSString);

extern_static!(HKMetadataKeyFitnessMachineDuration: &'static NSString);

extern_static!(HKMetadataKeyIndoorBikeDistance: &'static NSString);

extern_static!(HKMetadataKeyCrossTrainerDistance: &'static NSString);

extern_static!(HKMetadataKeyHeartRateEventThreshold: &'static NSString);

extern_static!(HKMetadataKeyAverageMETs: &'static NSString);

extern_static!(HKMetadataKeyAudioExposureLevel: &'static NSString);

extern_static!(HKMetadataKeyAudioExposureDuration: &'static NSString);

ns_enum!(
    #[underlying(NSInteger)]
    /**
     @enum          HKAppleECGAlgorithmVersion
    @abstract      Indicates which algorithm version number was used by the ECG app on Apple Watch.

    @constant      HKAppleECGAlgorithmVersion1   Apple Watch used a version 1 algorithm to generate this ECG.
    */
    pub enum HKAppleECGAlgorithmVersion {
        HKAppleECGAlgorithmVersion1 = 1,
        HKAppleECGAlgorithmVersion2 = 2,
    }
);

extern_static!(HKMetadataKeyAppleECGAlgorithmVersion: &'static NSString);

ns_enum!(
    #[underlying(NSInteger)]
    /**
     @enum          HKDevicePlacementSide
    @abstract      The detected placement of the device during the bout of walking
    @constant      HKDevicePlacementSideUnknown                     Unable to determine the placement of the device
    @constant      HKDevicePlacementSideLeft                        Device predominantly worn on left side
    @constant      HKDevicePlacementSideRight                       Device predominantly worn on right side
    @constant      HKDevicePlacementSideCentral                     Device predominantly worn on the middle of the body
    */
    pub enum HKDevicePlacementSide {
        HKDevicePlacementSideUnknown = 0,
        HKDevicePlacementSideLeft = 1,
        HKDevicePlacementSideRight = 2,
        HKDevicePlacementSideCentral = 3,
    }
);

extern_static!(HKMetadataKeyDevicePlacementSide: &'static NSString);

extern_static!(HKMetadataKeyBarometricPressure: &'static NSString);

extern_static!(HKMetadataKeyAppleDeviceCalibrated: &'static NSString);

extern_static!(HKMetadataKeyVO2MaxValue: &'static NSString);

extern_static!(HKMetadataKeyLowCardioFitnessEventThreshold: &'static NSString);

extern_static!(HKMetadataKeyDateOfEarliestDataUsedForEstimate: &'static NSString);

extern_static!(HKMetadataKeyAlgorithmVersion: &'static NSString);

extern_static!(HKMetadataKeySWOLFScore: &'static NSString);

extern_static!(HKMetadataKeyQuantityClampedToLowerBound: &'static NSString);

extern_static!(HKMetadataKeyQuantityClampedToUpperBound: &'static NSString);

extern_static!(HKMetadataKeyGlassesPrescriptionDescription: &'static NSString);
