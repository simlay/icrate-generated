//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::MailKit::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum MEComposeUserAction {
        MEComposeUserActionNewMessage = 1,
        MEComposeUserActionReply = 2,
        MEComposeUserActionReplyAll = 3,
        MEComposeUserActionForward = 4,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MailKit_MEComposeContext")]
    pub struct MEComposeContext;

    #[cfg(feature = "MailKit_MEComposeContext")]
    unsafe impl ClassType for MEComposeContext {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "MailKit_MEComposeContext")]
unsafe impl NSObjectProtocol for MEComposeContext {}

extern_methods!(
    #[cfg(feature = "MailKit_MEComposeContext")]
    unsafe impl MEComposeContext {
        #[cfg(feature = "Foundation_NSUUID")]
        #[method_id(@__retain_semantics Other contextID)]
        pub unsafe fn contextID(&self) -> Id<NSUUID>;

        #[cfg(feature = "MailKit_MEMessage")]
        #[method_id(@__retain_semantics Other originalMessage)]
        pub unsafe fn originalMessage(&self) -> Option<Id<MEMessage>>;

        #[method(action)]
        pub unsafe fn action(&self) -> MEComposeUserAction;

        #[method(isEncrypted)]
        pub unsafe fn isEncrypted(&self) -> bool;

        #[method(shouldEncrypt)]
        pub unsafe fn shouldEncrypt(&self) -> bool;

        #[method(isSigned)]
        pub unsafe fn isSigned(&self) -> bool;

        #[method(shouldSign)]
        pub unsafe fn shouldSign(&self) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "MailKit_MEComposeContext")]
    unsafe impl MEComposeContext {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
