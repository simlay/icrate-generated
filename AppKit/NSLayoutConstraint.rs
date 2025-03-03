//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

typed_extensible_enum!(
    pub type NSLayoutPriority = c_float;
);

extern_static!(NSLayoutPriorityRequired: NSLayoutPriority = 1000);

extern_static!(NSLayoutPriorityDefaultHigh: NSLayoutPriority = 750);

extern_static!(NSLayoutPriorityDragThatCanResizeWindow: NSLayoutPriority = 510);

extern_static!(NSLayoutPriorityWindowSizeStayPut: NSLayoutPriority = 500);

extern_static!(NSLayoutPriorityDragThatCannotResizeWindow: NSLayoutPriority = 490);

extern_static!(NSLayoutPriorityDefaultLow: NSLayoutPriority = 250);

extern_static!(NSLayoutPriorityFittingSizeCompression: NSLayoutPriority = 50);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSLayoutConstraintOrientation {
        NSLayoutConstraintOrientationHorizontal = 0,
        NSLayoutConstraintOrientationVertical = 1,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSLayoutRelation {
        NSLayoutRelationLessThanOrEqual = -1,
        NSLayoutRelationEqual = 0,
        NSLayoutRelationGreaterThanOrEqual = 1,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSLayoutAttribute {
        NSLayoutAttributeLeft = 1,
        NSLayoutAttributeRight = 2,
        NSLayoutAttributeTop = 3,
        NSLayoutAttributeBottom = 4,
        NSLayoutAttributeLeading = 5,
        NSLayoutAttributeTrailing = 6,
        NSLayoutAttributeWidth = 7,
        NSLayoutAttributeHeight = 8,
        NSLayoutAttributeCenterX = 9,
        NSLayoutAttributeCenterY = 10,
        NSLayoutAttributeLastBaseline = 11,
        NSLayoutAttributeBaseline = NSLayoutAttributeLastBaseline,
        NSLayoutAttributeFirstBaseline = 12,
        NSLayoutAttributeNotAnAttribute = 0,
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSLayoutFormatOptions {
        NSLayoutFormatAlignAllLeft = 1 << NSLayoutAttributeLeft,
        NSLayoutFormatAlignAllRight = 1 << NSLayoutAttributeRight,
        NSLayoutFormatAlignAllTop = 1 << NSLayoutAttributeTop,
        NSLayoutFormatAlignAllBottom = 1 << NSLayoutAttributeBottom,
        NSLayoutFormatAlignAllLeading = 1 << NSLayoutAttributeLeading,
        NSLayoutFormatAlignAllTrailing = 1 << NSLayoutAttributeTrailing,
        NSLayoutFormatAlignAllCenterX = 1 << NSLayoutAttributeCenterX,
        NSLayoutFormatAlignAllCenterY = 1 << NSLayoutAttributeCenterY,
        NSLayoutFormatAlignAllLastBaseline = 1 << NSLayoutAttributeLastBaseline,
        NSLayoutFormatAlignAllFirstBaseline = 1 << NSLayoutAttributeFirstBaseline,
        NSLayoutFormatAlignAllBaseline = NSLayoutFormatAlignAllLastBaseline,
        NSLayoutFormatAlignmentMask = 0xFFFF,
        NSLayoutFormatDirectionLeadingToTrailing = 0 << 16,
        NSLayoutFormatDirectionLeftToRight = 1 << 16,
        NSLayoutFormatDirectionRightToLeft = 2 << 16,
        NSLayoutFormatDirectionMask = 0x3 << 16,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSLayoutConstraint")]
    pub struct NSLayoutConstraint;

    #[cfg(feature = "AppKit_NSLayoutConstraint")]
    unsafe impl ClassType for NSLayoutConstraint {
        type Super = NSObject;
    }
);

#[cfg(feature = "AppKit_NSLayoutConstraint")]
unsafe impl NSObjectProtocol for NSLayoutConstraint {}

extern_methods!(
    #[cfg(feature = "AppKit_NSLayoutConstraint")]
    unsafe impl NSLayoutConstraint {
        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other constraintsWithVisualFormat:options:metrics:views:)]
        pub unsafe fn constraintsWithVisualFormat_options_metrics_views(
            format: &NSString,
            opts: NSLayoutFormatOptions,
            metrics: Option<&NSDictionary<NSString, Object>>,
            views: &NSDictionary<NSString, Object>,
        ) -> Id<NSArray<NSLayoutConstraint>>;

        #[method_id(@__retain_semantics Other constraintWithItem:attribute:relatedBy:toItem:attribute:multiplier:constant:)]
        pub unsafe fn constraintWithItem_attribute_relatedBy_toItem_attribute_multiplier_constant(
            view1: &Object,
            attr1: NSLayoutAttribute,
            relation: NSLayoutRelation,
            view2: Option<&Object>,
            attr2: NSLayoutAttribute,
            multiplier: CGFloat,
            c: CGFloat,
        ) -> Id<Self>;

        #[method(priority)]
        pub unsafe fn priority(&self) -> NSLayoutPriority;

        #[method(setPriority:)]
        pub unsafe fn setPriority(&self, priority: NSLayoutPriority);

        #[method(shouldBeArchived)]
        pub unsafe fn shouldBeArchived(&self) -> bool;

        #[method(setShouldBeArchived:)]
        pub unsafe fn setShouldBeArchived(&self, should_be_archived: bool);

        #[method_id(@__retain_semantics Other firstItem)]
        pub unsafe fn firstItem(&self) -> Option<Id<Object>>;

        #[method_id(@__retain_semantics Other secondItem)]
        pub unsafe fn secondItem(&self) -> Option<Id<Object>>;

        #[method(firstAttribute)]
        pub unsafe fn firstAttribute(&self) -> NSLayoutAttribute;

        #[method(secondAttribute)]
        pub unsafe fn secondAttribute(&self) -> NSLayoutAttribute;

        #[cfg(feature = "AppKit_NSLayoutAnchor")]
        #[method_id(@__retain_semantics Other firstAnchor)]
        pub unsafe fn firstAnchor(&self) -> Id<NSLayoutAnchor>;

        #[cfg(feature = "AppKit_NSLayoutAnchor")]
        #[method_id(@__retain_semantics Other secondAnchor)]
        pub unsafe fn secondAnchor(&self) -> Option<Id<NSLayoutAnchor>>;

        #[method(relation)]
        pub unsafe fn relation(&self) -> NSLayoutRelation;

        #[method(multiplier)]
        pub unsafe fn multiplier(&self) -> CGFloat;

        #[method(constant)]
        pub unsafe fn constant(&self) -> CGFloat;

        #[method(setConstant:)]
        pub unsafe fn setConstant(&self, constant: CGFloat);

        #[method(isActive)]
        pub unsafe fn isActive(&self) -> bool;

        #[method(setActive:)]
        pub unsafe fn setActive(&self, active: bool);

        #[cfg(feature = "Foundation_NSArray")]
        #[method(activateConstraints:)]
        pub unsafe fn activateConstraints(constraints: &NSArray<NSLayoutConstraint>);

        #[cfg(feature = "Foundation_NSArray")]
        #[method(deactivateConstraints:)]
        pub unsafe fn deactivateConstraints(constraints: &NSArray<NSLayoutConstraint>);
    }
);

extern_methods!(
    /// NSIdentifier
    #[cfg(feature = "AppKit_NSLayoutConstraint")]
    unsafe impl NSLayoutConstraint {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setIdentifier:)]
        pub unsafe fn setIdentifier(&self, identifier: Option<&NSString>);
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSLayoutConstraint")]
    unsafe impl NSLayoutConstraint {}
);

