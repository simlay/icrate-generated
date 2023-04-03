//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSTableRowView")]
    #[cfg(not(any(target_os = "ios")))]
    pub struct NSTableRowView;

    #[cfg(feature = "AppKit_NSTableRowView")]
    unsafe impl ClassType for NSTableRowView {
        #[inherits(NSResponder, NSObject)]
        type Super = NSView;
    }
);

#[cfg(feature = "AppKit_NSTableRowView")]
unsafe impl NSAccessibility for NSTableRowView {}

#[cfg(feature = "AppKit_NSTableRowView")]
unsafe impl NSAccessibilityElementProtocol for NSTableRowView {}

#[cfg(feature = "AppKit_NSTableRowView")]
unsafe impl NSAccessibilityGroup for NSTableRowView {}

#[cfg(feature = "AppKit_NSTableRowView")]
unsafe impl NSAccessibilityRow for NSTableRowView {}

#[cfg(feature = "AppKit_NSTableRowView")]
unsafe impl NSAnimatablePropertyContainer for NSTableRowView {}

#[cfg(feature = "AppKit_NSTableRowView")]
unsafe impl NSAppearanceCustomization for NSTableRowView {}

#[cfg(feature = "AppKit_NSTableRowView")]
unsafe impl NSCoding for NSTableRowView {}

#[cfg(feature = "AppKit_NSTableRowView")]
unsafe impl NSDraggingDestination for NSTableRowView {}

#[cfg(feature = "AppKit_NSTableRowView")]
unsafe impl NSObjectProtocol for NSTableRowView {}

#[cfg(feature = "AppKit_NSTableRowView")]
unsafe impl NSUserInterfaceItemIdentification for NSTableRowView {}

extern_methods!(
    #[cfg(feature = "AppKit_NSTableRowView")]
    unsafe impl NSTableRowView {
        #[cfg(not(any(target_os = "ios")))]
        #[method(selectionHighlightStyle)]
        pub unsafe fn selectionHighlightStyle(&self) -> NSTableViewSelectionHighlightStyle;

        #[cfg(not(any(target_os = "ios")))]
        #[method(setSelectionHighlightStyle:)]
        pub unsafe fn setSelectionHighlightStyle(
            &self,
            selection_highlight_style: NSTableViewSelectionHighlightStyle,
        );

        #[cfg(not(any(target_os = "ios")))]
        #[method(isEmphasized)]
        pub unsafe fn isEmphasized(&self) -> bool;

        #[cfg(not(any(target_os = "ios")))]
        #[method(setEmphasized:)]
        pub unsafe fn setEmphasized(&self, emphasized: bool);

        #[cfg(not(any(target_os = "ios")))]
        #[method(isGroupRowStyle)]
        pub unsafe fn isGroupRowStyle(&self) -> bool;

        #[cfg(not(any(target_os = "ios")))]
        #[method(setGroupRowStyle:)]
        pub unsafe fn setGroupRowStyle(&self, group_row_style: bool);

        #[cfg(not(any(target_os = "ios")))]
        #[method(isSelected)]
        pub unsafe fn isSelected(&self) -> bool;

        #[cfg(not(any(target_os = "ios")))]
        #[method(setSelected:)]
        pub unsafe fn setSelected(&self, selected: bool);

        #[cfg(not(any(target_os = "ios")))]
        #[method(isPreviousRowSelected)]
        pub unsafe fn isPreviousRowSelected(&self) -> bool;

        #[cfg(not(any(target_os = "ios")))]
        #[method(setPreviousRowSelected:)]
        pub unsafe fn setPreviousRowSelected(&self, previous_row_selected: bool);

        #[cfg(not(any(target_os = "ios")))]
        #[method(isNextRowSelected)]
        pub unsafe fn isNextRowSelected(&self) -> bool;

        #[cfg(not(any(target_os = "ios")))]
        #[method(setNextRowSelected:)]
        pub unsafe fn setNextRowSelected(&self, next_row_selected: bool);

        #[cfg(not(any(target_os = "ios")))]
        #[method(isFloating)]
        pub unsafe fn isFloating(&self) -> bool;

        #[cfg(not(any(target_os = "ios")))]
        #[method(setFloating:)]
        pub unsafe fn setFloating(&self, floating: bool);

        #[cfg(not(any(target_os = "ios")))]
        #[method(isTargetForDropOperation)]
        pub unsafe fn isTargetForDropOperation(&self) -> bool;

        #[cfg(not(any(target_os = "ios")))]
        #[method(setTargetForDropOperation:)]
        pub unsafe fn setTargetForDropOperation(&self, target_for_drop_operation: bool);

        #[cfg(not(any(target_os = "ios")))]
        #[method(draggingDestinationFeedbackStyle)]
        pub unsafe fn draggingDestinationFeedbackStyle(
            &self,
        ) -> NSTableViewDraggingDestinationFeedbackStyle;

        #[cfg(not(any(target_os = "ios")))]
        #[method(setDraggingDestinationFeedbackStyle:)]
        pub unsafe fn setDraggingDestinationFeedbackStyle(
            &self,
            dragging_destination_feedback_style: NSTableViewDraggingDestinationFeedbackStyle,
        );

        #[cfg(not(any(target_os = "ios")))]
        #[method(indentationForDropOperation)]
        pub unsafe fn indentationForDropOperation(&self) -> CGFloat;

        #[cfg(not(any(target_os = "ios")))]
        #[method(setIndentationForDropOperation:)]
        pub unsafe fn setIndentationForDropOperation(
            &self,
            indentation_for_drop_operation: CGFloat,
        );

        #[cfg(not(any(target_os = "ios")))]
        #[method(interiorBackgroundStyle)]
        pub unsafe fn interiorBackgroundStyle(&self) -> NSBackgroundStyle;

        #[cfg(feature = "AppKit_NSColor")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other backgroundColor)]
        pub unsafe fn backgroundColor(&self) -> Id<NSColor>;

        #[cfg(feature = "AppKit_NSColor")]
        #[cfg(not(any(target_os = "ios")))]
        #[method(setBackgroundColor:)]
        pub unsafe fn setBackgroundColor(&self, background_color: &NSColor);

        #[cfg(not(any(target_os = "ios")))]
        #[method(drawBackgroundInRect:)]
        pub unsafe fn drawBackgroundInRect(&self, dirty_rect: NSRect);

        #[cfg(not(any(target_os = "ios")))]
        #[method(drawSelectionInRect:)]
        pub unsafe fn drawSelectionInRect(&self, dirty_rect: NSRect);

        #[cfg(not(any(target_os = "ios")))]
        #[method(drawSeparatorInRect:)]
        pub unsafe fn drawSeparatorInRect(&self, dirty_rect: NSRect);

        #[cfg(not(any(target_os = "ios")))]
        #[method(drawDraggingDestinationFeedbackInRect:)]
        pub unsafe fn drawDraggingDestinationFeedbackInRect(&self, dirty_rect: NSRect);

        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other viewAtColumn:)]
        pub unsafe fn viewAtColumn(&self, column: NSInteger) -> Option<Id<Object>>;

        #[cfg(not(any(target_os = "ios")))]
        #[method(numberOfColumns)]
        pub unsafe fn numberOfColumns(&self) -> NSInteger;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSView`
    #[cfg(feature = "AppKit_NSTableRowView")]
    unsafe impl NSTableRowView {
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Option<Allocated<Self>>, frame_rect: NSRect) -> Id<Self>;
    }
);
