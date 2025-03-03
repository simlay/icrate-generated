//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSPortMessage")]
    pub struct NSPortMessage;

    #[cfg(feature = "Foundation_NSPortMessage")]
    unsafe impl ClassType for NSPortMessage {
        type Super = NSObject;
    }
);

#[cfg(feature = "Foundation_NSPortMessage")]
unsafe impl NSObjectProtocol for NSPortMessage {}

extern_methods!(
    #[cfg(feature = "Foundation_NSPortMessage")]
    unsafe impl NSPortMessage {
        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSPort"))]
        #[method_id(@__retain_semantics Init initWithSendPort:receivePort:components:)]
        pub unsafe fn initWithSendPort_receivePort_components(
            this: Option<Allocated<Self>>,
            send_port: Option<&NSPort>,
            reply_port: Option<&NSPort>,
            components: Option<&NSArray>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other components)]
        pub unsafe fn components(&self) -> Option<Id<NSArray>>;

        #[cfg(feature = "Foundation_NSPort")]
        #[method_id(@__retain_semantics Other receivePort)]
        pub unsafe fn receivePort(&self) -> Option<Id<NSPort>>;

        #[cfg(feature = "Foundation_NSPort")]
        #[method_id(@__retain_semantics Other sendPort)]
        pub unsafe fn sendPort(&self) -> Option<Id<NSPort>>;

        #[cfg(feature = "Foundation_NSDate")]
        #[method(sendBeforeDate:)]
        pub unsafe fn sendBeforeDate(&self, date: &NSDate) -> bool;

        #[method(msgid)]
        pub unsafe fn msgid(&self) -> u32;

        #[method(setMsgid:)]
        pub unsafe fn setMsgid(&self, msgid: u32);
    }
);
