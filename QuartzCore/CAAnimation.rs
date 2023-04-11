//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreAnimation::*;
use crate::Foundation::*;

typed_enum!(
    pub type CAAnimationCalculationMode = NSString;
);

typed_enum!(
    pub type CAAnimationRotationMode = NSString;
);

typed_enum!(
    pub type CATransitionType = NSString;
);

typed_enum!(
    pub type CATransitionSubtype = NSString;
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreAnimation_CAAnimation")]
    /**
      The base animation class.
    */
    pub struct CAAnimation;

    #[cfg(feature = "CoreAnimation_CAAnimation")]
    unsafe impl ClassType for CAAnimation {
        type Super = NSObject;
    }
);

#[cfg(feature = "CoreAnimation_CAAnimation")]
/**
  The base animation class.
*/
unsafe impl CAAction for CAAnimation {}

#[cfg(feature = "CoreAnimation_CAAnimation")]
/**
  The base animation class.
*/
unsafe impl CAMediaTiming for CAAnimation {}

#[cfg(feature = "CoreAnimation_CAAnimation")]
/**
  The base animation class.
*/
unsafe impl NSCoding for CAAnimation {}

#[cfg(feature = "CoreAnimation_CAAnimation")]
/**
  The base animation class.
*/
unsafe impl NSObjectProtocol for CAAnimation {}

#[cfg(feature = "CoreAnimation_CAAnimation")]
/**
  The base animation class.
*/
unsafe impl NSSecureCoding for CAAnimation {}

extern_methods!(
    /**
      The base animation class.
    */
    #[cfg(feature = "CoreAnimation_CAAnimation")]
    unsafe impl CAAnimation {
        #[method_id(@__retain_semantics Other animation)]
        pub unsafe fn animation() -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other defaultValueForKey:)]
        pub unsafe fn defaultValueForKey(key: &NSString) -> Option<Id<Object>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(shouldArchiveValueForKey:)]
        pub unsafe fn shouldArchiveValueForKey(&self, key: &NSString) -> bool;

        #[cfg(feature = "CoreAnimation_CAMediaTimingFunction")]
        /**
          A timing function defining the pacing of the animation. Defaults to
         nil indicating linear pacing.
        */
        #[method_id(@__retain_semantics Other timingFunction)]
        pub unsafe fn timingFunction(&self) -> Option<Id<CAMediaTimingFunction>>;

        #[cfg(feature = "CoreAnimation_CAMediaTimingFunction")]
        /**
          A timing function defining the pacing of the animation. Defaults to
         nil indicating linear pacing.
        */
        #[method(setTimingFunction:)]
        pub unsafe fn setTimingFunction(&self, timing_function: Option<&CAMediaTimingFunction>);

        /**
          The delegate of the animation. This object is retained for the
         lifetime of the animation object. Defaults to nil. See below for the
         supported delegate methods.
        */
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<ProtocolObject<dyn CAAnimationDelegate>>>;

        /**
          The delegate of the animation. This object is retained for the
         lifetime of the animation object. Defaults to nil. See below for the
         supported delegate methods.
        */
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn CAAnimationDelegate>>,
        );

        /**
          When true, the animation is removed from the render tree once its
         active duration has passed. Defaults to YES.
        */
        #[method(isRemovedOnCompletion)]
        pub unsafe fn isRemovedOnCompletion(&self) -> bool;

        /**
          When true, the animation is removed from the render tree once its
         active duration has passed. Defaults to YES.
        */
        #[method(setRemovedOnCompletion:)]
        pub unsafe fn setRemovedOnCompletion(&self, removed_on_completion: bool);

        /**
          Defines the range of desired frame rate in frames-per-second for this
        animation. The actual frame rate is dynamically adjusted to better align
        with other animation sources.
        */
        #[method(preferredFrameRateRange)]
        pub unsafe fn preferredFrameRateRange(&self) -> CAFrameRateRange;

        /**
          Defines the range of desired frame rate in frames-per-second for this
        animation. The actual frame rate is dynamically adjusted to better align
        with other animation sources.
        */
        #[method(setPreferredFrameRateRange:)]
        pub unsafe fn setPreferredFrameRateRange(
            &self,
            preferred_frame_rate_range: CAFrameRateRange,
        );
    }
);

