//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CallKit::*;
use crate::Foundation::*;

#[cfg(not(any(target_os = "macos")))]
ns_enum!(
    #[underlying(NSInteger)]
    pub enum CXPlayDTMFCallActionType {
        #[cfg(not(any(target_os = "macos")))]
        CXPlayDTMFCallActionTypeSingleTone = 1,
        #[cfg(not(any(target_os = "macos")))]
        CXPlayDTMFCallActionTypeSoftPause = 2,
        #[cfg(not(any(target_os = "macos")))]
        CXPlayDTMFCallActionTypeHardPause = 3,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CallKit_CXPlayDTMFCallAction")]
    #[cfg(not(any(target_os = "macos")))]
    pub struct CXPlayDTMFCallAction;

    #[cfg(not(any(target_os = "macos")))]
    #[cfg(feature = "CallKit_CXPlayDTMFCallAction")]
    unsafe impl ClassType for CXPlayDTMFCallAction {
        #[inherits(CXAction, NSObject)]
        type Super = CXCallAction;
    }
);

#[cfg(feature = "CallKit_CXPlayDTMFCallAction")]
#[cfg(not(any(target_os = "macos")))]
unsafe impl NSCoding for CXPlayDTMFCallAction {}

#[cfg(feature = "CallKit_CXPlayDTMFCallAction")]
#[cfg(not(any(target_os = "macos")))]
unsafe impl NSObjectProtocol for CXPlayDTMFCallAction {}

#[cfg(feature = "CallKit_CXPlayDTMFCallAction")]
#[cfg(not(any(target_os = "macos")))]
unsafe impl NSSecureCoding for CXPlayDTMFCallAction {}

extern_methods!(
    #[cfg(feature = "CallKit_CXPlayDTMFCallAction")]
    #[cfg(not(any(target_os = "macos")))]
    unsafe impl CXPlayDTMFCallAction {
        #[cfg(not(any(target_os = "macos")))]
        #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSUUID"))]
        #[method_id(@__retain_semantics Init initWithCallUUID:digits:type:)]
        pub unsafe fn initWithCallUUID_digits_type(
            this: Option<Allocated<Self>>,
            call_uuid: &NSUUID,
            digits: &NSString,
            r#type: CXPlayDTMFCallActionType,
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
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other digits)]
        pub unsafe fn digits(&self) -> Id<NSString>;

        #[cfg(not(any(target_os = "macos")))]
        #[cfg(feature = "Foundation_NSString")]
        #[method(setDigits:)]
        pub unsafe fn setDigits(&self, digits: &NSString);

        #[cfg(not(any(target_os = "macos")))]
        #[method(type)]
        pub unsafe fn r#type(&self) -> CXPlayDTMFCallActionType;

        #[cfg(not(any(target_os = "macos")))]
        #[method(setType:)]
        pub unsafe fn setType(&self, r#type: CXPlayDTMFCallActionType);
    }
);
