//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSControl")]
    pub struct NSControl;

    #[cfg(feature = "AppKit_NSControl")]
    unsafe impl ClassType for NSControl {
        #[inherits(NSResponder, NSObject)]
        type Super = NSView;
    }
);

#[cfg(feature = "AppKit_NSControl")]
unsafe impl NSAccessibility for NSControl {}

#[cfg(feature = "AppKit_NSControl")]
unsafe impl NSAccessibilityElementProtocol for NSControl {}

#[cfg(feature = "AppKit_NSControl")]
unsafe impl NSAnimatablePropertyContainer for NSControl {}

#[cfg(feature = "AppKit_NSControl")]
unsafe impl NSAppearanceCustomization for NSControl {}

#[cfg(feature = "AppKit_NSControl")]
unsafe impl NSCoding for NSControl {}

#[cfg(feature = "AppKit_NSControl")]
unsafe impl NSDraggingDestination for NSControl {}

#[cfg(feature = "AppKit_NSControl")]
unsafe impl NSObjectProtocol for NSControl {}

#[cfg(feature = "AppKit_NSControl")]
unsafe impl NSUserInterfaceItemIdentification for NSControl {}

extern_methods!(
    #[cfg(feature = "AppKit_NSControl")]
    unsafe impl NSControl {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Option<Allocated<Self>>, frame_rect: NSRect) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self>>;

        #[method_id(@__retain_semantics Other target)]
        pub unsafe fn target(&self) -> Option<Id<Object>>;

        #[method(setTarget:)]
        pub unsafe fn setTarget(&self, target: Option<&Object>);

        #[method(action)]
        pub unsafe fn action(&self) -> Option<Sel>;

        #[method(setAction:)]
        pub unsafe fn setAction(&self, action: Option<Sel>);

        #[method(tag)]
        pub unsafe fn tag(&self) -> NSInteger;

        #[method(setTag:)]
        pub unsafe fn setTag(&self, tag: NSInteger);

        #[method(ignoresMultiClick)]
        pub unsafe fn ignoresMultiClick(&self) -> bool;

        #[method(setIgnoresMultiClick:)]
        pub unsafe fn setIgnoresMultiClick(&self, ignores_multi_click: bool);

        #[method(isContinuous)]
        pub unsafe fn isContinuous(&self) -> bool;

        #[method(setContinuous:)]
        pub unsafe fn setContinuous(&self, continuous: bool);

        #[method(isEnabled)]
        pub unsafe fn isEnabled(&self) -> bool;

        #[method(setEnabled:)]
        pub unsafe fn setEnabled(&self, enabled: bool);

        #[method(refusesFirstResponder)]
        pub unsafe fn refusesFirstResponder(&self) -> bool;

        #[method(setRefusesFirstResponder:)]
        pub unsafe fn setRefusesFirstResponder(&self, refuses_first_responder: bool);

        #[method(isHighlighted)]
        pub unsafe fn isHighlighted(&self) -> bool;

        #[method(setHighlighted:)]
        pub unsafe fn setHighlighted(&self, highlighted: bool);

        #[method(controlSize)]
        pub unsafe fn controlSize(&self) -> NSControlSize;

        #[method(setControlSize:)]
        pub unsafe fn setControlSize(&self, control_size: NSControlSize);

        #[cfg(feature = "Foundation_NSFormatter")]
        #[method_id(@__retain_semantics Other formatter)]
        pub unsafe fn formatter(&self) -> Option<Id<NSFormatter>>;

        #[cfg(feature = "Foundation_NSFormatter")]
        #[method(setFormatter:)]
        pub unsafe fn setFormatter(&self, formatter: Option<&NSFormatter>);

        #[method_id(@__retain_semantics Other objectValue)]
        pub unsafe fn objectValue(&self) -> Option<Id<Object>>;

        #[method(setObjectValue:)]
        pub unsafe fn setObjectValue(&self, object_value: Option<&Object>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other stringValue)]
        pub unsafe fn stringValue(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setStringValue:)]
        pub unsafe fn setStringValue(&self, string_value: &NSString);

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method_id(@__retain_semantics Other attributedStringValue)]
        pub unsafe fn attributedStringValue(&self) -> Id<NSAttributedString>;

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method(setAttributedStringValue:)]
        pub unsafe fn setAttributedStringValue(&self, attributed_string_value: &NSAttributedString);

        #[method(intValue)]
        pub unsafe fn intValue(&self) -> c_int;

        #[method(setIntValue:)]
        pub unsafe fn setIntValue(&self, int_value: c_int);

        #[method(integerValue)]
        pub unsafe fn integerValue(&self) -> NSInteger;

        #[method(setIntegerValue:)]
        pub unsafe fn setIntegerValue(&self, integer_value: NSInteger);

        #[method(floatValue)]
        pub unsafe fn floatValue(&self) -> c_float;

        #[method(setFloatValue:)]
        pub unsafe fn setFloatValue(&self, float_value: c_float);

        #[method(doubleValue)]
        pub unsafe fn doubleValue(&self) -> c_double;

        #[method(setDoubleValue:)]
        pub unsafe fn setDoubleValue(&self, double_value: c_double);

        #[method(sizeThatFits:)]
        pub unsafe fn sizeThatFits(&self, size: NSSize) -> NSSize;

        #[method(sizeToFit)]
        pub unsafe fn sizeToFit(&self);

        #[method(sendActionOn:)]
        pub unsafe fn sendActionOn(&self, mask: NSEventMask) -> NSInteger;

        #[method(sendAction:to:)]
        pub unsafe fn sendAction_to(&self, action: Option<Sel>, target: Option<&Object>) -> bool;

        #[method(takeIntValueFrom:)]
        pub unsafe fn takeIntValueFrom(&self, sender: Option<&Object>);

        #[method(takeFloatValueFrom:)]
        pub unsafe fn takeFloatValueFrom(&self, sender: Option<&Object>);

        #[method(takeDoubleValueFrom:)]
        pub unsafe fn takeDoubleValueFrom(&self, sender: Option<&Object>);

        #[method(takeStringValueFrom:)]
        pub unsafe fn takeStringValueFrom(&self, sender: Option<&Object>);

        #[method(takeObjectValueFrom:)]
        pub unsafe fn takeObjectValueFrom(&self, sender: Option<&Object>);

        #[method(takeIntegerValueFrom:)]
        pub unsafe fn takeIntegerValueFrom(&self, sender: Option<&Object>);

        #[cfg(feature = "AppKit_NSEvent")]
        #[method(mouseDown:)]
        pub unsafe fn mouseDown(&self, event: &NSEvent);

        #[method(performClick:)]
        pub unsafe fn performClick(&self, sender: Option<&Object>);

        #[cfg(feature = "AppKit_NSFont")]
        #[method_id(@__retain_semantics Other font)]
        pub unsafe fn font(&self) -> Option<Id<NSFont>>;

        #[cfg(feature = "AppKit_NSFont")]
        #[method(setFont:)]
        pub unsafe fn setFont(&self, font: Option<&NSFont>);

        #[method(usesSingleLineMode)]
        pub unsafe fn usesSingleLineMode(&self) -> bool;

        #[method(setUsesSingleLineMode:)]
        pub unsafe fn setUsesSingleLineMode(&self, uses_single_line_mode: bool);

        #[method(lineBreakMode)]
        pub unsafe fn lineBreakMode(&self) -> NSLineBreakMode;

        #[method(setLineBreakMode:)]
        pub unsafe fn setLineBreakMode(&self, line_break_mode: NSLineBreakMode);

        #[method(alignment)]
        pub unsafe fn alignment(&self) -> NSTextAlignment;

        #[method(setAlignment:)]
        pub unsafe fn setAlignment(&self, alignment: NSTextAlignment);

        #[method(baseWritingDirection)]
        pub unsafe fn baseWritingDirection(&self) -> NSWritingDirection;

        #[method(setBaseWritingDirection:)]
        pub unsafe fn setBaseWritingDirection(&self, base_writing_direction: NSWritingDirection);

        #[method(allowsExpansionToolTips)]
        pub unsafe fn allowsExpansionToolTips(&self) -> bool;

        #[method(setAllowsExpansionToolTips:)]
        pub unsafe fn setAllowsExpansionToolTips(&self, allows_expansion_tool_tips: bool);

        #[method(expansionFrameWithFrame:)]
        pub unsafe fn expansionFrameWithFrame(&self, content_frame: NSRect) -> NSRect;

        #[method(drawWithExpansionFrame:inView:)]
        pub unsafe fn drawWithExpansionFrame_inView(&self, content_frame: NSRect, view: &NSView);
    }
);