#[cfg(feature = "AppKit_NSLayoutConstraint")]
unsafe impl NSAnimatablePropertyContainer for NSLayoutConstraint {}

extern_methods!(
    /// NSConstraintBasedLayoutInstallingConstraints
    #[cfg(feature = "AppKit_NSView")]
    unsafe impl NSView {
        #[cfg(feature = "AppKit_NSLayoutXAxisAnchor")]
        #[method_id(@__retain_semantics Other leadingAnchor)]
        pub unsafe fn leadingAnchor(&self) -> Id<NSLayoutXAxisAnchor>;

        #[cfg(feature = "AppKit_NSLayoutXAxisAnchor")]
        #[method_id(@__retain_semantics Other trailingAnchor)]
        pub unsafe fn trailingAnchor(&self) -> Id<NSLayoutXAxisAnchor>;

        #[cfg(feature = "AppKit_NSLayoutXAxisAnchor")]
        #[method_id(@__retain_semantics Other leftAnchor)]
        pub unsafe fn leftAnchor(&self) -> Id<NSLayoutXAxisAnchor>;

        #[cfg(feature = "AppKit_NSLayoutXAxisAnchor")]
        #[method_id(@__retain_semantics Other rightAnchor)]
        pub unsafe fn rightAnchor(&self) -> Id<NSLayoutXAxisAnchor>;

        #[cfg(feature = "AppKit_NSLayoutYAxisAnchor")]
        #[method_id(@__retain_semantics Other topAnchor)]
        pub unsafe fn topAnchor(&self) -> Id<NSLayoutYAxisAnchor>;

        #[cfg(feature = "AppKit_NSLayoutYAxisAnchor")]
        #[method_id(@__retain_semantics Other bottomAnchor)]
        pub unsafe fn bottomAnchor(&self) -> Id<NSLayoutYAxisAnchor>;

        #[cfg(feature = "AppKit_NSLayoutDimension")]
        #[method_id(@__retain_semantics Other widthAnchor)]
        pub unsafe fn widthAnchor(&self) -> Id<NSLayoutDimension>;

        #[cfg(feature = "AppKit_NSLayoutDimension")]
        #[method_id(@__retain_semantics Other heightAnchor)]
        pub unsafe fn heightAnchor(&self) -> Id<NSLayoutDimension>;

        #[cfg(feature = "AppKit_NSLayoutXAxisAnchor")]
        #[method_id(@__retain_semantics Other centerXAnchor)]
        pub unsafe fn centerXAnchor(&self) -> Id<NSLayoutXAxisAnchor>;

        #[cfg(feature = "AppKit_NSLayoutYAxisAnchor")]
        #[method_id(@__retain_semantics Other centerYAnchor)]
        pub unsafe fn centerYAnchor(&self) -> Id<NSLayoutYAxisAnchor>;

        #[cfg(feature = "AppKit_NSLayoutYAxisAnchor")]
        #[method_id(@__retain_semantics Other firstBaselineAnchor)]
        pub unsafe fn firstBaselineAnchor(&self) -> Id<NSLayoutYAxisAnchor>;

        #[cfg(feature = "AppKit_NSLayoutYAxisAnchor")]
        #[method_id(@__retain_semantics Other lastBaselineAnchor)]
        pub unsafe fn lastBaselineAnchor(&self) -> Id<NSLayoutYAxisAnchor>;

        #[cfg(all(feature = "AppKit_NSLayoutConstraint", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other constraints)]
        pub unsafe fn constraints(&self) -> Id<NSArray<NSLayoutConstraint>>;

        #[cfg(feature = "AppKit_NSLayoutConstraint")]
        #[method(addConstraint:)]
        pub unsafe fn addConstraint(&self, constraint: &NSLayoutConstraint);

        #[cfg(all(feature = "AppKit_NSLayoutConstraint", feature = "Foundation_NSArray"))]
        #[method(addConstraints:)]
        pub unsafe fn addConstraints(&self, constraints: &NSArray<NSLayoutConstraint>);

        #[cfg(feature = "AppKit_NSLayoutConstraint")]
        #[method(removeConstraint:)]
        pub unsafe fn removeConstraint(&self, constraint: &NSLayoutConstraint);

        #[cfg(all(feature = "AppKit_NSLayoutConstraint", feature = "Foundation_NSArray"))]
        #[method(removeConstraints:)]
        pub unsafe fn removeConstraints(&self, constraints: &NSArray<NSLayoutConstraint>);
    }
);

