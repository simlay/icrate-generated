//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameKit::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum GKGameCenterViewControllerState {
        GKGameCenterViewControllerStateDefault = -1,
        GKGameCenterViewControllerStateLeaderboards = 0,
        GKGameCenterViewControllerStateAchievements = 1,
        GKGameCenterViewControllerStateChallenges = 2,
        GKGameCenterViewControllerStateLocalPlayerProfile = 3,
        GKGameCenterViewControllerStateDashboard = 4,
        GKGameCenterViewControllerStateLocalPlayerFriendsList = 5,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameKit_GKGameCenterViewController")]
    pub struct GKGameCenterViewController;

    #[cfg(feature = "GameKit_GKGameCenterViewController")]
    unsafe impl ClassType for GKGameCenterViewController {
        #[inherits(NSResponder, NSObject)]
        type Super = NSViewController;
    }
);

#[cfg(feature = "GameKit_GKGameCenterViewController")]
unsafe impl GKViewController for GKGameCenterViewController {}

#[cfg(feature = "GameKit_GKGameCenterViewController")]
unsafe impl NSCoding for GKGameCenterViewController {}

#[cfg(feature = "GameKit_GKGameCenterViewController")]
unsafe impl NSEditor for GKGameCenterViewController {}

#[cfg(feature = "GameKit_GKGameCenterViewController")]
unsafe impl NSObjectProtocol for GKGameCenterViewController {}

#[cfg(feature = "GameKit_GKGameCenterViewController")]
unsafe impl NSSeguePerforming for GKGameCenterViewController {}

#[cfg(feature = "GameKit_GKGameCenterViewController")]
unsafe impl NSUserInterfaceItemIdentification for GKGameCenterViewController {}

extern_methods!(
    #[cfg(feature = "GameKit_GKGameCenterViewController")]
    unsafe impl GKGameCenterViewController {}
);

extern_methods!(
    #[cfg(feature = "GameKit_GKGameCenterViewController")]
    unsafe impl GKGameCenterViewController {
        #[method_id(@__retain_semantics Other gameCenterDelegate)]
        pub unsafe fn gameCenterDelegate(
            &self,
        ) -> Option<Id<ProtocolObject<dyn GKGameCenterControllerDelegate>>>;

        #[method(setGameCenterDelegate:)]
        pub unsafe fn setGameCenterDelegate(
            &self,
            game_center_delegate: Option<&ProtocolObject<dyn GKGameCenterControllerDelegate>>,
        );

        #[method_id(@__retain_semantics Init initWithState:)]
        pub unsafe fn initWithState(
            this: Option<Allocated<Self>>,
            state: GKGameCenterViewControllerState,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithLeaderboardID:playerScope:timeScope:)]
        pub unsafe fn initWithLeaderboardID_playerScope_timeScope(
            this: Option<Allocated<Self>>,
            leaderboard_id: &NSString,
            player_scope: GKLeaderboardPlayerScope,
            time_scope: GKLeaderboardTimeScope,
        ) -> Id<Self>;

        #[cfg(feature = "GameKit_GKLeaderboard")]
        #[method_id(@__retain_semantics Init initWithLeaderboard:playerScope:)]
        pub unsafe fn initWithLeaderboard_playerScope(
            this: Option<Allocated<Self>>,
            leaderboard: &GKLeaderboard,
            player_scope: GKLeaderboardPlayerScope,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithAchievementID:)]
        pub unsafe fn initWithAchievementID(
            this: Option<Allocated<Self>>,
            achievement_id: &NSString,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /// Deprecated
    #[cfg(feature = "GameKit_GKGameCenterViewController")]
    unsafe impl GKGameCenterViewController {
        #[deprecated = "Use -initWithState: instead"]
        #[method(viewState)]
        pub unsafe fn viewState(&self) -> GKGameCenterViewControllerState;

        #[deprecated = "Use -initWithState: instead"]
        #[method(setViewState:)]
        pub unsafe fn setViewState(&self, view_state: GKGameCenterViewControllerState);

        #[deprecated = "Use -initWithLeaderboard: instead"]
        #[method(leaderboardTimeScope)]
        pub unsafe fn leaderboardTimeScope(&self) -> GKLeaderboardTimeScope;

        #[deprecated = "Use -initWithLeaderboard: instead"]
        #[method(setLeaderboardTimeScope:)]
        pub unsafe fn setLeaderboardTimeScope(
            &self,
            leaderboard_time_scope: GKLeaderboardTimeScope,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Use -initWithLeaderboard: instead"]
        #[method_id(@__retain_semantics Other leaderboardIdentifier)]
        pub unsafe fn leaderboardIdentifier(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Use -initWithLeaderboard: instead"]
        #[method(setLeaderboardIdentifier:)]
        pub unsafe fn setLeaderboardIdentifier(&self, leaderboard_identifier: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "GKGameCenterViewController's leaderboardCategory property is deprecated. Use -initWithLeaderboard: instead"]
        #[method_id(@__retain_semantics Other leaderboardCategory)]
        pub unsafe fn leaderboardCategory(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "GKGameCenterViewController's leaderboardCategory property is deprecated. Use -initWithLeaderboard: instead"]
        #[method(setLeaderboardCategory:)]
        pub unsafe fn setLeaderboardCategory(&self, leaderboard_category: Option<&NSString>);
    }
);

extern_protocol!(
    pub unsafe trait GKGameCenterControllerDelegate: NSObjectProtocol {
        #[cfg(feature = "GameKit_GKGameCenterViewController")]
        #[method(gameCenterViewControllerDidFinish:)]
        unsafe fn gameCenterViewControllerDidFinish(
            &self,
            game_center_view_controller: &GKGameCenterViewController,
        );
    }

    unsafe impl ProtocolType for dyn GKGameCenterControllerDelegate {}
);

extern_methods!(
    /// Methods declared on superclass `NSViewController`
    #[cfg(feature = "GameKit_GKGameCenterViewController")]
    unsafe impl GKGameCenterViewController {
        #[cfg(feature = "Foundation_NSBundle")]
        #[method_id(@__retain_semantics Init initWithNibName:bundle:)]
        pub unsafe fn initWithNibName_bundle(
            this: Option<Allocated<Self>>,
            nib_name_or_nil: Option<&NSNibName>,
            nib_bundle_or_nil: Option<&NSBundle>,
        ) -> Id<Self>;
    }
);
