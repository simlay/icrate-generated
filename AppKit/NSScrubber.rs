//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_protocol!(
    #[cfg(not(any(target_os = "ios")))]
    pub unsafe trait NSScrubberDataSource: NSObjectProtocol {
        #[cfg(feature = "AppKit_NSScrubber")]
        #[cfg(not(any(target_os = "ios")))]
        #[method(numberOfItemsForScrubber:)]
        unsafe fn numberOfItemsForScrubber(&self, scrubber: &NSScrubber) -> NSInteger;

        #[cfg(all(feature = "AppKit_NSScrubber", feature = "AppKit_NSScrubberItemView"))]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other scrubber:viewForItemAtIndex:)]
        unsafe fn scrubber_viewForItemAtIndex(
            &self,
            scrubber: &NSScrubber,
            index: NSInteger,
        ) -> Id<NSScrubberItemView>;
    }

    unsafe impl ProtocolType for dyn NSScrubberDataSource {}
);

extern_protocol!(
    #[cfg(not(any(target_os = "ios")))]
    pub unsafe trait NSScrubberDelegate: NSObjectProtocol {
        #[cfg(feature = "AppKit_NSScrubber")]
        #[cfg(not(any(target_os = "ios")))]
        #[optional]
        #[method(scrubber:didSelectItemAtIndex:)]
        unsafe fn scrubber_didSelectItemAtIndex(
            &self,
            scrubber: &NSScrubber,
            selected_index: NSInteger,
        );

        #[cfg(feature = "AppKit_NSScrubber")]
        #[cfg(not(any(target_os = "ios")))]
        #[optional]
        #[method(scrubber:didHighlightItemAtIndex:)]
        unsafe fn scrubber_didHighlightItemAtIndex(
            &self,
            scrubber: &NSScrubber,
            highlighted_index: NSInteger,
        );

        #[cfg(feature = "AppKit_NSScrubber")]
        #[cfg(not(any(target_os = "ios")))]
        #[optional]
        #[method(scrubber:didChangeVisibleRange:)]
        unsafe fn scrubber_didChangeVisibleRange(
            &self,
            scrubber: &NSScrubber,
            visible_range: NSRange,
        );

        #[cfg(feature = "AppKit_NSScrubber")]
        #[cfg(not(any(target_os = "ios")))]
        #[optional]
        #[method(didBeginInteractingWithScrubber:)]
        unsafe fn didBeginInteractingWithScrubber(&self, scrubber: &NSScrubber);

        #[cfg(feature = "AppKit_NSScrubber")]
        #[cfg(not(any(target_os = "ios")))]
        #[optional]
        #[method(didFinishInteractingWithScrubber:)]
        unsafe fn didFinishInteractingWithScrubber(&self, scrubber: &NSScrubber);

        #[cfg(feature = "AppKit_NSScrubber")]
        #[cfg(not(any(target_os = "ios")))]
        #[optional]
        #[method(didCancelInteractingWithScrubber:)]
        unsafe fn didCancelInteractingWithScrubber(&self, scrubber: &NSScrubber);
    }

    unsafe impl ProtocolType for dyn NSScrubberDelegate {}
);

