//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameKit::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum GKTurnBasedMatchStatus {
        GKTurnBasedMatchStatusUnknown = 0,
        GKTurnBasedMatchStatusOpen = 1,
        GKTurnBasedMatchStatusEnded = 2,
        GKTurnBasedMatchStatusMatching = 3,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum GKTurnBasedParticipantStatus {
        GKTurnBasedParticipantStatusUnknown = 0,
        GKTurnBasedParticipantStatusInvited = 1,
        GKTurnBasedParticipantStatusDeclined = 2,
        GKTurnBasedParticipantStatusMatching = 3,
        GKTurnBasedParticipantStatusActive = 4,
        GKTurnBasedParticipantStatusDone = 5,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum GKTurnBasedMatchOutcome {
        GKTurnBasedMatchOutcomeNone = 0,
        GKTurnBasedMatchOutcomeQuit = 1,
        GKTurnBasedMatchOutcomeWon = 2,
        GKTurnBasedMatchOutcomeLost = 3,
        GKTurnBasedMatchOutcomeTied = 4,
        GKTurnBasedMatchOutcomeTimeExpired = 5,
        GKTurnBasedMatchOutcomeFirst = 6,
        GKTurnBasedMatchOutcomeSecond = 7,
        GKTurnBasedMatchOutcomeThird = 8,
        GKTurnBasedMatchOutcomeFourth = 9,
        GKTurnBasedMatchOutcomeCustomRange = 0x00FF0000,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameKit_GKTurnBasedParticipant")]
    pub struct GKTurnBasedParticipant;

    #[cfg(feature = "GameKit_GKTurnBasedParticipant")]
    unsafe impl ClassType for GKTurnBasedParticipant {
        type Super = NSObject;
    }
);

#[cfg(feature = "GameKit_GKTurnBasedParticipant")]
unsafe impl NSObjectProtocol for GKTurnBasedParticipant {}

extern_methods!(
    #[cfg(feature = "GameKit_GKTurnBasedParticipant")]
    unsafe impl GKTurnBasedParticipant {
        #[cfg(feature = "GameKit_GKPlayer")]
        #[method_id(@__retain_semantics Other player)]
        pub unsafe fn player(&self) -> Option<Id<GKPlayer>>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other lastTurnDate)]
        pub unsafe fn lastTurnDate(&self) -> Option<Id<NSDate>>;

        #[method(status)]
        pub unsafe fn status(&self) -> GKTurnBasedParticipantStatus;

        #[method(matchOutcome)]
        pub unsafe fn matchOutcome(&self) -> GKTurnBasedMatchOutcome;

        #[method(setMatchOutcome:)]
        pub unsafe fn setMatchOutcome(&self, match_outcome: GKTurnBasedMatchOutcome);

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other timeoutDate)]
        pub unsafe fn timeoutDate(&self) -> Option<Id<NSDate>>;
    }
);

extern_methods!(
    /// Obsoleted
    #[cfg(feature = "GameKit_GKTurnBasedParticipant")]
    unsafe impl GKTurnBasedParticipant {
        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "use player"]
        #[method_id(@__retain_semantics Other playerID)]
        pub unsafe fn playerID(&self) -> Option<Id<NSString>>;
    }
);

