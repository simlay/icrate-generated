//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameKit::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum GKChallengeState {
        GKChallengeStateInvalid = 0,
        GKChallengeStatePending = 1,
        GKChallengeStateCompleted = 2,
        GKChallengeStateDeclined = 3,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameKit_GKChallenge")]
    #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
    pub struct GKChallenge;

    #[cfg(feature = "GameKit_GKChallenge")]
    unsafe impl ClassType for GKChallenge {
        type Super = NSObject;
    }
);

#[cfg(feature = "GameKit_GKChallenge")]
unsafe impl NSCoding for GKChallenge {}

#[cfg(feature = "GameKit_GKChallenge")]
unsafe impl NSObjectProtocol for GKChallenge {}

#[cfg(feature = "GameKit_GKChallenge")]
unsafe impl NSSecureCoding for GKChallenge {}

extern_methods!(
    #[cfg(feature = "GameKit_GKChallenge")]
    unsafe impl GKChallenge {
        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSError"))]
        #[method(loadReceivedChallengesWithCompletionHandler:)]
        pub unsafe fn loadReceivedChallengesWithCompletionHandler(
            completion_handler: Option<&Block<(*mut NSArray<GKChallenge>, *mut NSError), ()>>,
        );

        #[method(decline)]
        pub unsafe fn decline(&self);

        #[cfg(feature = "GameKit_GKPlayer")]
        #[method_id(@__retain_semantics Other issuingPlayer)]
        pub unsafe fn issuingPlayer(&self) -> Option<Id<GKPlayer>>;

        #[cfg(feature = "GameKit_GKPlayer")]
        #[method_id(@__retain_semantics Other receivingPlayer)]
        pub unsafe fn receivingPlayer(&self) -> Option<Id<GKPlayer>>;

        #[method(state)]
        pub unsafe fn state(&self) -> GKChallengeState;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other issueDate)]
        pub unsafe fn issueDate(&self) -> Id<NSDate>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other completionDate)]
        pub unsafe fn completionDate(&self) -> Option<Id<NSDate>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other message)]
        pub unsafe fn message(&self) -> Option<Id<NSString>>;
    }
);

extern_methods!(
    /// Obsoleted
    #[cfg(feature = "GameKit_GKChallenge")]
    unsafe impl GKChallenge {
        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = " This property is obsolete, Use issuingPlayer instead"]
        #[method_id(@__retain_semantics Other issuingPlayerID)]
        pub unsafe fn issuingPlayerID(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = " This property is obsolete, Use receivingPlayer instead"]
        #[method_id(@__retain_semantics Other receivingPlayerID)]
        pub unsafe fn receivingPlayerID(&self) -> Option<Id<NSString>>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameKit_GKScoreChallenge")]
    #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
    pub struct GKScoreChallenge;

    #[cfg(feature = "GameKit_GKScoreChallenge")]
    unsafe impl ClassType for GKScoreChallenge {
        #[inherits(NSObject)]
        type Super = GKChallenge;
    }
);

#[cfg(feature = "GameKit_GKScoreChallenge")]
unsafe impl NSCoding for GKScoreChallenge {}

#[cfg(feature = "GameKit_GKScoreChallenge")]
unsafe impl NSObjectProtocol for GKScoreChallenge {}

#[cfg(feature = "GameKit_GKScoreChallenge")]
unsafe impl NSSecureCoding for GKScoreChallenge {}

extern_methods!(
    #[cfg(feature = "GameKit_GKScoreChallenge")]
    unsafe impl GKScoreChallenge {
        #[cfg(feature = "GameKit_GKScore")]
        #[method_id(@__retain_semantics Other score)]
        pub unsafe fn score(&self) -> Option<Id<GKScore>>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameKit_GKAchievementChallenge")]
    #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
    pub struct GKAchievementChallenge;

    #[cfg(feature = "GameKit_GKAchievementChallenge")]
    unsafe impl ClassType for GKAchievementChallenge {
        #[inherits(NSObject)]
        type Super = GKChallenge;
    }
);

#[cfg(feature = "GameKit_GKAchievementChallenge")]
unsafe impl NSCoding for GKAchievementChallenge {}

#[cfg(feature = "GameKit_GKAchievementChallenge")]
unsafe impl NSObjectProtocol for GKAchievementChallenge {}

#[cfg(feature = "GameKit_GKAchievementChallenge")]
unsafe impl NSSecureCoding for GKAchievementChallenge {}

extern_methods!(
    #[cfg(feature = "GameKit_GKAchievementChallenge")]
    unsafe impl GKAchievementChallenge {
        #[cfg(feature = "GameKit_GKAchievement")]
        #[method_id(@__retain_semantics Other achievement)]
        pub unsafe fn achievement(&self) -> Option<Id<GKAchievement>>;
    }
);

extern_methods!(
    /// GKChallenge
    #[cfg(feature = "GameKit_GKScore")]
    unsafe impl GKScore {
        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "GameKit_GKChallenge"
        ))]
        #[deprecated = "pass GKLeaderboardScore to reportLeaderboardScores:withEligibleChallenges:withCompletionHandler instead"]
        #[method(reportScores:withEligibleChallenges:withCompletionHandler:)]
        pub unsafe fn reportScores_withEligibleChallenges_withCompletionHandler(
            scores: &NSArray<GKScore>,
            challenges: &NSArray<GKChallenge>,
            completion_handler: Option<&Block<(*mut NSError,), ()>>,
        );

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "GameKit_GKChallenge",
            feature = "GameKit_GKLeaderboardScore"
        ))]
        #[method(reportLeaderboardScores:withEligibleChallenges:withCompletionHandler:)]
        pub unsafe fn reportLeaderboardScores_withEligibleChallenges_withCompletionHandler(
            scores: &NSArray<GKLeaderboardScore>,
            challenges: &NSArray<GKChallenge>,
            completion_handler: Option<&Block<(*mut NSError,), ()>>,
        );
    }
);

