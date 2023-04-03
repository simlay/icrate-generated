//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::MailKit::*;

ns_enum!(
    #[underlying(NSInteger)]
    #[cfg(not(any(target_os = "ios", target_os = "tvos", target_os = "watchos")))]
    pub enum MEMessageActionMessageColor {
        #[cfg(not(any(target_os = "ios", target_os = "tvos", target_os = "watchos")))]
        MEMessageActionMessageColorNone = 0,
        #[cfg(not(any(target_os = "ios", target_os = "tvos", target_os = "watchos")))]
        MEMessageActionMessageColorGreen = 1,
        #[cfg(not(any(target_os = "ios", target_os = "tvos", target_os = "watchos")))]
        MEMessageActionMessageColorYellow = 2,
        #[cfg(not(any(target_os = "ios", target_os = "tvos", target_os = "watchos")))]
        MEMessageActionMessageColorOrange = 3,
        #[cfg(not(any(target_os = "ios", target_os = "tvos", target_os = "watchos")))]
        MEMessageActionMessageColorRed = 4,
        #[cfg(not(any(target_os = "ios", target_os = "tvos", target_os = "watchos")))]
        MEMessageActionMessageColorPurple = 5,
        #[cfg(not(any(target_os = "ios", target_os = "tvos", target_os = "watchos")))]
        MEMessageActionMessageColorBlue = 6,
        #[cfg(not(any(target_os = "ios", target_os = "tvos", target_os = "watchos")))]
        MEMessageActionMessageColorGray = 7,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    #[cfg(not(any(target_os = "ios", target_os = "tvos", target_os = "watchos")))]
    pub enum MEMessageActionFlag {
        #[cfg(not(any(target_os = "ios", target_os = "tvos", target_os = "watchos")))]
        MEMessageActionFlagNone = 0,
        #[cfg(not(any(target_os = "ios", target_os = "tvos", target_os = "watchos")))]
        MEMessageActionFlagDefaultColor = 1,
        #[cfg(not(any(target_os = "ios", target_os = "tvos", target_os = "watchos")))]
        MEMessageActionFlagRed = 2,
        #[cfg(not(any(target_os = "ios", target_os = "tvos", target_os = "watchos")))]
        MEMessageActionFlagOrange = 3,
        #[cfg(not(any(target_os = "ios", target_os = "tvos", target_os = "watchos")))]
        MEMessageActionFlagYellow = 4,
        #[cfg(not(any(target_os = "ios", target_os = "tvos", target_os = "watchos")))]
        MEMessageActionFlagGreen = 5,
        #[cfg(not(any(target_os = "ios", target_os = "tvos", target_os = "watchos")))]
        MEMessageActionFlagBlue = 6,
        #[cfg(not(any(target_os = "ios", target_os = "tvos", target_os = "watchos")))]
        MEMessageActionFlagPurple = 7,
        #[cfg(not(any(target_os = "ios", target_os = "tvos", target_os = "watchos")))]
        MEMessageActionFlagGray = 8,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MailKit_MEMessageAction")]
    #[cfg(not(any(target_os = "ios", target_os = "tvos", target_os = "watchos")))]
    pub struct MEMessageAction;

    #[cfg(feature = "MailKit_MEMessageAction")]
    unsafe impl ClassType for MEMessageAction {
        type Super = NSObject;
    }
);

#[cfg(feature = "MailKit_MEMessageAction")]
unsafe impl NSCoding for MEMessageAction {}

#[cfg(feature = "MailKit_MEMessageAction")]
unsafe impl NSObjectProtocol for MEMessageAction {}

#[cfg(feature = "MailKit_MEMessageAction")]
unsafe impl NSSecureCoding for MEMessageAction {}

extern_methods!(
    #[cfg(feature = "MailKit_MEMessageAction")]
    unsafe impl MEMessageAction {
        #[method_id(@__retain_semantics Other moveToTrashAction)]
        pub unsafe fn moveToTrashAction() -> Id<MEMessageAction>;

        #[method_id(@__retain_semantics Other moveToArchiveAction)]
        pub unsafe fn moveToArchiveAction() -> Id<MEMessageAction>;

        #[method_id(@__retain_semantics Other moveToJunkAction)]
        pub unsafe fn moveToJunkAction() -> Id<MEMessageAction>;

        #[method_id(@__retain_semantics Other markAsReadAction)]
        pub unsafe fn markAsReadAction() -> Id<MEMessageAction>;

        #[method_id(@__retain_semantics Other markAsUnreadAction)]
        pub unsafe fn markAsUnreadAction() -> Id<MEMessageAction>;

        #[method_id(@__retain_semantics Other flagActionWithFlag:)]
        pub unsafe fn flagActionWithFlag(flag: MEMessageActionFlag) -> Id<Self>;

        #[method_id(@__retain_semantics Other setBackgroundColorActionWithColor:)]
        pub unsafe fn setBackgroundColorActionWithColor(
            color: MEMessageActionMessageColor,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