extern_methods!(
    /// NSConstraintBasedLayoutCoreMethods
    #[cfg(feature = "AppKit_NSWindow")]
    unsafe impl NSWindow {
        #[method(updateConstraintsIfNeeded)]
        pub unsafe fn updateConstraintsIfNeeded(&self);

        #[method(layoutIfNeeded)]
        pub unsafe fn layoutIfNeeded(&self);
    }
);

extern_methods!(
    /// NSConstraintBasedLayoutCoreMethods
    #[cfg(feature = "AppKit_NSView")]
    unsafe impl NSView {
        #[method(updateConstraintsForSubtreeIfNeeded)]
        pub unsafe fn updateConstraintsForSubtreeIfNeeded(&self);

        #[method(updateConstraints)]
        pub unsafe fn updateConstraints(&self);

        #[method(needsUpdateConstraints)]
        pub unsafe fn needsUpdateConstraints(&self) -> bool;

        #[method(setNeedsUpdateConstraints:)]
        pub unsafe fn setNeedsUpdateConstraints(&self, needs_update_constraints: bool);
    }
);

extern_methods!(
    /// NSConstraintBasedCompatibility
    #[cfg(feature = "AppKit_NSView")]
    unsafe impl NSView {
        #[method(translatesAutoresizingMaskIntoConstraints)]
        pub unsafe fn translatesAutoresizingMaskIntoConstraints(&self) -> bool;

        #[method(setTranslatesAutoresizingMaskIntoConstraints:)]
        pub unsafe fn setTranslatesAutoresizingMaskIntoConstraints(
            &self,
            translates_autoresizing_mask_into_constraints: bool,
        );

        #[method(requiresConstraintBasedLayout)]
        pub unsafe fn requiresConstraintBasedLayout() -> bool;
    }
);

