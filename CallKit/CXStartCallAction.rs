//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CallKit::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CallKit_CXStartCallAction")]
    #[cfg(not(any(target_os = "macos")))]
    pub struct CXStartCallAction;

    #[cfg(feature = "CallKit_CXStartCallAction")]
    unsafe impl ClassType for CXStartCallAction {
        #[inherits(CXAction, NSObject)]
        type Super = CXCallAction;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CallKit_CXStartCallAction")]
#[cfg(not(any(target_os = "macos")))]
unsafe impl NSCoding for CXStartCallAction {}

#[cfg(feature = "CallKit_CXStartCallAction")]
#[cfg(not(any(target_os = "macos")))]
unsafe impl NSCopying for CXStartCallAction {}

#[cfg(feature = "CallKit_CXStartCallAction")]
#[cfg(not(any(target_os = "macos")))]
unsafe impl NSObjectProtocol for CXStartCallAction {}

#[cfg(feature = "CallKit_CXStartCallAction")]
#[cfg(not(any(target_os = "macos")))]
unsafe impl NSSecureCoding for CXStartCallAction {}

#[cfg(not(any(target_os = "macos")))]
extern_methods!(
    #[cfg(feature = "CallKit_CXStartCallAction")]
    unsafe impl CXStartCallAction {
        #[cfg(all(feature = "CallKit_CXHandle", feature = "Foundation_NSUUID"))]
        #[method_id(@__retain_semantics Init initWithCallUUID:handle:)]
        pub unsafe fn initWithCallUUID_handle(
            this: Option<Allocated<Self>>,
            call_uuid: &NSUUID,
            handle: &CXHandle,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            a_decoder: &NSCoder,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSUUID")]
        #[method_id(@__retain_semantics Init initWithCallUUID:)]
        pub unsafe fn initWithCallUUID(
            this: Option<Allocated<Self>>,
            call_uuid: &NSUUID,
        ) -> Id<Self>;

        #[cfg(feature = "CallKit_CXHandle")]
        #[method_id(@__retain_semantics Other handle)]
        pub unsafe fn handle(&self) -> Id<CXHandle>;

        #[cfg(feature = "CallKit_CXHandle")]
        #[method(setHandle:)]
        pub unsafe fn setHandle(&self, handle: &CXHandle);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other contactIdentifier)]
        pub unsafe fn contactIdentifier(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setContactIdentifier:)]
        pub unsafe fn setContactIdentifier(&self, contact_identifier: Option<&NSString>);

        #[method(isVideo)]
        pub unsafe fn isVideo(&self) -> bool;

        #[method(setVideo:)]
        pub unsafe fn setVideo(&self, video: bool);

        #[cfg(feature = "Foundation_NSDate")]
        #[method(fulfillWithDateStarted:)]
        pub unsafe fn fulfillWithDateStarted(&self, date_started: &NSDate);
    }
);

#[cfg(not(any(target_os = "macos")))]
extern_methods!(
    /// Methods declared on superclass `CXCallAction`
    #[cfg(feature = "CallKit_CXStartCallAction")]
    unsafe impl CXStartCallAction {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CallKit_CXStartCallAction")]
    unsafe impl CXStartCallAction {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
