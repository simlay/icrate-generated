//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreLocation::*;
use crate::EventKit::*;
use crate::Foundation::*;
use crate::MapKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "EventKit_EKAlarm")]
    /**
     @class          EKAlarm
    @abstract       The EKAlarm class provides an interface for accessing and manipulating calendar event alarms.
    @discussion     The EKAlarm class represents alarms on an event. An alarm can be relative (e.g. 15 mins before)
    or absolute (specific time).
    */
    pub struct EKAlarm;

    #[cfg(feature = "EventKit_EKAlarm")]
    unsafe impl ClassType for EKAlarm {
        #[inherits(NSObject)]
        type Super = EKObject;
    }
);

#[cfg(feature = "EventKit_EKAlarm")]
/**
 @class          EKAlarm
@abstract       The EKAlarm class provides an interface for accessing and manipulating calendar event alarms.
@discussion     The EKAlarm class represents alarms on an event. An alarm can be relative (e.g. 15 mins before)
or absolute (specific time).
*/
unsafe impl NSObjectProtocol for EKAlarm {}

extern_methods!(
    /**
     @class          EKAlarm
    @abstract       The EKAlarm class provides an interface for accessing and manipulating calendar event alarms.
    @discussion     The EKAlarm class represents alarms on an event. An alarm can be relative (e.g. 15 mins before)
    or absolute (specific time).
    */
    #[cfg(feature = "EventKit_EKAlarm")]
    unsafe impl EKAlarm {
        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other alarmWithAbsoluteDate:)]
        pub unsafe fn alarmWithAbsoluteDate(date: &NSDate) -> Id<EKAlarm>;

        #[method_id(@__retain_semantics Other alarmWithRelativeOffset:)]
        pub unsafe fn alarmWithRelativeOffset(offset: NSTimeInterval) -> Id<EKAlarm>;

        /**
         @property   relativeOffset
        @abstract   Specifies a relative offset from an event start date to fire an alarm.
        @discussion Set this property to an appropriate negative value to establish an alarm trigger
        relative to the start date/time of an event. Setting this clears any existing
        date trigger.
        */
        #[method(relativeOffset)]
        pub unsafe fn relativeOffset(&self) -> NSTimeInterval;

        /**
         @property   relativeOffset
        @abstract   Specifies a relative offset from an event start date to fire an alarm.
        @discussion Set this property to an appropriate negative value to establish an alarm trigger
        relative to the start date/time of an event. Setting this clears any existing
        date trigger.
        */
        #[method(setRelativeOffset:)]
        pub unsafe fn setRelativeOffset(&self, relative_offset: NSTimeInterval);

        #[cfg(feature = "Foundation_NSDate")]
        /**
         @property   absoluteDate
        @abstract   Represents an alarm that fires at a specific date.
        @discussion Set this property to a date to establish an absolute alarm trigger. Setting this
        clears any relative interval trigger.
        */
        #[method_id(@__retain_semantics Other absoluteDate)]
        pub unsafe fn absoluteDate(&self) -> Option<Id<NSDate>>;

        #[cfg(feature = "Foundation_NSDate")]
        /**
         @property   absoluteDate
        @abstract   Represents an alarm that fires at a specific date.
        @discussion Set this property to a date to establish an absolute alarm trigger. Setting this
        clears any relative interval trigger.
        */
        #[method(setAbsoluteDate:)]
        pub unsafe fn setAbsoluteDate(&self, absolute_date: Option<&NSDate>);

        #[cfg(feature = "EventKit_EKStructuredLocation")]
        /**
         @property   structuredLocation
        @discussion Allows you to set a structured location (a location with a potential geo-coordinate)
        on an alarm. This is used in conjunction with proximity to do geofence-based
        triggering of reminders.
        */
        #[method_id(@__retain_semantics Other structuredLocation)]
        pub unsafe fn structuredLocation(&self) -> Option<Id<EKStructuredLocation>>;

        #[cfg(feature = "EventKit_EKStructuredLocation")]
        /**
         @property   structuredLocation
        @discussion Allows you to set a structured location (a location with a potential geo-coordinate)
        on an alarm. This is used in conjunction with proximity to do geofence-based
        triggering of reminders.
        */
        #[method(setStructuredLocation:)]
        pub unsafe fn setStructuredLocation(
            &self,
            structured_location: Option<&EKStructuredLocation>,
        );

        /**
         @property   proximity
        @discussion Defines whether this alarm triggers via entering/exiting a geofence as defined by
        structuredLocation.
        */
        #[method(proximity)]
        pub unsafe fn proximity(&self) -> EKAlarmProximity;

        /**
         @property   proximity
        @discussion Defines whether this alarm triggers via entering/exiting a geofence as defined by
        structuredLocation.
        */
        #[method(setProximity:)]
        pub unsafe fn setProximity(&self, proximity: EKAlarmProximity);

        /**
         @property   type
        @abstract   The type of alarm, based on the action taken when triggering the alarm.
        @discussion This field is read-only; to change the type of alarm, set emailAddress for EKAlarmTypeEmail,
        soundName for EKAlarmTypeAudio or url for EKAlarmTypeProcedure.
        Setting all of those to nil will change it to EKAlarmTypeDisplay.
        */
        #[method(type)]
        pub unsafe fn r#type(&self) -> EKAlarmType;

        #[cfg(feature = "Foundation_NSString")]
        /**
         @property   emailAddress
        @abstract   An email address that is the recipient of an email alarm, which is an alarm that triggers an email message.
        @discussion When you set the emailAddress property, the action property is set to EKAlarmTypeEmail,
        and the soundName and url properties are set to nil.
        */
        #[method_id(@__retain_semantics Other emailAddress)]
        pub unsafe fn emailAddress(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        /**
         @property   emailAddress
        @abstract   An email address that is the recipient of an email alarm, which is an alarm that triggers an email message.
        @discussion When you set the emailAddress property, the action property is set to EKAlarmTypeEmail,
        and the soundName and url properties are set to nil.
        */
        #[method(setEmailAddress:)]
        pub unsafe fn setEmailAddress(&self, email_address: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        /**
         @property   soundName
        @abstract   The name of the sound to play when the alarm triggers.
        @discussion The value of this property is the name of a system sound that can be used with
        the soundNamed: class method to create an NSSound object. When you set the soundName property,
        the action property is set to EKAlarmTypeAudio, and the emailAddress and url properties are set to nil.
        */
        #[method_id(@__retain_semantics Other soundName)]
        pub unsafe fn soundName(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        /**
         @property   soundName
        @abstract   The name of the sound to play when the alarm triggers.
        @discussion The value of this property is the name of a system sound that can be used with
        the soundNamed: class method to create an NSSound object. When you set the soundName property,
        the action property is set to EKAlarmTypeAudio, and the emailAddress and url properties are set to nil.
        */
        #[method(setSoundName:)]
        pub unsafe fn setSoundName(&self, sound_name: Option<&NSString>);

        #[cfg(feature = "Foundation_NSURL")]
        /**
         @property   url
        @abstract   The URL to open when the alarm triggers.
        @discussion When you set the url property, the action property is set to EKAlarmTypeProcedure,
        and the emailAddress and soundName properties are set to nil.
        Note: Starting with OS X 10.9, it is not possible to create new procedure alarms or view URLs for existing procedure alarms.
        Trying to save or modify a procedure alarm will result in a save error.
        Editing other aspects of events or reminders that have existing procedure alarms is allowed as long as the alarm isn't modified.
        */
        #[deprecated]
        #[method_id(@__retain_semantics Other url)]
        pub unsafe fn url(&self) -> Option<Id<NSURL>>;

        #[cfg(feature = "Foundation_NSURL")]
        /**
         @property   url
        @abstract   The URL to open when the alarm triggers.
        @discussion When you set the url property, the action property is set to EKAlarmTypeProcedure,
        and the emailAddress and soundName properties are set to nil.
        Note: Starting with OS X 10.9, it is not possible to create new procedure alarms or view URLs for existing procedure alarms.
        Trying to save or modify a procedure alarm will result in a save error.
        Editing other aspects of events or reminders that have existing procedure alarms is allowed as long as the alarm isn't modified.
        */
        #[deprecated]
        #[method(setUrl:)]
        pub unsafe fn setUrl(&self, url: Option<&NSURL>);
    }
);