extern_methods!(
    /// NSControlEditableTextMethods
    #[cfg(feature = "AppKit_NSControl")]
    unsafe impl NSControl {
        #[cfg(feature = "AppKit_NSText")]
        #[method_id(@__retain_semantics Other currentEditor)]
        pub unsafe fn currentEditor(&self) -> Option<Id<NSText>>;

        #[method(abortEditing)]
        pub unsafe fn abortEditing(&self) -> bool;

        #[method(validateEditing)]
        pub unsafe fn validateEditing(&self);

        #[cfg(all(feature = "AppKit_NSEvent", feature = "AppKit_NSText"))]
        #[method(editWithFrame:editor:delegate:event:)]
        pub unsafe fn editWithFrame_editor_delegate_event(
            &self,
            rect: NSRect,
            text_obj: &NSText,
            delegate: Option<&Object>,
            event: &NSEvent,
        );

        #[cfg(feature = "AppKit_NSText")]
        #[method(selectWithFrame:editor:delegate:start:length:)]
        pub unsafe fn selectWithFrame_editor_delegate_start_length(
            &self,
            rect: NSRect,
            text_obj: &NSText,
            delegate: Option<&Object>,
            sel_start: NSInteger,
            sel_length: NSInteger,
        );

        #[cfg(feature = "AppKit_NSText")]
        #[method(endEditing:)]
        pub unsafe fn endEditing(&self, text_obj: &NSText);
    }
);

