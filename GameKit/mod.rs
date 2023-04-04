//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#![allow(unused_imports)]
#![allow(deprecated)]
#[path = "GKAccessPoint.rs"]
mod __GKAccessPoint;
#[path = "GKAchievement.rs"]
mod __GKAchievement;
#[path = "GKAchievementDescription.rs"]
mod __GKAchievementDescription;
#[path = "GKAchievementViewController.rs"]
mod __GKAchievementViewController;
#[path = "GKBasePlayer.rs"]
mod __GKBasePlayer;
#[path = "GKChallenge.rs"]
mod __GKChallenge;
#[path = "GKChallengeEventHandler.rs"]
mod __GKChallengeEventHandler;
#[path = "GKChallengesViewController.rs"]
mod __GKChallengesViewController;
#[path = "GKCloudPlayer.rs"]
mod __GKCloudPlayer;
#[path = "GKDefines.rs"]
mod __GKDefines;
#[path = "GKDialogController.rs"]
mod __GKDialogController;
#[path = "GKError.rs"]
mod __GKError;
#[path = "GKEventListener.rs"]
mod __GKEventListener;
#[path = "GKFriendRequestComposeViewController.rs"]
mod __GKFriendRequestComposeViewController;
#[path = "GKGameCenterViewController.rs"]
mod __GKGameCenterViewController;
#[path = "GKGameSession.rs"]
mod __GKGameSession;
#[path = "GKGameSessionError.rs"]
mod __GKGameSessionError;
#[path = "GKGameSessionEventListener.rs"]
mod __GKGameSessionEventListener;
#[path = "GKGameSessionSharingViewController.rs"]
mod __GKGameSessionSharingViewController;
#[path = "GKLeaderboard.rs"]
mod __GKLeaderboard;
#[path = "GKLeaderboardEntry.rs"]
mod __GKLeaderboardEntry;
#[path = "GKLeaderboardScore.rs"]
mod __GKLeaderboardScore;
#[path = "GKLeaderboardSet.rs"]
mod __GKLeaderboardSet;
#[path = "GKLeaderboardViewController.rs"]
mod __GKLeaderboardViewController;
#[path = "GKLocalPlayer.rs"]
mod __GKLocalPlayer;
#[path = "GKMatch.rs"]
mod __GKMatch;
#[path = "GKMatchmaker.rs"]
mod __GKMatchmaker;
#[path = "GKMatchmakerViewController.rs"]
mod __GKMatchmakerViewController;
#[path = "GKNotificationBanner.rs"]
mod __GKNotificationBanner;
#[path = "GKPlayer.rs"]
mod __GKPlayer;
#[path = "GKPublicConstants.rs"]
mod __GKPublicConstants;
#[path = "GKPublicProtocols.rs"]
mod __GKPublicProtocols;
#[path = "GKSavedGame.rs"]
mod __GKSavedGame;
#[path = "GKSavedGameListener.rs"]
mod __GKSavedGameListener;
#[path = "GKScore.rs"]
mod __GKScore;
#[path = "GKSession.rs"]
mod __GKSession;
#[path = "GKSessionError.rs"]
mod __GKSessionError;
#[path = "GKTurnBasedMatch.rs"]
mod __GKTurnBasedMatch;
#[path = "GKTurnBasedMatchmakerViewController.rs"]
mod __GKTurnBasedMatchmakerViewController;
#[path = "GKVoiceChat.rs"]
mod __GKVoiceChat;
#[path = "GKVoiceChatService.rs"]
mod __GKVoiceChatService;

pub use self::__GKAccessPoint::GKAccessPointLocation;

pub use self::__GKAccessPoint::GKAccessPointLocationTopLeading;

pub use self::__GKAccessPoint::GKAccessPointLocationTopTrailing;

pub use self::__GKAccessPoint::GKAccessPointLocationBottomLeading;

#[cfg(feature = "GameKit_GKAccessPoint")]
pub use self::__GKAccessPoint::GKAccessPoint;
pub use self::__GKAccessPoint::GKAccessPointLocationBottomTrailing;
#[cfg(feature = "GameKit_GKAchievement")]
pub use self::__GKAchievement::GKAchievement;
#[cfg(feature = "GameKit_GKAchievementDescription")]
pub use self::__GKAchievementDescription::GKAchievementDescription;
#[cfg(feature = "GameKit_GKAchievementViewController")]
pub use self::__GKAchievementViewController::GKAchievementViewController;

