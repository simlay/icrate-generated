//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameKit::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum GKMatchSendDataMode {
        GKMatchSendDataReliable = 0,
        GKMatchSendDataUnreliable = 1,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum GKPlayerConnectionState {
        GKPlayerStateUnknown = 0,
        GKPlayerStateConnected = 1,
        GKPlayerStateDisconnected = 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameKit_GKMatch")]
    #[cfg(not(any(target_os = "watchos")))]
    pub struct GKMatch;

    #[cfg(feature = "GameKit_GKMatch")]
    unsafe impl ClassType for GKMatch {
        type Super = NSObject;
    }
);

#[cfg(feature = "GameKit_GKMatch")]
unsafe impl NSObjectProtocol for GKMatch {}

extern_methods!(
    #[cfg(feature = "GameKit_GKMatch")]
    unsafe impl GKMatch {
        #[cfg(all(feature = "Foundation_NSArray", feature = "GameKit_GKPlayer"))]
        #[method_id(@__retain_semantics Other players)]
        pub unsafe fn players(&self) -> Id<NSArray<GKPlayer>>;

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<ProtocolObject<dyn GKMatchDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&ProtocolObject<dyn GKMatchDelegate>>);

        #[method(expectedPlayerCount)]
        pub unsafe fn expectedPlayerCount(&self) -> NSUInteger;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSData",
            feature = "Foundation_NSError",
            feature = "GameKit_GKPlayer"
        ))]
        #[method(sendData:toPlayers:dataMode:error:_)]
        pub unsafe fn sendData_toPlayers_dataMode_error(
            &self,
            data: &NSData,
            players: &NSArray<GKPlayer>,
            mode: GKMatchSendDataMode,
        ) -> Result<(), Id<NSError>>;

        #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSError"))]
        #[method(sendDataToAllPlayers:withDataMode:error:_)]
        pub unsafe fn sendDataToAllPlayers_withDataMode_error(
            &self,
            data: &NSData,
            mode: GKMatchSendDataMode,
        ) -> Result<(), Id<NSError>>;

        #[method(disconnect)]
        pub unsafe fn disconnect(&self);

        #[cfg(all(feature = "Foundation_NSString", feature = "GameKit_GKVoiceChat"))]
        #[method_id(@__retain_semantics Other voiceChatWithName:)]
        pub unsafe fn voiceChatWithName(&self, name: &NSString) -> Option<Id<GKVoiceChat>>;

        #[cfg(feature = "GameKit_GKPlayer")]
        #[method(chooseBestHostingPlayerWithCompletionHandler:)]
        pub unsafe fn chooseBestHostingPlayerWithCompletionHandler(
            &self,
            completion_handler: &Block<(*mut GKPlayer,), ()>,
        );

        #[cfg(feature = "Foundation_NSError")]
        #[method(rematchWithCompletionHandler:)]
        pub unsafe fn rematchWithCompletionHandler(
            &self,
            completion_handler: Option<&Block<(*mut GKMatch, *mut NSError), ()>>,
        );
    }
);

