//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSButton")]
    pub struct NSButton;

    #[cfg(feature = "AppKit_NSButton")]
    unsafe impl ClassType for NSButton {
        #[inherits(NSView, NSResponder, NSObject)]
        type Super = NSControl;
    }
);

#[cfg(feature = "AppKit_NSButton")]
unsafe impl NSAccessibility for NSButton {}

#[cfg(feature = "AppKit_NSButton")]
unsafe impl NSAccessibilityButton for NSButton {}

#[cfg(feature = "AppKit_NSButton")]
unsafe impl NSAccessibilityElementProtocol for NSButton {}

#[cfg(feature = "AppKit_NSButton")]
unsafe impl NSAnimatablePropertyContainer for NSButton {}

#[cfg(feature = "AppKit_NSButton")]
unsafe impl NSAppearanceCustomization for NSButton {}

#[cfg(feature = "AppKit_NSButton")]
unsafe impl NSCoding for NSButton {}

#[cfg(feature = "AppKit_NSButton")]
unsafe impl NSDraggingDestination for NSButton {}

#[cfg(feature = "AppKit_NSButton")]
unsafe impl NSObjectProtocol for NSButton {}

#[cfg(feature = "AppKit_NSButton")]
unsafe impl NSUserInterfaceCompression for NSButton {}

#[cfg(feature = "AppKit_NSButton")]
unsafe impl NSUserInterfaceItemIdentification for NSButton {}

#[cfg(feature = "AppKit_NSButton")]
unsafe impl NSUserInterfaceValidations for NSButton {}