pub use self::__GKAchievementViewController::GKAchievementViewControllerDelegate;
#[cfg(feature = "GameKit_GKBasePlayer")]
pub use self::__GKBasePlayer::GKBasePlayer;

pub use self::__GKChallenge::GKChallengeState;

pub use self::__GKChallenge::GKChallengeStateInvalid;

pub use self::__GKChallenge::GKChallengeStatePending;

pub use self::__GKChallenge::GKChallengeStateCompleted;

#[cfg(feature = "GameKit_GKAchievementChallenge")]
#[cfg(not(any(target_os = "watchos")))]
pub use self::__GKChallenge::GKAchievementChallenge;
#[cfg(feature = "GameKit_GKChallenge")]
#[cfg(not(any(target_os = "watchos")))]
pub use self::__GKChallenge::GKChallenge;
pub use self::__GKChallenge::GKChallengeStateDeclined;
#[cfg(feature = "GameKit_GKScoreChallenge")]
#[cfg(not(any(target_os = "watchos")))]
pub use self::__GKChallenge::GKScoreChallenge;

pub use self::__GKChallenge::GKChallengeComposeCompletionBlock;
#[cfg(feature = "GameKit_GKChallengeEventHandler")]
pub use self::__GKChallengeEventHandler::GKChallengeEventHandler;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__GKChallengeEventHandler::GKChallengeEventHandlerDelegate;
#[cfg(feature = "GameKit_GKChallengesViewController")]
pub use self::__GKChallengesViewController::GKChallengesViewController;

pub use self::__GKChallengesViewController::GKChallengesViewControllerDelegate;
#[cfg(feature = "GameKit_GKCloudPlayer")]
#[cfg(not(any(target_os = "watchos")))]
pub use self::__GKCloudPlayer::GKCloudPlayer;

#[cfg(feature = "GameKit_GKDialogController")]
pub use self::__GKDialogController::GKDialogController;
pub use self::__GKDialogController::GKViewController;

pub use self::__GKError::GKErrorDomain;

pub use self::__GKError::GKErrorCode;

pub use self::__GKError::GKErrorUnknown;

pub use self::__GKError::GKErrorCancelled;

pub use self::__GKError::GKErrorCommunicationsFailure;

pub use self::__GKError::GKErrorUserDenied;

pub use self::__GKError::GKErrorInvalidCredentials;

pub use self::__GKError::GKErrorNotAuthenticated;

pub use self::__GKError::GKErrorAuthenticationInProgress;

pub use self::__GKError::GKErrorInvalidPlayer;

pub use self::__GKError::GKErrorScoreNotSet;

pub use self::__GKError::GKErrorParentalControlsBlocked;

pub use self::__GKError::GKErrorPlayerStatusExceedsMaximumLength;

pub use self::__GKError::GKErrorPlayerStatusInvalid;

pub use self::__GKError::GKErrorMatchRequestInvalid;

pub use self::__GKError::GKErrorUnderage;

pub use self::__GKError::GKErrorGameUnrecognized;

pub use self::__GKError::GKErrorNotSupported;

pub use self::__GKError::GKErrorInvalidParameter;

pub use self::__GKError::GKErrorUnexpectedConnection;

pub use self::__GKError::GKErrorChallengeInvalid;

pub use self::__GKError::GKErrorTurnBasedMatchDataTooLarge;

pub use self::__GKError::GKErrorTurnBasedTooManySessions;

pub use self::__GKError::GKErrorTurnBasedInvalidParticipant;

pub use self::__GKError::GKErrorTurnBasedInvalidTurn;

pub use self::__GKError::GKErrorTurnBasedInvalidState;

pub use self::__GKError::GKErrorInvitationsDisabled;

pub use self::__GKError::GKErrorPlayerPhotoFailure;

pub use self::__GKError::GKErrorUbiquityContainerUnavailable;

pub use self::__GKError::GKErrorMatchNotConnected;

pub use self::__GKError::GKErrorGameSessionRequestInvalid;

pub use self::__GKError::GKErrorRestrictedToAutomatch;

pub use self::__GKError::GKErrorAPINotAvailable;

pub use self::__GKError::GKErrorNotAuthorized;

pub use self::__GKError::GKErrorConnectionTimeout;

