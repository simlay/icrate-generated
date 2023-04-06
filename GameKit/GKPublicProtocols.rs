//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameKit::*;

extern_protocol!(
    #[deprecated = "Use MCSession in association with MCSessionDelegate from the MultipeerConnectivity framework instead"]
    pub unsafe trait GKSessionDelegate: NSObjectProtocol {
        #[cfg(all(feature = "Foundation_NSString", feature = "GameKit_GKSession"))]
        #[optional]
        #[method(session:peer:didChangeState:)]
        unsafe fn session_peer_didChangeState(
            &self,
            session: &GKSession,
            peer_id: &NSString,
            state: GKPeerConnectionState,
        );

        #[cfg(all(feature = "Foundation_NSString", feature = "GameKit_GKSession"))]
        #[optional]
        #[method(session:didReceiveConnectionRequestFromPeer:)]
        unsafe fn session_didReceiveConnectionRequestFromPeer(
            &self,
            session: &GKSession,
            peer_id: &NSString,
        );

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "GameKit_GKSession"
        ))]
        #[optional]
        #[method(session:connectionWithPeerFailed:withError:)]
        unsafe fn session_connectionWithPeerFailed_withError(
            &self,
            session: &GKSession,
            peer_id: &NSString,
            error: &NSError,
        );

        #[cfg(all(feature = "Foundation_NSError", feature = "GameKit_GKSession"))]
        #[optional]
        #[method(session:didFailWithError:)]
        unsafe fn session_didFailWithError(&self, session: &GKSession, error: &NSError);
    }

    unsafe impl ProtocolType for dyn GKSessionDelegate {}
);

extern_protocol!(
    #[cfg(not(any(target_os = "macos")))]
    pub unsafe trait GKVoiceChatClient: NSObjectProtocol {
        #[cfg(all(
            feature = "Foundation_NSData",
            feature = "Foundation_NSString",
            feature = "GameKit_GKVoiceChatService"
        ))]
        #[method(voiceChatService:sendData:toParticipantID:)]
        unsafe fn voiceChatService_sendData_toParticipantID(
            &self,
            voice_chat_service: &GKVoiceChatService,
            data: &NSData,
            participant_id: &NSString,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other participantID)]
        unsafe fn participantID(&self) -> Id<NSString>;

        #[cfg(all(
            feature = "Foundation_NSData",
            feature = "Foundation_NSString",
            feature = "GameKit_GKVoiceChatService"
        ))]
        #[optional]
        #[method(voiceChatService:sendRealTimeData:toParticipantID:)]
        unsafe fn voiceChatService_sendRealTimeData_toParticipantID(
            &self,
            voice_chat_service: &GKVoiceChatService,
            data: &NSData,
            participant_id: &NSString,
        );

        #[cfg(all(
            feature = "Foundation_NSString",
            feature = "GameKit_GKVoiceChatService"
        ))]
        #[optional]
        #[method(voiceChatService:didStartWithParticipantID:)]
        unsafe fn voiceChatService_didStartWithParticipantID(
            &self,
            voice_chat_service: &GKVoiceChatService,
            participant_id: &NSString,
        );

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "GameKit_GKVoiceChatService"
        ))]
        #[optional]
        #[method(voiceChatService:didNotStartWithParticipantID:error:)]
        unsafe fn voiceChatService_didNotStartWithParticipantID_error(
            &self,
            voice_chat_service: &GKVoiceChatService,
            participant_id: &NSString,
            error: Option<&NSError>,
        );

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "GameKit_GKVoiceChatService"
        ))]
        #[optional]
        #[method(voiceChatService:didStopWithParticipantID:error:)]
        unsafe fn voiceChatService_didStopWithParticipantID_error(
            &self,
            voice_chat_service: &GKVoiceChatService,
            participant_id: &NSString,
            error: Option<&NSError>,
        );

        #[cfg(all(
            feature = "Foundation_NSString",
            feature = "GameKit_GKVoiceChatService"
        ))]
        #[optional]
        #[method(voiceChatService:didReceiveInvitationFromParticipantID:callID:)]
        unsafe fn voiceChatService_didReceiveInvitationFromParticipantID_callID(
            &self,
            voice_chat_service: &GKVoiceChatService,
            participant_id: &NSString,
            call_id: NSInteger,
        );
    }

    #[cfg(not(any(target_os = "macos")))]
    unsafe impl ProtocolType for dyn GKVoiceChatClient {}
);
