//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::MailKit::*;

extern_protocol!(
    #[cfg(not(any(target_os = "ios", target_os = "tvos", target_os = "watchos")))]
    pub unsafe trait MEMessageActionHandler: NSObjectProtocol {
        #[cfg(all(
            feature = "MailKit_MEMessage",
            feature = "MailKit_MEMessageActionDecision"
        ))]
        #[method(decideActionForMessage:completionHandler:)]
        unsafe fn decideActionForMessage_completionHandler(
            &self,
            message: &MEMessage,
            completion_handler: &Block<(*mut MEMessageActionDecision,), ()>,
        );

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[optional]
        #[method_id(@__retain_semantics Other requiredHeaders)]
        unsafe fn requiredHeaders(&self) -> Id<NSArray<NSString>>;
    }

    unsafe impl ProtocolType for dyn MEMessageActionHandler {}
);
