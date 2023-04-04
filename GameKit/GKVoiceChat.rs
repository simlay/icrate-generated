//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameKit::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum GKVoiceChatPlayerState {
        GKVoiceChatPlayerConnected = 0,
        GKVoiceChatPlayerDisconnected = 1,
        GKVoiceChatPlayerSpeaking = 2,
        GKVoiceChatPlayerSilent = 3,
        GKVoiceChatPlayerConnecting = 4,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameKit_GKVoiceChat")]
    #[cfg(not(any(target_os = "watchos")))]
    pub struct GKVoiceChat;

    #[cfg(feature = "GameKit_GKVoiceChat")]
    unsafe impl ClassType for GKVoiceChat {
        type Super = NSObject;
    }
);

#[cfg(feature = "GameKit_GKVoiceChat")]
unsafe impl NSObjectProtocol for GKVoiceChat {}

extern_methods!(
    #[cfg(feature = "GameKit_GKVoiceChat")]
    unsafe impl GKVoiceChat {
        #[method(start)]
        pub unsafe fn start(&self);

        #[method(stop)]
        pub unsafe fn stop(&self);

        #[cfg(feature = "GameKit_GKPlayer")]
        #[method(setPlayer:muted:)]
        pub unsafe fn setPlayer_muted(&self, player: &GKPlayer, is_muted: bool);

        #[cfg(feature = "GameKit_GKPlayer")]
        #[method(playerVoiceChatStateDidChangeHandler)]
        pub unsafe fn playerVoiceChatStateDidChangeHandler(
            &self,
        ) -> NonNull<Block<(NonNull<GKPlayer>, GKVoiceChatPlayerState), ()>>;

        #[cfg(feature = "GameKit_GKPlayer")]
        #[method(setPlayerVoiceChatStateDidChangeHandler:)]
        pub unsafe fn setPlayerVoiceChatStateDidChangeHandler(
            &self,
            player_voice_chat_state_did_change_handler: &Block<
                (NonNull<GKPlayer>, GKVoiceChatPlayerState),
                (),
            >,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Id<NSString>;

        #[method(isActive)]
        pub unsafe fn isActive(&self) -> bool;

        #[method(setActive:)]
        pub unsafe fn setActive(&self, active: bool);

        #[method(volume)]
        pub unsafe fn volume(&self) -> c_float;

        #[method(setVolume:)]
        pub unsafe fn setVolume(&self, volume: c_float);

        #[cfg(all(feature = "Foundation_NSArray", feature = "GameKit_GKPlayer"))]
        #[method_id(@__retain_semantics Other players)]
        pub unsafe fn players(&self) -> Id<NSArray<GKPlayer>>;

        #[method(isVoIPAllowed)]
        pub unsafe fn isVoIPAllowed() -> bool;
    }
);

extern_methods!(
    /// Deprecated
    #[cfg(feature = "GameKit_GKVoiceChat")]
    unsafe impl GKVoiceChat {
        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "use setPlayerVoiceChatStateDidChangeHandler:"]
        #[method(playerStateUpdateHandler)]
        pub unsafe fn playerStateUpdateHandler(
            &self,
        ) -> NonNull<Block<(NonNull<NSString>, GKVoiceChatPlayerState), ()>>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "use setPlayerVoiceChatStateDidChangeHandler:"]
        #[method(setPlayerStateUpdateHandler:)]
        pub unsafe fn setPlayerStateUpdateHandler(
            &self,
            player_state_update_handler: &Block<(NonNull<NSString>, GKVoiceChatPlayerState), ()>,
        );
    }
);

extern_methods!(
    /// Obsoleted
    #[cfg(feature = "GameKit_GKVoiceChat")]
    unsafe impl GKVoiceChat {
        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[deprecated = "use players"]
        #[method_id(@__retain_semantics Other playerIDs)]
        pub unsafe fn playerIDs(&self) -> Option<Id<NSArray<NSString>>>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "This is never invoked and its implementation does nothing, use setPlayer:muted:"]
        #[method(setMute:forPlayer:)]
        pub unsafe fn setMute_forPlayer(&self, is_muted: bool, player_id: &NSString);
    }
);
