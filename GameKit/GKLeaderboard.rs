//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameKit::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum GKLeaderboardTimeScope {
        GKLeaderboardTimeScopeToday = 0,
        GKLeaderboardTimeScopeWeek = 1,
        GKLeaderboardTimeScopeAllTime = 2,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum GKLeaderboardPlayerScope {
        GKLeaderboardPlayerScopeGlobal = 0,
        GKLeaderboardPlayerScopeFriendsOnly = 1,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum GKLeaderboardType {
        GKLeaderboardTypeClassic = 0,
        GKLeaderboardTypeRecurring = 1,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameKit_GKLeaderboard")]
    pub struct GKLeaderboard;

    #[cfg(feature = "GameKit_GKLeaderboard")]
    unsafe impl ClassType for GKLeaderboard {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "GameKit_GKLeaderboard")]
unsafe impl NSObjectProtocol for GKLeaderboard {}

extern_methods!(
    #[cfg(feature = "GameKit_GKLeaderboard")]
    unsafe impl GKLeaderboard {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other groupIdentifier)]
        pub unsafe fn groupIdentifier(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other baseLeaderboardID)]
        pub unsafe fn baseLeaderboardID(&self) -> Id<NSString>;

        #[method(type)]
        pub unsafe fn r#type(&self) -> GKLeaderboardType;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other startDate)]
        pub unsafe fn startDate(&self) -> Option<Id<NSDate>>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other nextStartDate)]
        pub unsafe fn nextStartDate(&self) -> Option<Id<NSDate>>;

        #[method(duration)]
        pub unsafe fn duration(&self) -> NSTimeInterval;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString"
        ))]
        #[method(loadLeaderboardsWithIDs:completionHandler:)]
        pub unsafe fn loadLeaderboardsWithIDs_completionHandler(
            leaderboard_i_ds: Option<&NSArray<NSString>>,
            completion_handler: &Block<(*mut NSArray<GKLeaderboard>, *mut NSError), ()>,
        );

        #[cfg(feature = "Foundation_NSError")]
        #[method(loadPreviousOccurrenceWithCompletionHandler:)]
        pub unsafe fn loadPreviousOccurrenceWithCompletionHandler(
            &self,
            completion_handler: &Block<(*mut GKLeaderboard, *mut NSError), ()>,
        );

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "GameKit_GKPlayer"
        ))]
        #[method(submitScore:context:player:leaderboardIDs:completionHandler:)]
        pub unsafe fn submitScore_context_player_leaderboardIDs_completionHandler(
            score: NSInteger,
            context: NSUInteger,
            player: &GKPlayer,
            leaderboard_i_ds: &NSArray<NSString>,
            completion_handler: &Block<(*mut NSError,), ()>,
        );

        #[cfg(all(feature = "Foundation_NSError", feature = "GameKit_GKPlayer"))]
        #[method(submitScore:context:player:completionHandler:)]
        pub unsafe fn submitScore_context_player_completionHandler(
            &self,
            score: NSInteger,
            context: NSUInteger,
            player: &GKPlayer,
            completion_handler: &Block<(*mut NSError,), ()>,
        );

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "GameKit_GKLeaderboardEntry"
        ))]
        #[method(loadEntriesForPlayerScope:timeScope:range:completionHandler:)]
        pub unsafe fn loadEntriesForPlayerScope_timeScope_range_completionHandler(
            &self,
            player_scope: GKLeaderboardPlayerScope,
            time_scope: GKLeaderboardTimeScope,
            range: NSRange,
            completion_handler: &Block<
                (
                    *mut GKLeaderboardEntry,
                    *mut NSArray<GKLeaderboardEntry>,
                    NSInteger,
                    *mut NSError,
                ),
                (),
            >,
        );

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "GameKit_GKLeaderboardEntry",
            feature = "GameKit_GKPlayer"
        ))]
        #[method(loadEntriesForPlayers:timeScope:completionHandler:)]
        pub unsafe fn loadEntriesForPlayers_timeScope_completionHandler(
            &self,
            players: &NSArray<GKPlayer>,
            time_scope: GKLeaderboardTimeScope,
            completion_handler: &Block<
                (
                    *mut GKLeaderboardEntry,
                    *mut NSArray<GKLeaderboardEntry>,
                    *mut NSError,
                ),
                (),
            >,
        );
    }
);