extern_methods!(
    #[cfg(feature = "AppKit_NSButton")]
    unsafe impl NSButton {
        #[cfg(all(feature = "AppKit_NSImage", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other buttonWithTitle:image:target:action:)]
        pub unsafe fn buttonWithTitle_image_target_action(
            title: &NSString,
            image: &NSImage,
            target: Option<&Object>,
            action: Option<Sel>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other buttonWithTitle:target:action:)]
        pub unsafe fn buttonWithTitle_target_action(
            title: &NSString,
            target: Option<&Object>,
            action: Option<Sel>,
        ) -> Id<Self>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other buttonWithImage:target:action:)]
        pub unsafe fn buttonWithImage_target_action(
            image: &NSImage,
            target: Option<&Object>,
            action: Option<Sel>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other checkboxWithTitle:target:action:)]
        pub unsafe fn checkboxWithTitle_target_action(
            title: &NSString,
            target: Option<&Object>,
            action: Option<Sel>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other radioButtonWithTitle:target:action:)]
        pub unsafe fn radioButtonWithTitle_target_action(
            title: &NSString,
            target: Option<&Object>,
            action: Option<Sel>,
        ) -> Id<Self>;

        #[method(setButtonType:)]
        pub unsafe fn setButtonType(&self, r#type: NSButtonType);

        #[cfg(feature = "Foundation_NSString")]
        /**
          The title displayed on the button when it’s in an off state, or an empty string if the button does not display a title. By default, a button's title is "Button".
        */
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        /**
          The title displayed on the button when it’s in an off state, or an empty string if the button does not display a title. By default, a button's title is "Button".
        */
        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: &NSString);

        #[cfg(feature = "Foundation_NSAttributedString")]
        /**
          The button's title, expressed as an attributed string.
        */
        #[method_id(@__retain_semantics Other attributedTitle)]
        pub unsafe fn attributedTitle(&self) -> Id<NSAttributedString>;

        #[cfg(feature = "Foundation_NSAttributedString")]
        /**
          The button's title, expressed as an attributed string.
        */
        #[method(setAttributedTitle:)]
        pub unsafe fn setAttributedTitle(&self, attributed_title: &NSAttributedString);

        #[cfg(feature = "Foundation_NSString")]
        /**
          The title that the button displays when the button is in an on state, or an empty string if there is no such title. Note that some button types do not display an alternate title.
        */
        #[method_id(@__retain_semantics Other alternateTitle)]
        pub unsafe fn alternateTitle(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        /**
          The title that the button displays when the button is in an on state, or an empty string if there is no such title. Note that some button types do not display an alternate title.
        */
        #[method(setAlternateTitle:)]
        pub unsafe fn setAlternateTitle(&self, alternate_title: &NSString);

        #[cfg(feature = "Foundation_NSAttributedString")]
        /**
          The alternate title, expressed as an attributed string.
        */
        #[method_id(@__retain_semantics Other attributedAlternateTitle)]
        pub unsafe fn attributedAlternateTitle(&self) -> Id<NSAttributedString>;

        #[cfg(feature = "Foundation_NSAttributedString")]
        /**
          The alternate title, expressed as an attributed string.
        */
        #[method(setAttributedAlternateTitle:)]
        pub unsafe fn setAttributedAlternateTitle(
            &self,
            attributed_alternate_title: &NSAttributedString,
        );

        /**
          Indicates whether the button's action has a destructive effect on user data.  AppKit may guard a destructive-actioned button against accidental presses, and may give the button a special appearance in certain contexts to caution against unintentional use.  Defaults to NO.
        */
        #[method(hasDestructiveAction)]
        pub unsafe fn hasDestructiveAction(&self) -> bool;

        /**
          Indicates whether the button's action has a destructive effect on user data.  AppKit may guard a destructive-actioned button against accidental presses, and may give the button a special appearance in certain contexts to caution against unintentional use.  Defaults to NO.
        */
        #[method(setHasDestructiveAction:)]
        pub unsafe fn setHasDestructiveAction(&self, has_destructive_action: bool);

        #[cfg(feature = "AppKit_NSSound")]
        /**
          The sound that plays when the user clicks the button, or nil if the button should not play a sound. The default value is nil.
        */
        #[method_id(@__retain_semantics Other sound)]
        pub unsafe fn sound(&self) -> Option<Id<NSSound>>;

        #[cfg(feature = "AppKit_NSSound")]
        /**
          The sound that plays when the user clicks the button, or nil if the button should not play a sound. The default value is nil.
        */
        #[method(setSound:)]
        pub unsafe fn setSound(&self, sound: Option<&NSSound>);

        /**
          Sends action on deep-press or extended hover while dragging. Defaults to NO.
        */
        #[method(isSpringLoaded)]
        pub unsafe fn isSpringLoaded(&self) -> bool;

        /**
          Sends action on deep-press or extended hover while dragging. Defaults to NO.
        */
        #[method(setSpringLoaded:)]
        pub unsafe fn setSpringLoaded(&self, spring_loaded: bool);

        /**
          Configures the maximum allowed level for an NSMultiLevelAcceleratorButton, allowed values range from [1,5]. Defaults to 2.
        */
        #[method(maxAcceleratorLevel)]
        pub unsafe fn maxAcceleratorLevel(&self) -> NSInteger;

        /**
          Configures the maximum allowed level for an NSMultiLevelAcceleratorButton, allowed values range from [1,5]. Defaults to 2.
        */
        #[method(setMaxAcceleratorLevel:)]
        pub unsafe fn setMaxAcceleratorLevel(&self, max_accelerator_level: NSInteger);

        #[method(setPeriodicDelay:interval:)]
        pub unsafe fn setPeriodicDelay_interval(&self, delay: c_float, interval: c_float);

        #[method(getPeriodicDelay:interval:)]
        pub unsafe fn getPeriodicDelay_interval(
            &self,
            delay: NonNull<c_float>,
            interval: NonNull<c_float>,
        );

        /**
          The bezel style of the button, which provides a set of bezel artwork, layout metrics, and content styling from a set of system-provided styles. See the NSBezelStyle enumeration for a list of available styles. The bezel style is not used if the `bordered` property is set to `NO`.
        */
        #[method(bezelStyle)]
        pub unsafe fn bezelStyle(&self) -> NSBezelStyle;

        /**
          The bezel style of the button, which provides a set of bezel artwork, layout metrics, and content styling from a set of system-provided styles. See the NSBezelStyle enumeration for a list of available styles. The bezel style is not used if the `bordered` property is set to `NO`.
        */
        #[method(setBezelStyle:)]
        pub unsafe fn setBezelStyle(&self, bezel_style: NSBezelStyle);

        /**
          A Boolean value that determines whether the button draws a border.
        */
        #[method(isBordered)]
        pub unsafe fn isBordered(&self) -> bool;

        /**
          A Boolean value that determines whether the button draws a border.
        */
        #[method(setBordered:)]
        pub unsafe fn setBordered(&self, bordered: bool);

        /**
          A Boolean value that indicates whether the button is transparent. A transparent button never draws itself, but it receives mouse events, sends its action, and tracks the mouse properly.
        */
        #[method(isTransparent)]
        pub unsafe fn isTransparent(&self) -> bool;

        /**
          A Boolean value that indicates whether the button is transparent. A transparent button never draws itself, but it receives mouse events, sends its action, and tracks the mouse properly.
        */
        #[method(setTransparent:)]
        pub unsafe fn setTransparent(&self, transparent: bool);

        /**
          A Boolean value that determines whether the button displays its border only when the pointer is over it.
        */
        #[method(showsBorderOnlyWhileMouseInside)]
        pub unsafe fn showsBorderOnlyWhileMouseInside(&self) -> bool;

        /**
          A Boolean value that determines whether the button displays its border only when the pointer is over it.
        */
        #[method(setShowsBorderOnlyWhileMouseInside:)]
        pub unsafe fn setShowsBorderOnlyWhileMouseInside(
            &self,
            shows_border_only_while_mouse_inside: bool,
        );

        #[cfg(feature = "AppKit_NSImage")]
        /**
          The image that appears on the button when it’s in an off state, or nil if there is no such image.
        */
        #[method_id(@__retain_semantics Other image)]
        pub unsafe fn image(&self) -> Option<Id<NSImage>>;

        #[cfg(feature = "AppKit_NSImage")]
        /**
          The image that appears on the button when it’s in an off state, or nil if there is no such image.
        */
        #[method(setImage:)]
        pub unsafe fn setImage(&self, image: Option<&NSImage>);

        #[cfg(feature = "AppKit_NSImage")]
        /**
          An alternate image that appears on the button when the button is in an on state, or nil if there is no such image. Note that some button types do not display an alternate image.
        */
        #[method_id(@__retain_semantics Other alternateImage)]
        pub unsafe fn alternateImage(&self) -> Option<Id<NSImage>>;

        #[cfg(feature = "AppKit_NSImage")]
        /**
          An alternate image that appears on the button when the button is in an on state, or nil if there is no such image. Note that some button types do not display an alternate image.
        */
        #[method(setAlternateImage:)]
        pub unsafe fn setAlternateImage(&self, alternate_image: Option<&NSImage>);

        /**
          The position of the button's image relative to its title. See the NSCellImagePosition enumeration for possible values.
        */
        #[method(imagePosition)]
        pub unsafe fn imagePosition(&self) -> NSCellImagePosition;

        /**
          The position of the button's image relative to its title. See the NSCellImagePosition enumeration for possible values.
        */
        #[method(setImagePosition:)]
        pub unsafe fn setImagePosition(&self, image_position: NSCellImagePosition);

        /**
          The scaling mode applied to make the button's image fit within its bounds.
        */
        #[method(imageScaling)]
        pub unsafe fn imageScaling(&self) -> NSImageScaling;

        /**
          The scaling mode applied to make the button's image fit within its bounds.
        */
        #[method(setImageScaling:)]
        pub unsafe fn setImageScaling(&self, image_scaling: NSImageScaling);

        /**
          A Boolean value that determines how the button's image and title are positioned together within the button bezel. If false, the image is positioned according to the imagePosition property at the edge of the button bezel, and the title is positioned within the remaining space. If true, the button’s image is positioned directly adjacent to the title based on the imagePosition property, and the image and title are positioned within the button bezel as a single unit.
        */
        #[method(imageHugsTitle)]
        pub unsafe fn imageHugsTitle(&self) -> bool;

        /**
          A Boolean value that determines how the button's image and title are positioned together within the button bezel. If false, the image is positioned according to the imagePosition property at the edge of the button bezel, and the title is positioned within the remaining space. If true, the button’s image is positioned directly adjacent to the title based on the imagePosition property, and the image and title are positioned within the button bezel as a single unit.
        */
        #[method(setImageHugsTitle:)]
        pub unsafe fn setImageHugsTitle(&self, image_hugs_title: bool);

        #[cfg(feature = "AppKit_NSImageSymbolConfiguration")]
        /**
         Specifies a combination of point size, weight, and scale to use when sizing and displaying symbol images. If a symbol configuration isn't provided, the symbol is matched to the button's `font` property. The default value is nil.
        */
        #[method_id(@__retain_semantics Other symbolConfiguration)]
        pub unsafe fn symbolConfiguration(&self) -> Option<Id<NSImageSymbolConfiguration>>;

        #[cfg(feature = "AppKit_NSImageSymbolConfiguration")]
        /**
         Specifies a combination of point size, weight, and scale to use when sizing and displaying symbol images. If a symbol configuration isn't provided, the symbol is matched to the button's `font` property. The default value is nil.
        */
        #[method(setSymbolConfiguration:)]
        pub unsafe fn setSymbolConfiguration(
            &self,
            symbol_configuration: Option<&NSImageSymbolConfiguration>,
        );

        #[cfg(feature = "AppKit_NSColor")]
        /**
          Applies a custom color to the button's bezel, in appearances that support it. A nil value indicates an unmodified button appearance. The default value is nil.
        */
        #[method_id(@__retain_semantics Other bezelColor)]
        pub unsafe fn bezelColor(&self) -> Option<Id<NSColor>>;

        #[cfg(feature = "AppKit_NSColor")]
        /**
          Applies a custom color to the button's bezel, in appearances that support it. A nil value indicates an unmodified button appearance. The default value is nil.
        */
        #[method(setBezelColor:)]
        pub unsafe fn setBezelColor(&self, bezel_color: Option<&NSColor>);

        #[cfg(feature = "AppKit_NSColor")]
        /**
          Applies a tint color to template image and text content, in combination with other theme-appropriate effects. Only applicable to borderless buttons. A nil value indicates the standard set of effects without color modification. The default value is nil. Non-template images and attributed string values are not affected by the contentTintColor.
        */
        #[method_id(@__retain_semantics Other contentTintColor)]
        pub unsafe fn contentTintColor(&self) -> Option<Id<NSColor>>;

        #[cfg(feature = "AppKit_NSColor")]
        /**
          Applies a tint color to template image and text content, in combination with other theme-appropriate effects. Only applicable to borderless buttons. A nil value indicates the standard set of effects without color modification. The default value is nil. Non-template images and attributed string values are not affected by the contentTintColor.
        */
        #[method(setContentTintColor:)]
        pub unsafe fn setContentTintColor(&self, content_tint_color: Option<&NSColor>);

        /**
          The button's state. Buttons support the off and on states, and an additional mixed state depending on the value of the `allowsMixedState` property.
        */
        #[method(state)]
        pub unsafe fn state(&self) -> NSControlStateValue;

        /**
          The button's state. Buttons support the off and on states, and an additional mixed state depending on the value of the `allowsMixedState` property.
        */
        #[method(setState:)]
        pub unsafe fn setState(&self, state: NSControlStateValue);

        /**
          A Boolean value that indicates whether the button allows a mixed state. If NO, the button has two states (on and off), and if YES, the button has three states (on, off, and mixed). The mixed state is commonly used with checkboxes and radio buttons to indicate a value which is partially on.
        */
        #[method(allowsMixedState)]
        pub unsafe fn allowsMixedState(&self) -> bool;

        /**
          A Boolean value that indicates whether the button allows a mixed state. If NO, the button has two states (on and off), and if YES, the button has three states (on, off, and mixed). The mixed state is commonly used with checkboxes and radio buttons to indicate a value which is partially on.
        */
        #[method(setAllowsMixedState:)]
        pub unsafe fn setAllowsMixedState(&self, allows_mixed_state: bool);

        #[method(setNextState)]
        pub unsafe fn setNextState(&self);

        #[method(highlight:)]
        pub unsafe fn highlight(&self, flag: bool);

        #[cfg(feature = "Foundation_NSString")]
        /**
          This property contains the button's key equivalent, or the empty string if no equivalent has been defined. Buttons don’t have a default key equivalent. Setting the key equivalent to the Return character causes it to act as the default button for its window.
        */
        #[method_id(@__retain_semantics Other keyEquivalent)]
        pub unsafe fn keyEquivalent(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        /**
          This property contains the button's key equivalent, or the empty string if no equivalent has been defined. Buttons don’t have a default key equivalent. Setting the key equivalent to the Return character causes it to act as the default button for its window.
        */
        #[method(setKeyEquivalent:)]
        pub unsafe fn setKeyEquivalent(&self, key_equivalent: &NSString);

        /**
          A bitmask specifying the modifier keys that are applied to the button's key equivalent. Mask bits are defined by the NSEventModifierFlags option set. The only mask bits relevant in button key-equivalent modifier masks are NSEventModifierFlagControl, NSEventModifierFlagOption, and NSEventModifierFlagCommand.
        */
        #[method(keyEquivalentModifierMask)]
        pub unsafe fn keyEquivalentModifierMask(&self) -> NSEventModifierFlags;

        /**
          A bitmask specifying the modifier keys that are applied to the button's key equivalent. Mask bits are defined by the NSEventModifierFlags option set. The only mask bits relevant in button key-equivalent modifier masks are NSEventModifierFlagControl, NSEventModifierFlagOption, and NSEventModifierFlagCommand.
        */
        #[method(setKeyEquivalentModifierMask:)]
        pub unsafe fn setKeyEquivalentModifierMask(
            &self,
            key_equivalent_modifier_mask: NSEventModifierFlags,
        );

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(performKeyEquivalent:)]
        pub unsafe fn performKeyEquivalent(&self, key: &NSEvent) -> bool;

        #[cfg(all(
            feature = "AppKit_NSUserInterfaceCompressionOptions",
            feature = "Foundation_NSArray"
        ))]
        #[method(compressWithPrioritizedCompressionOptions:)]
        pub unsafe fn compressWithPrioritizedCompressionOptions(
            &self,
            prioritized_options: &NSArray<NSUserInterfaceCompressionOptions>,
        );

        #[cfg(all(
            feature = "AppKit_NSUserInterfaceCompressionOptions",
            feature = "Foundation_NSArray"
        ))]
        #[method(minimumSizeWithPrioritizedCompressionOptions:)]
        pub unsafe fn minimumSizeWithPrioritizedCompressionOptions(
            &self,
            prioritized_options: &NSArray<NSUserInterfaceCompressionOptions>,
        ) -> NSSize;

        #[cfg(feature = "AppKit_NSUserInterfaceCompressionOptions")]
        #[method_id(@__retain_semantics Other activeCompressionOptions)]
        pub unsafe fn activeCompressionOptions(&self) -> Id<NSUserInterfaceCompressionOptions>;
    }
);

extern_methods!(
    /// NSButtonDeprecated
    #[cfg(feature = "AppKit_NSButton")]
    unsafe impl NSButton {
        #[cfg(feature = "Foundation_NSString")]
        #[deprecated = "Mnemonics are not used on macOS. Set the title property directly instead."]
        #[method(setTitleWithMnemonic:)]
        pub unsafe fn setTitleWithMnemonic(&self, string_with_ampersand: Option<&NSString>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSControl`
    #[cfg(feature = "AppKit_NSButton")]
    unsafe impl NSButton {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Option<Allocated<Self>>, frame_rect: NSRect) -> Id<Self>;
    }
);
