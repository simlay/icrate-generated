//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CallKit::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CallKit_CXCall")]
    #[cfg(not(any(target_os = "macos")))]
    pub struct CXCall;

    #[cfg(not(any(target_os = "macos")))]
    #[cfg(feature = "CallKit_CXCall")]
    unsafe impl ClassType for CXCall {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CallKit_CXCall")]
#[cfg(not(any(target_os = "macos")))]
unsafe impl NSObjectProtocol for CXCall {}

#[cfg(not(any(target_os = "macos")))]
extern_methods!(
    #[cfg(feature = "CallKit_CXCall")]
    #[cfg(not(any(target_os = "macos")))]
    unsafe impl CXCall {
        #[cfg(feature = "Foundation_NSUUID")]
        #[method_id(@__retain_semantics Other UUID)]
        pub unsafe fn UUID(&self) -> Id<NSUUID>;

        #[method(isOutgoing)]
        pub unsafe fn isOutgoing(&self) -> bool;

        #[method(isOnHold)]
        pub unsafe fn isOnHold(&self) -> bool;

        #[method(hasConnected)]
        pub unsafe fn hasConnected(&self) -> bool;

        #[method(hasEnded)]
        pub unsafe fn hasEnded(&self) -> bool;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method(isEqualToCall:)]
        pub unsafe fn isEqualToCall(&self, call: &CXCall) -> bool;
    }
);

#[cfg(not(any(target_os = "macos")))]
extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CallKit_CXCall")]
    #[cfg(not(any(target_os = "macos")))]
    unsafe impl CXCall {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