extern_protocol!(
    /**
      Delegate methods for CAAnimation.
    */
    pub unsafe trait CAAnimationDelegate: NSObjectProtocol {
        #[cfg(feature = "CoreAnimation_CAAnimation")]
        #[optional]
        #[method(animationDidStart:)]
        unsafe fn animationDidStart(&self, anim: &CAAnimation);

        #[cfg(feature = "CoreAnimation_CAAnimation")]
        #[optional]
        #[method(animationDidStop:finished:)]
        unsafe fn animationDidStop_finished(&self, anim: &CAAnimation, flag: bool);
    }

    unsafe impl ProtocolType for dyn CAAnimationDelegate {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreAnimation_CAPropertyAnimation")]
    /**
      Subclass for property-based animations.
    */
    pub struct CAPropertyAnimation;

    #[cfg(feature = "CoreAnimation_CAPropertyAnimation")]
    unsafe impl ClassType for CAPropertyAnimation {
        #[inherits(NSObject)]
        type Super = CAAnimation;
    }
);

#[cfg(feature = "CoreAnimation_CAPropertyAnimation")]
/**
  Subclass for property-based animations.
*/
unsafe impl CAAction for CAPropertyAnimation {}

#[cfg(feature = "CoreAnimation_CAPropertyAnimation")]
/**
  Subclass for property-based animations.
*/
unsafe impl CAMediaTiming for CAPropertyAnimation {}

#[cfg(feature = "CoreAnimation_CAPropertyAnimation")]
/**
  Subclass for property-based animations.
*/
unsafe impl NSCoding for CAPropertyAnimation {}

#[cfg(feature = "CoreAnimation_CAPropertyAnimation")]
/**
  Subclass for property-based animations.
*/
unsafe impl NSObjectProtocol for CAPropertyAnimation {}

#[cfg(feature = "CoreAnimation_CAPropertyAnimation")]
/**
  Subclass for property-based animations.
*/
unsafe impl NSSecureCoding for CAPropertyAnimation {}

extern_methods!(
    /**
      Subclass for property-based animations.
    */
    #[cfg(feature = "CoreAnimation_CAPropertyAnimation")]
    unsafe impl CAPropertyAnimation {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other animationWithKeyPath:)]
        pub unsafe fn animationWithKeyPath(path: Option<&NSString>) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        /**
          The key-path describing the property to be animated.
        */
        #[method_id(@__retain_semantics Other keyPath)]
        pub unsafe fn keyPath(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        /**
          The key-path describing the property to be animated.
        */
        #[method(setKeyPath:)]
        pub unsafe fn setKeyPath(&self, key_path: Option<&NSString>);

        /**
          When true the value specified by the animation will be "added" to
         the current presentation value of the property to produce the new
         presentation value. The addition function is type-dependent, e.g.
         for affine transforms the two matrices are concatenated. Defaults to
         NO.
        */
        #[method(isAdditive)]
        pub unsafe fn isAdditive(&self) -> bool;

        /**
          When true the value specified by the animation will be "added" to
         the current presentation value of the property to produce the new
         presentation value. The addition function is type-dependent, e.g.
         for affine transforms the two matrices are concatenated. Defaults to
         NO.
        */
        #[method(setAdditive:)]
        pub unsafe fn setAdditive(&self, additive: bool);

        /**
          The `cumulative' property affects how repeating animations produce
         their result. If true then the current value of the animation is the
         value at the end of the previous repeat cycle, plus the value of the
         current repeat cycle. If false, the value is simply the value
         calculated for the current repeat cycle. Defaults to NO.
        */
        #[method(isCumulative)]
        pub unsafe fn isCumulative(&self) -> bool;

        /**
          The `cumulative' property affects how repeating animations produce
         their result. If true then the current value of the animation is the
         value at the end of the previous repeat cycle, plus the value of the
         current repeat cycle. If false, the value is simply the value
         calculated for the current repeat cycle. Defaults to NO.
        */
        #[method(setCumulative:)]
        pub unsafe fn setCumulative(&self, cumulative: bool);

        #[cfg(feature = "CoreAnimation_CAValueFunction")]
        /**
          If non-nil a function that is applied to interpolated values
         before they are set as the new presentation value of the animation's
         target property. Defaults to nil.
        */
        #[method_id(@__retain_semantics Other valueFunction)]
        pub unsafe fn valueFunction(&self) -> Option<Id<CAValueFunction>>;

        #[cfg(feature = "CoreAnimation_CAValueFunction")]
        /**
          If non-nil a function that is applied to interpolated values
         before they are set as the new presentation value of the animation's
         target property. Defaults to nil.
        */
        #[method(setValueFunction:)]
        pub unsafe fn setValueFunction(&self, value_function: Option<&CAValueFunction>);
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreAnimation_CABasicAnimation")]
    /**
      Subclass for basic (single-keyframe) animations.
    */
    pub struct CABasicAnimation;

    #[cfg(feature = "CoreAnimation_CABasicAnimation")]
    unsafe impl ClassType for CABasicAnimation {
        #[inherits(CAAnimation, NSObject)]
        type Super = CAPropertyAnimation;
    }
);

#[cfg(feature = "CoreAnimation_CABasicAnimation")]
/**
  Subclass for basic (single-keyframe) animations.
*/
unsafe impl CAAction for CABasicAnimation {}

#[cfg(feature = "CoreAnimation_CABasicAnimation")]
/**
  Subclass for basic (single-keyframe) animations.
*/
unsafe impl CAMediaTiming for CABasicAnimation {}

#[cfg(feature = "CoreAnimation_CABasicAnimation")]
/**
  Subclass for basic (single-keyframe) animations.
*/
unsafe impl NSCoding for CABasicAnimation {}

#[cfg(feature = "CoreAnimation_CABasicAnimation")]
/**
  Subclass for basic (single-keyframe) animations.
*/
unsafe impl NSObjectProtocol for CABasicAnimation {}

#[cfg(feature = "CoreAnimation_CABasicAnimation")]
/**
  Subclass for basic (single-keyframe) animations.
*/
unsafe impl NSSecureCoding for CABasicAnimation {}

extern_methods!(
    /**
      Subclass for basic (single-keyframe) animations.
    */
    #[cfg(feature = "CoreAnimation_CABasicAnimation")]
    unsafe impl CABasicAnimation {
        /**
          The objects defining the property values being interpolated between.
         All are optional, and no more than two should be non-nil. The object
         type should match the type of the property being animated (using the
         standard rules described in CALayer.h). The supported modes of
         animation are:

         - both `fromValue' and `toValue' non-nil. Interpolates between
         `fromValue' and `toValue'.

         - `fromValue' and `byValue' non-nil. Interpolates between
         `fromValue' and `fromValue' plus `byValue'.

         - `byValue' and `toValue' non-nil. Interpolates between `toValue'
         minus `byValue' and `toValue'.

         - `fromValue' non-nil. Interpolates between `fromValue' and the
         current presentation value of the property.

         - `toValue' non-nil. Interpolates between the layer's current value
         of the property in the render tree and `toValue'.

         - `byValue' non-nil. Interpolates between the layer's current value
         of the property in the render tree and that plus `byValue'.
        */
        #[method_id(@__retain_semantics Other fromValue)]
        pub unsafe fn fromValue(&self) -> Option<Id<Object>>;

        /**
          The objects defining the property values being interpolated between.
         All are optional, and no more than two should be non-nil. The object
         type should match the type of the property being animated (using the
         standard rules described in CALayer.h). The supported modes of
         animation are:

         - both `fromValue' and `toValue' non-nil. Interpolates between
         `fromValue' and `toValue'.

         - `fromValue' and `byValue' non-nil. Interpolates between
         `fromValue' and `fromValue' plus `byValue'.

         - `byValue' and `toValue' non-nil. Interpolates between `toValue'
         minus `byValue' and `toValue'.

         - `fromValue' non-nil. Interpolates between `fromValue' and the
         current presentation value of the property.

         - `toValue' non-nil. Interpolates between the layer's current value
         of the property in the render tree and `toValue'.

         - `byValue' non-nil. Interpolates between the layer's current value
         of the property in the render tree and that plus `byValue'.
        */
        #[method(setFromValue:)]
        pub unsafe fn setFromValue(&self, from_value: Option<&Object>);

        #[method_id(@__retain_semantics Other toValue)]
        pub unsafe fn toValue(&self) -> Option<Id<Object>>;

        #[method(setToValue:)]
        pub unsafe fn setToValue(&self, to_value: Option<&Object>);

        #[method_id(@__retain_semantics Other byValue)]
        pub unsafe fn byValue(&self) -> Option<Id<Object>>;

        #[method(setByValue:)]
        pub unsafe fn setByValue(&self, by_value: Option<&Object>);
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreAnimation_CAKeyframeAnimation")]
    /**
      General keyframe animation class.
    */
    pub struct CAKeyframeAnimation;

    #[cfg(feature = "CoreAnimation_CAKeyframeAnimation")]
    unsafe impl ClassType for CAKeyframeAnimation {
        #[inherits(CAAnimation, NSObject)]
        type Super = CAPropertyAnimation;
    }
);

#[cfg(feature = "CoreAnimation_CAKeyframeAnimation")]
/**
  General keyframe animation class.
*/
unsafe impl CAAction for CAKeyframeAnimation {}

#[cfg(feature = "CoreAnimation_CAKeyframeAnimation")]
/**
  General keyframe animation class.
*/
unsafe impl CAMediaTiming for CAKeyframeAnimation {}

#[cfg(feature = "CoreAnimation_CAKeyframeAnimation")]
/**
  General keyframe animation class.
*/
unsafe impl NSCoding for CAKeyframeAnimation {}

#[cfg(feature = "CoreAnimation_CAKeyframeAnimation")]
/**
  General keyframe animation class.
*/
unsafe impl NSObjectProtocol for CAKeyframeAnimation {}

#[cfg(feature = "CoreAnimation_CAKeyframeAnimation")]
/**
  General keyframe animation class.
*/
unsafe impl NSSecureCoding for CAKeyframeAnimation {}

extern_methods!(
    /**
      General keyframe animation class.
    */
    #[cfg(feature = "CoreAnimation_CAKeyframeAnimation")]
    unsafe impl CAKeyframeAnimation {
        #[cfg(feature = "Foundation_NSArray")]
        /**
          An array of objects providing the value of the animation function for
         each keyframe.
        */
        #[method_id(@__retain_semantics Other values)]
        pub unsafe fn values(&self) -> Option<Id<NSArray>>;

        #[cfg(feature = "Foundation_NSArray")]
        /**
          An array of objects providing the value of the animation function for
         each keyframe.
        */
        #[method(setValues:)]
        pub unsafe fn setValues(&self, values: Option<&NSArray>);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSNumber"))]
        /**
          An optional array of `NSNumber' objects defining the pacing of the
         animation. Each time corresponds to one value in the `values' array,
         and defines when the value should be used in the animation function.
         Each value in the array is a floating point number in the range
         [0,1].
        */
        #[method_id(@__retain_semantics Other keyTimes)]
        pub unsafe fn keyTimes(&self) -> Option<Id<NSArray<NSNumber>>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSNumber"))]
        /**
          An optional array of `NSNumber' objects defining the pacing of the
         animation. Each time corresponds to one value in the `values' array,
         and defines when the value should be used in the animation function.
         Each value in the array is a floating point number in the range
         [0,1].
        */
        #[method(setKeyTimes:)]
        pub unsafe fn setKeyTimes(&self, key_times: Option<&NSArray<NSNumber>>);

        #[cfg(all(
            feature = "CoreAnimation_CAMediaTimingFunction",
            feature = "Foundation_NSArray"
        ))]
        /**
          An optional array of CAMediaTimingFunction objects. If the `values' array
         defines n keyframes, there should be n-1 objects in the
         `timingFunctions' array. Each function describes the pacing of one
         keyframe to keyframe segment.
        */
        #[method_id(@__retain_semantics Other timingFunctions)]
        pub unsafe fn timingFunctions(&self) -> Option<Id<NSArray<CAMediaTimingFunction>>>;

        #[cfg(all(
            feature = "CoreAnimation_CAMediaTimingFunction",
            feature = "Foundation_NSArray"
        ))]
        /**
          An optional array of CAMediaTimingFunction objects. If the `values' array
         defines n keyframes, there should be n-1 objects in the
         `timingFunctions' array. Each function describes the pacing of one
         keyframe to keyframe segment.
        */
        #[method(setTimingFunctions:)]
        pub unsafe fn setTimingFunctions(
            &self,
            timing_functions: Option<&NSArray<CAMediaTimingFunction>>,
        );

        /**
          The "calculation mode". Possible values are `discrete', `linear',
         `paced', `cubic' and `cubicPaced'. Defaults to `linear'. When set to
         `paced' or `cubicPaced' the `keyTimes' and `timingFunctions'
         properties of the animation are ignored and calculated implicitly.
        */
        #[method_id(@__retain_semantics Other calculationMode)]
        pub unsafe fn calculationMode(&self) -> Id<CAAnimationCalculationMode>;

        /**
          The "calculation mode". Possible values are `discrete', `linear',
         `paced', `cubic' and `cubicPaced'. Defaults to `linear'. When set to
         `paced' or `cubicPaced' the `keyTimes' and `timingFunctions'
         properties of the animation are ignored and calculated implicitly.
        */
        #[method(setCalculationMode:)]
        pub unsafe fn setCalculationMode(&self, calculation_mode: &CAAnimationCalculationMode);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSNumber"))]
        /**
          For animations with the cubic calculation modes, these properties
         provide control over the interpolation scheme. Each keyframe may
         have a tension, continuity and bias value associated with it, each
         in the range [-1, 1] (this defines a Kochanek-Bartels spline, see
         http://en.wikipedia.org/wiki/Kochanek-Bartels_spline).

         The tension value controls the "tightness" of the curve (positive
         values are tighter, negative values are rounder). The continuity
         value controls how segments are joined (positive values give sharp
         corners, negative values give inverted corners). The bias value
         defines where the curve occurs (positive values move the curve before
         the control point, negative values move it after the control point).

         The first value in each array defines the behavior of the tangent to
         the first control point, the second value controls the second
         point's tangents, and so on. Any unspecified values default to zero
         (giving a Catmull-Rom spline if all are unspecified).
        */
        #[method_id(@__retain_semantics Other tensionValues)]
        pub unsafe fn tensionValues(&self) -> Option<Id<NSArray<NSNumber>>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSNumber"))]
        /**
          For animations with the cubic calculation modes, these properties
         provide control over the interpolation scheme. Each keyframe may
         have a tension, continuity and bias value associated with it, each
         in the range [-1, 1] (this defines a Kochanek-Bartels spline, see
         http://en.wikipedia.org/wiki/Kochanek-Bartels_spline).

         The tension value controls the "tightness" of the curve (positive
         values are tighter, negative values are rounder). The continuity
         value controls how segments are joined (positive values give sharp
         corners, negative values give inverted corners). The bias value
         defines where the curve occurs (positive values move the curve before
         the control point, negative values move it after the control point).

         The first value in each array defines the behavior of the tangent to
         the first control point, the second value controls the second
         point's tangents, and so on. Any unspecified values default to zero
         (giving a Catmull-Rom spline if all are unspecified).
        */
        #[method(setTensionValues:)]
        pub unsafe fn setTensionValues(&self, tension_values: Option<&NSArray<NSNumber>>);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSNumber"))]
        #[method_id(@__retain_semantics Other continuityValues)]
        pub unsafe fn continuityValues(&self) -> Option<Id<NSArray<NSNumber>>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSNumber"))]
        #[method(setContinuityValues:)]
        pub unsafe fn setContinuityValues(&self, continuity_values: Option<&NSArray<NSNumber>>);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSNumber"))]
        #[method_id(@__retain_semantics Other biasValues)]
        pub unsafe fn biasValues(&self) -> Option<Id<NSArray<NSNumber>>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSNumber"))]
        #[method(setBiasValues:)]
        pub unsafe fn setBiasValues(&self, bias_values: Option<&NSArray<NSNumber>>);

        /**
          Defines whether or objects animating along paths rotate to match the
         path tangent. Possible values are `auto' and `autoReverse'. Defaults
         to nil. The effect of setting this property to a non-nil value when
         no path object is supplied is undefined. `autoReverse' rotates to
         match the tangent plus 180 degrees.
        */
        #[method_id(@__retain_semantics Other rotationMode)]
        pub unsafe fn rotationMode(&self) -> Option<Id<CAAnimationRotationMode>>;

        /**
          Defines whether or objects animating along paths rotate to match the
         path tangent. Possible values are `auto' and `autoReverse'. Defaults
         to nil. The effect of setting this property to a non-nil value when
         no path object is supplied is undefined. `autoReverse' rotates to
         match the tangent plus 180 degrees.
        */
        #[method(setRotationMode:)]
        pub unsafe fn setRotationMode(&self, rotation_mode: Option<&CAAnimationRotationMode>);
    }
);