extern_protocol!(
    pub unsafe trait NSControlTextEditingDelegate: NSObjectProtocol {
        #[cfg(feature = "Foundation_NSNotification")]
        #[optional]
        #[method(controlTextDidBeginEditing:)]
        unsafe fn controlTextDidBeginEditing(&self, obj: &NSNotification);

        #[cfg(feature = "Foundation_NSNotification")]
        #[optional]
        #[method(controlTextDidEndEditing:)]
        unsafe fn controlTextDidEndEditing(&self, obj: &NSNotification);

        #[cfg(feature = "Foundation_NSNotification")]
        #[optional]
        #[method(controlTextDidChange:)]
        unsafe fn controlTextDidChange(&self, obj: &NSNotification);

        #[cfg(all(feature = "AppKit_NSControl", feature = "AppKit_NSText"))]
        #[optional]
        #[method(control:textShouldBeginEditing:)]
        unsafe fn control_textShouldBeginEditing(
            &self,
            control: &NSControl,
            field_editor: &NSText,
        ) -> bool;

        #[cfg(all(feature = "AppKit_NSControl", feature = "AppKit_NSText"))]
        #[optional]
        #[method(control:textShouldEndEditing:)]
        unsafe fn control_textShouldEndEditing(
            &self,
            control: &NSControl,
            field_editor: &NSText,
        ) -> bool;

        #[cfg(all(feature = "AppKit_NSControl", feature = "Foundation_NSString"))]
        #[optional]
        #[method(control:didFailToFormatString:errorDescription:)]
        unsafe fn control_didFailToFormatString_errorDescription(
            &self,
            control: &NSControl,
            string: &NSString,
            error: Option<&NSString>,
        ) -> bool;

        #[cfg(all(feature = "AppKit_NSControl", feature = "Foundation_NSString"))]
        #[optional]
        #[method(control:didFailToValidatePartialString:errorDescription:)]
        unsafe fn control_didFailToValidatePartialString_errorDescription(
            &self,
            control: &NSControl,
            string: &NSString,
            error: Option<&NSString>,
        );

        #[cfg(feature = "AppKit_NSControl")]
        #[optional]
        #[method(control:isValidObject:)]
        unsafe fn control_isValidObject(&self, control: &NSControl, obj: Option<&Object>) -> bool;

        #[cfg(all(feature = "AppKit_NSControl", feature = "AppKit_NSTextView"))]
        #[optional]
        #[method(control:textView:doCommandBySelector:)]
        unsafe fn control_textView_doCommandBySelector(
            &self,
            control: &NSControl,
            text_view: &NSTextView,
            command_selector: Sel,
        ) -> bool;

        #[cfg(all(
            feature = "AppKit_NSControl",
            feature = "AppKit_NSTextView",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSString"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other control:textView:completions:forPartialWordRange:indexOfSelectedItem:)]
        unsafe fn control_textView_completions_forPartialWordRange_indexOfSelectedItem(
            &self,
            control: &NSControl,
            text_view: &NSTextView,
            words: &NSArray<NSString>,
            char_range: NSRange,
            index: NonNull<NSInteger>,
        ) -> Id<NSArray<NSString>>;
    }

    unsafe impl ProtocolType for dyn NSControlTextEditingDelegate {}
);

