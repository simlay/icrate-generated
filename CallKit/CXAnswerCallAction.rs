//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CallKit::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CallKit_CXAnswerCallAction")]
    #[cfg(not(any(target_os = "macos")))]
    pub struct CXAnswerCallAction;

    #[cfg(feature = "CallKit_CXAnswerCallAction")]
    unsafe impl ClassType for CXAnswerCallAction {
        #[inherits(CXAction, NSObject)]
        type Super = CXCallAction;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CallKit_CXAnswerCallAction")]
#[cfg(not(any(target_os = "macos")))]
unsafe impl NSCoding for CXAnswerCallAction {}

#[cfg(feature = "CallKit_CXAnswerCallAction")]
#[cfg(not(any(target_os = "macos")))]
unsafe impl NSCopying for CXAnswerCallAction {}

#[cfg(feature = "CallKit_CXAnswerCallAction")]
#[cfg(not(any(target_os = "macos")))]
unsafe impl NSObjectProtocol for CXAnswerCallAction {}

#[cfg(feature = "CallKit_CXAnswerCallAction")]
#[cfg(not(any(target_os = "macos")))]
unsafe impl NSSecureCoding for CXAnswerCallAction {}

#[cfg(not(any(target_os = "macos")))]
extern_methods!(
    #[cfg(feature = "CallKit_CXAnswerCallAction")]
    unsafe impl CXAnswerCallAction {
        #[cfg(feature = "Foundation_NSDate")]
        #[method(fulfillWithDateConnected:)]
        pub unsafe fn fulfillWithDateConnected(&self, date_connected: &NSDate);
    }
);

#[cfg(not(any(target_os = "macos")))]
extern_methods!(
    /// Methods declared on superclass `CXCallAction`
    #[cfg(feature = "CallKit_CXAnswerCallAction")]
    unsafe impl CXAnswerCallAction {
        #[cfg(feature = "Foundation_NSUUID")]
        #[method_id(@__retain_semantics Init initWithCallUUID:)]
        pub unsafe fn initWithCallUUID(
            this: Option<Allocated<Self>>,
            call_uuid: &NSUUID,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            a_decoder: &NSCoder,
        ) -> Option<Id<Self>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CallKit_CXAnswerCallAction")]
    unsafe impl CXAnswerCallAction {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
