//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameKit::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum GKAccessPointLocation {
        GKAccessPointLocationTopLeading = 0,
        GKAccessPointLocationTopTrailing = 1,
        GKAccessPointLocationBottomLeading = 2,
        GKAccessPointLocationBottomTrailing = 3,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameKit_GKAccessPoint")]
    pub struct GKAccessPoint;

    #[cfg(feature = "GameKit_GKAccessPoint")]
    unsafe impl ClassType for GKAccessPoint {
        type Super = NSObject;
    }
);

#[cfg(feature = "GameKit_GKAccessPoint")]
unsafe impl NSObjectProtocol for GKAccessPoint {}

extern_methods!(
    #[cfg(feature = "GameKit_GKAccessPoint")]
    unsafe impl GKAccessPoint {
        #[method_id(@__retain_semantics Other shared)]
        pub unsafe fn shared() -> Id<GKAccessPoint>;

        /**
          set this true to enable access point in your app.  Setting this will cause the access point to appear after the notification banner is presented.  If it already was presented it will appear immediately
        */
        #[method(isActive)]
        pub unsafe fn isActive(&self) -> bool;

        /**
          set this true to enable access point in your app.  Setting this will cause the access point to appear after the notification banner is presented.  If it already was presented it will appear immediately
        */
        #[method(setActive:)]
        pub unsafe fn setActive(&self, active: bool);

        /**
          set this on tvOS to put the accessPoint into focused mode
        */
        #[method(isFocused)]
        pub unsafe fn isFocused(&self) -> bool;

        /**
          set this on tvOS to put the accessPoint into focused mode
        */
        #[method(setFocused:)]
        pub unsafe fn setFocused(&self, focused: bool);

        #[method(isVisible)]
        pub unsafe fn isVisible(&self) -> bool;

        /**
          observable property that indicates when the access point is visible.
        */
        #[method(isPresentingGameCenter)]
        pub unsafe fn isPresentingGameCenter(&self) -> bool;

        /**
          Set this property to true if you wish to show the highlights for most recent acheivement, current rank on default leaderboard, etc
        */
        #[method(showHighlights)]
        pub unsafe fn showHighlights(&self) -> bool;

        /**
          Set this property to true if you wish to show the highlights for most recent acheivement, current rank on default leaderboard, etc
        */
        #[method(setShowHighlights:)]
        pub unsafe fn setShowHighlights(&self, show_highlights: bool);

        /**
          These properties control the placement of the widget
        */
        #[method(location)]
        pub unsafe fn location(&self) -> GKAccessPointLocation;

        /**
          These properties control the placement of the widget
        */
        #[method(setLocation:)]
        pub unsafe fn setLocation(&self, location: GKAccessPointLocation);

        #[method(frameInScreenCoordinates)]
        pub unsafe fn frameInScreenCoordinates(&self) -> NSRect;

        #[cfg(feature = "AppKit_NSWindow")]
        #[method_id(@__retain_semantics Other parentWindow)]
        pub unsafe fn parentWindow(&self) -> Option<Id<NSWindow>>;

        #[cfg(feature = "AppKit_NSWindow")]
        #[method(setParentWindow:)]
        pub unsafe fn setParentWindow(&self, parent_window: Option<&NSWindow>);

        #[method(triggerAccessPointWithHandler:)]
        pub unsafe fn triggerAccessPointWithHandler(&self, handler: &Block<(), ()>);

        #[method(triggerAccessPointWithState:handler:)]
        pub unsafe fn triggerAccessPointWithState_handler(
            &self,
            state: GKGameCenterViewControllerState,
            handler: &Block<(), ()>,
        );
    }
);