extern_static!(NSViewNoInstrinsicMetric: CGFloat);

extern_static!(NSViewNoIntrinsicMetric: CGFloat);

extern_methods!(
    /// NSConstraintBasedLayoutLayering
    #[cfg(feature = "AppKit_NSView")]
    unsafe impl NSView {
        #[method(alignmentRectForFrame:)]
        pub unsafe fn alignmentRectForFrame(&self, frame: NSRect) -> NSRect;

        #[method(frameForAlignmentRect:)]
        pub unsafe fn frameForAlignmentRect(&self, alignment_rect: NSRect) -> NSRect;

        #[method(alignmentRectInsets)]
        pub unsafe fn alignmentRectInsets(&self) -> NSEdgeInsets;

        #[method(firstBaselineOffsetFromTop)]
        pub unsafe fn firstBaselineOffsetFromTop(&self) -> CGFloat;

        #[method(lastBaselineOffsetFromBottom)]
        pub unsafe fn lastBaselineOffsetFromBottom(&self) -> CGFloat;

        #[method(baselineOffsetFromBottom)]
        pub unsafe fn baselineOffsetFromBottom(&self) -> CGFloat;

        #[method(intrinsicContentSize)]
        pub unsafe fn intrinsicContentSize(&self) -> NSSize;

        #[method(invalidateIntrinsicContentSize)]
        pub unsafe fn invalidateIntrinsicContentSize(&self);

        #[method(contentHuggingPriorityForOrientation:)]
        pub unsafe fn contentHuggingPriorityForOrientation(
            &self,
            orientation: NSLayoutConstraintOrientation,
        ) -> NSLayoutPriority;

        #[method(setContentHuggingPriority:forOrientation:)]
        pub unsafe fn setContentHuggingPriority_forOrientation(
            &self,
            priority: NSLayoutPriority,
            orientation: NSLayoutConstraintOrientation,
        );

        #[method(contentCompressionResistancePriorityForOrientation:)]
        pub unsafe fn contentCompressionResistancePriorityForOrientation(
            &self,
            orientation: NSLayoutConstraintOrientation,
        ) -> NSLayoutPriority;

        #[method(setContentCompressionResistancePriority:forOrientation:)]
        pub unsafe fn setContentCompressionResistancePriority_forOrientation(
            &self,
            priority: NSLayoutPriority,
            orientation: NSLayoutConstraintOrientation,
        );

        #[method(isHorizontalContentSizeConstraintActive)]
        pub unsafe fn isHorizontalContentSizeConstraintActive(&self) -> bool;

        #[method(setHorizontalContentSizeConstraintActive:)]
        pub unsafe fn setHorizontalContentSizeConstraintActive(
            &self,
            horizontal_content_size_constraint_active: bool,
        );

        #[method(isVerticalContentSizeConstraintActive)]
        pub unsafe fn isVerticalContentSizeConstraintActive(&self) -> bool;

        #[method(setVerticalContentSizeConstraintActive:)]
        pub unsafe fn setVerticalContentSizeConstraintActive(
            &self,
            vertical_content_size_constraint_active: bool,
        );
    }
);

