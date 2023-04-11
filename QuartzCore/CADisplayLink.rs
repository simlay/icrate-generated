//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreAnimation::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreAnimation_CADisplayLink")]
    /**
      Class representing a timer bound to the display vsync.
    */
    pub struct CADisplayLink;

    #[cfg(feature = "CoreAnimation_CADisplayLink")]
    unsafe impl ClassType for CADisplayLink {
        type Super = NSObject;
    }
);

#[cfg(feature = "CoreAnimation_CADisplayLink")]
/**
  Class representing a timer bound to the display vsync.
*/
unsafe impl NSObjectProtocol for CADisplayLink {}

extern_methods!(
    /**
      Class representing a timer bound to the display vsync.
    */
    #[cfg(feature = "CoreAnimation_CADisplayLink")]
    unsafe impl CADisplayLink {
        #[method_id(@__retain_semantics Other displayLinkWithTarget:selector:)]
        pub unsafe fn displayLinkWithTarget_selector(
            target: &Object,
            sel: Sel,
        ) -> Id<CADisplayLink>;

        #[cfg(feature = "Foundation_NSRunLoop")]
        #[method(addToRunLoop:forMode:)]
        pub unsafe fn addToRunLoop_forMode(&self, runloop: &NSRunLoop, mode: &NSRunLoopMode);

        #[cfg(feature = "Foundation_NSRunLoop")]
        #[method(removeFromRunLoop:forMode:)]
        pub unsafe fn removeFromRunLoop_forMode(&self, runloop: &NSRunLoop, mode: &NSRunLoopMode);

        #[method(invalidate)]
        pub unsafe fn invalidate(&self);

        /**
          The current time, and duration of the display frame associated with
         the most recent target invocation. Time is represented using the
         normal Core Animation conventions, i.e. Mach host time converted to
         seconds.
        */
        #[method(timestamp)]
        pub unsafe fn timestamp(&self) -> CFTimeInterval;

        #[method(duration)]
        pub unsafe fn duration(&self) -> CFTimeInterval;

        /**
          The next timestamp that the client should target their render for.
        */
        #[method(targetTimestamp)]
        pub unsafe fn targetTimestamp(&self) -> CFTimeInterval;

        /**
          When true the object is prevented from firing. Initial state is
         false.
        */
        #[method(isPaused)]
        pub unsafe fn isPaused(&self) -> bool;

        /**
          When true the object is prevented from firing. Initial state is
         false.
        */
        #[method(setPaused:)]
        pub unsafe fn setPaused(&self, paused: bool);

        /**
          Defines how many display frames must pass between each time the
         display link fires. Default value is one, which means the display
         link will fire for every display frame. Setting the interval to two
         will cause the display link to fire every other display frame, and
         so on. The behavior when using values less than one is undefined.
         DEPRECATED - use preferredFramesPerSecond.
        */
        #[deprecated = "preferredFramesPerSecond"]
        #[method(frameInterval)]
        pub unsafe fn frameInterval(&self) -> NSInteger;

        /**
          Defines how many display frames must pass between each time the
         display link fires. Default value is one, which means the display
         link will fire for every display frame. Setting the interval to two
         will cause the display link to fire every other display frame, and
         so on. The behavior when using values less than one is undefined.
         DEPRECATED - use preferredFramesPerSecond.
        */
        #[deprecated = "preferredFramesPerSecond"]
        #[method(setFrameInterval:)]
        pub unsafe fn setFrameInterval(&self, frame_interval: NSInteger);

        /**
          Defines the desired callback rate in frames-per-second for this display
         link. If set to zero, the default value, the display link will fire at the
         native cadence of the display hardware. The display link will make a
         best-effort attempt at issuing callbacks at the requested rate.
        */
        #[deprecated]
        #[method(preferredFramesPerSecond)]
        pub unsafe fn preferredFramesPerSecond(&self) -> NSInteger;

        /**
          Defines the desired callback rate in frames-per-second for this display
         link. If set to zero, the default value, the display link will fire at the
         native cadence of the display hardware. The display link will make a
         best-effort attempt at issuing callbacks at the requested rate.
        */
        #[deprecated]
        #[method(setPreferredFramesPerSecond:)]
        pub unsafe fn setPreferredFramesPerSecond(&self, preferred_frames_per_second: NSInteger);

        /**
          Defines the range of desired callback rate in frames-per-second for this
        display link. If the range contains the same minimum and maximum frame rate,
        this property is identical as preferredFramesPerSecond. Otherwise, the actual
        callback rate will be dynamically adjusted to better align with other
        animation sources.
        */
        #[method(preferredFrameRateRange)]
        pub unsafe fn preferredFrameRateRange(&self) -> CAFrameRateRange;

        /**
          Defines the range of desired callback rate in frames-per-second for this
        display link. If the range contains the same minimum and maximum frame rate,
        this property is identical as preferredFramesPerSecond. Otherwise, the actual
        callback rate will be dynamically adjusted to better align with other
        animation sources.
        */
        #[method(setPreferredFrameRateRange:)]
        pub unsafe fn setPreferredFrameRateRange(
            &self,
            preferred_frame_rate_range: CAFrameRateRange,
        );
    }
);
