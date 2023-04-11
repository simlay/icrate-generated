//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSColorPickerTouchBarItem")]
    pub struct NSColorPickerTouchBarItem;

    #[cfg(feature = "AppKit_NSColorPickerTouchBarItem")]
    unsafe impl ClassType for NSColorPickerTouchBarItem {
        #[inherits(NSObject)]
        type Super = NSTouchBarItem;
    }
);

#[cfg(feature = "AppKit_NSColorPickerTouchBarItem")]
unsafe impl NSCoding for NSColorPickerTouchBarItem {}

#[cfg(feature = "AppKit_NSColorPickerTouchBarItem")]
unsafe impl NSObjectProtocol for NSColorPickerTouchBarItem {}

extern_methods!(
    #[cfg(feature = "AppKit_NSColorPickerTouchBarItem")]
    unsafe impl NSColorPickerTouchBarItem {
        #[method_id(@__retain_semantics Other colorPickerWithIdentifier:)]
        pub unsafe fn colorPickerWithIdentifier(identifier: &NSTouchBarItemIdentifier) -> Id<Self>;

        #[method_id(@__retain_semantics Other textColorPickerWithIdentifier:)]
        pub unsafe fn textColorPickerWithIdentifier(
            identifier: &NSTouchBarItemIdentifier,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Other strokeColorPickerWithIdentifier:)]
        pub unsafe fn strokeColorPickerWithIdentifier(
            identifier: &NSTouchBarItemIdentifier,
        ) -> Id<Self>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other colorPickerWithIdentifier:buttonImage:)]
        pub unsafe fn colorPickerWithIdentifier_buttonImage(
            identifier: &NSTouchBarItemIdentifier,
            image: &NSImage,
        ) -> Id<Self>;

        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Other color)]
        pub unsafe fn color(&self) -> Id<NSColor>;

        #[cfg(feature = "AppKit_NSColor")]
        #[method(setColor:)]
        pub unsafe fn setColor(&self, color: &NSColor);

        /**
          Whether or not the picker should allow picking a color with non-1.0 alpha. Defaults to `!NSColor.ignoresAlpha`.
        */
        #[method(showsAlpha)]
        pub unsafe fn showsAlpha(&self) -> bool;

        /**
          Whether or not the picker should allow picking a color with non-1.0 alpha. Defaults to `!NSColor.ignoresAlpha`.
        */
        #[method(setShowsAlpha:)]
        pub unsafe fn setShowsAlpha(&self, shows_alpha: bool);

        #[cfg(all(feature = "AppKit_NSColorSpace", feature = "Foundation_NSArray"))]
        /**
          Controls the color spaces that the receiver is able to produce. If a color outside of the allowed spaces are displayed or selected, it will first be converted to the first color space in the array. `nil` signifies any color space is allowed. Empty array is an invalid value and will raise an exception if set. Defaults to `nil`.
        */
        #[method_id(@__retain_semantics Other allowedColorSpaces)]
        pub unsafe fn allowedColorSpaces(&self) -> Option<Id<NSArray<NSColorSpace>>>;

        #[cfg(all(feature = "AppKit_NSColorSpace", feature = "Foundation_NSArray"))]
        /**
          Controls the color spaces that the receiver is able to produce. If a color outside of the allowed spaces are displayed or selected, it will first be converted to the first color space in the array. `nil` signifies any color space is allowed. Empty array is an invalid value and will raise an exception if set. Defaults to `nil`.
        */
        #[method(setAllowedColorSpaces:)]
        pub unsafe fn setAllowedColorSpaces(
            &self,
            allowed_color_spaces: Option<&NSArray<NSColorSpace>>,
        );

        #[cfg(feature = "AppKit_NSColorList")]
        /**
          The color list displayed in the list color picker. Defaults to the standard system color list. Setting a custom color list will disable the additional tints/shades that appear on long-press.
        */
        #[method_id(@__retain_semantics Other colorList)]
        pub unsafe fn colorList(&self) -> Option<Id<NSColorList>>;

        #[cfg(feature = "AppKit_NSColorList")]
        /**
          The color list displayed in the list color picker. Defaults to the standard system color list. Setting a custom color list will disable the additional tints/shades that appear on long-press.
        */
        #[method(setColorList:)]
        pub unsafe fn setColorList(&self, color_list: Option<&NSColorList>);

        #[cfg(feature = "Foundation_NSString")]
        /**
          The localized string labelling this item during user customization. The default value is the localized string of "Color Picker".
        */
        #[method_id(@__retain_semantics Other customizationLabel)]
        pub unsafe fn customizationLabel(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        /**
          The localized string labelling this item during user customization. The default value is the localized string of "Color Picker".
        */
        #[method(setCustomizationLabel:)]
        pub unsafe fn setCustomizationLabel(&self, customization_label: Option<&NSString>);

        #[method_id(@__retain_semantics Other target)]
        pub unsafe fn target(&self) -> Option<Id<Object>>;

        #[method(setTarget:)]
        pub unsafe fn setTarget(&self, target: Option<&Object>);

        #[method(action)]
        pub unsafe fn action(&self) -> Option<Sel>;

        #[method(setAction:)]
        pub unsafe fn setAction(&self, action: Option<Sel>);

        /**
          Enables or disabled the color picker. If it is currently being shown in a popover, it will be dismissed.
        */
        #[method(isEnabled)]
        pub unsafe fn isEnabled(&self) -> bool;

        /**
          Enables or disabled the color picker. If it is currently being shown in a popover, it will be dismissed.
        */
        #[method(setEnabled:)]
        pub unsafe fn setEnabled(&self, enabled: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSTouchBarItem`
    #[cfg(feature = "AppKit_NSColorPickerTouchBarItem")]
    unsafe impl NSColorPickerTouchBarItem {
        #[method_id(@__retain_semantics Init initWithIdentifier:)]
        pub unsafe fn initWithIdentifier(
            this: Option<Allocated<Self>>,
            identifier: &NSTouchBarItemIdentifier,
        ) -> Id<Self>;
    }
);