pub use self::__GKError::GKErrorAPIObsolete;

pub use self::__GKError::GKErrorFriendListDescriptionMissing;

pub use self::__GKError::GKErrorFriendListRestricted;

pub use self::__GKError::GKErrorFriendListDenied;

pub use self::__GKError::GKErrorFriendRequestNotAvailable;

pub use self::__GKEventListener::GKChallengeListener;
#[cfg(feature = "GameKit_GKFriendRequestComposeViewController")]
pub use self::__GKFriendRequestComposeViewController::GKFriendRequestComposeViewController;

pub use self::__GKFriendRequestComposeViewController::GKFriendRequestComposeViewControllerDelegate;

pub use self::__GKGameCenterViewController::GKGameCenterViewControllerState;

pub use self::__GKGameCenterViewController::GKGameCenterViewControllerStateDefault;

pub use self::__GKGameCenterViewController::GKGameCenterViewControllerStateLeaderboards;

pub use self::__GKGameCenterViewController::GKGameCenterViewControllerStateAchievements;

pub use self::__GKGameCenterViewController::GKGameCenterViewControllerStateChallenges;

pub use self::__GKGameCenterViewController::GKGameCenterViewControllerStateLocalPlayerProfile;

pub use self::__GKGameCenterViewController::GKGameCenterViewControllerStateDashboard;

#[cfg(feature = "GameKit_GKGameCenterViewController")]
pub use self::__GKGameCenterViewController::GKGameCenterViewController;
pub use self::__GKGameCenterViewController::GKGameCenterViewControllerStateLocalPlayerFriendsList;

pub use self::__GKGameCenterViewController::GKGameCenterControllerDelegate;

pub use self::__GKGameSession::GKConnectionState;

pub use self::__GKGameSession::GKConnectionStateNotConnected;

pub use self::__GKGameSession::GKConnectionStateConnected;

pub use self::__GKGameSession::GKTransportType;

pub use self::__GKGameSession::GKTransportTypeUnreliable;

#[cfg(feature = "GameKit_GKGameSession")]
#[cfg(not(any(target_os = "watchos")))]
pub use self::__GKGameSession::GKGameSession;
pub use self::__GKGameSession::GKTransportTypeReliable;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__GKGameSessionError::GKGameSessionErrorBadContainer;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__GKGameSessionError::GKGameSessionErrorCloudDriveDisabled;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__GKGameSessionError::GKGameSessionErrorCloudQuotaExceeded;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__GKGameSessionError::GKGameSessionErrorCode;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__GKGameSessionError::GKGameSessionErrorConnectionCancelledByUser;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__GKGameSessionError::GKGameSessionErrorConnectionFailed;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__GKGameSessionError::GKGameSessionErrorDomain;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__GKGameSessionError::GKGameSessionErrorInvalidSession;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__GKGameSessionError::GKGameSessionErrorNetworkFailure;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__GKGameSessionError::GKGameSessionErrorNotAuthenticated;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__GKGameSessionError::GKGameSessionErrorSendDataNoRecipients;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__GKGameSessionError::GKGameSessionErrorSendDataNotConnected;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__GKGameSessionError::GKGameSessionErrorSendDataNotReachable;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__GKGameSessionError::GKGameSessionErrorSendRateLimitReached;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__GKGameSessionError::GKGameSessionErrorSessionConflict;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__GKGameSessionError::GKGameSessionErrorSessionHasMaxConnectedPlayers;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__GKGameSessionError::GKGameSessionErrorSessionNotShared;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__GKGameSessionError::GKGameSessionErrorUnknown;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__GKGameSessionEventListener::GKGameSessionEventListener;

pub use self::__GKLeaderboard::GKLeaderboardTimeScope;

pub use self::__GKLeaderboard::GKLeaderboardTimeScopeToday;

pub use self::__GKLeaderboard::GKLeaderboardTimeScopeWeek;

pub use self::__GKLeaderboard::GKLeaderboardTimeScopeAllTime;

pub use self::__GKLeaderboard::GKLeaderboardPlayerScope;

pub use self::__GKLeaderboard::GKLeaderboardPlayerScopeGlobal;

pub use self::__GKLeaderboard::GKLeaderboardPlayerScopeFriendsOnly;

pub use self::__GKLeaderboard::GKLeaderboardType;

pub use self::__GKLeaderboard::GKLeaderboardTypeClassic;