extern_static!(NSControlTextDidBeginEditingNotification: &'static NSNotificationName);

extern_static!(NSControlTextDidEndEditingNotification: &'static NSNotificationName);

extern_static!(NSControlTextDidChangeNotification: &'static NSNotificationName);

extern_methods!(
    /// NSDeprecated
    #[cfg(feature = "AppKit_NSControl")]
    unsafe impl NSControl {
        #[deprecated]
        #[method(setFloatingPointFormat:left:right:)]
        pub unsafe fn setFloatingPointFormat_left_right(
            &self,
            auto_range: bool,
            left_digits: NSUInteger,
            right_digits: NSUInteger,
        );

        #[method(cellClass)]
        pub unsafe fn cellClass() -> Option<&'static Class>;

        #[method(setCellClass:)]
        pub unsafe fn setCellClass(cell_class: Option<&Class>);

        #[cfg(feature = "AppKit_NSCell")]
        #[method_id(@__retain_semantics Other cell)]
        pub unsafe fn cell(&self) -> Option<Id<NSCell>>;

        #[cfg(feature = "AppKit_NSCell")]
        #[method(setCell:)]
        pub unsafe fn setCell(&self, cell: Option<&NSCell>);

        #[cfg(feature = "AppKit_NSCell")]
        #[method_id(@__retain_semantics Other selectedCell)]
        pub unsafe fn selectedCell(&self) -> Option<Id<NSCell>>;

        #[method(selectedTag)]
        pub unsafe fn selectedTag(&self) -> NSInteger;

        #[deprecated = "Set the needsDisplay property to YES instead"]
        #[method(setNeedsDisplay)]
        pub unsafe fn setNeedsDisplay(&self);

        #[deprecated = "Override -layout instead. This method should never be called"]
        #[method(calcSize)]
        pub unsafe fn calcSize(&self);

        #[cfg(feature = "AppKit_NSCell")]
        #[method(updateCell:)]
        pub unsafe fn updateCell(&self, cell: &NSCell);

        #[cfg(feature = "AppKit_NSCell")]
        #[method(updateCellInside:)]
        pub unsafe fn updateCellInside(&self, cell: &NSCell);

        #[cfg(feature = "AppKit_NSCell")]
        #[method(drawCellInside:)]
        pub unsafe fn drawCellInside(&self, cell: &NSCell);

        #[cfg(feature = "AppKit_NSCell")]
        #[method(drawCell:)]
        pub unsafe fn drawCell(&self, cell: &NSCell);

        #[cfg(feature = "AppKit_NSCell")]
        #[method(selectCell:)]
        pub unsafe fn selectCell(&self, cell: &NSCell);
    }
);
