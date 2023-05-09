//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Speech::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Speech_SFTranscriptionSegment")]
    pub struct SFTranscriptionSegment;

    #[cfg(feature = "Speech_SFTranscriptionSegment")]
    unsafe impl ClassType for SFTranscriptionSegment {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Speech_SFTranscriptionSegment")]
unsafe impl NSCoding for SFTranscriptionSegment {}

#[cfg(feature = "Speech_SFTranscriptionSegment")]
unsafe impl NSCopying for SFTranscriptionSegment {}

#[cfg(feature = "Speech_SFTranscriptionSegment")]
unsafe impl NSObjectProtocol for SFTranscriptionSegment {}

#[cfg(feature = "Speech_SFTranscriptionSegment")]
unsafe impl NSSecureCoding for SFTranscriptionSegment {}

extern_methods!(
    #[cfg(feature = "Speech_SFTranscriptionSegment")]
    unsafe impl SFTranscriptionSegment {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other substring)]
        pub unsafe fn substring(&self) -> Id<NSString>;

        #[method(substringRange)]
        pub unsafe fn substringRange(&self) -> NSRange;

        #[method(timestamp)]
        pub unsafe fn timestamp(&self) -> NSTimeInterval;

        #[method(duration)]
        pub unsafe fn duration(&self) -> NSTimeInterval;

        #[method(confidence)]
        pub unsafe fn confidence(&self) -> c_float;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other alternativeSubstrings)]
        pub unsafe fn alternativeSubstrings(&self) -> Id<NSArray<NSString>>;

        #[cfg(feature = "Speech_SFVoiceAnalytics")]
        #[deprecated = "voiceAnalytics is moved to SFSpeechRecognitionMetadata"]
        #[method_id(@__retain_semantics Other voiceAnalytics)]
        pub unsafe fn voiceAnalytics(&self) -> Option<Id<SFVoiceAnalytics>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Speech_SFTranscriptionSegment")]
    unsafe impl SFTranscriptionSegment {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
