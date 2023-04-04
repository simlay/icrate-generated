//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSSecureTextField")]
    pub struct NSSecureTextField;

    #[cfg(feature = "AppKit_NSSecureTextField")]
    unsafe impl ClassType for NSSecureTextField {
        #[inherits(NSControl, NSView, NSResponder, NSObject)]
        type Super = NSTextField;
    }
);

#[cfg(feature = "AppKit_NSSecureTextField")]
unsafe impl NSAccessibility for NSSecureTextField {}

#[cfg(feature = "AppKit_NSSecureTextField")]
unsafe impl NSAccessibilityElementProtocol for NSSecureTextField {}

#[cfg(feature = "AppKit_NSSecureTextField")]
unsafe impl NSAccessibilityNavigableStaticText for NSSecureTextField {}

#[cfg(feature = "AppKit_NSSecureTextField")]
unsafe impl NSAccessibilityStaticText for NSSecureTextField {}

#[cfg(feature = "AppKit_NSSecureTextField")]
unsafe impl NSAnimatablePropertyContainer for NSSecureTextField {}

#[cfg(feature = "AppKit_NSSecureTextField")]
unsafe impl NSAppearanceCustomization for NSSecureTextField {}

#[cfg(feature = "AppKit_NSSecureTextField")]
unsafe impl NSCoding for NSSecureTextField {}

#[cfg(feature = "AppKit_NSSecureTextField")]
unsafe impl NSDraggingDestination for NSSecureTextField {}

#[cfg(feature = "AppKit_NSSecureTextField")]
unsafe impl NSObjectProtocol for NSSecureTextField {}

#[cfg(feature = "AppKit_NSSecureTextField")]
unsafe impl NSTextContent for NSSecureTextField {}

#[cfg(feature = "AppKit_NSSecureTextField")]
unsafe impl NSUserInterfaceItemIdentification for NSSecureTextField {}

#[cfg(feature = "AppKit_NSSecureTextField")]
unsafe impl NSUserInterfaceValidations for NSSecureTextField {}

extern_methods!(
    #[cfg(feature = "AppKit_NSSecureTextField")]
    unsafe impl NSSecureTextField {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSSecureTextFieldCell")]
    pub struct NSSecureTextFieldCell;

    #[cfg(feature = "AppKit_NSSecureTextFieldCell")]
    unsafe impl ClassType for NSSecureTextFieldCell {
        #[inherits(NSActionCell, NSCell, NSObject)]
        type Super = NSTextFieldCell;
    }
);

#[cfg(feature = "AppKit_NSSecureTextFieldCell")]
unsafe impl NSAccessibility for NSSecureTextFieldCell {}

#[cfg(feature = "AppKit_NSSecureTextFieldCell")]
unsafe impl NSAccessibilityElementProtocol for NSSecureTextFieldCell {}

#[cfg(feature = "AppKit_NSSecureTextFieldCell")]
unsafe impl NSCoding for NSSecureTextFieldCell {}

#[cfg(feature = "AppKit_NSSecureTextFieldCell")]
unsafe impl NSObjectProtocol for NSSecureTextFieldCell {}

#[cfg(feature = "AppKit_NSSecureTextFieldCell")]
unsafe impl NSUserInterfaceItemIdentification for NSSecureTextFieldCell {}

extern_methods!(
    #[cfg(feature = "AppKit_NSSecureTextFieldCell")]
    unsafe impl NSSecureTextFieldCell {
        #[method(echosBullets)]
        pub unsafe fn echosBullets(&self) -> bool;

        #[method(setEchosBullets:)]
        pub unsafe fn setEchosBullets(&self, echos_bullets: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSTextField`
    ///
    /// NSTextFieldConvenience
    #[cfg(feature = "AppKit_NSSecureTextField")]
    unsafe impl NSSecureTextField {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other labelWithString:)]
        pub unsafe fn labelWithString(string_value: &NSString) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other wrappingLabelWithString:)]
        pub unsafe fn wrappingLabelWithString(string_value: &NSString) -> Id<Self>;

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method_id(@__retain_semantics Other labelWithAttributedString:)]
        pub unsafe fn labelWithAttributedString(
            attributed_string_value: &NSAttributedString,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other textFieldWithString:)]
        pub unsafe fn textFieldWithString(string_value: &NSString) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSControl`
    #[cfg(feature = "AppKit_NSSecureTextField")]
    unsafe impl NSSecureTextField {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Option<Allocated<Self>>, frame_rect: NSRect) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSTextFieldCell`
    #[cfg(feature = "AppKit_NSSecureTextFieldCell")]
    unsafe impl NSSecureTextFieldCell {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initTextCell:)]
        pub unsafe fn initTextCell(this: Option<Allocated<Self>>, string: &NSString) -> Id<Self>;

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Init initImageCell:)]
        pub unsafe fn initImageCell(
            this: Option<Allocated<Self>>,
            image: Option<&NSImage>,
        ) -> Id<Self>;
    }
);
