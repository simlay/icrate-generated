//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CallKit::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CallKit_CXSetMutedCallAction")]
    #[cfg(not(any(target_os = "macos")))]
    pub struct CXSetMutedCallAction;

    #[cfg(not(any(target_os = "macos")))]
    #[cfg(feature = "CallKit_CXSetMutedCallAction")]
    unsafe impl ClassType for CXSetMutedCallAction {
        #[inherits(CXAction, NSObject)]
        type Super = CXCallAction;
    }
);

#[cfg(feature = "CallKit_CXSetMutedCallAction")]
#[cfg(not(any(target_os = "macos")))]
unsafe impl NSCoding for CXSetMutedCallAction {}

#[cfg(feature = "CallKit_CXSetMutedCallAction")]
#[cfg(not(any(target_os = "macos")))]
unsafe impl NSObjectProtocol for CXSetMutedCallAction {}

#[cfg(feature = "CallKit_CXSetMutedCallAction")]
#[cfg(not(any(target_os = "macos")))]
unsafe impl NSSecureCoding for CXSetMutedCallAction {}

extern_methods!(
    #[cfg(feature = "CallKit_CXSetMutedCallAction")]
    #[cfg(not(any(target_os = "macos")))]
    unsafe impl CXSetMutedCallAction {
        #[cfg(not(any(target_os = "macos")))]
        #[cfg(feature = "Foundation_NSUUID")]
        #[method_id(@__retain_semantics Init initWithCallUUID:muted:)]
        pub unsafe fn initWithCallUUID_muted(
            this: Option<Allocated<Self>>,
            call_uuid: &NSUUID,
            muted: bool,
        ) -> Id<Self>;

        #[cfg(not(any(target_os = "macos")))]
        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            a_decoder: &NSCoder,
        ) -> Option<Id<Self>>;

        #[cfg(not(any(target_os = "macos")))]
        #[cfg(feature = "Foundation_NSUUID")]
        #[method_id(@__retain_semantics Init initWithCallUUID:)]
        pub unsafe fn initWithCallUUID(
            this: Option<Allocated<Self>>,
            call_uuid: &NSUUID,
        ) -> Id<Self>;

        #[cfg(not(any(target_os = "macos")))]
        #[method(isMuted)]
        pub unsafe fn isMuted(&self) -> bool;

        #[cfg(not(any(target_os = "macos")))]
        #[method(setMuted:)]
        pub unsafe fn setMuted(&self, muted: bool);
    }
);
