//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Speech::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Speech_SFSpeechRecognitionResult")]
    pub struct SFSpeechRecognitionResult;

    #[cfg(feature = "Speech_SFSpeechRecognitionResult")]
    unsafe impl ClassType for SFSpeechRecognitionResult {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "Speech_SFSpeechRecognitionResult")]
unsafe impl NSCoding for SFSpeechRecognitionResult {}

#[cfg(feature = "Speech_SFSpeechRecognitionResult")]
unsafe impl NSCopying for SFSpeechRecognitionResult {}

#[cfg(feature = "Speech_SFSpeechRecognitionResult")]
unsafe impl NSObjectProtocol for SFSpeechRecognitionResult {}

#[cfg(feature = "Speech_SFSpeechRecognitionResult")]
unsafe impl NSSecureCoding for SFSpeechRecognitionResult {}

extern_methods!(
    #[cfg(feature = "Speech_SFSpeechRecognitionResult")]
    unsafe impl SFSpeechRecognitionResult {
        #[cfg(feature = "Speech_SFTranscription")]
        #[method_id(@__retain_semantics Other bestTranscription)]
        pub unsafe fn bestTranscription(&self) -> Id<SFTranscription>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Speech_SFTranscription"))]
        #[method_id(@__retain_semantics Other transcriptions)]
        pub unsafe fn transcriptions(&self) -> Id<NSArray<SFTranscription>>;

        #[method(isFinal)]
        pub unsafe fn isFinal(&self) -> bool;

        #[cfg(feature = "Speech_SFSpeechRecognitionMetadata")]
        #[method_id(@__retain_semantics Other speechRecognitionMetadata)]
        pub unsafe fn speechRecognitionMetadata(&self) -> Option<Id<SFSpeechRecognitionMetadata>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Speech_SFSpeechRecognitionResult")]
    unsafe impl SFSpeechRecognitionResult {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
