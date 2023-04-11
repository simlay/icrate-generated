//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameKit::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum GKMatchmakingMode {
        GKMatchmakingModeDefault = 0,
        GKMatchmakingModeNearbyOnly = 1,
        GKMatchmakingModeAutomatchOnly = 2,
        GKMatchmakingModeInviteOnly = 3,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameKit_GKMatchmakerViewController")]
    pub struct GKMatchmakerViewController;

    #[cfg(feature = "GameKit_GKMatchmakerViewController")]
    unsafe impl ClassType for GKMatchmakerViewController {
        #[inherits(NSResponder, NSObject)]
        type Super = NSViewController;
    }
);

#[cfg(feature = "GameKit_GKMatchmakerViewController")]
unsafe impl GKViewController for GKMatchmakerViewController {}

#[cfg(feature = "GameKit_GKMatchmakerViewController")]
unsafe impl NSCoding for GKMatchmakerViewController {}

#[cfg(feature = "GameKit_GKMatchmakerViewController")]
unsafe impl NSEditor for GKMatchmakerViewController {}

#[cfg(feature = "GameKit_GKMatchmakerViewController")]
unsafe impl NSObjectProtocol for GKMatchmakerViewController {}

#[cfg(feature = "GameKit_GKMatchmakerViewController")]
unsafe impl NSSeguePerforming for GKMatchmakerViewController {}

#[cfg(feature = "GameKit_GKMatchmakerViewController")]
unsafe impl NSUserInterfaceItemIdentification for GKMatchmakerViewController {}

extern_methods!(
    #[cfg(feature = "GameKit_GKMatchmakerViewController")]
    unsafe impl GKMatchmakerViewController {
        #[method_id(@__retain_semantics Other matchmakerDelegate)]
        pub unsafe fn matchmakerDelegate(
            &self,
        ) -> Option<Id<ProtocolObject<dyn GKMatchmakerViewControllerDelegate>>>;

        #[method(setMatchmakerDelegate:)]
        pub unsafe fn setMatchmakerDelegate(
            &self,
            matchmaker_delegate: Option<&ProtocolObject<dyn GKMatchmakerViewControllerDelegate>>,
        );

        #[cfg(feature = "GameKit_GKMatchRequest")]
        #[method_id(@__retain_semantics Other matchRequest)]
        pub unsafe fn matchRequest(&self) -> Id<GKMatchRequest>;

        /**
          set to YES to receive hosted (eg. not peer-to-peer) match results. Will cause the controller to return an array of players instead of a match.
        */
        #[method(isHosted)]
        pub unsafe fn isHosted(&self) -> bool;

        /**
          set to YES to receive hosted (eg. not peer-to-peer) match results. Will cause the controller to return an array of players instead of a match.
        */
        #[method(setHosted:)]
        pub unsafe fn setHosted(&self, hosted: bool);

        /**
          this controls which mode of matchmaking to support in the UI (all, nearby only, automatch only, invite only).  Throws an exeption if you can not set to the desired mode (due to restrictions)
        */
        #[method(matchmakingMode)]
        pub unsafe fn matchmakingMode(&self) -> GKMatchmakingMode;

        /**
          this controls which mode of matchmaking to support in the UI (all, nearby only, automatch only, invite only).  Throws an exeption if you can not set to the desired mode (due to restrictions)
        */
        #[method(setMatchmakingMode:)]
        pub unsafe fn setMatchmakingMode(&self, matchmaking_mode: GKMatchmakingMode);

        /**
          A BOOL value to allow the GKMatchMakerViewController to return control to the game once the minimum number of players are connected.
         By default the value is NO, and the multiplayer match can only proceed after all players are connected.
         If the value is set to YES, then once the number of connected players is greater than or equal to minPlayers of the match request, matchmakerViewController:didFindMatch: will be called and the game can get the match instance, and update the game scene accordingly. The remaining players wil continue to connect.
        */
        #[method(canStartWithMinimumPlayers)]
        pub unsafe fn canStartWithMinimumPlayers(&self) -> bool;

        /**
          A BOOL value to allow the GKMatchMakerViewController to return control to the game once the minimum number of players are connected.
         By default the value is NO, and the multiplayer match can only proceed after all players are connected.
         If the value is set to YES, then once the number of connected players is greater than or equal to minPlayers of the match request, matchmakerViewController:didFindMatch: will be called and the game can get the match instance, and update the game scene accordingly. The remaining players wil continue to connect.
        */
        #[method(setCanStartWithMinimumPlayers:)]
        pub unsafe fn setCanStartWithMinimumPlayers(&self, can_start_with_minimum_players: bool);

        #[cfg(feature = "GameKit_GKMatchRequest")]
        #[method_id(@__retain_semantics Init initWithMatchRequest:)]
        pub unsafe fn initWithMatchRequest(
            this: Option<Allocated<Self>>,
            request: &GKMatchRequest,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "GameKit_GKInvite")]
        #[method_id(@__retain_semantics Init initWithInvite:)]
        pub unsafe fn initWithInvite(
            this: Option<Allocated<Self>>,
            invite: &GKInvite,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "GameKit_GKMatch")]
        #[method(addPlayersToMatch:)]
        pub unsafe fn addPlayersToMatch(&self, r#match: &GKMatch);

        #[cfg(feature = "GameKit_GKPlayer")]
        #[method(setHostedPlayer:didConnect:)]
        pub unsafe fn setHostedPlayer_didConnect(&self, player: &GKPlayer, connected: bool);

        #[cfg(feature = "Foundation_NSString")]
        /**
          default message to use when inviting friends. Can be edited by the user.
        */
        #[deprecated]
        #[method_id(@__retain_semantics Other defaultInvitationMessage)]
        pub unsafe fn defaultInvitationMessage(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        /**
          default message to use when inviting friends. Can be edited by the user.
        */
        #[deprecated]
        #[method(setDefaultInvitationMessage:)]
        pub unsafe fn setDefaultInvitationMessage(
            &self,
            default_invitation_message: Option<&NSString>,
        );
    }
);