extern_protocol!(
    pub unsafe trait GKTurnBasedEventListener {
        #[cfg(all(feature = "Foundation_NSArray", feature = "GameKit_GKPlayer"))]
        #[cfg(not(any(target_os = "watchos")))]
        #[optional]
        #[method(player:didRequestMatchWithOtherPlayers:)]
        unsafe fn player_didRequestMatchWithOtherPlayers(
            &self,
            player: &GKPlayer,
            players_to_invite: &NSArray<GKPlayer>,
        );

        #[cfg(all(feature = "GameKit_GKPlayer", feature = "GameKit_GKTurnBasedMatch"))]
        #[optional]
        #[method(player:receivedTurnEventForMatch:didBecomeActive:)]
        unsafe fn player_receivedTurnEventForMatch_didBecomeActive(
            &self,
            player: &GKPlayer,
            r#match: &GKTurnBasedMatch,
            did_become_active: bool,
        );

        #[cfg(all(feature = "GameKit_GKPlayer", feature = "GameKit_GKTurnBasedMatch"))]
        #[optional]
        #[method(player:matchEnded:)]
        unsafe fn player_matchEnded(&self, player: &GKPlayer, r#match: &GKTurnBasedMatch);

        #[cfg(all(
            feature = "GameKit_GKPlayer",
            feature = "GameKit_GKTurnBasedExchange",
            feature = "GameKit_GKTurnBasedMatch"
        ))]
        #[optional]
        #[method(player:receivedExchangeRequest:forMatch:)]
        unsafe fn player_receivedExchangeRequest_forMatch(
            &self,
            player: &GKPlayer,
            exchange: &GKTurnBasedExchange,
            r#match: &GKTurnBasedMatch,
        );

        #[cfg(all(
            feature = "GameKit_GKPlayer",
            feature = "GameKit_GKTurnBasedExchange",
            feature = "GameKit_GKTurnBasedMatch"
        ))]
        #[optional]
        #[method(player:receivedExchangeCancellation:forMatch:)]
        unsafe fn player_receivedExchangeCancellation_forMatch(
            &self,
            player: &GKPlayer,
            exchange: &GKTurnBasedExchange,
            r#match: &GKTurnBasedMatch,
        );

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "GameKit_GKPlayer",
            feature = "GameKit_GKTurnBasedExchange",
            feature = "GameKit_GKTurnBasedExchangeReply",
            feature = "GameKit_GKTurnBasedMatch"
        ))]
        #[optional]
        #[method(player:receivedExchangeReplies:forCompletedExchange:forMatch:)]
        unsafe fn player_receivedExchangeReplies_forCompletedExchange_forMatch(
            &self,
            player: &GKPlayer,
            replies: &NSArray<GKTurnBasedExchangeReply>,
            exchange: &GKTurnBasedExchange,
            r#match: &GKTurnBasedMatch,
        );

        #[cfg(all(feature = "GameKit_GKPlayer", feature = "GameKit_GKTurnBasedMatch"))]
        #[optional]
        #[method(player:wantsToQuitMatch:)]
        unsafe fn player_wantsToQuitMatch(&self, player: &GKPlayer, r#match: &GKTurnBasedMatch);

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSString",
            feature = "GameKit_GKPlayer"
        ))]
        #[cfg(not(any(target_os = "macos")))]
        #[optional]
        #[method(player:didRequestMatchWithPlayers:)]
        unsafe fn player_didRequestMatchWithPlayers(
            &self,
            player: &GKPlayer,
            player_i_ds_to_invite: &NSArray<NSString>,
        );
    }

    unsafe impl ProtocolType for dyn GKTurnBasedEventListener {}
);

extern_static!(GKTurnTimeoutDefault: NSTimeInterval);

extern_static!(GKTurnTimeoutNone: NSTimeInterval);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameKit_GKTurnBasedMatch")]
    pub struct GKTurnBasedMatch;

    #[cfg(feature = "GameKit_GKTurnBasedMatch")]
    unsafe impl ClassType for GKTurnBasedMatch {
        type Super = NSObject;
    }
);

#[cfg(feature = "GameKit_GKTurnBasedMatch")]
unsafe impl NSObjectProtocol for GKTurnBasedMatch {}