#[cfg(feature = "GameKit_GKLeaderboard")]
pub use self::__GKLeaderboard::GKLeaderboard;
pub use self::__GKLeaderboard::GKLeaderboardTypeRecurring;
#[cfg(feature = "GameKit_GKLeaderboardEntry")]
pub use self::__GKLeaderboardEntry::GKLeaderboardEntry;
#[cfg(feature = "GameKit_GKLeaderboardScore")]
pub use self::__GKLeaderboardScore::GKLeaderboardScore;
#[cfg(feature = "GameKit_GKLeaderboardSet")]
pub use self::__GKLeaderboardSet::GKLeaderboardSet;
#[cfg(feature = "GameKit_GKLeaderboardViewController")]
pub use self::__GKLeaderboardViewController::GKLeaderboardViewController;

pub use self::__GKLeaderboardViewController::GKLeaderboardViewControllerDelegate;
#[cfg(feature = "GameKit_GKLocalPlayer")]
pub use self::__GKLocalPlayer::GKLocalPlayer;

pub use self::__GKLocalPlayer::GKLocalPlayerListener;

pub use self::__GKLocalPlayer::GKPlayerAuthenticationDidChangeNotificationName;

pub use self::__GKLocalPlayer::GKFriendsAuthorizationStatus;

pub use self::__GKLocalPlayer::GKFriendsAuthorizationStatusNotDetermined;

pub use self::__GKLocalPlayer::GKFriendsAuthorizationStatusRestricted;

pub use self::__GKLocalPlayer::GKFriendsAuthorizationStatusDenied;

pub use self::__GKLocalPlayer::GKFriendsAuthorizationStatusAuthorized;

pub use self::__GKMatch::GKMatchSendDataMode;

pub use self::__GKMatch::GKMatchSendDataReliable;

pub use self::__GKMatch::GKMatchSendDataUnreliable;

pub use self::__GKMatch::GKPlayerConnectionState;

pub use self::__GKMatch::GKPlayerStateUnknown;

pub use self::__GKMatch::GKPlayerStateConnected;

#[cfg(feature = "GameKit_GKMatch")]
#[cfg(not(any(target_os = "watchos")))]
pub use self::__GKMatch::GKMatch;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__GKMatch::GKMatchDelegate;
pub use self::__GKMatch::GKPlayerStateDisconnected;

pub use self::__GKMatchmaker::GKInviteRecipientResponse;

pub use self::__GKMatchmaker::GKInviteRecipientResponseAccepted;

pub use self::__GKMatchmaker::GKInviteRecipientResponseDeclined;

pub use self::__GKMatchmaker::GKInviteRecipientResponseFailed;

pub use self::__GKMatchmaker::GKInviteRecipientResponseIncompatible;

pub use self::__GKMatchmaker::GKInviteRecipientResponseUnableToConnect;

pub use self::__GKMatchmaker::GKInviteRecipientResponseNoAnswer;

pub use self::__GKMatchmaker::GKInviteeResponseAccepted;

pub use self::__GKMatchmaker::GKInviteeResponseDeclined;

pub use self::__GKMatchmaker::GKInviteeResponseFailed;

pub use self::__GKMatchmaker::GKInviteeResponseIncompatible;

pub use self::__GKMatchmaker::GKInviteeResponseUnableToConnect;

pub use self::__GKMatchmaker::GKInviteeResponseNoAnswer;

pub use self::__GKMatchmaker::GKInviteeResponse;

pub use self::__GKMatchmaker::GKMatchType;

pub use self::__GKMatchmaker::GKMatchTypePeerToPeer;

pub use self::__GKMatchmaker::GKMatchTypeHosted;

#[cfg(feature = "GameKit_GKInvite")]
#[cfg(not(any(target_os = "watchos")))]
pub use self::__GKMatchmaker::GKInvite;
#[cfg(feature = "GameKit_GKMatchRequest")]
pub use self::__GKMatchmaker::GKMatchRequest;
pub use self::__GKMatchmaker::GKMatchTypeTurnBased;

pub use self::__GKMatchmaker::GKInviteEventListener;
#[cfg(feature = "GameKit_GKMatchmaker")]
#[cfg(not(any(target_os = "watchos")))]
pub use self::__GKMatchmaker::GKMatchmaker;

pub use self::__GKMatchmakerViewController::GKMatchmakingMode;

