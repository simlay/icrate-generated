//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::IdentityLookup::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "IdentityLookup_ILClassificationResponse")]
    #[cfg(not(any(target_os = "macos")))]
    pub struct ILClassificationResponse;

    #[cfg(not(any(target_os = "macos")))]
    #[cfg(feature = "IdentityLookup_ILClassificationResponse")]
    unsafe impl ClassType for ILClassificationResponse {
        type Super = NSObject;
    }
);

#[cfg(feature = "IdentityLookup_ILClassificationResponse")]
#[cfg(not(any(target_os = "macos")))]
unsafe impl NSCoding for ILClassificationResponse {}

#[cfg(feature = "IdentityLookup_ILClassificationResponse")]
#[cfg(not(any(target_os = "macos")))]
unsafe impl NSObjectProtocol for ILClassificationResponse {}

#[cfg(feature = "IdentityLookup_ILClassificationResponse")]
#[cfg(not(any(target_os = "macos")))]
unsafe impl NSSecureCoding for ILClassificationResponse {}

extern_methods!(
    #[cfg(feature = "IdentityLookup_ILClassificationResponse")]
    #[cfg(not(any(target_os = "macos")))]
    unsafe impl ILClassificationResponse {
        #[cfg(not(any(target_os = "macos")))]
        #[method(action)]
        pub unsafe fn action(&self) -> ILClassificationAction;

        #[cfg(not(any(target_os = "macos")))]
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other userString)]
        pub unsafe fn userString(&self) -> Option<Id<NSString>>;

        #[cfg(not(any(target_os = "macos")))]
        #[cfg(feature = "Foundation_NSString")]
        #[method(setUserString:)]
        pub unsafe fn setUserString(&self, user_string: Option<&NSString>);

        #[cfg(not(any(target_os = "macos")))]
        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other userInfo)]
        pub unsafe fn userInfo(&self) -> Option<Id<NSDictionary<NSString, Object>>>;

        #[cfg(not(any(target_os = "macos")))]
        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method(setUserInfo:)]
        pub unsafe fn setUserInfo(&self, user_info: Option<&NSDictionary<NSString, Object>>);

        #[cfg(not(any(target_os = "macos")))]
        #[method_id(@__retain_semantics Init initWithClassificationAction:)]
        pub unsafe fn initWithClassificationAction(
            this: Option<Allocated<Self>>,
            action: ILClassificationAction,
        ) -> Id<Self>;

        #[cfg(not(any(target_os = "macos")))]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);