extern_methods!(
    #[cfg(feature = "GameKit_GKTurnBasedMatch")]
    unsafe impl GKTurnBasedMatch {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other matchID)]
        pub unsafe fn matchID(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other creationDate)]
        pub unsafe fn creationDate(&self) -> Option<Id<NSDate>>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "GameKit_GKTurnBasedParticipant"
        ))]
        #[method_id(@__retain_semantics Other participants)]
        pub unsafe fn participants(&self) -> Option<Id<NSArray<GKTurnBasedParticipant>>>;

        #[method(status)]
        pub unsafe fn status(&self) -> GKTurnBasedMatchStatus;

        #[cfg(feature = "GameKit_GKTurnBasedParticipant")]
        #[method_id(@__retain_semantics Other currentParticipant)]
        pub unsafe fn currentParticipant(&self) -> Option<Id<GKTurnBasedParticipant>>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other matchData)]
        pub unsafe fn matchData(&self) -> Option<Id<NSData>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method(setLocalizableMessageWithKey:arguments:)]
        pub unsafe fn setLocalizableMessageWithKey_arguments(
            &self,
            key: &NSString,
            arguments: Option<&NSArray<NSString>>,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other message)]
        pub unsafe fn message(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setMessage:)]
        pub unsafe fn setMessage(&self, message: Option<&NSString>);

        #[method(matchDataMaximumSize)]
        pub unsafe fn matchDataMaximumSize(&self) -> NSUInteger;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "GameKit_GKTurnBasedExchange"
        ))]
        #[method_id(@__retain_semantics Other exchanges)]
        pub unsafe fn exchanges(&self) -> Option<Id<NSArray<GKTurnBasedExchange>>>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "GameKit_GKTurnBasedExchange"
        ))]
        #[method_id(@__retain_semantics Other activeExchanges)]
        pub unsafe fn activeExchanges(&self) -> Option<Id<NSArray<GKTurnBasedExchange>>>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "GameKit_GKTurnBasedExchange"
        ))]
        #[method_id(@__retain_semantics Other completedExchanges)]
        pub unsafe fn completedExchanges(&self) -> Option<Id<NSArray<GKTurnBasedExchange>>>;

        #[method(exchangeDataMaximumSize)]
        pub unsafe fn exchangeDataMaximumSize(&self) -> NSUInteger;

        #[method(exchangeMaxInitiatedExchangesPerPlayer)]
        pub unsafe fn exchangeMaxInitiatedExchangesPerPlayer(&self) -> NSUInteger;

        #[cfg(all(feature = "Foundation_NSError", feature = "GameKit_GKMatchRequest"))]
        #[method(findMatchForRequest:withCompletionHandler:)]
        pub unsafe fn findMatchForRequest_withCompletionHandler(
            request: &GKMatchRequest,
            completion_handler: &Block<(*mut GKTurnBasedMatch, *mut NSError), ()>,
        );

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSError"))]
        #[method(loadMatchesWithCompletionHandler:)]
        pub unsafe fn loadMatchesWithCompletionHandler(
            completion_handler: Option<&Block<(*mut NSArray<GKTurnBasedMatch>, *mut NSError), ()>>,
        );

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[method(loadMatchWithID:withCompletionHandler:)]
        pub unsafe fn loadMatchWithID_withCompletionHandler(
            match_id: &NSString,
            completion_handler: Option<&Block<(*mut GKTurnBasedMatch, *mut NSError), ()>>,
        );

        #[cfg(feature = "Foundation_NSError")]
        #[method(rematchWithCompletionHandler:)]
        pub unsafe fn rematchWithCompletionHandler(
            &self,
            completion_handler: Option<&Block<(*mut GKTurnBasedMatch, *mut NSError), ()>>,
        );

        #[cfg(feature = "Foundation_NSError")]
        #[method(acceptInviteWithCompletionHandler:)]
        pub unsafe fn acceptInviteWithCompletionHandler(
            &self,
            completion_handler: Option<&Block<(*mut GKTurnBasedMatch, *mut NSError), ()>>,
        );

        #[cfg(feature = "Foundation_NSError")]
        #[method(declineInviteWithCompletionHandler:)]
        pub unsafe fn declineInviteWithCompletionHandler(
            &self,
            completion_handler: Option<&Block<(*mut NSError,), ()>>,
        );

        #[cfg(feature = "Foundation_NSError")]
        #[method(removeWithCompletionHandler:)]
        pub unsafe fn removeWithCompletionHandler(
            &self,
            completion_handler: Option<&Block<(*mut NSError,), ()>>,
        );

        #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSError"))]
        #[method(loadMatchDataWithCompletionHandler:)]
        pub unsafe fn loadMatchDataWithCompletionHandler(
            &self,
            completion_handler: Option<&Block<(*mut NSData, *mut NSError), ()>>,
        );

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSData",
            feature = "Foundation_NSError",
            feature = "GameKit_GKTurnBasedParticipant"
        ))]
        #[method(endTurnWithNextParticipants:turnTimeout:matchData:completionHandler:)]
        pub unsafe fn endTurnWithNextParticipants_turnTimeout_matchData_completionHandler(
            &self,
            next_participants: &NSArray<GKTurnBasedParticipant>,
            timeout: NSTimeInterval,
            match_data: &NSData,
            completion_handler: Option<&Block<(*mut NSError,), ()>>,
        );

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSData",
            feature = "Foundation_NSError",
            feature = "GameKit_GKTurnBasedParticipant"
        ))]
        #[method(participantQuitInTurnWithOutcome:nextParticipants:turnTimeout:matchData:completionHandler:)]
        pub unsafe fn participantQuitInTurnWithOutcome_nextParticipants_turnTimeout_matchData_completionHandler(
            &self,
            match_outcome: GKTurnBasedMatchOutcome,
            next_participants: &NSArray<GKTurnBasedParticipant>,
            timeout: NSTimeInterval,
            match_data: &NSData,
            completion_handler: Option<&Block<(*mut NSError,), ()>>,
        );

        #[cfg(feature = "Foundation_NSError")]
        #[method(participantQuitOutOfTurnWithOutcome:withCompletionHandler:)]
        pub unsafe fn participantQuitOutOfTurnWithOutcome_withCompletionHandler(
            &self,
            match_outcome: GKTurnBasedMatchOutcome,
            completion_handler: Option<&Block<(*mut NSError,), ()>>,
        );

        #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSError"))]
        #[method(endMatchInTurnWithMatchData:completionHandler:)]
        pub unsafe fn endMatchInTurnWithMatchData_completionHandler(
            &self,
            match_data: &NSData,
            completion_handler: Option<&Block<(*mut NSError,), ()>>,
        );

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSData",
            feature = "Foundation_NSError",
            feature = "GameKit_GKAchievement",
            feature = "GameKit_GKScore"
        ))]
        #[deprecated = "pass GKLeaderboardScore to endMatchInTurnWithMatchData:scores:completionHandler instead"]
        #[method(endMatchInTurnWithMatchData:scores:achievements:completionHandler:)]
        pub unsafe fn endMatchInTurnWithMatchData_scores_achievements_completionHandler(
            &self,
            match_data: &NSData,
            scores: Option<&NSArray<GKScore>>,
            achievements: Option<&NSArray<GKAchievement>>,
            completion_handler: Option<&Block<(*mut NSError,), ()>>,
        );

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSData",
            feature = "Foundation_NSError",
            feature = "GameKit_GKLeaderboardScore"
        ))]
        #[method(endMatchInTurnWithMatchData:leaderboardScores:achievements:completionHandler:)]
        pub unsafe fn endMatchInTurnWithMatchData_leaderboardScores_achievements_completionHandler(
            &self,
            match_data: &NSData,
            scores: &NSArray<GKLeaderboardScore>,
            achievements: &NSArray,
            completion_handler: &Block<(*mut NSError,), ()>,
        );

        #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSError"))]
        #[method(saveCurrentTurnWithMatchData:completionHandler:)]
        pub unsafe fn saveCurrentTurnWithMatchData_completionHandler(
            &self,
            match_data: &NSData,
            completion_handler: Option<&Block<(*mut NSError,), ()>>,
        );

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSData",
            feature = "Foundation_NSError",
            feature = "GameKit_GKTurnBasedExchange"
        ))]
        #[method(saveMergedMatchData:withResolvedExchanges:completionHandler:)]
        pub unsafe fn saveMergedMatchData_withResolvedExchanges_completionHandler(
            &self,
            match_data: &NSData,
            exchanges: &NSArray<GKTurnBasedExchange>,
            completion_handler: Option<&Block<(*mut NSError,), ()>>,
        );

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSData",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "GameKit_GKTurnBasedExchange",
            feature = "GameKit_GKTurnBasedParticipant"
        ))]
        #[method(sendExchangeToParticipants:data:localizableMessageKey:arguments:timeout:completionHandler:)]
        pub unsafe fn sendExchangeToParticipants_data_localizableMessageKey_arguments_timeout_completionHandler(
            &self,
            participants: &NSArray<GKTurnBasedParticipant>,
            data: &NSData,
            key: &NSString,
            arguments: &NSArray<NSString>,
            timeout: NSTimeInterval,
            completion_handler: Option<&Block<(*mut GKTurnBasedExchange, *mut NSError), ()>>,
        );

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "GameKit_GKTurnBasedParticipant"
        ))]
        #[method(sendReminderToParticipants:localizableMessageKey:arguments:completionHandler:)]
        pub unsafe fn sendReminderToParticipants_localizableMessageKey_arguments_completionHandler(
            &self,
            participants: &NSArray<GKTurnBasedParticipant>,
            key: &NSString,
            arguments: &NSArray<NSString>,
            completion_handler: Option<&Block<(*mut NSError,), ()>>,
        );

        #[cfg(all(
            feature = "Foundation_NSData",
            feature = "Foundation_NSError",
            feature = "GameKit_GKTurnBasedParticipant"
        ))]
        #[deprecated = "Use endTurnWithNextParticipants:... instead"]
        #[method(endTurnWithNextParticipant:matchData:completionHandler:)]
        pub unsafe fn endTurnWithNextParticipant_matchData_completionHandler(
            &self,
            next_participant: &GKTurnBasedParticipant,
            match_data: &NSData,
            completion_handler: Option<&Block<(*mut NSError,), ()>>,
        );

        #[cfg(all(
            feature = "Foundation_NSData",
            feature = "Foundation_NSError",
            feature = "GameKit_GKTurnBasedParticipant"
        ))]
        #[deprecated = "Use participantQuitInTurnWithOutcome:nextParticipants:turnTimeout:... instead"]
        #[method(participantQuitInTurnWithOutcome:nextParticipant:matchData:completionHandler:)]
        pub unsafe fn participantQuitInTurnWithOutcome_nextParticipant_matchData_completionHandler(
            &self,
            match_outcome: GKTurnBasedMatchOutcome,
            next_participant: &GKTurnBasedParticipant,
            match_data: &NSData,
            completion_handler: Option<&Block<(*mut NSError,), ()>>,
        );
    }
);