pub use self::__GKMatchmakerViewController::GKMatchmakingModeDefault;

pub use self::__GKMatchmakerViewController::GKMatchmakingModeNearbyOnly;

pub use self::__GKMatchmakerViewController::GKMatchmakingModeAutomatchOnly;

#[cfg(feature = "GameKit_GKMatchmakerViewController")]
pub use self::__GKMatchmakerViewController::GKMatchmakerViewController;
pub use self::__GKMatchmakerViewController::GKMatchmakingModeInviteOnly;

pub use self::__GKMatchmakerViewController::GKMatchmakerViewControllerDelegate;
#[cfg(feature = "GameKit_GKNotificationBanner")]
pub use self::__GKNotificationBanner::GKNotificationBanner;
#[cfg(feature = "GameKit_GKPlayer")]
pub use self::__GKPlayer::GKPlayer;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__GKPlayer::GKPlayerIDNoLongerAvailable;

pub use self::__GKPlayer::GKPhotoSize;

pub use self::__GKPlayer::GKPhotoSizeSmall;

pub use self::__GKPlayer::GKPhotoSizeNormal;

pub use self::__GKPlayer::GKPlayerDidChangeNotificationName;

pub use self::__GKPublicConstants::GKSendDataMode;

pub use self::__GKPublicConstants::GKSendDataReliable;

pub use self::__GKPublicConstants::GKSendDataUnreliable;

pub use self::__GKPublicConstants::GKSessionMode;

pub use self::__GKPublicConstants::GKSessionModeServer;

pub use self::__GKPublicConstants::GKSessionModeClient;

pub use self::__GKPublicConstants::GKSessionModePeer;

pub use self::__GKPublicConstants::GKPeerConnectionState;

pub use self::__GKPublicConstants::GKPeerStateAvailable;

pub use self::__GKPublicConstants::GKPeerStateUnavailable;

pub use self::__GKPublicConstants::GKPeerStateConnected;

pub use self::__GKPublicConstants::GKPeerStateDisconnected;

pub use self::__GKPublicConstants::GKPeerStateConnecting;

pub use self::__GKPublicConstants::GKPeerStateConnectedRelay;

#[cfg(not(any(target_os = "macos")))]
pub use self::__GKPublicConstants::GKVoiceChatServiceAudioUnavailableError;
#[cfg(not(any(target_os = "macos")))]
pub use self::__GKPublicConstants::GKVoiceChatServiceClientMissingRequiredMethodsError;
#[cfg(not(any(target_os = "macos")))]
pub use self::__GKPublicConstants::GKVoiceChatServiceError;
pub use self::__GKPublicConstants::GKVoiceChatServiceErrorDomain;
#[cfg(not(any(target_os = "macos")))]
pub use self::__GKPublicConstants::GKVoiceChatServiceInternalError;
#[cfg(not(any(target_os = "macos")))]
pub use self::__GKPublicConstants::GKVoiceChatServiceInvalidCallIDError;
#[cfg(not(any(target_os = "macos")))]
pub use self::__GKPublicConstants::GKVoiceChatServiceInvalidParameterError;
#[cfg(not(any(target_os = "macos")))]
pub use self::__GKPublicConstants::GKVoiceChatServiceMethodCurrentlyInvalidError;
#[cfg(not(any(target_os = "macos")))]
pub use self::__GKPublicConstants::GKVoiceChatServiceNetworkConfigurationError;
#[cfg(not(any(target_os = "macos")))]
pub use self::__GKPublicConstants::GKVoiceChatServiceNoRemotePacketsError;
#[cfg(not(any(target_os = "macos")))]
pub use self::__GKPublicConstants::GKVoiceChatServiceOutOfMemoryError;
#[cfg(not(any(target_os = "macos")))]
pub use self::__GKPublicConstants::GKVoiceChatServiceRemoteParticipantBusyError;
#[cfg(not(any(target_os = "macos")))]
pub use self::__GKPublicConstants::GKVoiceChatServiceRemoteParticipantCancelledError;
#[cfg(not(any(target_os = "macos")))]
pub use self::__GKPublicConstants::GKVoiceChatServiceRemoteParticipantDeclinedInviteError;
#[cfg(not(any(target_os = "macos")))]
pub use self::__GKPublicConstants::GKVoiceChatServiceRemoteParticipantHangupError;
#[cfg(not(any(target_os = "macos")))]
pub use self::__GKPublicConstants::GKVoiceChatServiceRemoteParticipantResponseInvalidError;
#[cfg(not(any(target_os = "macos")))]
pub use self::__GKPublicConstants::GKVoiceChatServiceUnableToConnectError;
#[cfg(not(any(target_os = "macos")))]
pub use self::__GKPublicConstants::GKVoiceChatServiceUninitializedClientError;
#[cfg(not(any(target_os = "macos")))]
pub use self::__GKPublicConstants::GKVoiceChatServiceUnsupportedRemoteVersionError;

