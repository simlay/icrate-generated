//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    /**
      NSTextSelectionNavigation is an interface exposing methods for obtaining results from actions performed on text selections. It returns the essential information necessary for editing, selecting, and navigating operations.
    */
    pub enum NSTextSelectionNavigationDirection {
        NSTextSelectionNavigationDirectionForward = 0,
        NSTextSelectionNavigationDirectionBackward = 1,
        NSTextSelectionNavigationDirectionRight = 2,
        NSTextSelectionNavigationDirectionLeft = 3,
        NSTextSelectionNavigationDirectionUp = 4,
        NSTextSelectionNavigationDirectionDown = 5,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSTextSelectionNavigationDestination {
        NSTextSelectionNavigationDestinationCharacter = 0,
        NSTextSelectionNavigationDestinationWord = 1,
        NSTextSelectionNavigationDestinationLine = 2,
        NSTextSelectionNavigationDestinationSentence = 3,
        NSTextSelectionNavigationDestinationParagraph = 4,
        NSTextSelectionNavigationDestinationContainer = 5,
        NSTextSelectionNavigationDestinationDocument = 6,
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSTextSelectionNavigationModifier {
        NSTextSelectionNavigationModifierExtend = 1 << 0,
        NSTextSelectionNavigationModifierVisual = 1 << 1,
        NSTextSelectionNavigationModifierMultiple = 1 << 2,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSTextSelectionNavigationWritingDirection {
        NSTextSelectionNavigationWritingDirectionLeftToRight = 0,
        NSTextSelectionNavigationWritingDirectionRightToLeft = 1,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSTextSelectionNavigationLayoutOrientation {
        NSTextSelectionNavigationLayoutOrientationHorizontal = 0,
        NSTextSelectionNavigationLayoutOrientationVertical = 1,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSTextSelectionNavigation")]
    pub struct NSTextSelectionNavigation;

    #[cfg(feature = "AppKit_NSTextSelectionNavigation")]
    unsafe impl ClassType for NSTextSelectionNavigation {
        type Super = NSObject;
    }
);

#[cfg(feature = "AppKit_NSTextSelectionNavigation")]
unsafe impl NSObjectProtocol for NSTextSelectionNavigation {}

extern_methods!(
    #[cfg(feature = "AppKit_NSTextSelectionNavigation")]
    unsafe impl NSTextSelectionNavigation {
        #[method_id(@__retain_semantics Init initWithDataSource:)]
        pub unsafe fn initWithDataSource(
            this: Option<Allocated<Self>>,
            data_source: &ProtocolObject<dyn NSTextSelectionDataSource>,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        /**
          The data source object providing the layout and document content information.
        */
        #[method_id(@__retain_semantics Other textSelectionDataSource)]
        pub unsafe fn textSelectionDataSource(
            &self,
        ) -> Option<Id<ProtocolObject<dyn NSTextSelectionDataSource>>>;

        /**
          If YES, the object could produce selections with multiple disjoint ranges.
        */
        #[method(allowsNonContiguousRanges)]
        pub unsafe fn allowsNonContiguousRanges(&self) -> bool;

        /**
          If YES, the object could produce selections with multiple disjoint ranges.
        */
        #[method(setAllowsNonContiguousRanges:)]
        pub unsafe fn setAllowsNonContiguousRanges(&self, allows_non_contiguous_ranges: bool);

        /**
          If YES, rotates the coordinate system for arguments passed to the navigation methods such as -textSelectionsInteractingAtPoint:inContainerAtLocation:anchors:modifiers:selecting:bounds: based on the text container layout orientation. NO by default.
        */
        #[method(rotatesCoordinateSystemForLayoutOrientation)]
        pub unsafe fn rotatesCoordinateSystemForLayoutOrientation(&self) -> bool;

        /**
          If YES, rotates the coordinate system for arguments passed to the navigation methods such as -textSelectionsInteractingAtPoint:inContainerAtLocation:anchors:modifiers:selecting:bounds: based on the text container layout orientation. NO by default.
        */
        #[method(setRotatesCoordinateSystemForLayoutOrientation:)]
        pub unsafe fn setRotatesCoordinateSystemForLayoutOrientation(
            &self,
            rotates_coordinate_system_for_layout_orientation: bool,
        );

        #[method(flushLayoutCache)]
        pub unsafe fn flushLayoutCache(&self);

        #[cfg(feature = "AppKit_NSTextSelection")]
        #[method_id(@__retain_semantics Other destinationSelectionForTextSelection:direction:destination:extending:confined:)]
        pub unsafe fn destinationSelectionForTextSelection_direction_destination_extending_confined(
            &self,
            text_selection: &NSTextSelection,
            direction: NSTextSelectionNavigationDirection,
            destination: NSTextSelectionNavigationDestination,
            extending: bool,
            confined: bool,
        ) -> Option<Id<NSTextSelection>>;

        #[cfg(all(feature = "AppKit_NSTextSelection", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other textSelectionsInteractingAtPoint:inContainerAtLocation:anchors:modifiers:selecting:bounds:)]
        pub unsafe fn textSelectionsInteractingAtPoint_inContainerAtLocation_anchors_modifiers_selecting_bounds(
            &self,
            point: CGPoint,
            container_location: &ProtocolObject<dyn NSTextLocation>,
            anchors: &NSArray<NSTextSelection>,
            modifiers: NSTextSelectionNavigationModifier,
            selecting: bool,
            bounds: CGRect,
        ) -> Id<NSArray<NSTextSelection>>;

        #[cfg(feature = "AppKit_NSTextSelection")]
        #[method_id(@__retain_semantics Other textSelectionForSelectionGranularity:enclosingTextSelection:)]
        pub unsafe fn textSelectionForSelectionGranularity_enclosingTextSelection(
            &self,
            selection_granularity: NSTextSelectionGranularity,
            text_selection: &NSTextSelection,
        ) -> Id<NSTextSelection>;

        #[cfg(feature = "AppKit_NSTextSelection")]
        #[method_id(@__retain_semantics Other textSelectionForSelectionGranularity:enclosingPoint:inContainerAtLocation:)]
        pub unsafe fn textSelectionForSelectionGranularity_enclosingPoint_inContainerAtLocation(
            &self,
            selection_granularity: NSTextSelectionGranularity,
            point: CGPoint,
            location: &ProtocolObject<dyn NSTextLocation>,
        ) -> Option<Id<NSTextSelection>>;

        #[cfg(feature = "AppKit_NSTextSelection")]
        #[method_id(@__retain_semantics Other resolvedInsertionLocationForTextSelection:writingDirection:)]
        pub unsafe fn resolvedInsertionLocationForTextSelection_writingDirection(
            &self,
            text_selection: &NSTextSelection,
            writing_direction: NSTextSelectionNavigationWritingDirection,
        ) -> Option<Id<ProtocolObject<dyn NSTextLocation>>>;

        #[cfg(all(
            feature = "AppKit_NSTextRange",
            feature = "AppKit_NSTextSelection",
            feature = "Foundation_NSArray"
        ))]
        #[method_id(@__retain_semantics Other deletionRangesForTextSelection:direction:destination:allowsDecomposition:)]
        pub unsafe fn deletionRangesForTextSelection_direction_destination_allowsDecomposition(
            &self,
            text_selection: &NSTextSelection,
            direction: NSTextSelectionNavigationDirection,
            destination: NSTextSelectionNavigationDestination,
            allows_decomposition: bool,
        ) -> Id<NSArray<NSTextRange>>;
    }
);

extern_protocol!(
    pub unsafe trait NSTextSelectionDataSource: NSObjectProtocol {
        #[cfg(feature = "AppKit_NSTextRange")]
        /**
          Declares the starting and ending locations for the document.
        */
        #[method_id(@__retain_semantics Other documentRange)]
        unsafe fn documentRange(&self) -> Id<NSTextRange>;

        #[cfg(all(feature = "AppKit_NSTextRange", feature = "Foundation_NSString"))]
        #[method(enumerateSubstringsFromLocation:options:usingBlock:)]
        unsafe fn enumerateSubstringsFromLocation_options_usingBlock(
            &self,
            location: &ProtocolObject<dyn NSTextLocation>,
            options: NSStringEnumerationOptions,
            block: &Block<
                (
                    *mut NSString,
                    NonNull<NSTextRange>,
                    *mut NSTextRange,
                    NonNull<Bool>,
                ),
                (),
            >,
        );

        #[cfg(feature = "AppKit_NSTextRange")]
        #[method_id(@__retain_semantics Other textRangeForSelectionGranularity:enclosingLocation:)]
        unsafe fn textRangeForSelectionGranularity_enclosingLocation(
            &self,
            selection_granularity: NSTextSelectionGranularity,
            location: &ProtocolObject<dyn NSTextLocation>,
        ) -> Option<Id<NSTextRange>>;

        #[method_id(@__retain_semantics Other locationFromLocation:withOffset:)]
        unsafe fn locationFromLocation_withOffset(
            &self,
            location: &ProtocolObject<dyn NSTextLocation>,
            offset: NSInteger,
        ) -> Option<Id<ProtocolObject<dyn NSTextLocation>>>;

        #[method(offsetFromLocation:toLocation:)]
        unsafe fn offsetFromLocation_toLocation(
            &self,
            from: &ProtocolObject<dyn NSTextLocation>,
            to: &ProtocolObject<dyn NSTextLocation>,
        ) -> NSInteger;

        #[method(baseWritingDirectionAtLocation:)]
        unsafe fn baseWritingDirectionAtLocation(
            &self,
            location: &ProtocolObject<dyn NSTextLocation>,
        ) -> NSTextSelectionNavigationWritingDirection;

        #[method(enumerateCaretOffsetsInLineFragmentAtLocation:usingBlock:)]
        unsafe fn enumerateCaretOffsetsInLineFragmentAtLocation_usingBlock(
            &self,
            location: &ProtocolObject<dyn NSTextLocation>,
            block: &Block<
                (
                    CGFloat,
                    NonNull<ProtocolObject<dyn NSTextLocation>>,
                    Bool,
                    NonNull<Bool>,
                ),
                (),
            >,
        );

        #[cfg(feature = "AppKit_NSTextRange")]
        #[method_id(@__retain_semantics Other lineFragmentRangeForPoint:inContainerAtLocation:)]
        unsafe fn lineFragmentRangeForPoint_inContainerAtLocation(
            &self,
            point: CGPoint,
            location: &ProtocolObject<dyn NSTextLocation>,
        ) -> Option<Id<NSTextRange>>;

        #[optional]
        #[method(enumerateContainerBoundariesFromLocation:reverse:usingBlock:)]
        unsafe fn enumerateContainerBoundariesFromLocation_reverse_usingBlock(
            &self,
            location: &ProtocolObject<dyn NSTextLocation>,
            reverse: bool,
            block: &Block<(NonNull<ProtocolObject<dyn NSTextLocation>>, NonNull<Bool>), ()>,
        );

        #[optional]
        #[method(textLayoutOrientationAtLocation:)]
        unsafe fn textLayoutOrientationAtLocation(
            &self,
            location: &ProtocolObject<dyn NSTextLocation>,
        ) -> NSTextSelectionNavigationLayoutOrientation;
    }

    unsafe impl ProtocolType for dyn NSTextSelectionDataSource {}
);