ns_enum!(
    #[underlying(NSInteger)]
    #[cfg(not(any(target_os = "ios")))]
    pub enum NSScrubberMode {
        #[cfg(not(any(target_os = "ios")))]
        NSScrubberModeFixed = 0,
        #[cfg(not(any(target_os = "ios")))]
        NSScrubberModeFree = 1,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    #[cfg(not(any(target_os = "ios")))]
    pub enum NSScrubberAlignment {
        #[cfg(not(any(target_os = "ios")))]
        NSScrubberAlignmentNone = 0,
        #[cfg(not(any(target_os = "ios")))]
        NSScrubberAlignmentLeading = 1,
        #[cfg(not(any(target_os = "ios")))]
        NSScrubberAlignmentTrailing = 2,
        #[cfg(not(any(target_os = "ios")))]
        NSScrubberAlignmentCenter = 3,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSScrubberSelectionStyle")]
    #[cfg(not(any(target_os = "ios")))]
    pub struct NSScrubberSelectionStyle;

    #[cfg(feature = "AppKit_NSScrubberSelectionStyle")]
    unsafe impl ClassType for NSScrubberSelectionStyle {
        type Super = NSObject;
    }
);

#[cfg(feature = "AppKit_NSScrubberSelectionStyle")]
unsafe impl NSCoding for NSScrubberSelectionStyle {}

#[cfg(feature = "AppKit_NSScrubberSelectionStyle")]
unsafe impl NSObjectProtocol for NSScrubberSelectionStyle {}

extern_methods!(
    #[cfg(feature = "AppKit_NSScrubberSelectionStyle")]
    unsafe impl NSScrubberSelectionStyle {
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other outlineOverlayStyle)]
        pub unsafe fn outlineOverlayStyle() -> Id<NSScrubberSelectionStyle>;

        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other roundedBackgroundStyle)]
        pub unsafe fn roundedBackgroundStyle() -> Id<NSScrubberSelectionStyle>;

        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Option<Allocated<Self>>, coder: &NSCoder) -> Id<Self>;

        #[cfg(feature = "AppKit_NSScrubberSelectionView")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other makeSelectionView)]
        pub unsafe fn makeSelectionView(&self) -> Option<Id<NSScrubberSelectionView>>;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSScrubber")]
    #[cfg(not(any(target_os = "ios")))]
    pub struct NSScrubber;

    #[cfg(feature = "AppKit_NSScrubber")]
    unsafe impl ClassType for NSScrubber {
        #[inherits(NSResponder, NSObject)]
        type Super = NSView;
    }
);

#[cfg(feature = "AppKit_NSScrubber")]
unsafe impl NSAccessibility for NSScrubber {}

#[cfg(feature = "AppKit_NSScrubber")]
unsafe impl NSAccessibilityElementProtocol for NSScrubber {}

#[cfg(feature = "AppKit_NSScrubber")]
unsafe impl NSAnimatablePropertyContainer for NSScrubber {}

#[cfg(feature = "AppKit_NSScrubber")]
unsafe impl NSAppearanceCustomization for NSScrubber {}

#[cfg(feature = "AppKit_NSScrubber")]
unsafe impl NSCoding for NSScrubber {}

#[cfg(feature = "AppKit_NSScrubber")]
unsafe impl NSDraggingDestination for NSScrubber {}

#[cfg(feature = "AppKit_NSScrubber")]
unsafe impl NSObjectProtocol for NSScrubber {}

#[cfg(feature = "AppKit_NSScrubber")]
unsafe impl NSUserInterfaceItemIdentification for NSScrubber {}

