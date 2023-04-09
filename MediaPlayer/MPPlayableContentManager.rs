//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::MediaPlayer::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MediaPlayer_MPPlayableContentManager")]
    #[deprecated = "Use CarPlay framework"]
    #[cfg(not(any(target_os = "macos", target_os = "tvos", target_os = "watchos")))]
    pub struct MPPlayableContentManager;

    #[deprecated = "Use CarPlay framework"]
    #[cfg(not(any(target_os = "macos", target_os = "tvos", target_os = "watchos")))]
    #[cfg(feature = "MediaPlayer_MPPlayableContentManager")]
    unsafe impl ClassType for MPPlayableContentManager {
        type Super = NSObject;
    }
);

#[cfg(feature = "MediaPlayer_MPPlayableContentManager")]
#[cfg(not(any(target_os = "macos", target_os = "tvos", target_os = "watchos")))]
unsafe impl NSObjectProtocol for MPPlayableContentManager {}

#[cfg(not(any(target_os = "macos", target_os = "tvos", target_os = "watchos")))]
extern_methods!(
    #[cfg(feature = "MediaPlayer_MPPlayableContentManager")]
    unsafe impl MPPlayableContentManager {
        #[method_id(@__retain_semantics Other dataSource)]
        pub unsafe fn dataSource(
            &self,
        ) -> Option<Id<ProtocolObject<dyn MPPlayableContentDataSource>>>;

        #[method(setDataSource:)]
        pub unsafe fn setDataSource(
            &self,
            data_source: Option<&ProtocolObject<dyn MPPlayableContentDataSource>>,
        );

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<ProtocolObject<dyn MPPlayableContentDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn MPPlayableContentDelegate>>,
        );

        #[cfg(feature = "MediaPlayer_MPPlayableContentManagerContext")]
        #[deprecated = "Use CarPlay framework"]
        #[method_id(@__retain_semantics Other context)]
        pub unsafe fn context(&self) -> Id<MPPlayableContentManagerContext>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[deprecated = "Use CarPlay framework"]
        #[method_id(@__retain_semantics Other nowPlayingIdentifiers)]
        pub unsafe fn nowPlayingIdentifiers(&self) -> Id<NSArray<NSString>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[deprecated = "Use CarPlay framework"]
        #[method(setNowPlayingIdentifiers:)]
        pub unsafe fn setNowPlayingIdentifiers(&self, now_playing_identifiers: &NSArray<NSString>);

        #[method_id(@__retain_semantics Other sharedContentManager)]
        pub unsafe fn sharedContentManager() -> Id<Self>;

        #[method(reloadData)]
        pub unsafe fn reloadData(&self);

        #[method(beginUpdates)]
        pub unsafe fn beginUpdates(&self);

        #[method(endUpdates)]
        pub unsafe fn endUpdates(&self);
    }
);
