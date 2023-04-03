//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_options!(
    #[underlying(NSUInteger)]
    #[cfg(not(any(target_os = "ios")))]
    pub enum NSTrackingAreaOptions {
        #[cfg(not(any(target_os = "ios")))]
        NSTrackingMouseEnteredAndExited = 0x01,
        #[cfg(not(any(target_os = "ios")))]
        NSTrackingMouseMoved = 0x02,
        #[cfg(not(any(target_os = "ios")))]
        NSTrackingCursorUpdate = 0x04,
        #[cfg(not(any(target_os = "ios")))]
        NSTrackingActiveWhenFirstResponder = 0x10,
        #[cfg(not(any(target_os = "ios")))]
        NSTrackingActiveInKeyWindow = 0x20,
        #[cfg(not(any(target_os = "ios")))]
        NSTrackingActiveInActiveApp = 0x40,
        #[cfg(not(any(target_os = "ios")))]
        NSTrackingActiveAlways = 0x80,
        #[cfg(not(any(target_os = "ios")))]
        NSTrackingAssumeInside = 0x100,
        #[cfg(not(any(target_os = "ios")))]
        NSTrackingInVisibleRect = 0x200,
        #[cfg(not(any(target_os = "ios")))]
        NSTrackingEnabledDuringMouseDrag = 0x400,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSTrackingArea")]
    #[cfg(not(any(target_os = "ios")))]
    pub struct NSTrackingArea;

    #[cfg(feature = "AppKit_NSTrackingArea")]
    unsafe impl ClassType for NSTrackingArea {
        type Super = NSObject;
    }
);

#[cfg(feature = "AppKit_NSTrackingArea")]
unsafe impl NSCoding for NSTrackingArea {}

#[cfg(feature = "AppKit_NSTrackingArea")]
unsafe impl NSObjectProtocol for NSTrackingArea {}

extern_methods!(
    #[cfg(feature = "AppKit_NSTrackingArea")]
    unsafe impl NSTrackingArea {
        #[cfg(feature = "Foundation_NSDictionary")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Init initWithRect:options:owner:userInfo:)]
        pub unsafe fn initWithRect_options_owner_userInfo(
            this: Option<Allocated<Self>>,
            rect: NSRect,
            options: NSTrackingAreaOptions,
            owner: Option<&Object>,
            user_info: Option<&NSDictionary<Object, Object>>,
        ) -> Id<Self>;

        #[cfg(not(any(target_os = "ios")))]
        #[method(rect)]
        pub unsafe fn rect(&self) -> NSRect;

        #[cfg(not(any(target_os = "ios")))]
        #[method(options)]
        pub unsafe fn options(&self) -> NSTrackingAreaOptions;

        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other owner)]
        pub unsafe fn owner(&self) -> Option<Id<Object>>;

        #[cfg(feature = "Foundation_NSDictionary")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other userInfo)]
        pub unsafe fn userInfo(&self) -> Option<Id<NSDictionary<Object, Object>>>;
    }
);
