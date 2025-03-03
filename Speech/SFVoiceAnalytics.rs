//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Speech::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Speech_SFAcousticFeature")]
    pub struct SFAcousticFeature;

    #[cfg(feature = "Speech_SFAcousticFeature")]
    unsafe impl ClassType for SFAcousticFeature {
        type Super = NSObject;
    }
);

#[cfg(feature = "Speech_SFAcousticFeature")]
unsafe impl NSCoding for SFAcousticFeature {}

#[cfg(feature = "Speech_SFAcousticFeature")]
unsafe impl NSObjectProtocol for SFAcousticFeature {}

#[cfg(feature = "Speech_SFAcousticFeature")]
unsafe impl NSSecureCoding for SFAcousticFeature {}

extern_methods!(
    #[cfg(feature = "Speech_SFAcousticFeature")]
    unsafe impl SFAcousticFeature {
        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSNumber"))]
        #[method_id(@__retain_semantics Other acousticFeatureValuePerFrame)]
        pub unsafe fn acousticFeatureValuePerFrame(&self) -> Id<NSArray<NSNumber>>;

        #[method(frameDuration)]
        pub unsafe fn frameDuration(&self) -> NSTimeInterval;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Speech_SFVoiceAnalytics")]
    pub struct SFVoiceAnalytics;

    #[cfg(feature = "Speech_SFVoiceAnalytics")]
    unsafe impl ClassType for SFVoiceAnalytics {
        type Super = NSObject;
    }
);

#[cfg(feature = "Speech_SFVoiceAnalytics")]
unsafe impl NSCoding for SFVoiceAnalytics {}

#[cfg(feature = "Speech_SFVoiceAnalytics")]
unsafe impl NSObjectProtocol for SFVoiceAnalytics {}

#[cfg(feature = "Speech_SFVoiceAnalytics")]
unsafe impl NSSecureCoding for SFVoiceAnalytics {}

extern_methods!(
    #[cfg(feature = "Speech_SFVoiceAnalytics")]
    unsafe impl SFVoiceAnalytics {
        #[cfg(feature = "Speech_SFAcousticFeature")]
        #[method_id(@__retain_semantics Other jitter)]
        pub unsafe fn jitter(&self) -> Id<SFAcousticFeature>;

        #[cfg(feature = "Speech_SFAcousticFeature")]
        #[method_id(@__retain_semantics Other shimmer)]
        pub unsafe fn shimmer(&self) -> Id<SFAcousticFeature>;

        #[cfg(feature = "Speech_SFAcousticFeature")]
        #[method_id(@__retain_semantics Other pitch)]
        pub unsafe fn pitch(&self) -> Id<SFAcousticFeature>;

        #[cfg(feature = "Speech_SFAcousticFeature")]
        #[method_id(@__retain_semantics Other voicing)]
        pub unsafe fn voicing(&self) -> Id<SFAcousticFeature>;
    }
);