pub use self::__GKPublicProtocols::GKSessionDelegate;
#[cfg(not(any(target_os = "macos")))]
pub use self::__GKPublicProtocols::GKVoiceChatClient;
#[cfg(feature = "GameKit_GKSavedGame")]
#[cfg(not(any(target_os = "watchos")))]
pub use self::__GKSavedGame::GKSavedGame;
#[cfg(not(any(target_os = "watchos")))]
pub use self::__GKSavedGameListener::GKSavedGameListener;
#[cfg(feature = "GameKit_GKScore")]
pub use self::__GKScore::GKScore;
#[cfg(feature = "GameKit_GKSession")]
pub use self::__GKSession::GKSession;

#[cfg(not(any(target_os = "macos")))]
pub use self::__GKSessionError::GKSessionCancelledError;
#[cfg(not(any(target_os = "macos")))]
pub use self::__GKSessionError::GKSessionCannotEnableError;
#[cfg(not(any(target_os = "macos")))]
pub use self::__GKSessionError::GKSessionConnectionClosedError;
#[cfg(not(any(target_os = "macos")))]
pub use self::__GKSessionError::GKSessionConnectionFailedError;
#[cfg(not(any(target_os = "macos")))]
pub use self::__GKSessionError::GKSessionConnectivityError;
#[cfg(not(any(target_os = "macos")))]
pub use self::__GKSessionError::GKSessionDataTooBigError;
#[cfg(not(any(target_os = "macos")))]
pub use self::__GKSessionError::GKSessionDeclinedError;
#[cfg(not(any(target_os = "macos")))]
pub use self::__GKSessionError::GKSessionError;
pub use self::__GKSessionError::GKSessionErrorDomain;
#[cfg(not(any(target_os = "macos")))]
pub use self::__GKSessionError::GKSessionInProgressError;
#[cfg(not(any(target_os = "macos")))]
pub use self::__GKSessionError::GKSessionInternalError;
#[cfg(not(any(target_os = "macos")))]
pub use self::__GKSessionError::GKSessionInvalidParameterError;
#[cfg(not(any(target_os = "macos")))]
pub use self::__GKSessionError::GKSessionNotConnectedError;
#[cfg(not(any(target_os = "macos")))]
pub use self::__GKSessionError::GKSessionPeerNotFoundError;
#[cfg(not(any(target_os = "macos")))]
pub use self::__GKSessionError::GKSessionSystemError;
#[cfg(not(any(target_os = "macos")))]
pub use self::__GKSessionError::GKSessionTimedOutError;
#[cfg(not(any(target_os = "macos")))]
pub use self::__GKSessionError::GKSessionTransportError;
#[cfg(not(any(target_os = "macos")))]
pub use self::__GKSessionError::GKSessionUnknownError;

pub use self::__GKTurnBasedMatch::GKTurnBasedMatchStatus;

pub use self::__GKTurnBasedMatch::GKTurnBasedMatchStatusUnknown;

pub use self::__GKTurnBasedMatch::GKTurnBasedMatchStatusOpen;

pub use self::__GKTurnBasedMatch::GKTurnBasedMatchStatusEnded;

pub use self::__GKTurnBasedMatch::GKTurnBasedMatchStatusMatching;

pub use self::__GKTurnBasedMatch::GKTurnBasedParticipantStatus;

pub use self::__GKTurnBasedMatch::GKTurnBasedParticipantStatusUnknown;

pub use self::__GKTurnBasedMatch::GKTurnBasedParticipantStatusInvited;

pub use self::__GKTurnBasedMatch::GKTurnBasedParticipantStatusDeclined;

pub use self::__GKTurnBasedMatch::GKTurnBasedParticipantStatusMatching;

pub use self::__GKTurnBasedMatch::GKTurnBasedParticipantStatusActive;