extern_protocol!(
    #[cfg(not(any(target_os = "watchos")))]
    pub unsafe trait GKMatchDelegate: NSObjectProtocol {
        #[cfg(all(
            feature = "Foundation_NSData",
            feature = "GameKit_GKMatch",
            feature = "GameKit_GKPlayer"
        ))]
        #[optional]
        #[method(match:didReceiveData:fromRemotePlayer:)]
        unsafe fn match_didReceiveData_fromRemotePlayer(
            &self,
            r#match: &GKMatch,
            data: &NSData,
            player: &GKPlayer,
        );

        #[cfg(all(
            feature = "Foundation_NSData",
            feature = "GameKit_GKMatch",
            feature = "GameKit_GKPlayer"
        ))]
        #[optional]
        #[method(match:didReceiveData:forRecipient:fromRemotePlayer:)]
        unsafe fn match_didReceiveData_forRecipient_fromRemotePlayer(
            &self,
            r#match: &GKMatch,
            data: &NSData,
            recipient: &GKPlayer,
            player: &GKPlayer,
        );

        #[cfg(all(feature = "GameKit_GKMatch", feature = "GameKit_GKPlayer"))]
        #[optional]
        #[method(match:player:didChangeConnectionState:)]
        unsafe fn match_player_didChangeConnectionState(
            &self,
            r#match: &GKMatch,
            player: &GKPlayer,
            state: GKPlayerConnectionState,
        );

        #[cfg(all(feature = "Foundation_NSError", feature = "GameKit_GKMatch"))]
        #[optional]
        #[method(match:didFailWithError:)]
        unsafe fn match_didFailWithError(&self, r#match: &GKMatch, error: Option<&NSError>);

        #[cfg(all(feature = "GameKit_GKMatch", feature = "GameKit_GKPlayer"))]
        #[optional]
        #[method(match:shouldReinviteDisconnectedPlayer:)]
        unsafe fn match_shouldReinviteDisconnectedPlayer(
            &self,
            r#match: &GKMatch,
            player: &GKPlayer,
        ) -> bool;

        #[cfg(all(
            feature = "Foundation_NSData",
            feature = "Foundation_NSString",
            feature = "GameKit_GKMatch"
        ))]
        #[deprecated = "This is never invoked and its implementation does nothing, use match:didReceiveData:fromRemotePlayer:"]
        #[optional]
        #[method(match:didReceiveData:fromPlayer:)]
        unsafe fn match_didReceiveData_fromPlayer(
            &self,
            r#match: &GKMatch,
            data: &NSData,
            player_id: &NSString,
        );

        #[cfg(all(feature = "Foundation_NSString", feature = "GameKit_GKMatch"))]
        #[deprecated = "This is never invoked and its implementation does nothing, use match:player:didChangeConnectionState:"]
        #[optional]
        #[method(match:player:didChangeState:)]
        unsafe fn match_player_didChangeState(
            &self,
            r#match: &GKMatch,
            player_id: &NSString,
            state: GKPlayerConnectionState,
        );

        #[cfg(all(feature = "Foundation_NSString", feature = "GameKit_GKMatch"))]
        #[deprecated = "This is never invoked and its implementation does nothing, use shouldReinviteDisconnectedPlayer:"]
        #[optional]
        #[method(match:shouldReinvitePlayer:)]
        unsafe fn match_shouldReinvitePlayer(
            &self,
            r#match: &GKMatch,
            player_id: &NSString,
        ) -> bool;
    }

    unsafe impl ProtocolType for dyn GKMatchDelegate {}
);

extern_methods!(
    /// Obsoleted
    #[cfg(feature = "GameKit_GKMatch")]
    unsafe impl GKMatch {
        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "This is never invoked and its implementation does nothing, use chooseBestHostingPlayerWithCompletionHandler:"]
        #[method(chooseBestHostPlayerWithCompletionHandler:)]
        pub unsafe fn chooseBestHostPlayerWithCompletionHandler(
            &self,
            completion_handler: &Block<(*mut NSString,), ()>,
        );

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSData",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString"
        ))]
        #[deprecated = "This is never invoked and its implementation does nothing, use sendData:toPlayers:dataMode:error:"]
        #[method(sendData:toPlayers:withDataMode:error:_)]
        pub unsafe fn sendData_toPlayers_withDataMode_error(
            &self,
            data: &NSData,
            player_i_ds: &NSArray<NSString>,
            mode: GKMatchSendDataMode,
        ) -> Result<(), Id<NSError>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[deprecated = "This is never invoked and its implementation does nothing, use players instead."]
        #[method_id(@__retain_semantics Other playerIDs)]
        pub unsafe fn playerIDs(&self) -> Option<Id<NSArray<NSString>>>;
    }
);
