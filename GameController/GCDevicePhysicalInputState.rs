//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameController::*;

extern_protocol!(
    /**
     An object conforming to \c GCDevicePhysicalInputState contains the state of
    a device's physical inputs.  This may be either the "live" physical input
    state if the same object also conforms to \c GCDevicePhysicalInput, or a
    snapshot of the physical input state.
    */
    pub unsafe trait GCDevicePhysicalInputState: NSObjectProtocol {
        /**
         The device that this profile is mapping input from.
        */
        #[method_id(@__retain_semantics Other device)]
        unsafe fn device(&self) -> Option<Id<ProtocolObject<dyn GCDevice>>>;

        /**
         The internal time stamp of the last event.

        This time interval is not relative to any specific point in time.  Your
        application can subtract a previous timestamp from the returned timestamp to
        determine the time (in seconds) between events.  The \c lastEventTimestamp of
        the inputs from two different devices can be compared to determine which event
        occurred first.
        */
        #[method(lastEventTimestamp)]
        unsafe fn lastEventTimestamp(&self) -> NSTimeInterval;

        /**
         The interval (in seconds) between the timestamp of the last event and the
        current time.

        This value should be treated as a lower bound of the event latency.  It may
        not include (wired or wireless) transmission latency, or latency accrued on
        the device before the event was transmitted to the host.

        @note
        If the system has gone to sleep between when the event occurred and when this
        property is read, the returned value may not reflect the true latency.
        */
        #[method(lastEventLatency)]
        unsafe fn lastEventLatency(&self) -> NSTimeInterval;

        #[cfg(all(
            feature = "Foundation_NSString",
            feature = "GameController_GCPhysicalInputElementCollection"
        ))]
        /**
         The following properties allow for runtime lookup of any input element on a
        profile, when provided with a valid alias.

        @example input.elements[GCInputButtonA]
        @example input.dpads[GCInputLeftThumbstick]
        @example input.dpads[GCInputButtonB] // fails, "Button B" is not a DirectionPad
        */
        #[method_id(@__retain_semantics Other elements)]
        unsafe fn elements(
            &self,
        ) -> Id<
            GCPhysicalInputElementCollection<NSString, ProtocolObject<dyn GCPhysicalInputElement>>,
        >;

        #[cfg(all(
            feature = "Foundation_NSString",
            feature = "GameController_GCPhysicalInputElementCollection"
        ))]
        #[method_id(@__retain_semantics Other buttons)]
        unsafe fn buttons(
            &self,
        ) -> Id<GCPhysicalInputElementCollection<NSString, ProtocolObject<dyn GCButtonElement>>>;

        #[cfg(all(
            feature = "Foundation_NSString",
            feature = "GameController_GCPhysicalInputElementCollection"
        ))]
        #[method_id(@__retain_semantics Other axes)]
        unsafe fn axes(
            &self,
        ) -> Id<GCPhysicalInputElementCollection<NSString, ProtocolObject<dyn GCAxisElement>>>;

        #[cfg(all(
            feature = "Foundation_NSString",
            feature = "GameController_GCPhysicalInputElementCollection"
        ))]
        #[method_id(@__retain_semantics Other switches)]
        unsafe fn switches(
            &self,
        ) -> Id<GCPhysicalInputElementCollection<NSString, ProtocolObject<dyn GCSwitchElement>>>;

        #[cfg(all(
            feature = "Foundation_NSString",
            feature = "GameController_GCPhysicalInputElementCollection"
        ))]
        #[method_id(@__retain_semantics Other dpads)]
        unsafe fn dpads(
            &self,
        ) -> Id<GCPhysicalInputElementCollection<NSString, ProtocolObject<dyn GCDirectionPadElement>>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other objectForKeyedSubscript:)]
        unsafe fn objectForKeyedSubscript(
            &self,
            key: &NSString,
        ) -> Option<Id<ProtocolObject<dyn GCPhysicalInputElement>>>;
    }

    unsafe impl ProtocolType for dyn GCDevicePhysicalInputState {}
);