ns_enum!(
    #[underlying(i8)]
    pub enum GKTurnBasedExchangeStatus {
        GKTurnBasedExchangeStatusUnknown = 0,
        GKTurnBasedExchangeStatusActive = 1,
        GKTurnBasedExchangeStatusComplete = 2,
        GKTurnBasedExchangeStatusResolved = 3,
        GKTurnBasedExchangeStatusCanceled = 4,
    }
);

extern_static!(GKExchangeTimeoutDefault: NSTimeInterval);

extern_static!(GKExchangeTimeoutNone: NSTimeInterval);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameKit_GKTurnBasedExchange")]
    pub struct GKTurnBasedExchange;

    #[cfg(feature = "GameKit_GKTurnBasedExchange")]
    unsafe impl ClassType for GKTurnBasedExchange {
        type Super = NSObject;
    }
);

#[cfg(feature = "GameKit_GKTurnBasedExchange")]
unsafe impl NSObjectProtocol for GKTurnBasedExchange {}

extern_methods!(
    #[cfg(feature = "GameKit_GKTurnBasedExchange")]
    unsafe impl GKTurnBasedExchange {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other exchangeID)]
        pub unsafe fn exchangeID(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "GameKit_GKTurnBasedParticipant")]
        #[method_id(@__retain_semantics Other sender)]
        pub unsafe fn sender(&self) -> Option<Id<GKTurnBasedParticipant>>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "GameKit_GKTurnBasedParticipant"
        ))]
        #[method_id(@__retain_semantics Other recipients)]
        pub unsafe fn recipients(&self) -> Option<Id<NSArray<GKTurnBasedParticipant>>>;

        #[method(status)]
        pub unsafe fn status(&self) -> GKTurnBasedExchangeStatus;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other message)]
        pub unsafe fn message(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other data)]
        pub unsafe fn data(&self) -> Option<Id<NSData>>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other sendDate)]
        pub unsafe fn sendDate(&self) -> Option<Id<NSDate>>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other timeoutDate)]
        pub unsafe fn timeoutDate(&self) -> Option<Id<NSDate>>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other completionDate)]
        pub unsafe fn completionDate(&self) -> Option<Id<NSDate>>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "GameKit_GKTurnBasedExchangeReply"
        ))]
        #[method_id(@__retain_semantics Other replies)]
        pub unsafe fn replies(&self) -> Option<Id<NSArray<GKTurnBasedExchangeReply>>>;

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString"
        ))]
        #[method(cancelWithLocalizableMessageKey:arguments:completionHandler:)]
        pub unsafe fn cancelWithLocalizableMessageKey_arguments_completionHandler(
            &self,
            key: &NSString,
            arguments: &NSArray<NSString>,
            completion_handler: Option<&Block<(*mut NSError,), ()>>,
        );

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSData",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString"
        ))]
        #[method(replyWithLocalizableMessageKey:arguments:data:completionHandler:)]
        pub unsafe fn replyWithLocalizableMessageKey_arguments_data_completionHandler(
            &self,
            key: &NSString,
            arguments: &NSArray<NSString>,
            data: &NSData,
            completion_handler: Option<&Block<(*mut NSError,), ()>>,
        );
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameKit_GKTurnBasedExchangeReply")]
    pub struct GKTurnBasedExchangeReply;

    #[cfg(feature = "GameKit_GKTurnBasedExchangeReply")]
    unsafe impl ClassType for GKTurnBasedExchangeReply {
        type Super = NSObject;
    }
);

