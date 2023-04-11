//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    #[deprecated]
    pub enum NSPopoverAppearance {
        #[deprecated]
        NSPopoverAppearanceMinimal = 0,
        #[deprecated]
        NSPopoverAppearanceHUD = 1,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    /**
       AppKit supports transient, semi-transient, and application-defined behaviors. Please see the class description above for more information.  The default popover behavior is NSPopoverBehaviorApplicationDefined.
    */
    pub enum NSPopoverBehavior {
        NSPopoverBehaviorApplicationDefined = 0,
        NSPopoverBehaviorTransient = 1,
        NSPopoverBehaviorSemitransient = 2,
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSPopover")]
    unsafe impl NSPopover {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self>>;

        /**
           The delegate of the popover. The delegate is not retained.
        */
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<ProtocolObject<dyn NSPopoverDelegate>>>;

        /**
           The delegate of the popover. The delegate is not retained.
        */
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&ProtocolObject<dyn NSPopoverDelegate>>);

        /**
           The behavior of the popover.  The default behavior is NSPopoverBehaviorApplicationDefined. See the declaration of NSPopoverBehavior above for more information about popover behaviors.
        */
        #[method(behavior)]
        pub unsafe fn behavior(&self) -> NSPopoverBehavior;

        /**
           The behavior of the popover.  The default behavior is NSPopoverBehaviorApplicationDefined. See the declaration of NSPopoverBehavior above for more information about popover behaviors.
        */
        #[method(setBehavior:)]
        pub unsafe fn setBehavior(&self, behavior: NSPopoverBehavior);

        /**
           Should the popover be animated when it shows, closes, or appears to transition to a detachable window.  This property also controls whether the popover animates when the content view or content size changes. AppKit does not guarantee which behaviors will be animated or that this property will be respected; it is regarded as a hint.  The default value is YES.
        */
        #[method(animates)]
        pub unsafe fn animates(&self) -> bool;

        /**
           Should the popover be animated when it shows, closes, or appears to transition to a detachable window.  This property also controls whether the popover animates when the content view or content size changes. AppKit does not guarantee which behaviors will be animated or that this property will be respected; it is regarded as a hint.  The default value is YES.
        */
        #[method(setAnimates:)]
        pub unsafe fn setAnimates(&self, animates: bool);

        #[cfg(feature = "AppKit_NSViewController")]
        /**
           The view controller that manages the content of the popover.  The default value is nil.  You must set the content view controller of the popover to a non-nil value before the popover is shown.  Changes to the popover's content view controller while the popover is shown will animate (provided animates is YES).
        */
        #[method_id(@__retain_semantics Other contentViewController)]
        pub unsafe fn contentViewController(&self) -> Option<Id<NSViewController>>;

        #[cfg(feature = "AppKit_NSViewController")]
        /**
           The view controller that manages the content of the popover.  The default value is nil.  You must set the content view controller of the popover to a non-nil value before the popover is shown.  Changes to the popover's content view controller while the popover is shown will animate (provided animates is YES).
        */
        #[method(setContentViewController:)]
        pub unsafe fn setContentViewController(
            &self,
            content_view_controller: Option<&NSViewController>,
        );

        /**
           The content size of the popover.  The popover's content size is set to match the size of the content view when the content view controller is set.  Changes to the content size of the popover will animate while the popover is shown (provided animates is YES).
        */
        #[method(contentSize)]
        pub unsafe fn contentSize(&self) -> NSSize;

        /**
           The content size of the popover.  The popover's content size is set to match the size of the content view when the content view controller is set.  Changes to the content size of the popover will animate while the popover is shown (provided animates is YES).
        */
        #[method(setContentSize:)]
        pub unsafe fn setContentSize(&self, content_size: NSSize);

        /**
           YES if the popover is being shown, NO otherwise. The popover is considered to be shown from the point when -showRelativeToRect:ofView:preferredEdge: is invoked until the popover is closed in response to an invocation of either -close or -performClose:.
        */
        #[method(isShown)]
        pub unsafe fn isShown(&self) -> bool;

        /**
          Returns \c YES if the window is detached to an implicitly created detached window, \c NO otherwise. This method does not apply when the popover is detached to a window returned with \c -detachableWindowForPopover:.
        */
        #[method(isDetached)]
        pub unsafe fn isDetached(&self) -> bool;

        /**
           Popovers are positioned relative to a positioning view and are automatically moved when the location or size of the positioning view changes.  Sometimes it is desirable to position popovers relative to a rectangle within the positioning view.  In this case, you must update the positioningRect binding whenever this rectangle changes, or use the positioningRect binding so AppKit can re-position the popover when appropriate.
        */
        #[method(positioningRect)]
        pub unsafe fn positioningRect(&self) -> NSRect;

        /**
           Popovers are positioned relative to a positioning view and are automatically moved when the location or size of the positioning view changes.  Sometimes it is desirable to position popovers relative to a rectangle within the positioning view.  In this case, you must update the positioningRect binding whenever this rectangle changes, or use the positioningRect binding so AppKit can re-position the popover when appropriate.
        */
        #[method(setPositioningRect:)]
        pub unsafe fn setPositioningRect(&self, positioning_rect: NSRect);

        #[cfg(feature = "AppKit_NSView")]
        #[method(showRelativeToRect:ofView:preferredEdge:)]
        pub unsafe fn showRelativeToRect_ofView_preferredEdge(
            &self,
            positioning_rect: NSRect,
            positioning_view: &NSView,
            preferred_edge: NSRectEdge,
        );

        #[method(performClose:)]
        pub unsafe fn performClose(&self, sender: Option<&Object>);

        #[method(close)]
        pub unsafe fn close(&self);
    }
);