extern_methods!(
    /// Obsoleted
    #[cfg(feature = "GameKit_GKMatchmakerViewController")]
    unsafe impl GKMatchmakerViewController {
        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "This is never invoked and its implementation does nothing, use setHostedPlayer:didConnect:"]
        #[method(setHostedPlayer:connected:)]
        pub unsafe fn setHostedPlayer_connected(&self, player_id: &NSString, connected: bool);
    }
);

extern_protocol!(
    pub unsafe trait GKMatchmakerViewControllerDelegate: NSObjectProtocol {
        #[cfg(feature = "GameKit_GKMatchmakerViewController")]
        #[method(matchmakerViewControllerWasCancelled:)]
        unsafe fn matchmakerViewControllerWasCancelled(
            &self,
            view_controller: &GKMatchmakerViewController,
        );

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "GameKit_GKMatchmakerViewController"
        ))]
        #[method(matchmakerViewController:didFailWithError:)]
        unsafe fn matchmakerViewController_didFailWithError(
            &self,
            view_controller: &GKMatchmakerViewController,
            error: &NSError,
        );

        #[cfg(all(
            feature = "GameKit_GKMatch",
            feature = "GameKit_GKMatchmakerViewController"
        ))]
        #[optional]
        #[method(matchmakerViewController:didFindMatch:)]
        unsafe fn matchmakerViewController_didFindMatch(
            &self,
            view_controller: &GKMatchmakerViewController,
            r#match: &GKMatch,
        );

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "GameKit_GKMatchmakerViewController",
            feature = "GameKit_GKPlayer"
        ))]
        #[optional]
        #[method(matchmakerViewController:didFindHostedPlayers:)]
        unsafe fn matchmakerViewController_didFindHostedPlayers(
            &self,
            view_controller: &GKMatchmakerViewController,
            players: &NSArray<GKPlayer>,
        );

        #[cfg(all(
            feature = "GameKit_GKMatchmakerViewController",
            feature = "GameKit_GKPlayer"
        ))]
        #[optional]
        #[method(matchmakerViewController:hostedPlayerDidAccept:)]
        unsafe fn matchmakerViewController_hostedPlayerDidAccept(
            &self,
            view_controller: &GKMatchmakerViewController,
            player: &GKPlayer,
        );

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSString",
            feature = "GameKit_GKMatchmakerViewController"
        ))]
        #[deprecated = "This is never invoked and its implementation does nothing, use matchmakerViewController:didFindHostedPlayers:"]
        #[optional]
        #[method(matchmakerViewController:didFindPlayers:)]
        unsafe fn matchmakerViewController_didFindPlayers(
            &self,
            view_controller: &GKMatchmakerViewController,
            player_i_ds: &NSArray<NSString>,
        );

        #[cfg(all(
            feature = "Foundation_NSString",
            feature = "GameKit_GKMatchmakerViewController"
        ))]
        #[deprecated = "This is never invoked and its implementation does nothing, use matchmakerViewController:hostedPlayerDidAccept:"]
        #[optional]
        #[method(matchmakerViewController:didReceiveAcceptFromHostedPlayer:)]
        unsafe fn matchmakerViewController_didReceiveAcceptFromHostedPlayer(
            &self,
            view_controller: &GKMatchmakerViewController,
            player_id: &NSString,
        );
    }

    unsafe impl ProtocolType for dyn GKMatchmakerViewControllerDelegate {}
);

extern_methods!(
    /// Methods declared on superclass `NSViewController`
    #[cfg(feature = "GameKit_GKMatchmakerViewController")]
    unsafe impl GKMatchmakerViewController {
        #[cfg(feature = "Foundation_NSBundle")]
        #[method_id(@__retain_semantics Init initWithNibName:bundle:)]
        pub unsafe fn initWithNibName_bundle(
            this: Option<Allocated<Self>>,
            nib_name_or_nil: Option<&NSNibName>,
            nib_bundle_or_nil: Option<&NSBundle>,
        ) -> Id<Self>;
    }
);