#[cfg(feature = "GameKit_GKTurnBasedExchangeReply")]
unsafe impl NSObjectProtocol for GKTurnBasedExchangeReply {}

extern_methods!(
    #[cfg(feature = "GameKit_GKTurnBasedExchangeReply")]
    unsafe impl GKTurnBasedExchangeReply {
        #[cfg(feature = "GameKit_GKTurnBasedParticipant")]
        #[method_id(@__retain_semantics Other recipient)]
        pub unsafe fn recipient(&self) -> Option<Id<GKTurnBasedParticipant>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other message)]
        pub unsafe fn message(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other data)]
        pub unsafe fn data(&self) -> Option<Id<NSData>>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method_id(@__retain_semantics Other replyDate)]
        pub unsafe fn replyDate(&self) -> Option<Id<NSDate>>;
    }
);

extern_protocol!(
    #[deprecated = "Use registerListener on GKLocalPlayer with an object that implements the GKTurnBasedEventListener protocol"]
    pub unsafe trait GKTurnBasedEventHandlerDelegate {
        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[deprecated]
        #[method(handleInviteFromGameCenter:)]
        unsafe fn handleInviteFromGameCenter(&self, players_to_invite: &NSArray<NSString>);

        #[cfg(feature = "GameKit_GKTurnBasedMatch")]
        #[deprecated]
        #[method(handleTurnEventForMatch:didBecomeActive:)]
        unsafe fn handleTurnEventForMatch_didBecomeActive(
            &self,
            r#match: &GKTurnBasedMatch,
            did_become_active: bool,
        );

        #[cfg(feature = "GameKit_GKTurnBasedMatch")]
        #[deprecated]
        #[optional]
        #[method(handleTurnEventForMatch:)]
        unsafe fn handleTurnEventForMatch(&self, r#match: &GKTurnBasedMatch);

        #[cfg(feature = "GameKit_GKTurnBasedMatch")]
        #[deprecated]
        #[optional]
        #[method(handleMatchEnded:)]
        unsafe fn handleMatchEnded(&self, r#match: &GKTurnBasedMatch);
    }

    unsafe impl ProtocolType for dyn GKTurnBasedEventHandlerDelegate {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameKit_GKTurnBasedEventHandler")]
    #[deprecated = "Use registerListener on GKLocalPlayer with an object that implements the GKTurnBasedEventListener protocol"]
    pub struct GKTurnBasedEventHandler;

    #[deprecated = "Use registerListener on GKLocalPlayer with an object that implements the GKTurnBasedEventListener protocol"]
    #[cfg(feature = "GameKit_GKTurnBasedEventHandler")]
    unsafe impl ClassType for GKTurnBasedEventHandler {
        type Super = NSObject;
    }
);

#[cfg(feature = "GameKit_GKTurnBasedEventHandler")]
unsafe impl NSObjectProtocol for GKTurnBasedEventHandler {}

extern_methods!(
    #[cfg(feature = "GameKit_GKTurnBasedEventHandler")]
    unsafe impl GKTurnBasedEventHandler {
        #[deprecated]
        #[method_id(@__retain_semantics Other sharedTurnBasedEventHandler)]
        pub unsafe fn sharedTurnBasedEventHandler() -> Id<GKTurnBasedEventHandler>;

        #[deprecated]
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSObject>>;

        #[deprecated]
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSObject>);
    }
);