extern_methods!(
    /// GKChallenge
    #[cfg(feature = "GameKit_GKAchievement")]
    unsafe impl GKAchievement {
        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "GameKit_GKPlayer"
        ))]
        #[method(selectChallengeablePlayers:withCompletionHandler:)]
        pub unsafe fn selectChallengeablePlayers_withCompletionHandler(
            &self,
            players: &NSArray<GKPlayer>,
            completion_handler: Option<&Block<(*mut NSArray<GKPlayer>, *mut NSError), ()>>,
        );

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "GameKit_GKChallenge"
        ))]
        #[method(reportAchievements:withEligibleChallenges:withCompletionHandler:)]
        pub unsafe fn reportAchievements_withEligibleChallenges_withCompletionHandler(
            achievements: &NSArray<GKAchievement>,
            challenges: &NSArray<GKChallenge>,
            completion_handler: Option<&Block<(*mut NSError,), ()>>,
        );
    }
);

extern_methods!(
    /// GKChallengeObsoleted
    #[cfg(feature = "GameKit_GKScore")]
    unsafe impl GKScore {
        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[deprecated = "This is never invoked and its implementation does nothing, pass GKPlayers to challengeComposeControllerWithMessage:players:completionHandler: and present the view controller instead"]
        #[method(issueChallengeToPlayers:message:)]
        pub unsafe fn issueChallengeToPlayers_message(
            &self,
            player_i_ds: Option<&NSArray<NSString>>,
            message: Option<&NSString>,
        );
    }
);

extern_methods!(
    /// GKChallengeObsoleted
    #[cfg(feature = "GameKit_GKAchievement")]
    unsafe impl GKAchievement {
        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[deprecated = "This is never invoked and its implementation does nothing, pass GKPlayers to challengeComposeControllerWithMessage:players:completionHandler: and present the view controller instead"]
        #[method(issueChallengeToPlayers:message:)]
        pub unsafe fn issueChallengeToPlayers_message(
            &self,
            player_i_ds: Option<&NSArray<NSString>>,
            message: Option<&NSString>,
        );

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString"
        ))]
        #[deprecated = "This is never invoked and its implementation does nothing, pass GKPlayers to selectChallengeablePlayers:"]
        #[method(selectChallengeablePlayerIDs:withCompletionHandler:)]
        pub unsafe fn selectChallengeablePlayerIDs_withCompletionHandler(
            &self,
            player_i_ds: Option<&NSArray<NSString>>,
            completion_handler: Option<&Block<(*mut NSArray<NSString>, *mut NSError), ()>>,
        );
    }
);

pub type GKChallengeComposeCompletionBlock =
    *mut Block<(NonNull<NSViewController>, Bool, *mut NSArray<NSString>), ()>;

extern_methods!(
    /// GKChallengeUI
    #[cfg(feature = "GameKit_GKScore")]
    unsafe impl GKScore {
        #[cfg(all(
            feature = "AppKit_NSViewController",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSString",
            feature = "GameKit_GKPlayer"
        ))]
        #[method_id(@__retain_semantics Other challengeComposeControllerWithMessage:players:completionHandler:)]
        pub unsafe fn challengeComposeControllerWithMessage_players_completionHandler(
            &self,
            message: Option<&NSString>,
            players: Option<&NSArray<GKPlayer>>,
            completion_handler: GKChallengeComposeCompletionBlock,
        ) -> Id<NSViewController>;
    }
);

extern_methods!(
    /// GKChallengeUI
    #[cfg(feature = "GameKit_GKLeaderboardEntry")]
    unsafe impl GKLeaderboardEntry {
        #[cfg(all(
            feature = "AppKit_NSViewController",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSString",
            feature = "GameKit_GKPlayer"
        ))]
        #[method_id(@__retain_semantics Other challengeComposeControllerWithMessage:players:completionHandler:)]
        pub unsafe fn challengeComposeControllerWithMessage_players_completionHandler(
            &self,
            message: Option<&NSString>,
            players: Option<&NSArray<GKPlayer>>,
            completion_handler: GKChallengeComposeCompletionBlock,
        ) -> Id<NSViewController>;
    }
);

extern_methods!(
    /// GKChallengeUI
    #[cfg(feature = "GameKit_GKAchievement")]
    unsafe impl GKAchievement {
        #[cfg(all(
            feature = "AppKit_NSViewController",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSString",
            feature = "GameKit_GKPlayer"
        ))]
        #[method_id(@__retain_semantics Other challengeComposeControllerWithMessage:players:completionHandler:)]
        pub unsafe fn challengeComposeControllerWithMessage_players_completionHandler(
            &self,
            message: Option<&NSString>,
            players: &NSArray<GKPlayer>,
            completion_handler: GKChallengeComposeCompletionBlock,
        ) -> Id<NSViewController>;
    }
);

extern_methods!(
    /// GKChallengeObsoletedUI
    #[cfg(feature = "GameKit_GKScore")]
    unsafe impl GKScore {}
);

extern_methods!(
    /// GKChallengeObsoletedUI
    #[cfg(feature = "GameKit_GKAchievement")]
    unsafe impl GKAchievement {}
);
