//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::MediaPlayer::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum MPMusicPlaybackState {
        MPMusicPlaybackStateStopped = 0,
        MPMusicPlaybackStatePlaying = 1,
        MPMusicPlaybackStatePaused = 2,
        MPMusicPlaybackStateInterrupted = 3,
        MPMusicPlaybackStateSeekingForward = 4,
        MPMusicPlaybackStateSeekingBackward = 5,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum MPMusicRepeatMode {
        MPMusicRepeatModeDefault = 0,
        MPMusicRepeatModeNone = 1,
        MPMusicRepeatModeOne = 2,
        MPMusicRepeatModeAll = 3,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum MPMusicShuffleMode {
        MPMusicShuffleModeDefault = 0,
        MPMusicShuffleModeOff = 1,
        MPMusicShuffleModeSongs = 2,
        MPMusicShuffleModeAlbums = 3,
    }
);

extern_protocol!(
    pub unsafe trait MPSystemMusicPlayerController: NSObjectProtocol {
        #[cfg(feature = "MediaPlayer_MPMusicPlayerQueueDescriptor")]
        #[method(openToPlayQueueDescriptor:)]
        unsafe fn openToPlayQueueDescriptor(&self, queue_descriptor: &MPMusicPlayerQueueDescriptor);
    }

    unsafe impl ProtocolType for dyn MPSystemMusicPlayerController {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MediaPlayer_MPMusicPlayerController")]
    pub struct MPMusicPlayerController;

    #[cfg(feature = "MediaPlayer_MPMusicPlayerController")]
    unsafe impl ClassType for MPMusicPlayerController {
        type Super = NSObject;
    }
);

#[cfg(feature = "MediaPlayer_MPMusicPlayerController")]
unsafe impl MPMediaPlayback for MPMusicPlayerController {}

#[cfg(feature = "MediaPlayer_MPMusicPlayerController")]
unsafe impl NSObjectProtocol for MPMusicPlayerController {}

extern_methods!(
    #[cfg(feature = "MediaPlayer_MPMusicPlayerController")]
    unsafe impl MPMusicPlayerController {
        #[method_id(@__retain_semantics Other applicationMusicPlayer)]
        pub unsafe fn applicationMusicPlayer() -> Id<MPMusicPlayerController>;

        #[cfg(feature = "MediaPlayer_MPMusicPlayerApplicationController")]
        #[method_id(@__retain_semantics Other applicationQueuePlayer)]
        pub unsafe fn applicationQueuePlayer() -> Id<MPMusicPlayerApplicationController>;

        #[method_id(@__retain_semantics Other systemMusicPlayer)]
        pub unsafe fn systemMusicPlayer() -> Id<MPMusicPlayerController>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method(playbackState)]
        pub unsafe fn playbackState(&self) -> MPMusicPlaybackState;

        #[method(repeatMode)]
        pub unsafe fn repeatMode(&self) -> MPMusicRepeatMode;

        #[method(setRepeatMode:)]
        pub unsafe fn setRepeatMode(&self, repeat_mode: MPMusicRepeatMode);

        #[method(shuffleMode)]
        pub unsafe fn shuffleMode(&self) -> MPMusicShuffleMode;

        #[method(setShuffleMode:)]
        pub unsafe fn setShuffleMode(&self, shuffle_mode: MPMusicShuffleMode);

        #[deprecated = "Use MPVolumeView for volume control."]
        #[method(volume)]
        pub unsafe fn volume(&self) -> c_float;

        #[deprecated = "Use MPVolumeView for volume control."]
        #[method(setVolume:)]
        pub unsafe fn setVolume(&self, volume: c_float);

        #[cfg(feature = "MediaPlayer_MPMediaItem")]
        #[method_id(@__retain_semantics Other nowPlayingItem)]
        pub unsafe fn nowPlayingItem(&self) -> Option<Id<MPMediaItem>>;

        #[cfg(feature = "MediaPlayer_MPMediaItem")]
        #[method(setNowPlayingItem:)]
        pub unsafe fn setNowPlayingItem(&self, now_playing_item: Option<&MPMediaItem>);

        #[method(indexOfNowPlayingItem)]
        pub unsafe fn indexOfNowPlayingItem(&self) -> NSUInteger;

        #[cfg(feature = "MediaPlayer_MPMediaQuery")]
        #[method(setQueueWithQuery:)]
        pub unsafe fn setQueueWithQuery(&self, query: &MPMediaQuery);

        #[cfg(feature = "MediaPlayer_MPMediaItemCollection")]
        #[method(setQueueWithItemCollection:)]
        pub unsafe fn setQueueWithItemCollection(&self, item_collection: &MPMediaItemCollection);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method(setQueueWithStoreIDs:)]
        pub unsafe fn setQueueWithStoreIDs(&self, store_i_ds: &NSArray<NSString>);

        #[cfg(feature = "MediaPlayer_MPMusicPlayerQueueDescriptor")]
        #[method(setQueueWithDescriptor:)]
        pub unsafe fn setQueueWithDescriptor(&self, descriptor: &MPMusicPlayerQueueDescriptor);

        #[cfg(feature = "MediaPlayer_MPMusicPlayerQueueDescriptor")]
        #[method(prependQueueDescriptor:)]
        pub unsafe fn prependQueueDescriptor(&self, descriptor: &MPMusicPlayerQueueDescriptor);

        #[cfg(feature = "MediaPlayer_MPMusicPlayerQueueDescriptor")]
        #[method(appendQueueDescriptor:)]
        pub unsafe fn appendQueueDescriptor(&self, descriptor: &MPMusicPlayerQueueDescriptor);

        #[cfg(feature = "Foundation_NSError")]
        #[method(prepareToPlayWithCompletionHandler:)]
        pub unsafe fn prepareToPlayWithCompletionHandler(
            &self,
            completion_handler: &Block<(*mut NSError,), ()>,
        );

        #[method(skipToNextItem)]
        pub unsafe fn skipToNextItem(&self);

        #[method(skipToBeginning)]
        pub unsafe fn skipToBeginning(&self);

        #[method(skipToPreviousItem)]
        pub unsafe fn skipToPreviousItem(&self);

        #[method(beginGeneratingPlaybackNotifications)]
        pub unsafe fn beginGeneratingPlaybackNotifications(&self);

        #[method(endGeneratingPlaybackNotifications)]
        pub unsafe fn endGeneratingPlaybackNotifications(&self);

        #[deprecated]
        #[method_id(@__retain_semantics Other iPodMusicPlayer)]
        pub unsafe fn iPodMusicPlayer() -> Id<MPMusicPlayerController>;
    }
);

extern_static!(
    MPMusicPlayerControllerPlaybackStateDidChangeNotification: &'static NSNotificationName
);

extern_static!(
    MPMusicPlayerControllerNowPlayingItemDidChangeNotification: &'static NSNotificationName
);

extern_static!(MPMusicPlayerControllerVolumeDidChangeNotification: &'static NSNotificationName);
