//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameKit_GKVoiceChatService")]
    #[cfg(not(any(target_os = "macos")))]
    pub struct GKVoiceChatService;

    #[cfg(not(any(target_os = "macos")))]
    #[cfg(feature = "GameKit_GKVoiceChatService")]
    unsafe impl ClassType for GKVoiceChatService {
        type Super = NSObject;
    }
);

#[cfg(feature = "GameKit_GKVoiceChatService")]
#[cfg(not(any(target_os = "macos")))]
unsafe impl NSObjectProtocol for GKVoiceChatService {}

extern_methods!(
    #[cfg(feature = "GameKit_GKVoiceChatService")]
    #[cfg(not(any(target_os = "macos")))]
    unsafe impl GKVoiceChatService {
        #[cfg(not(any(target_os = "macos")))]
        #[method_id(@__retain_semantics Other defaultVoiceChatService)]
        pub unsafe fn defaultVoiceChatService() -> Option<Id<GKVoiceChatService>>;

        #[cfg(not(any(target_os = "macos")))]
        #[method(isVoIPAllowed)]
        pub unsafe fn isVoIPAllowed() -> bool;

        #[cfg(not(any(target_os = "macos")))]
        #[cfg(not(any(target_os = "macos")))]
        #[method_id(@__retain_semantics Other client)]
        pub unsafe fn client(&self) -> Option<Id<ProtocolObject<dyn GKVoiceChatClient>>>;

        #[cfg(not(any(target_os = "macos")))]
        #[cfg(not(any(target_os = "macos")))]
        #[method(setClient:)]
        pub unsafe fn setClient(&self, client: Option<&ProtocolObject<dyn GKVoiceChatClient>>);

        #[cfg(not(any(target_os = "macos")))]
        #[cfg(feature = "Foundation_NSString")]
        #[method(stopVoiceChatWithParticipantID:)]
        pub unsafe fn stopVoiceChatWithParticipantID(&self, participant_id: Option<&NSString>);

        #[cfg(not(any(target_os = "macos")))]
        #[method(denyCallID:)]
        pub unsafe fn denyCallID(&self, call_id: NSInteger);

        #[cfg(not(any(target_os = "macos")))]
        #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSString"))]
        #[method(receivedRealTimeData:fromParticipantID:)]
        pub unsafe fn receivedRealTimeData_fromParticipantID(
            &self,
            audio: Option<&NSData>,
            participant_id: Option<&NSString>,
        );

        #[cfg(not(any(target_os = "macos")))]
        #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSString"))]
        #[method(receivedData:fromParticipantID:)]
        pub unsafe fn receivedData_fromParticipantID(
            &self,
            arbitrary_data: Option<&NSData>,
            participant_id: Option<&NSString>,
        );

        #[cfg(not(any(target_os = "macos")))]
        #[method(isMicrophoneMuted)]
        pub unsafe fn isMicrophoneMuted(&self) -> bool;

        #[cfg(not(any(target_os = "macos")))]
        #[method(setMicrophoneMuted:)]
        pub unsafe fn setMicrophoneMuted(&self, microphone_muted: bool);

        #[cfg(not(any(target_os = "macos")))]
        #[method(remoteParticipantVolume)]
        pub unsafe fn remoteParticipantVolume(&self) -> c_float;

        #[cfg(not(any(target_os = "macos")))]
        #[method(setRemoteParticipantVolume:)]
        pub unsafe fn setRemoteParticipantVolume(&self, remote_participant_volume: c_float);

        #[cfg(not(any(target_os = "macos")))]
        #[method(isOutputMeteringEnabled)]
        pub unsafe fn isOutputMeteringEnabled(&self) -> bool;

        #[cfg(not(any(target_os = "macos")))]
        #[method(setOutputMeteringEnabled:)]
        pub unsafe fn setOutputMeteringEnabled(&self, output_metering_enabled: bool);

        #[cfg(not(any(target_os = "macos")))]
        #[method(isInputMeteringEnabled)]
        pub unsafe fn isInputMeteringEnabled(&self) -> bool;

        #[cfg(not(any(target_os = "macos")))]
        #[method(setInputMeteringEnabled:)]
        pub unsafe fn setInputMeteringEnabled(&self, input_metering_enabled: bool);

        #[cfg(not(any(target_os = "macos")))]
        #[method(outputMeterLevel)]
        pub unsafe fn outputMeterLevel(&self) -> c_float;

        #[cfg(not(any(target_os = "macos")))]
        #[method(inputMeterLevel)]
        pub unsafe fn inputMeterLevel(&self) -> c_float;
    }
);
