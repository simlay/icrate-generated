//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

typed_enum!(
    pub type NSSpeechSynthesizerVoiceName = NSString;
);

typed_enum!(
    pub type NSVoiceAttributeKey = NSString;
);

extern_static!(NSVoiceName: &'static NSVoiceAttributeKey);

extern_static!(NSVoiceIdentifier: &'static NSVoiceAttributeKey);

extern_static!(NSVoiceAge: &'static NSVoiceAttributeKey);

extern_static!(NSVoiceGender: &'static NSVoiceAttributeKey);

extern_static!(NSVoiceDemoText: &'static NSVoiceAttributeKey);

extern_static!(NSVoiceLocaleIdentifier: &'static NSVoiceAttributeKey);

extern_static!(NSVoiceSupportedCharacters: &'static NSVoiceAttributeKey);

extern_static!(NSVoiceIndividuallySpokenCharacters: &'static NSVoiceAttributeKey);

typed_enum!(
    pub type NSSpeechDictionaryKey = NSString;
);

extern_static!(NSSpeechDictionaryLocaleIdentifier: &'static NSSpeechDictionaryKey);

extern_static!(NSSpeechDictionaryModificationDate: &'static NSSpeechDictionaryKey);

extern_static!(NSSpeechDictionaryPronunciations: &'static NSSpeechDictionaryKey);

extern_static!(NSSpeechDictionaryAbbreviations: &'static NSSpeechDictionaryKey);

extern_static!(NSSpeechDictionaryEntrySpelling: &'static NSSpeechDictionaryKey);

extern_static!(NSSpeechDictionaryEntryPhonemes: &'static NSSpeechDictionaryKey);

typed_enum!(
    pub type NSVoiceGenderName = NSString;
);

extern_static!(NSVoiceGenderNeuter: &'static NSVoiceGenderName);

extern_static!(NSVoiceGenderMale: &'static NSVoiceGenderName);

extern_static!(NSVoiceGenderFemale: &'static NSVoiceGenderName);

extern_static!(NSVoiceGenderNeutral: &'static NSVoiceGenderName);

typed_enum!(
    pub type NSSpeechPropertyKey = NSString;
);

extern_static!(NSSpeechStatusProperty: &'static NSSpeechPropertyKey);

extern_static!(NSSpeechErrorsProperty: &'static NSSpeechPropertyKey);

extern_static!(NSSpeechInputModeProperty: &'static NSSpeechPropertyKey);

extern_static!(NSSpeechCharacterModeProperty: &'static NSSpeechPropertyKey);

extern_static!(NSSpeechNumberModeProperty: &'static NSSpeechPropertyKey);

extern_static!(NSSpeechRateProperty: &'static NSSpeechPropertyKey);

extern_static!(NSSpeechPitchBaseProperty: &'static NSSpeechPropertyKey);

extern_static!(NSSpeechPitchModProperty: &'static NSSpeechPropertyKey);

extern_static!(NSSpeechVolumeProperty: &'static NSSpeechPropertyKey);

extern_static!(NSSpeechSynthesizerInfoProperty: &'static NSSpeechPropertyKey);

extern_static!(NSSpeechRecentSyncProperty: &'static NSSpeechPropertyKey);

extern_static!(NSSpeechPhonemeSymbolsProperty: &'static NSSpeechPropertyKey);

extern_static!(NSSpeechCurrentVoiceProperty: &'static NSSpeechPropertyKey);

extern_static!(NSSpeechCommandDelimiterProperty: &'static NSSpeechPropertyKey);

extern_static!(NSSpeechResetProperty: &'static NSSpeechPropertyKey);

extern_static!(NSSpeechOutputToFileURLProperty: &'static NSSpeechPropertyKey);

extern_static!(NSVoiceLanguage: &'static NSVoiceAttributeKey);

ns_enum!(
    #[underlying(NSUInteger)]
    #[cfg(not(any(target_os = "ios")))]
    pub enum NSSpeechBoundary {
        #[cfg(not(any(target_os = "ios")))]
        NSSpeechImmediateBoundary = 0,
        #[cfg(not(any(target_os = "ios")))]
        NSSpeechWordBoundary = 1,
        #[cfg(not(any(target_os = "ios")))]
        NSSpeechSentenceBoundary = 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSSpeechSynthesizer")]
    #[cfg(not(any(target_os = "ios")))]
    pub struct NSSpeechSynthesizer;

    #[cfg(feature = "AppKit_NSSpeechSynthesizer")]
    unsafe impl ClassType for NSSpeechSynthesizer {
        type Super = NSObject;
    }
);

#[cfg(feature = "AppKit_NSSpeechSynthesizer")]
unsafe impl NSObjectProtocol for NSSpeechSynthesizer {}

extern_methods!(
    #[cfg(feature = "AppKit_NSSpeechSynthesizer")]
    unsafe impl NSSpeechSynthesizer {
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Init initWithVoice:)]
        pub unsafe fn initWithVoice(
            this: Option<Allocated<Self>>,
            voice: Option<&NSSpeechSynthesizerVoiceName>,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSString")]
        #[cfg(not(any(target_os = "ios")))]
        #[method(startSpeakingString:)]
        pub unsafe fn startSpeakingString(&self, string: &NSString) -> bool;

        #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSURL"))]
        #[cfg(not(any(target_os = "ios")))]
        #[method(startSpeakingString:toURL:)]
        pub unsafe fn startSpeakingString_toURL(&self, string: &NSString, url: &NSURL) -> bool;

        #[cfg(not(any(target_os = "ios")))]
        #[method(isSpeaking)]
        pub unsafe fn isSpeaking(&self) -> bool;

        #[cfg(not(any(target_os = "ios")))]
        #[method(stopSpeaking)]
        pub unsafe fn stopSpeaking(&self);

        #[cfg(not(any(target_os = "ios")))]
        #[method(stopSpeakingAtBoundary:)]
        pub unsafe fn stopSpeakingAtBoundary(&self, boundary: NSSpeechBoundary);

        #[cfg(not(any(target_os = "ios")))]
        #[method(pauseSpeakingAtBoundary:)]
        pub unsafe fn pauseSpeakingAtBoundary(&self, boundary: NSSpeechBoundary);

        #[cfg(not(any(target_os = "ios")))]
        #[method(continueSpeaking)]
        pub unsafe fn continueSpeaking(&self);

        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Id<ProtocolObject<dyn NSSpeechSynthesizerDelegate>>>;

        #[cfg(not(any(target_os = "ios")))]
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn NSSpeechSynthesizerDelegate>>,
        );

        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other voice)]
        pub unsafe fn voice(&self) -> Option<Id<NSSpeechSynthesizerVoiceName>>;

        #[cfg(not(any(target_os = "ios")))]
        #[method(setVoice:)]
        pub unsafe fn setVoice(&self, voice: Option<&NSSpeechSynthesizerVoiceName>) -> bool;

        #[cfg(not(any(target_os = "ios")))]
        #[method(rate)]
        pub unsafe fn rate(&self) -> c_float;

        #[cfg(not(any(target_os = "ios")))]
        #[method(setRate:)]
        pub unsafe fn setRate(&self, rate: c_float);

        #[cfg(not(any(target_os = "ios")))]
        #[method(volume)]
        pub unsafe fn volume(&self) -> c_float;

        #[cfg(not(any(target_os = "ios")))]
        #[method(setVolume:)]
        pub unsafe fn setVolume(&self, volume: c_float);

        #[cfg(not(any(target_os = "ios")))]
        #[method(usesFeedbackWindow)]
        pub unsafe fn usesFeedbackWindow(&self) -> bool;

        #[cfg(not(any(target_os = "ios")))]
        #[method(setUsesFeedbackWindow:)]
        pub unsafe fn setUsesFeedbackWindow(&self, uses_feedback_window: bool);

        #[cfg(feature = "Foundation_NSDictionary")]
        #[cfg(not(any(target_os = "ios")))]
        #[method(addSpeechDictionary:)]
        pub unsafe fn addSpeechDictionary(
            &self,
            speech_dictionary: &NSDictionary<NSSpeechDictionaryKey, Object>,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other phonemesFromText:)]
        pub unsafe fn phonemesFromText(&self, text: &NSString) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSError")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other objectForProperty:error:_)]
        pub unsafe fn objectForProperty_error(
            &self,
            property: &NSSpeechPropertyKey,
        ) -> Result<Id<Object>, Id<NSError>>;

        #[cfg(feature = "Foundation_NSError")]
        #[cfg(not(any(target_os = "ios")))]
        #[method(setObject:forProperty:error:_)]
        pub unsafe fn setObject_forProperty_error(
            &self,
            object: Option<&Object>,
            property: &NSSpeechPropertyKey,
        ) -> Result<(), Id<NSError>>;

        #[cfg(not(any(target_os = "ios")))]
        #[method(isAnyApplicationSpeaking)]
        pub unsafe fn isAnyApplicationSpeaking() -> bool;

        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other defaultVoice)]
        pub unsafe fn defaultVoice() -> Id<NSSpeechSynthesizerVoiceName>;

        #[cfg(feature = "Foundation_NSArray")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other availableVoices)]
        pub unsafe fn availableVoices() -> Id<NSArray<NSSpeechSynthesizerVoiceName>>;

        #[cfg(feature = "Foundation_NSDictionary")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other attributesForVoice:)]
        pub unsafe fn attributesForVoice(
            voice: &NSSpeechSynthesizerVoiceName,
        ) -> Id<NSDictionary<NSVoiceAttributeKey, Object>>;
    }
);

extern_protocol!(
    #[cfg(not(any(target_os = "ios")))]
    pub unsafe trait NSSpeechSynthesizerDelegate: NSObjectProtocol {
        #[cfg(feature = "AppKit_NSSpeechSynthesizer")]
        #[cfg(not(any(target_os = "ios")))]
        #[optional]
        #[method(speechSynthesizer:didFinishSpeaking:)]
        unsafe fn speechSynthesizer_didFinishSpeaking(
            &self,
            sender: &NSSpeechSynthesizer,
            finished_speaking: bool,
        );

        #[cfg(all(
            feature = "AppKit_NSSpeechSynthesizer",
            feature = "Foundation_NSString"
        ))]
        #[cfg(not(any(target_os = "ios")))]
        #[optional]
        #[method(speechSynthesizer:willSpeakWord:ofString:)]
        unsafe fn speechSynthesizer_willSpeakWord_ofString(
            &self,
            sender: &NSSpeechSynthesizer,
            character_range: NSRange,
            string: &NSString,
        );

        #[cfg(feature = "AppKit_NSSpeechSynthesizer")]
        #[cfg(not(any(target_os = "ios")))]
        #[optional]
        #[method(speechSynthesizer:willSpeakPhoneme:)]
        unsafe fn speechSynthesizer_willSpeakPhoneme(
            &self,
            sender: &NSSpeechSynthesizer,
            phoneme_opcode: c_short,
        );

        #[cfg(all(
            feature = "AppKit_NSSpeechSynthesizer",
            feature = "Foundation_NSString"
        ))]
        #[cfg(not(any(target_os = "ios")))]
        #[optional]
        #[method(speechSynthesizer:didEncounterErrorAtIndex:ofString:message:)]
        unsafe fn speechSynthesizer_didEncounterErrorAtIndex_ofString_message(
            &self,
            sender: &NSSpeechSynthesizer,
            character_index: NSUInteger,
            string: &NSString,
            message: &NSString,
        );

        #[cfg(all(
            feature = "AppKit_NSSpeechSynthesizer",
            feature = "Foundation_NSString"
        ))]
        #[cfg(not(any(target_os = "ios")))]
        #[optional]
        #[method(speechSynthesizer:didEncounterSyncMessage:)]
        unsafe fn speechSynthesizer_didEncounterSyncMessage(
            &self,
            sender: &NSSpeechSynthesizer,
            message: &NSString,
        );
    }

    unsafe impl ProtocolType for dyn NSSpeechSynthesizerDelegate {}
);

typed_enum!(
    pub type NSSpeechMode = NSString;
);

extern_static!(NSSpeechModeText: &'static NSSpeechMode);

extern_static!(NSSpeechModePhoneme: &'static NSSpeechMode);

extern_static!(NSSpeechModeNormal: &'static NSSpeechMode);

extern_static!(NSSpeechModeLiteral: &'static NSSpeechMode);

typed_enum!(
    pub type NSSpeechStatusKey = NSString;
);

extern_static!(NSSpeechStatusOutputBusy: &'static NSSpeechStatusKey);

extern_static!(NSSpeechStatusOutputPaused: &'static NSSpeechStatusKey);

extern_static!(NSSpeechStatusNumberOfCharactersLeft: &'static NSSpeechStatusKey);

extern_static!(NSSpeechStatusPhonemeCode: &'static NSSpeechStatusKey);

typed_enum!(
    pub type NSSpeechErrorKey = NSString;
);

extern_static!(NSSpeechErrorCount: &'static NSSpeechErrorKey);

extern_static!(NSSpeechErrorOldestCode: &'static NSSpeechErrorKey);

extern_static!(NSSpeechErrorOldestCharacterOffset: &'static NSSpeechErrorKey);

extern_static!(NSSpeechErrorNewestCode: &'static NSSpeechErrorKey);

extern_static!(NSSpeechErrorNewestCharacterOffset: &'static NSSpeechErrorKey);

typed_enum!(
    pub type NSSpeechSynthesizerInfoKey = NSString;
);

extern_static!(NSSpeechSynthesizerInfoIdentifier: &'static NSSpeechSynthesizerInfoKey);

extern_static!(NSSpeechSynthesizerInfoVersion: &'static NSSpeechSynthesizerInfoKey);

typed_enum!(
    pub type NSSpeechPhonemeInfoKey = NSString;
);

extern_static!(NSSpeechPhonemeInfoOpcode: &'static NSSpeechPhonemeInfoKey);

extern_static!(NSSpeechPhonemeInfoSymbol: &'static NSSpeechPhonemeInfoKey);

extern_static!(NSSpeechPhonemeInfoExample: &'static NSSpeechPhonemeInfoKey);

extern_static!(NSSpeechPhonemeInfoHiliteStart: &'static NSSpeechPhonemeInfoKey);

extern_static!(NSSpeechPhonemeInfoHiliteEnd: &'static NSSpeechPhonemeInfoKey);

typed_enum!(
    pub type NSSpeechCommandDelimiterKey = NSString;
);

extern_static!(NSSpeechCommandPrefix: &'static NSSpeechCommandDelimiterKey);

extern_static!(NSSpeechCommandSuffix: &'static NSSpeechCommandDelimiterKey);
