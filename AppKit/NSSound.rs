//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_static!(NSSoundPboardType: &'static NSPasteboardType);

pub type NSSoundName = NSString;

pub type NSSoundPlaybackDeviceIdentifier = NSString;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSSound")]
    pub struct NSSound;

    #[cfg(feature = "AppKit_NSSound")]
    unsafe impl ClassType for NSSound {
        type Super = NSObject;
    }
);

#[cfg(feature = "AppKit_NSSound")]
unsafe impl NSCoding for NSSound {}

#[cfg(feature = "AppKit_NSSound")]
unsafe impl NSObjectProtocol for NSSound {}

#[cfg(feature = "AppKit_NSSound")]
unsafe impl NSPasteboardReading for NSSound {}

#[cfg(feature = "AppKit_NSSound")]
unsafe impl NSPasteboardWriting for NSSound {}

#[cfg(feature = "AppKit_NSSound")]
unsafe impl NSSecureCoding for NSSound {}

extern_methods!(
    #[cfg(feature = "AppKit_NSSound")]
    unsafe impl NSSound {
        #[method_id(@__retain_semantics Other soundNamed:)]
        pub unsafe fn soundNamed(name: &NSSoundName) -> Option<Id<NSSound>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Init initWithContentsOfURL:byReference:)]
        pub unsafe fn initWithContentsOfURL_byReference(
            this: Option<Allocated<Self>>,
            url: &NSURL,
            by_ref: bool,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithContentsOfFile:byReference:)]
        pub unsafe fn initWithContentsOfFile_byReference(
            this: Option<Allocated<Self>>,
            path: &NSString,
            by_ref: bool,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Init initWithData:)]
        pub unsafe fn initWithData(
            this: Option<Allocated<Self>>,
            data: &NSData,
        ) -> Option<Id<Self>>;

        #[method(setName:)]
        pub unsafe fn setName(&self, string: Option<&NSSoundName>) -> bool;

        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Option<Id<NSSoundName>>;

        #[cfg(feature = "AppKit_NSPasteboard")]
        #[method(canInitWithPasteboard:)]
        pub unsafe fn canInitWithPasteboard(pasteboard: &NSPasteboard) -> bool;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other soundUnfilteredTypes)]
        pub unsafe fn soundUnfilteredTypes() -> Id<NSArray<NSString>>;

        #[cfg(feature = "AppKit_NSPasteboard")]
        #[method_id(@__retain_semantics Init initWithPasteboard:)]
        pub unsafe fn initWithPasteboard(
            this: Option<Allocated<Self>>,
            pasteboard: &NSPasteboard,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "AppKit_NSPasteboard")]
        #[method(writeToPasteboard:)]
        pub unsafe fn writeToPasteboard(&self, pasteboard: &NSPasteboard);

        #[method(play)]
        pub unsafe fn play(&self) -> bool;

        #[method(pause)]
        pub unsafe fn pause(&self) -> bool;

        #[method(resume)]
        pub unsafe fn resume(&self) -> bool;

        #[method(stop)]
        pub unsafe fn stop(&self) -> bool;

        #[method(isPlaying)]
        pub unsafe fn isPlaying(&self) -> bool;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<ProtocolObject<dyn NSSoundDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&ProtocolObject<dyn NSSoundDelegate>>);

        #[method(duration)]
        pub unsafe fn duration(&self) -> NSTimeInterval;

        #[method(volume)]
        pub unsafe fn volume(&self) -> c_float;

        #[method(setVolume:)]
        pub unsafe fn setVolume(&self, volume: c_float);

        #[method(currentTime)]
        pub unsafe fn currentTime(&self) -> NSTimeInterval;

        #[method(setCurrentTime:)]
        pub unsafe fn setCurrentTime(&self, current_time: NSTimeInterval);

        #[method(loops)]
        pub unsafe fn loops(&self) -> bool;

        #[method(setLoops:)]
        pub unsafe fn setLoops(&self, loops: bool);

        #[method_id(@__retain_semantics Other playbackDeviceIdentifier)]
        pub unsafe fn playbackDeviceIdentifier(
            &self,
        ) -> Option<Id<NSSoundPlaybackDeviceIdentifier>>;

        #[method(setPlaybackDeviceIdentifier:)]
        pub unsafe fn setPlaybackDeviceIdentifier(
            &self,
            playback_device_identifier: Option<&NSSoundPlaybackDeviceIdentifier>,
        );

        #[cfg(feature = "Foundation_NSArray")]
        #[deprecated]
        #[method(setChannelMapping:)]
        pub unsafe fn setChannelMapping(&self, channel_mapping: Option<&NSArray>);

        #[cfg(feature = "Foundation_NSArray")]
        #[deprecated]
        #[method_id(@__retain_semantics Other channelMapping)]
        pub unsafe fn channelMapping(&self) -> Option<Id<NSArray>>;
    }
);

extern_methods!(
    /// NSDeprecated
    #[cfg(feature = "AppKit_NSSound")]
    unsafe impl NSSound {
        #[cfg(feature = "Foundation_NSArray")]
        #[deprecated]
        #[method_id(@__retain_semantics Other soundUnfilteredFileTypes)]
        pub unsafe fn soundUnfilteredFileTypes() -> Option<Id<NSArray>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[deprecated]
        #[method_id(@__retain_semantics Other soundUnfilteredPasteboardTypes)]
        pub unsafe fn soundUnfilteredPasteboardTypes() -> Option<Id<NSArray>>;
    }
);

extern_protocol!(
    pub unsafe trait NSSoundDelegate: NSObjectProtocol {
        #[cfg(feature = "AppKit_NSSound")]
        #[optional]
        #[method(sound:didFinishPlaying:)]
        unsafe fn sound_didFinishPlaying(&self, sound: &NSSound, flag: bool);
    }

    unsafe impl ProtocolType for dyn NSSoundDelegate {}
);

extern_methods!(
    /// NSBundleSoundExtensions
    #[cfg(feature = "Foundation_NSBundle")]
    unsafe impl NSBundle {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other pathForSoundResource:)]
        pub unsafe fn pathForSoundResource(&self, name: &NSSoundName) -> Option<Id<NSString>>;
    }
);