pub use self::__GKTurnBasedMatch::GKTurnBasedParticipantStatusDone;

pub use self::__GKTurnBasedMatch::GKTurnBasedMatchOutcome;

pub use self::__GKTurnBasedMatch::GKTurnBasedMatchOutcomeNone;

pub use self::__GKTurnBasedMatch::GKTurnBasedMatchOutcomeQuit;

pub use self::__GKTurnBasedMatch::GKTurnBasedMatchOutcomeWon;

pub use self::__GKTurnBasedMatch::GKTurnBasedMatchOutcomeLost;

pub use self::__GKTurnBasedMatch::GKTurnBasedMatchOutcomeTied;

pub use self::__GKTurnBasedMatch::GKTurnBasedMatchOutcomeTimeExpired;

pub use self::__GKTurnBasedMatch::GKTurnBasedMatchOutcomeFirst;

pub use self::__GKTurnBasedMatch::GKTurnBasedMatchOutcomeSecond;

pub use self::__GKTurnBasedMatch::GKTurnBasedMatchOutcomeThird;

pub use self::__GKTurnBasedMatch::GKTurnBasedMatchOutcomeFourth;

pub use self::__GKTurnBasedMatch::GKTurnBasedMatchOutcomeCustomRange;
#[cfg(feature = "GameKit_GKTurnBasedParticipant")]
pub use self::__GKTurnBasedMatch::GKTurnBasedParticipant;

pub use self::__GKTurnBasedMatch::GKTurnBasedEventListener;

pub use self::__GKTurnBasedMatch::GKTurnTimeoutDefault;

#[cfg(feature = "GameKit_GKTurnBasedMatch")]
pub use self::__GKTurnBasedMatch::GKTurnBasedMatch;
pub use self::__GKTurnBasedMatch::GKTurnTimeoutNone;

pub use self::__GKTurnBasedMatch::GKTurnBasedExchangeStatus;

pub use self::__GKTurnBasedMatch::GKTurnBasedExchangeStatusUnknown;

pub use self::__GKTurnBasedMatch::GKTurnBasedExchangeStatusActive;

pub use self::__GKTurnBasedMatch::GKTurnBasedExchangeStatusComplete;

pub use self::__GKTurnBasedMatch::GKTurnBasedExchangeStatusResolved;

pub use self::__GKTurnBasedMatch::GKTurnBasedExchangeStatusCanceled;

pub use self::__GKTurnBasedMatch::GKExchangeTimeoutDefault;

pub use self::__GKTurnBasedMatch::GKExchangeTimeoutNone;
#[cfg(feature = "GameKit_GKTurnBasedExchange")]
pub use self::__GKTurnBasedMatch::GKTurnBasedExchange;
#[cfg(feature = "GameKit_GKTurnBasedExchangeReply")]
pub use self::__GKTurnBasedMatch::GKTurnBasedExchangeReply;

#[cfg(feature = "GameKit_GKTurnBasedEventHandler")]
pub use self::__GKTurnBasedMatch::GKTurnBasedEventHandler;
pub use self::__GKTurnBasedMatch::GKTurnBasedEventHandlerDelegate;
#[cfg(feature = "GameKit_GKTurnBasedMatchmakerViewController")]
pub use self::__GKTurnBasedMatchmakerViewController::GKTurnBasedMatchmakerViewController;

pub use self::__GKTurnBasedMatchmakerViewController::GKTurnBasedMatchmakerViewControllerDelegate;

pub use self::__GKVoiceChat::GKVoiceChatPlayerState;

pub use self::__GKVoiceChat::GKVoiceChatPlayerConnected;

pub use self::__GKVoiceChat::GKVoiceChatPlayerDisconnected;

pub use self::__GKVoiceChat::GKVoiceChatPlayerSpeaking;

pub use self::__GKVoiceChat::GKVoiceChatPlayerSilent;

#[cfg(feature = "GameKit_GKVoiceChat")]
#[cfg(not(any(target_os = "watchos")))]
pub use self::__GKVoiceChat::GKVoiceChat;
pub use self::__GKVoiceChat::GKVoiceChatPlayerConnecting;
#[cfg(feature = "GameKit_GKVoiceChatService")]
#[cfg(not(any(target_os = "macos")))]
pub use self::__GKVoiceChatService::GKVoiceChatService;
