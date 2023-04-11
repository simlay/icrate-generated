//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSPredicateEditor")]
    pub struct NSPredicateEditor;

    #[cfg(feature = "AppKit_NSPredicateEditor")]
    unsafe impl ClassType for NSPredicateEditor {
        #[inherits(NSControl, NSView, NSResponder, NSObject)]
        type Super = NSRuleEditor;
    }
);

#[cfg(feature = "AppKit_NSPredicateEditor")]
unsafe impl NSAccessibility for NSPredicateEditor {}

#[cfg(feature = "AppKit_NSPredicateEditor")]
unsafe impl NSAccessibilityElementProtocol for NSPredicateEditor {}

#[cfg(feature = "AppKit_NSPredicateEditor")]
unsafe impl NSAnimatablePropertyContainer for NSPredicateEditor {}

#[cfg(feature = "AppKit_NSPredicateEditor")]
unsafe impl NSAppearanceCustomization for NSPredicateEditor {}

#[cfg(feature = "AppKit_NSPredicateEditor")]
unsafe impl NSCoding for NSPredicateEditor {}

#[cfg(feature = "AppKit_NSPredicateEditor")]
unsafe impl NSDraggingDestination for NSPredicateEditor {}

#[cfg(feature = "AppKit_NSPredicateEditor")]
unsafe impl NSObjectProtocol for NSPredicateEditor {}

#[cfg(feature = "AppKit_NSPredicateEditor")]
unsafe impl NSUserInterfaceItemIdentification for NSPredicateEditor {}

extern_methods!(
    #[cfg(feature = "AppKit_NSPredicateEditor")]
    unsafe impl NSPredicateEditor {
        #[cfg(all(
            feature = "AppKit_NSPredicateEditorRowTemplate",
            feature = "Foundation_NSArray"
        ))]
        /**
          Setter - Sets the NSPredicateEditorRowTemplates for the NSPredicateEditor.  When created, NSPredicateEditor contains a template representing compound predicates; if you wish to keep it, you should take care to include it in this array.

         Getter - Returns the row templates for this NSPredicateEditor.
        */
        #[method_id(@__retain_semantics Other rowTemplates)]
        pub unsafe fn rowTemplates(&self) -> Id<NSArray<NSPredicateEditorRowTemplate>>;

        #[cfg(all(
            feature = "AppKit_NSPredicateEditorRowTemplate",
            feature = "Foundation_NSArray"
        ))]
        /**
          Setter - Sets the NSPredicateEditorRowTemplates for the NSPredicateEditor.  When created, NSPredicateEditor contains a template representing compound predicates; if you wish to keep it, you should take care to include it in this array.

         Getter - Returns the row templates for this NSPredicateEditor.
        */
        #[method(setRowTemplates:)]
        pub unsafe fn setRowTemplates(&self, row_templates: &NSArray<NSPredicateEditorRowTemplate>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSControl`
    #[cfg(feature = "AppKit_NSPredicateEditor")]
    unsafe impl NSPredicateEditor {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Option<Allocated<Self>>, frame_rect: NSRect) -> Id<Self>;
    }
);