extern_methods!(
    /// NSConstraintBasedLayoutLayering
    #[cfg(feature = "AppKit_NSControl")]
    unsafe impl NSControl {
        #[cfg(feature = "AppKit_NSCell")]
        #[method(invalidateIntrinsicContentSizeForCell:)]
        pub unsafe fn invalidateIntrinsicContentSizeForCell(&self, cell: &NSCell);
    }
);

extern_methods!(
    /// NSConstraintBasedLayoutAnchoring
    #[cfg(feature = "AppKit_NSWindow")]
    unsafe impl NSWindow {
        #[method(anchorAttributeForOrientation:)]
        pub unsafe fn anchorAttributeForOrientation(
            &self,
            orientation: NSLayoutConstraintOrientation,
        ) -> NSLayoutAttribute;

        #[method(setAnchorAttribute:forOrientation:)]
        pub unsafe fn setAnchorAttribute_forOrientation(
            &self,
            attr: NSLayoutAttribute,
            orientation: NSLayoutConstraintOrientation,
        );
    }
);

extern_methods!(
    /// NSConstraintBasedLayoutFittingSize
    #[cfg(feature = "AppKit_NSView")]
    unsafe impl NSView {
        #[method(fittingSize)]
        pub unsafe fn fittingSize(&self) -> NSSize;
    }
);

extern_methods!(
    /// NSConstraintBasedLayoutDebugging
    #[cfg(feature = "AppKit_NSView")]
    unsafe impl NSView {
        #[cfg(all(feature = "AppKit_NSLayoutConstraint", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other constraintsAffectingLayoutForOrientation:)]
        pub unsafe fn constraintsAffectingLayoutForOrientation(
            &self,
            orientation: NSLayoutConstraintOrientation,
        ) -> Id<NSArray<NSLayoutConstraint>>;

        #[method(hasAmbiguousLayout)]
        pub unsafe fn hasAmbiguousLayout(&self) -> bool;

        #[method(exerciseAmbiguityInLayout)]
        pub unsafe fn exerciseAmbiguityInLayout(&self);
    }
);

extern_methods!(
    /// NSConstraintBasedLayoutDebugging
    #[cfg(feature = "AppKit_NSWindow")]
    unsafe impl NSWindow {
        #[cfg(all(feature = "AppKit_NSLayoutConstraint", feature = "Foundation_NSArray"))]
        #[method(visualizeConstraints:)]
        pub unsafe fn visualizeConstraints(
            &self,
            constraints: Option<&NSArray<NSLayoutConstraint>>,
        );
    }
);
