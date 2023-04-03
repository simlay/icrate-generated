//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    #[cfg(not(any(target_os = "ios")))]
    pub enum NSVisualEffectMaterial {
        #[cfg(not(any(target_os = "ios")))]
        NSVisualEffectMaterialTitlebar = 3,
        #[cfg(not(any(target_os = "ios")))]
        NSVisualEffectMaterialSelection = 4,
        #[cfg(not(any(target_os = "ios")))]
        NSVisualEffectMaterialMenu = 5,
        #[cfg(not(any(target_os = "ios")))]
        NSVisualEffectMaterialPopover = 6,
        #[cfg(not(any(target_os = "ios")))]
        NSVisualEffectMaterialSidebar = 7,
        #[cfg(not(any(target_os = "ios")))]
        NSVisualEffectMaterialHeaderView = 10,
        #[cfg(not(any(target_os = "ios")))]
        NSVisualEffectMaterialSheet = 11,
        #[cfg(not(any(target_os = "ios")))]
        NSVisualEffectMaterialWindowBackground = 12,
        #[cfg(not(any(target_os = "ios")))]
        NSVisualEffectMaterialHUDWindow = 13,
        #[cfg(not(any(target_os = "ios")))]
        NSVisualEffectMaterialFullScreenUI = 15,
        #[cfg(not(any(target_os = "ios")))]
        NSVisualEffectMaterialToolTip = 17,
        #[cfg(not(any(target_os = "ios")))]
        NSVisualEffectMaterialContentBackground = 18,
        #[cfg(not(any(target_os = "ios")))]
        NSVisualEffectMaterialUnderWindowBackground = 21,
        #[cfg(not(any(target_os = "ios")))]
        NSVisualEffectMaterialUnderPageBackground = 22,
        #[deprecated = "Use a specific semantic material instead."]
        #[cfg(not(any(target_os = "ios")))]
        NSVisualEffectMaterialAppearanceBased = 0,
        #[deprecated = "Use a semantic material instead.  To force the appearance of a view hierarchy, set the `appearance` property to an appropriate NSAppearance value."]
        #[cfg(not(any(target_os = "ios")))]
        NSVisualEffectMaterialLight = 1,
        #[deprecated = "Use a semantic material instead.  To force the appearance of a view hierarchy, set the `appearance` property to an appropriate NSAppearance value."]
        #[cfg(not(any(target_os = "ios")))]
        NSVisualEffectMaterialDark = 2,
        #[deprecated = "Use a semantic material instead.  To force the appearance of a view hierarchy, set the `appearance` property to an appropriate NSAppearance value."]
        #[cfg(not(any(target_os = "ios")))]
        NSVisualEffectMaterialMediumLight = 8,
        #[deprecated = "Use a semantic material instead.  To force the appearance of a view hierarchy, set the `appearance` property to an appropriate NSAppearance value."]
        #[cfg(not(any(target_os = "ios")))]
        NSVisualEffectMaterialUltraDark = 9,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    #[cfg(not(any(target_os = "ios")))]
    pub enum NSVisualEffectBlendingMode {
        #[cfg(not(any(target_os = "ios")))]
        NSVisualEffectBlendingModeBehindWindow = 0,
        #[cfg(not(any(target_os = "ios")))]
        NSVisualEffectBlendingModeWithinWindow = 1,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    #[cfg(not(any(target_os = "ios")))]
    pub enum NSVisualEffectState {
        #[cfg(not(any(target_os = "ios")))]
        NSVisualEffectStateFollowsWindowActiveState = 0,
        #[cfg(not(any(target_os = "ios")))]
        NSVisualEffectStateActive = 1,
        #[cfg(not(any(target_os = "ios")))]
        NSVisualEffectStateInactive = 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSVisualEffectView")]
    #[cfg(not(any(target_os = "ios")))]
    pub struct NSVisualEffectView;

    #[cfg(feature = "AppKit_NSVisualEffectView")]
    unsafe impl ClassType for NSVisualEffectView {
        #[inherits(NSResponder, NSObject)]
        type Super = NSView;
    }
);

#[cfg(feature = "AppKit_NSVisualEffectView")]
unsafe impl NSAccessibility for NSVisualEffectView {}

#[cfg(feature = "AppKit_NSVisualEffectView")]
unsafe impl NSAccessibilityElementProtocol for NSVisualEffectView {}

#[cfg(feature = "AppKit_NSVisualEffectView")]
unsafe impl NSAnimatablePropertyContainer for NSVisualEffectView {}

#[cfg(feature = "AppKit_NSVisualEffectView")]
unsafe impl NSAppearanceCustomization for NSVisualEffectView {}

#[cfg(feature = "AppKit_NSVisualEffectView")]
unsafe impl NSCoding for NSVisualEffectView {}

#[cfg(feature = "AppKit_NSVisualEffectView")]
unsafe impl NSDraggingDestination for NSVisualEffectView {}

#[cfg(feature = "AppKit_NSVisualEffectView")]
unsafe impl NSObjectProtocol for NSVisualEffectView {}

#[cfg(feature = "AppKit_NSVisualEffectView")]
unsafe impl NSUserInterfaceItemIdentification for NSVisualEffectView {}

extern_methods!(
    #[cfg(feature = "AppKit_NSVisualEffectView")]
    unsafe impl NSVisualEffectView {
        #[cfg(not(any(target_os = "ios")))]
        #[method(material)]
        pub unsafe fn material(&self) -> NSVisualEffectMaterial;

        #[cfg(not(any(target_os = "ios")))]
        #[method(setMaterial:)]
        pub unsafe fn setMaterial(&self, material: NSVisualEffectMaterial);

        #[cfg(not(any(target_os = "ios")))]
        #[method(interiorBackgroundStyle)]
        pub unsafe fn interiorBackgroundStyle(&self) -> NSBackgroundStyle;

        #[cfg(not(any(target_os = "ios")))]
        #[method(blendingMode)]
        pub unsafe fn blendingMode(&self) -> NSVisualEffectBlendingMode;

        #[cfg(not(any(target_os = "ios")))]
        #[method(setBlendingMode:)]
        pub unsafe fn setBlendingMode(&self, blending_mode: NSVisualEffectBlendingMode);

        #[cfg(not(any(target_os = "ios")))]
        #[method(state)]
        pub unsafe fn state(&self) -> NSVisualEffectState;

        #[cfg(not(any(target_os = "ios")))]
        #[method(setState:)]
        pub unsafe fn setState(&self, state: NSVisualEffectState);

        #[cfg(feature = "AppKit_NSImage")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other maskImage)]
        pub unsafe fn maskImage(&self) -> Option<Id<NSImage>>;

        #[cfg(feature = "AppKit_NSImage")]
        #[cfg(not(any(target_os = "ios")))]
        #[method(setMaskImage:)]
        pub unsafe fn setMaskImage(&self, mask_image: Option<&NSImage>);

        #[cfg(not(any(target_os = "ios")))]
        #[method(isEmphasized)]
        pub unsafe fn isEmphasized(&self) -> bool;

        #[cfg(not(any(target_os = "ios")))]
        #[method(setEmphasized:)]
        pub unsafe fn setEmphasized(&self, emphasized: bool);

        #[cfg(not(any(target_os = "ios")))]
        #[method(viewDidMoveToWindow)]
        pub unsafe fn viewDidMoveToWindow(&self);

        #[cfg(feature = "AppKit_NSWindow")]
        #[cfg(not(any(target_os = "ios")))]
        #[method(viewWillMoveToWindow:)]
        pub unsafe fn viewWillMoveToWindow(&self, new_window: Option<&NSWindow>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSView`
    #[cfg(feature = "AppKit_NSVisualEffectView")]
    unsafe impl NSVisualEffectView {
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Option<Allocated<Self>>, frame_rect: NSRect) -> Id<Self>;
    }
);