extern_methods!(
    /// Deprecated
    #[cfg(feature = "GameKit_GKLeaderboard")]
    unsafe impl GKLeaderboard {
        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Use identifier instead"]
        #[method_id(@__retain_semantics Other category)]
        pub unsafe fn category(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Use identifier instead"]
        #[method(setCategory:)]
        pub unsafe fn setCategory(&self, category: Option<&NSString>);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[deprecated = "Use initWithPlayers: instead"]
        #[method_id(@__retain_semantics Init initWithPlayerIDs:)]
        pub unsafe fn initWithPlayerIDs(
            this: Option<Allocated<Self>>,
            player_i_ds: Option<&NSArray<NSString>>,
        ) -> Option<Id<Self>>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString"
        ))]
        #[deprecated = "Use loadLeaderboardsWithIDs:completionHandler: instead"]
        #[method(loadCategoriesWithCompletionHandler:)]
        pub unsafe fn loadCategoriesWithCompletionHandler(
            completion_handler: Option<
                &Block<(*mut NSArray<NSString>, *mut NSArray<NSString>, *mut NSError), ()>,
            >,
        );

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[deprecated = "Use setDefaultLeaderboardIdentifier:completionHandler: on GKLocalPlayer instead"]
        #[method(setDefaultLeaderboard:withCompletionHandler:)]
        pub unsafe fn setDefaultLeaderboard_withCompletionHandler(
            leaderboard_identifier: Option<&NSString>,
            completion_handler: Option<&Block<(*mut NSError,), ()>>,
        );

        #[deprecated = "Use loadEntriesForPlayerScope:timeScope:range:completionHandler: instead."]
        #[method(timeScope)]
        pub unsafe fn timeScope(&self) -> GKLeaderboardTimeScope;

        #[deprecated = "Use loadEntriesForPlayerScope:timeScope:range:completionHandler: instead."]
        #[method(setTimeScope:)]
        pub unsafe fn setTimeScope(&self, time_scope: GKLeaderboardTimeScope);

        #[deprecated = "Use loadEntriesForPlayerScope:timeScope:range:completionHandler: instead."]
        #[method(playerScope)]
        pub unsafe fn playerScope(&self) -> GKLeaderboardPlayerScope;

        #[deprecated = "Use loadEntriesForPlayerScope:timeScope:range:completionHandler: instead."]
        #[method(setPlayerScope:)]
        pub unsafe fn setPlayerScope(&self, player_scope: GKLeaderboardPlayerScope);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Use loadEntriesForPlayerScope:timeScope:range:completionHandler: instead."]
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Use loadEntriesForPlayerScope:timeScope:range:completionHandler: instead."]
        #[method(setIdentifier:)]
        pub unsafe fn setIdentifier(&self, identifier: Option<&NSString>);

        #[deprecated = "Use loadEntriesForPlayerScope:timeScope:range:completionHandler: instead."]
        #[method(range)]
        pub unsafe fn range(&self) -> NSRange;

        #[deprecated = "Use loadEntriesForPlayerScope:timeScope:range:completionHandler: instead."]
        #[method(setRange:)]
        pub unsafe fn setRange(&self, range: NSRange);

        #[cfg(all(feature = "Foundation_NSArray", feature = "GameKit_GKScore"))]
        #[deprecated = "Use loadEntriesForPlayerScope:timeScope:range:completionHandler: to obtain scores."]
        #[method_id(@__retain_semantics Other scores)]
        pub unsafe fn scores(&self) -> Option<Id<NSArray<GKScore>>>;

        #[deprecated = "Use loadEntriesForPlayerScope:timeScope:range:completionHandler: method to obtain the size of the leaderboard."]
        #[method(maxRange)]
        pub unsafe fn maxRange(&self) -> NSUInteger;

        #[cfg(feature = "GameKit_GKScore")]
        #[deprecated = "Use loadEntriesForPlayerScope:timeScope:range:completionHandler: method to obtain scores."]
        #[method_id(@__retain_semantics Other localPlayerScore)]
        pub unsafe fn localPlayerScore(&self) -> Option<Id<GKScore>>;

        #[deprecated = "Use loadEntriesForPlayerScope:timeScope:range:completionHandler: method to obtain scores."]
        #[method(isLoading)]
        pub unsafe fn isLoading(&self) -> bool;

        #[cfg(all(feature = "Foundation_NSArray", feature = "GameKit_GKPlayer"))]
        #[deprecated = "Use instance method loadEntriesForPlayers:timeScope:completionHandler: instead."]
        #[method_id(@__retain_semantics Init initWithPlayers:)]
        pub unsafe fn initWithPlayers(
            this: Option<Allocated<Self>>,
            players: &NSArray<GKPlayer>,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "GameKit_GKScore"
        ))]
        #[deprecated = "Use loadEntriesForPlayerScope:timeScope:range:completionHandler:."]
        #[method(loadScoresWithCompletionHandler:)]
        pub unsafe fn loadScoresWithCompletionHandler(
            &self,
            completion_handler: Option<&Block<(*mut NSArray<GKScore>, *mut NSError), ()>>,
        );

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSError"))]
        #[deprecated = "Use class method loadLeaderboardsWithIDs:completionHandler:"]
        #[method(loadLeaderboardsWithCompletionHandler:)]
        pub unsafe fn loadLeaderboardsWithCompletionHandler(
            completion_handler: Option<&Block<(*mut NSArray<GKLeaderboard>, *mut NSError), ()>>,
        );
    }
);

extern_methods!(
    /// UI
    #[cfg(feature = "GameKit_GKLeaderboard")]
    unsafe impl GKLeaderboard {
        #[cfg(all(feature = "AppKit_NSImage", feature = "Foundation_NSError"))]
        #[method(loadImageWithCompletionHandler:)]
        pub unsafe fn loadImageWithCompletionHandler(
            &self,
            completion_handler: Option<&Block<(*mut NSImage, *mut NSError), ()>>,
        );
    }
);