extern_static!(NSPopoverCloseReasonKey: &'static NSString);

typed_enum!(
    pub type NSPopoverCloseReasonValue = NSString;
);

extern_static!(NSPopoverCloseReasonStandard: &'static NSPopoverCloseReasonValue);

extern_static!(NSPopoverCloseReasonDetachToWindow: &'static NSPopoverCloseReasonValue);

extern_static!(NSPopoverWillShowNotification: &'static NSNotificationName);

extern_static!(NSPopoverDidShowNotification: &'static NSNotificationName);

extern_static!(NSPopoverWillCloseNotification: &'static NSNotificationName);

extern_static!(NSPopoverDidCloseNotification: &'static NSNotificationName);

extern_protocol!(
    pub unsafe trait NSPopoverDelegate: NSObjectProtocol {
        #[cfg(feature = "AppKit_NSPopover")]
        #[optional]
        #[method(popoverShouldClose:)]
        unsafe fn popoverShouldClose(&self, popover: &NSPopover) -> bool;

        #[cfg(feature = "AppKit_NSPopover")]
        #[optional]
        #[method(popoverShouldDetach:)]
        unsafe fn popoverShouldDetach(&self, popover: &NSPopover) -> bool;

        #[cfg(feature = "AppKit_NSPopover")]
        #[optional]
        #[method(popoverDidDetach:)]
        unsafe fn popoverDidDetach(&self, popover: &NSPopover);

        #[cfg(all(feature = "AppKit_NSPopover", feature = "AppKit_NSWindow"))]
        #[optional]
        #[method_id(@__retain_semantics Other detachableWindowForPopover:)]
        unsafe fn detachableWindowForPopover(&self, popover: &NSPopover) -> Option<Id<NSWindow>>;

        #[cfg(feature = "Foundation_NSNotification")]
        #[optional]
        #[method(popoverWillShow:)]
        unsafe fn popoverWillShow(&self, notification: &NSNotification);

        #[cfg(feature = "Foundation_NSNotification")]
        #[optional]
        #[method(popoverDidShow:)]
        unsafe fn popoverDidShow(&self, notification: &NSNotification);

        #[cfg(feature = "Foundation_NSNotification")]
        #[optional]
        #[method(popoverWillClose:)]
        unsafe fn popoverWillClose(&self, notification: &NSNotification);

        #[cfg(feature = "Foundation_NSNotification")]
        #[optional]
        #[method(popoverDidClose:)]
        unsafe fn popoverDidClose(&self, notification: &NSNotification);
    }

    unsafe impl ProtocolType for dyn NSPopoverDelegate {}
);