extern_methods!(
    #[cfg(feature = "AppKit_NSScrubber")]
    unsafe impl NSScrubber {
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other dataSource)]
        pub unsafe fn dataSource(&self) -> Option<Id<ProtocolObject<dyn NSScrubberDataSource>>>;

        #[cfg(not(any(target_os = "ios")))]
        #[method(setDataSource:)]
        pub unsafe fn setDataSource(
            &self,
            data_source: Option<&ProtocolObject<dyn NSScrubberDataSource>>,
        );

        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<ProtocolObject<dyn NSScrubberDelegate>>>;

        #[cfg(not(any(target_os = "ios")))]
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&ProtocolObject<dyn NSScrubberDelegate>>);

        #[cfg(feature = "AppKit_NSScrubberLayout")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other scrubberLayout)]
        pub unsafe fn scrubberLayout(&self) -> Id<NSScrubberLayout>;

        #[cfg(feature = "AppKit_NSScrubberLayout")]
        #[cfg(not(any(target_os = "ios")))]
        #[method(setScrubberLayout:)]
        pub unsafe fn setScrubberLayout(&self, scrubber_layout: &NSScrubberLayout);

        #[cfg(not(any(target_os = "ios")))]
        #[method(numberOfItems)]
        pub unsafe fn numberOfItems(&self) -> NSInteger;

        #[cfg(not(any(target_os = "ios")))]
        #[method(highlightedIndex)]
        pub unsafe fn highlightedIndex(&self) -> NSInteger;

        #[cfg(not(any(target_os = "ios")))]
        #[method(selectedIndex)]
        pub unsafe fn selectedIndex(&self) -> NSInteger;

        #[cfg(not(any(target_os = "ios")))]
        #[method(setSelectedIndex:)]
        pub unsafe fn setSelectedIndex(&self, selected_index: NSInteger);

        #[cfg(not(any(target_os = "ios")))]
        #[method(mode)]
        pub unsafe fn mode(&self) -> NSScrubberMode;

        #[cfg(not(any(target_os = "ios")))]
        #[method(setMode:)]
        pub unsafe fn setMode(&self, mode: NSScrubberMode);

        #[cfg(not(any(target_os = "ios")))]
        #[method(itemAlignment)]
        pub unsafe fn itemAlignment(&self) -> NSScrubberAlignment;

        #[cfg(not(any(target_os = "ios")))]
        #[method(setItemAlignment:)]
        pub unsafe fn setItemAlignment(&self, item_alignment: NSScrubberAlignment);

        #[cfg(not(any(target_os = "ios")))]
        #[method(isContinuous)]
        pub unsafe fn isContinuous(&self) -> bool;

        #[cfg(not(any(target_os = "ios")))]
        #[method(setContinuous:)]
        pub unsafe fn setContinuous(&self, continuous: bool);

        #[cfg(not(any(target_os = "ios")))]
        #[method(floatsSelectionViews)]
        pub unsafe fn floatsSelectionViews(&self) -> bool;

        #[cfg(not(any(target_os = "ios")))]
        #[method(setFloatsSelectionViews:)]
        pub unsafe fn setFloatsSelectionViews(&self, floats_selection_views: bool);

        #[cfg(feature = "AppKit_NSScrubberSelectionStyle")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other selectionBackgroundStyle)]
        pub unsafe fn selectionBackgroundStyle(&self) -> Option<Id<NSScrubberSelectionStyle>>;

        #[cfg(feature = "AppKit_NSScrubberSelectionStyle")]
        #[cfg(not(any(target_os = "ios")))]
        #[method(setSelectionBackgroundStyle:)]
        pub unsafe fn setSelectionBackgroundStyle(
            &self,
            selection_background_style: Option<&NSScrubberSelectionStyle>,
        );

        #[cfg(feature = "AppKit_NSScrubberSelectionStyle")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other selectionOverlayStyle)]
        pub unsafe fn selectionOverlayStyle(&self) -> Option<Id<NSScrubberSelectionStyle>>;

        #[cfg(feature = "AppKit_NSScrubberSelectionStyle")]
        #[cfg(not(any(target_os = "ios")))]
        #[method(setSelectionOverlayStyle:)]
        pub unsafe fn setSelectionOverlayStyle(
            &self,
            selection_overlay_style: Option<&NSScrubberSelectionStyle>,
        );

        #[cfg(not(any(target_os = "ios")))]
        #[method(showsArrowButtons)]
        pub unsafe fn showsArrowButtons(&self) -> bool;

        #[cfg(not(any(target_os = "ios")))]
        #[method(setShowsArrowButtons:)]
        pub unsafe fn setShowsArrowButtons(&self, shows_arrow_buttons: bool);

        #[cfg(not(any(target_os = "ios")))]
        #[method(showsAdditionalContentIndicators)]
        pub unsafe fn showsAdditionalContentIndicators(&self) -> bool;

        #[cfg(not(any(target_os = "ios")))]
        #[method(setShowsAdditionalContentIndicators:)]
        pub unsafe fn setShowsAdditionalContentIndicators(
            &self,
            shows_additional_content_indicators: bool,
        );

        #[cfg(feature = "AppKit_NSColor")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other backgroundColor)]
        pub unsafe fn backgroundColor(&self) -> Option<Id<NSColor>>;

        #[cfg(feature = "AppKit_NSColor")]
        #[cfg(not(any(target_os = "ios")))]
        #[method(setBackgroundColor:)]
        pub unsafe fn setBackgroundColor(&self, background_color: Option<&NSColor>);

        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other backgroundView)]
        pub unsafe fn backgroundView(&self) -> Option<Id<NSView>>;

        #[cfg(not(any(target_os = "ios")))]
        #[method(setBackgroundView:)]
        pub unsafe fn setBackgroundView(&self, background_view: Option<&NSView>);

        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Option<Allocated<Self>>, frame_rect: NSRect) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Option<Allocated<Self>>, coder: &NSCoder) -> Id<Self>;

        #[cfg(not(any(target_os = "ios")))]
        #[method(reloadData)]
        pub unsafe fn reloadData(&self);

        #[cfg(not(any(target_os = "ios")))]
        #[method(performSequentialBatchUpdates:)]
        pub unsafe fn performSequentialBatchUpdates(&self, update_block: &Block<(), ()>);

        #[cfg(feature = "Foundation_NSIndexSet")]
        #[cfg(not(any(target_os = "ios")))]
        #[method(insertItemsAtIndexes:)]
        pub unsafe fn insertItemsAtIndexes(&self, indexes: &NSIndexSet);

        #[cfg(feature = "Foundation_NSIndexSet")]
        #[cfg(not(any(target_os = "ios")))]
        #[method(removeItemsAtIndexes:)]
        pub unsafe fn removeItemsAtIndexes(&self, indexes: &NSIndexSet);

        #[cfg(feature = "Foundation_NSIndexSet")]
        #[cfg(not(any(target_os = "ios")))]
        #[method(reloadItemsAtIndexes:)]
        pub unsafe fn reloadItemsAtIndexes(&self, indexes: &NSIndexSet);

        #[cfg(not(any(target_os = "ios")))]
        #[method(moveItemAtIndex:toIndex:)]
        pub unsafe fn moveItemAtIndex_toIndex(&self, old_index: NSInteger, new_index: NSInteger);

        #[cfg(not(any(target_os = "ios")))]
        #[method(scrollItemAtIndex:toAlignment:)]
        pub unsafe fn scrollItemAtIndex_toAlignment(
            &self,
            index: NSInteger,
            alignment: NSScrubberAlignment,
        );

        #[cfg(feature = "AppKit_NSScrubberItemView")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other itemViewForItemAtIndex:)]
        pub unsafe fn itemViewForItemAtIndex(
            &self,
            index: NSInteger,
        ) -> Option<Id<NSScrubberItemView>>;

        #[cfg(not(any(target_os = "ios")))]
        #[method(registerClass:forItemIdentifier:)]
        pub unsafe fn registerClass_forItemIdentifier(
            &self,
            item_view_class: Option<&Class>,
            item_identifier: &NSUserInterfaceItemIdentifier,
        );

        #[cfg(feature = "AppKit_NSNib")]
        #[cfg(not(any(target_os = "ios")))]
        #[method(registerNib:forItemIdentifier:)]
        pub unsafe fn registerNib_forItemIdentifier(
            &self,
            nib: Option<&NSNib>,
            item_identifier: &NSUserInterfaceItemIdentifier,
        );

        #[cfg(feature = "AppKit_NSScrubberItemView")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other makeItemWithIdentifier:owner:)]
        pub unsafe fn makeItemWithIdentifier_owner(
            &self,
            item_identifier: &NSUserInterfaceItemIdentifier,
            owner: Option<&Object>,
        ) -> Option<Id<NSScrubberItemView>>;
    }
);