extern_static!(kCAAnimationLinear: &'static CAAnimationCalculationMode);

extern_static!(kCAAnimationDiscrete: &'static CAAnimationCalculationMode);

extern_static!(kCAAnimationPaced: &'static CAAnimationCalculationMode);

extern_static!(kCAAnimationCubic: &'static CAAnimationCalculationMode);

extern_static!(kCAAnimationCubicPaced: &'static CAAnimationCalculationMode);

extern_static!(kCAAnimationRotateAuto: &'static CAAnimationRotationMode);

extern_static!(kCAAnimationRotateAutoReverse: &'static CAAnimationRotationMode);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreAnimation_CASpringAnimation")]
    /**
      Subclass for mass-spring animations.
    */
    pub struct CASpringAnimation;

    #[cfg(feature = "CoreAnimation_CASpringAnimation")]
    unsafe impl ClassType for CASpringAnimation {
        #[inherits(CAPropertyAnimation, CAAnimation, NSObject)]
        type Super = CABasicAnimation;
    }
);

#[cfg(feature = "CoreAnimation_CASpringAnimation")]
/**
  Subclass for mass-spring animations.
*/
unsafe impl CAAction for CASpringAnimation {}

#[cfg(feature = "CoreAnimation_CASpringAnimation")]
/**
  Subclass for mass-spring animations.
*/
unsafe impl CAMediaTiming for CASpringAnimation {}

#[cfg(feature = "CoreAnimation_CASpringAnimation")]
/**
  Subclass for mass-spring animations.
*/
unsafe impl NSCoding for CASpringAnimation {}

#[cfg(feature = "CoreAnimation_CASpringAnimation")]
/**
  Subclass for mass-spring animations.
*/
unsafe impl NSObjectProtocol for CASpringAnimation {}

#[cfg(feature = "CoreAnimation_CASpringAnimation")]
/**
  Subclass for mass-spring animations.
*/
unsafe impl NSSecureCoding for CASpringAnimation {}

extern_methods!(
    /**
      Subclass for mass-spring animations.
    */
    #[cfg(feature = "CoreAnimation_CASpringAnimation")]
    unsafe impl CASpringAnimation {
        /**
          The mass of the object attached to the end of the spring. Must be greater
        than 0. Defaults to one.
        */
        #[method(mass)]
        pub unsafe fn mass(&self) -> CGFloat;

        /**
          The mass of the object attached to the end of the spring. Must be greater
        than 0. Defaults to one.
        */
        #[method(setMass:)]
        pub unsafe fn setMass(&self, mass: CGFloat);

        /**
          The spring stiffness coefficient. Must be greater than 0.
         Defaults to 100.
        */
        #[method(stiffness)]
        pub unsafe fn stiffness(&self) -> CGFloat;

        /**
          The spring stiffness coefficient. Must be greater than 0.
         Defaults to 100.
        */
        #[method(setStiffness:)]
        pub unsafe fn setStiffness(&self, stiffness: CGFloat);

        /**
          The damping coefficient. Must be greater than or equal to 0.
         Defaults to 10.
        */
        #[method(damping)]
        pub unsafe fn damping(&self) -> CGFloat;

        /**
          The damping coefficient. Must be greater than or equal to 0.
         Defaults to 10.
        */
        #[method(setDamping:)]
        pub unsafe fn setDamping(&self, damping: CGFloat);

        /**
          The initial velocity of the object attached to the spring. Defaults
         to zero, which represents an unmoving object. Negative values
         represent the object moving away from the spring attachment point,
         positive values represent the object moving towards the spring
         attachment point.
        */
        #[method(initialVelocity)]
        pub unsafe fn initialVelocity(&self) -> CGFloat;

        /**
          The initial velocity of the object attached to the spring. Defaults
         to zero, which represents an unmoving object. Negative values
         represent the object moving away from the spring attachment point,
         positive values represent the object moving towards the spring
         attachment point.
        */
        #[method(setInitialVelocity:)]
        pub unsafe fn setInitialVelocity(&self, initial_velocity: CGFloat);

        /**
          Returns the estimated duration required for the spring system to be
         considered at rest. The duration is evaluated for the current animation
         parameters.
        */
        #[method(settlingDuration)]
        pub unsafe fn settlingDuration(&self) -> CFTimeInterval;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreAnimation_CATransition")]
    /**
      Transition animation subclass.
    */
    pub struct CATransition;

    #[cfg(feature = "CoreAnimation_CATransition")]
    unsafe impl ClassType for CATransition {
        #[inherits(NSObject)]
        type Super = CAAnimation;
    }
);

#[cfg(feature = "CoreAnimation_CATransition")]
/**
  Transition animation subclass.
*/
unsafe impl CAAction for CATransition {}

#[cfg(feature = "CoreAnimation_CATransition")]
/**
  Transition animation subclass.
*/
unsafe impl CAMediaTiming for CATransition {}

#[cfg(feature = "CoreAnimation_CATransition")]
/**
  Transition animation subclass.
*/
unsafe impl NSCoding for CATransition {}

#[cfg(feature = "CoreAnimation_CATransition")]
/**
  Transition animation subclass.
*/
unsafe impl NSObjectProtocol for CATransition {}

#[cfg(feature = "CoreAnimation_CATransition")]
/**
  Transition animation subclass.
*/
unsafe impl NSSecureCoding for CATransition {}

extern_methods!(
    /**
      Transition animation subclass.
    */
    #[cfg(feature = "CoreAnimation_CATransition")]
    unsafe impl CATransition {
        /**
          The name of the transition. Current legal transition types include
         `fade', `moveIn', `push' and `reveal'. Defaults to `fade'.
        */
        #[method_id(@__retain_semantics Other type)]
        pub unsafe fn r#type(&self) -> Id<CATransitionType>;

        /**
          The name of the transition. Current legal transition types include
         `fade', `moveIn', `push' and `reveal'. Defaults to `fade'.
        */
        #[method(setType:)]
        pub unsafe fn setType(&self, r#type: &CATransitionType);

        /**
          An optional subtype for the transition. E.g. used to specify the
         transition direction for motion-based transitions, in which case
         the legal values are `fromLeft', `fromRight', `fromTop' and
         `fromBottom'.
        */
        #[method_id(@__retain_semantics Other subtype)]
        pub unsafe fn subtype(&self) -> Option<Id<CATransitionSubtype>>;

        /**
          An optional subtype for the transition. E.g. used to specify the
         transition direction for motion-based transitions, in which case
         the legal values are `fromLeft', `fromRight', `fromTop' and
         `fromBottom'.
        */
        #[method(setSubtype:)]
        pub unsafe fn setSubtype(&self, subtype: Option<&CATransitionSubtype>);

        /**
          The amount of progress through to the transition at which to begin
         and end execution. Legal values are numbers in the range [0,1].
         `endProgress' must be greater than or equal to `startProgress'.
         Default values are 0 and 1 respectively.
        */
        #[method(startProgress)]
        pub unsafe fn startProgress(&self) -> c_float;

        /**
          The amount of progress through to the transition at which to begin
         and end execution. Legal values are numbers in the range [0,1].
         `endProgress' must be greater than or equal to `startProgress'.
         Default values are 0 and 1 respectively.
        */
        #[method(setStartProgress:)]
        pub unsafe fn setStartProgress(&self, start_progress: c_float);

        #[method(endProgress)]
        pub unsafe fn endProgress(&self) -> c_float;

        #[method(setEndProgress:)]
        pub unsafe fn setEndProgress(&self, end_progress: c_float);

        /**
          An optional filter object implementing the transition. When set the
         `type' and `subtype' properties are ignored. The filter must
         implement `inputImage', `inputTargetImage' and `inputTime' input
         keys, and the `outputImage' output key. Optionally it may support
         the `inputExtent' key, which will be set to a rectangle describing
         the region in which the transition should run. Defaults to nil.
        */
        #[method_id(@__retain_semantics Other filter)]
        pub unsafe fn filter(&self) -> Option<Id<Object>>;

        /**
          An optional filter object implementing the transition. When set the
         `type' and `subtype' properties are ignored. The filter must
         implement `inputImage', `inputTargetImage' and `inputTime' input
         keys, and the `outputImage' output key. Optionally it may support
         the `inputExtent' key, which will be set to a rectangle describing
         the region in which the transition should run. Defaults to nil.
        */
        #[method(setFilter:)]
        pub unsafe fn setFilter(&self, filter: Option<&Object>);
    }
);

extern_static!(kCATransitionFade: &'static CATransitionType);

extern_static!(kCATransitionMoveIn: &'static CATransitionType);

extern_static!(kCATransitionPush: &'static CATransitionType);

extern_static!(kCATransitionReveal: &'static CATransitionType);

extern_static!(kCATransitionFromRight: &'static CATransitionSubtype);

extern_static!(kCATransitionFromLeft: &'static CATransitionSubtype);

extern_static!(kCATransitionFromTop: &'static CATransitionSubtype);

extern_static!(kCATransitionFromBottom: &'static CATransitionSubtype);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreAnimation_CAAnimationGroup")]
    /**
      Animation subclass for grouped animations.
    */
    pub struct CAAnimationGroup;

    #[cfg(feature = "CoreAnimation_CAAnimationGroup")]
    unsafe impl ClassType for CAAnimationGroup {
        #[inherits(NSObject)]
        type Super = CAAnimation;
    }
);

#[cfg(feature = "CoreAnimation_CAAnimationGroup")]
/**
  Animation subclass for grouped animations.
*/
unsafe impl CAAction for CAAnimationGroup {}

#[cfg(feature = "CoreAnimation_CAAnimationGroup")]
/**
  Animation subclass for grouped animations.
*/
unsafe impl CAMediaTiming for CAAnimationGroup {}

#[cfg(feature = "CoreAnimation_CAAnimationGroup")]
/**
  Animation subclass for grouped animations.
*/
unsafe impl NSCoding for CAAnimationGroup {}

#[cfg(feature = "CoreAnimation_CAAnimationGroup")]
/**
  Animation subclass for grouped animations.
*/
unsafe impl NSObjectProtocol for CAAnimationGroup {}

#[cfg(feature = "CoreAnimation_CAAnimationGroup")]
/**
  Animation subclass for grouped animations.
*/
unsafe impl NSSecureCoding for CAAnimationGroup {}

extern_methods!(
    /**
      Animation subclass for grouped animations.
    */
    #[cfg(feature = "CoreAnimation_CAAnimationGroup")]
    unsafe impl CAAnimationGroup {
        #[cfg(feature = "Foundation_NSArray")]
        /**
          An array of CAAnimation objects. Each member of the array will run
         concurrently in the time space of the parent animation using the
         normal rules.
        */
        #[method_id(@__retain_semantics Other animations)]
        pub unsafe fn animations(&self) -> Option<Id<NSArray<CAAnimation>>>;

        #[cfg(feature = "Foundation_NSArray")]
        /**
          An array of CAAnimation objects. Each member of the array will run
         concurrently in the time space of the parent animation using the
         normal rules.
        */
        #[method(setAnimations:)]
        pub unsafe fn setAnimations(&self, animations: Option<&NSArray<CAAnimation>>);
    }
);

extern_methods!(
    /// Methods declared on superclass `CAAnimation`
    /**
      Subclass for property-based animations.
    */
    #[cfg(feature = "CoreAnimation_CAPropertyAnimation")]
    unsafe impl CAPropertyAnimation {
        #[method_id(@__retain_semantics Other animation)]
        pub unsafe fn animation() -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `CAPropertyAnimation`
    /**
      Subclass for basic (single-keyframe) animations.
    */
    #[cfg(feature = "CoreAnimation_CABasicAnimation")]
    unsafe impl CABasicAnimation {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other animationWithKeyPath:)]
        pub unsafe fn animationWithKeyPath(path: Option<&NSString>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `CAAnimation`
    /**
      Subclass for basic (single-keyframe) animations.
    */
    #[cfg(feature = "CoreAnimation_CABasicAnimation")]
    unsafe impl CABasicAnimation {
        #[method_id(@__retain_semantics Other animation)]
        pub unsafe fn animation() -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `CAPropertyAnimation`
    /**
      General keyframe animation class.
    */
    #[cfg(feature = "CoreAnimation_CAKeyframeAnimation")]
    unsafe impl CAKeyframeAnimation {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other animationWithKeyPath:)]
        pub unsafe fn animationWithKeyPath(path: Option<&NSString>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `CAAnimation`
    /**
      General keyframe animation class.
    */
    #[cfg(feature = "CoreAnimation_CAKeyframeAnimation")]
    unsafe impl CAKeyframeAnimation {
        #[method_id(@__retain_semantics Other animation)]
        pub unsafe fn animation() -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `CAPropertyAnimation`
    /**
      Subclass for mass-spring animations.
    */
    #[cfg(feature = "CoreAnimation_CASpringAnimation")]
    unsafe impl CASpringAnimation {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other animationWithKeyPath:)]
        pub unsafe fn animationWithKeyPath(path: Option<&NSString>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `CAAnimation`
    /**
      Subclass for mass-spring animations.
    */
    #[cfg(feature = "CoreAnimation_CASpringAnimation")]
    unsafe impl CASpringAnimation {
        #[method_id(@__retain_semantics Other animation)]
        pub unsafe fn animation() -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `CAAnimation`
    /**
      Transition animation subclass.
    */
    #[cfg(feature = "CoreAnimation_CATransition")]
    unsafe impl CATransition {
        #[method_id(@__retain_semantics Other animation)]
        pub unsafe fn animation() -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `CAAnimation`
    /**
      Animation subclass for grouped animations.
    */
    #[cfg(feature = "CoreAnimation_CAAnimationGroup")]
    unsafe impl CAAnimationGroup {
        #[method_id(@__retain_semantics Other animation)]
        pub unsafe fn animation() -> Id<Self>;
    }
);
