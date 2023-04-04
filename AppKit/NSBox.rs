//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSTitlePosition {
        NSNoTitle = 0,
        NSAboveTop = 1,
        NSAtTop = 2,
        NSBelowTop = 3,
        NSAboveBottom = 4,
        NSAtBottom = 5,
        NSBelowBottom = 6,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSBoxType {
        NSBoxPrimary = 0,
        NSBoxSeparator = 2,
        NSBoxCustom = 4,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSBox")]
    pub struct NSBox;

    #[cfg(feature = "AppKit_NSBox")]
    unsafe impl ClassType for NSBox {
        #[inherits(NSResponder, NSObject)]
        type Super = NSView;
    }
);

#[cfg(feature = "AppKit_NSBox")]
unsafe impl NSAccessibility for NSBox {}

#[cfg(feature = "AppKit_NSBox")]
unsafe impl NSAccessibilityElementProtocol for NSBox {}

#[cfg(feature = "AppKit_NSBox")]
unsafe impl NSAnimatablePropertyContainer for NSBox {}

#[cfg(feature = "AppKit_NSBox")]
unsafe impl NSAppearanceCustomization for NSBox {}

#[cfg(feature = "AppKit_NSBox")]
unsafe impl NSCoding for NSBox {}

#[cfg(feature = "AppKit_NSBox")]
unsafe impl NSDraggingDestination for NSBox {}

#[cfg(feature = "AppKit_NSBox")]
unsafe impl NSObjectProtocol for NSBox {}

#[cfg(feature = "AppKit_NSBox")]
unsafe impl NSUserInterfaceItemIdentification for NSBox {}

extern_methods!(
    #[cfg(feature = "AppKit_NSBox")]
    unsafe impl NSBox {
        #[method(boxType)]
        pub unsafe fn boxType(&self) -> NSBoxType;

        #[method(setBoxType:)]
        pub unsafe fn setBoxType(&self, box_type: NSBoxType);

        #[method(titlePosition)]
        pub unsafe fn titlePosition(&self) -> NSTitlePosition;

        #[method(setTitlePosition:)]
        pub unsafe fn setTitlePosition(&self, title_position: NSTitlePosition);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: &NSString);

        #[cfg(feature = "AppKit_NSFont")]
        #[method_id(@__retain_semantics Other titleFont)]
        pub unsafe fn titleFont(&self) -> Id<NSFont>;

        #[cfg(feature = "AppKit_NSFont")]
        #[method(setTitleFont:)]
        pub unsafe fn setTitleFont(&self, title_font: &NSFont);

        #[method(borderRect)]
        pub unsafe fn borderRect(&self) -> NSRect;

        #[method(titleRect)]
        pub unsafe fn titleRect(&self) -> NSRect;

        #[method_id(@__retain_semantics Other titleCell)]
        pub unsafe fn titleCell(&self) -> Id<Object>;

        #[method(contentViewMargins)]
        pub unsafe fn contentViewMargins(&self) -> NSSize;

        #[method(setContentViewMargins:)]
        pub unsafe fn setContentViewMargins(&self, content_view_margins: NSSize);

        #[method(sizeToFit)]
        pub unsafe fn sizeToFit(&self);

        #[method(setFrameFromContentFrame:)]
        pub unsafe fn setFrameFromContentFrame(&self, content_frame: NSRect);

        #[method_id(@__retain_semantics Other contentView)]
        pub unsafe fn contentView(&self) -> Option<Id<NSView>>;

        #[method(setContentView:)]
        pub unsafe fn setContentView(&self, content_view: Option<&NSView>);

        #[method(isTransparent)]
        pub unsafe fn isTransparent(&self) -> bool;

        #[method(setTransparent:)]
        pub unsafe fn setTransparent(&self, transparent: bool);

        #[method(borderWidth)]
        pub unsafe fn borderWidth(&self) -> CGFloat;

        #[method(setBorderWidth:)]
        pub unsafe fn setBorderWidth(&self, border_width: CGFloat);

        #[method(cornerRadius)]
        pub unsafe fn cornerRadius(&self) -> CGFloat;

        #[method(setCornerRadius:)]
        pub unsafe fn setCornerRadius(&self, corner_radius: CGFloat);

        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Other borderColor)]
        pub unsafe fn borderColor(&self) -> Id<NSColor>;

        #[cfg(feature = "AppKit_NSColor")]
        #[method(setBorderColor:)]
        pub unsafe fn setBorderColor(&self, border_color: &NSColor);

        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Other fillColor)]
        pub unsafe fn fillColor(&self) -> Id<NSColor>;

        #[cfg(feature = "AppKit_NSColor")]
        #[method(setFillColor:)]
        pub unsafe fn setFillColor(&self, fill_color: &NSColor);
    }
);

extern_methods!(
    /// NSDeprecated
    #[cfg(feature = "AppKit_NSBox")]
    unsafe impl NSBox {
        #[deprecated = "borderType is only applicable to NSBoxOldStyle, which is deprecated. To replace a borderType of NSNoBorder, use the `transparent` property."]
        #[method(borderType)]
        pub unsafe fn borderType(&self) -> NSBorderType;

        #[deprecated = "borderType is only applicable to NSBoxOldStyle, which is deprecated. To replace a borderType of NSNoBorder, use the `transparent` property."]
        #[method(setBorderType:)]
        pub unsafe fn setBorderType(&self, border_type: NSBorderType);

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(setTitleWithMnemonic:)]
        pub unsafe fn setTitleWithMnemonic(&self, string_with_ampersand: Option<&NSString>);
    }
);

extern_static!(NSBoxSecondary: NSBoxType = 1);

extern_static!(NSBoxOldStyle: NSBoxType = 3);

extern_methods!(
    /// Methods declared on superclass `NSView`
    #[cfg(feature = "AppKit_NSBox")]
    unsafe impl NSBox {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Option<Allocated<Self>>, frame_rect: NSRect) -> Id<Self>;
    }
);
