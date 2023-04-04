//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameKit::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum GKConnectionState {
        GKConnectionStateNotConnected = 0,
        GKConnectionStateConnected = 1,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum GKTransportType {
        GKTransportTypeUnreliable = 0,
        GKTransportTypeReliable = 1,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameKit_GKGameSession")]
    #[deprecated = "For real-time matches, use GKMatchmakerViewController. For turn-based matches, use GKTurnBasedMatchmakerViewController."]
    #[cfg(not(any(target_os = "watchos")))]
    pub struct GKGameSession;

    #[cfg(feature = "GameKit_GKGameSession")]
    unsafe impl ClassType for GKGameSession {
        type Super = NSObject;
    }
);

#[cfg(feature = "GameKit_GKGameSession")]
unsafe impl NSObjectProtocol for GKGameSession {}

extern_methods!(
    #[cfg(feature = "GameKit_GKGameSession")]
    unsafe impl GKGameSession {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Id<NSString>;

        #[cfg(feature = "GameKit_GKCloudPlayer")]
        #[method_id(@__retain_semantics Other owner)]
        pub unsafe fn owner(&self) -> Id<GKCloudPlayer>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "GameKit_GKCloudPlayer"))]
        #[method_id(@__retain_semantics Other players)]
        pub unsafe fn players(&self) -> Id<NSArray<GKCloudPlayer>>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other lastModifiedDate)]
        pub unsafe fn lastModifiedDate(&self) -> Id<NSDate>;

        #[cfg(feature = "GameKit_GKCloudPlayer")]
        #[method_id(@__retain_semantics Other lastModifiedPlayer)]
        pub unsafe fn lastModifiedPlayer(&self) -> Id<GKCloudPlayer>;

        #[method(maxNumberOfConnectedPlayers)]
        pub unsafe fn maxNumberOfConnectedPlayers(&self) -> NSInteger;

        #[cfg(all(feature = "Foundation_NSArray", feature = "GameKit_GKCloudPlayer"))]
        #[method_id(@__retain_semantics Other badgedPlayers)]
        pub unsafe fn badgedPlayers(&self) -> Id<NSArray<GKCloudPlayer>>;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[method(createSessionInContainer:withTitle:maxConnectedPlayers:completionHandler:)]
        pub unsafe fn createSessionInContainer_withTitle_maxConnectedPlayers_completionHandler(
            container_name: Option<&NSString>,
            title: &NSString,
            max_players: NSInteger,
            completion_handler: &Block<(*mut GKGameSession, *mut NSError), ()>,
        );

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString"
        ))]
        #[method(loadSessionsInContainer:completionHandler:)]
        pub unsafe fn loadSessionsInContainer_completionHandler(
            container_name: Option<&NSString>,
            completion_handler: &Block<(*mut NSArray<GKGameSession>, *mut NSError), ()>,
        );

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[method(loadSessionWithIdentifier:completionHandler:)]
        pub unsafe fn loadSessionWithIdentifier_completionHandler(
            identifier: &NSString,
            completion_handler: &Block<(*mut GKGameSession, *mut NSError), ()>,
        );

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[method(removeSessionWithIdentifier:completionHandler:)]
        pub unsafe fn removeSessionWithIdentifier_completionHandler(
            identifier: &NSString,
            completion_handler: &Block<(*mut NSError,), ()>,
        );

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
        #[method(getShareURLWithCompletionHandler:)]
        pub unsafe fn getShareURLWithCompletionHandler(
            &self,
            completion_handler: &Block<(*mut NSURL, *mut NSError), ()>,
        );

        #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSError"))]
        #[method(loadDataWithCompletionHandler:)]
        pub unsafe fn loadDataWithCompletionHandler(
            &self,
            completion_handler: &Block<(*mut NSData, *mut NSError), ()>,
        );

        #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSError"))]
        #[method(saveData:completionHandler:)]
        pub unsafe fn saveData_completionHandler(
            &self,
            data: &NSData,
            completion_handler: &Block<(*mut NSData, *mut NSError), ()>,
        );

        #[cfg(feature = "Foundation_NSError")]
        #[method(setConnectionState:completionHandler:)]
        pub unsafe fn setConnectionState_completionHandler(
            &self,
            state: GKConnectionState,
            completion_handler: &Block<(*mut NSError,), ()>,
        );

        #[cfg(all(feature = "Foundation_NSArray", feature = "GameKit_GKCloudPlayer"))]
        #[method_id(@__retain_semantics Other playersWithConnectionState:)]
        pub unsafe fn playersWithConnectionState(
            &self,
            state: GKConnectionState,
        ) -> Id<NSArray<GKCloudPlayer>>;

        #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSError"))]
        #[method(sendData:withTransportType:completionHandler:)]
        pub unsafe fn sendData_withTransportType_completionHandler(
            &self,
            data: &NSData,
            transport: GKTransportType,
            completion_handler: &Block<(*mut NSError,), ()>,
        );

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSData",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "GameKit_GKCloudPlayer"
        ))]
        #[method(sendMessageWithLocalizedFormatKey:arguments:data:toPlayers:badgePlayers:completionHandler:)]
        pub unsafe fn sendMessageWithLocalizedFormatKey_arguments_data_toPlayers_badgePlayers_completionHandler(
            &self,
            key: &NSString,
            arguments: &NSArray<NSString>,
            data: Option<&NSData>,
            players: &NSArray<GKCloudPlayer>,
            badge_players: bool,
            completion_handler: &Block<(*mut NSError,), ()>,
        );

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "GameKit_GKCloudPlayer"
        ))]
        #[method(clearBadgeForPlayers:completionHandler:)]
        pub unsafe fn clearBadgeForPlayers_completionHandler(
            &self,
            players: &NSArray<GKCloudPlayer>,
            completion_handler: &Block<(*mut NSError,), ()>,
        );
    }
);
