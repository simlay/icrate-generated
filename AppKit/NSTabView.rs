//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_static!(NSAppKitVersionNumberWithDirectionalTabs: NSAppKitVersion = 631.0);

ns_enum!(
    #[underlying(NSUInteger)]
    /**
      Use tabPosition and tabViewBorderType instead
    */
    pub enum NSTabViewType {
        NSTopTabsBezelBorder = 0,
        NSLeftTabsBezelBorder = 1,
        NSBottomTabsBezelBorder = 2,
        NSRightTabsBezelBorder = 3,
        NSNoTabsBezelBorder = 4,
        NSNoTabsLineBorder = 5,
        NSNoTabsNoBorder = 6,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSTabPosition {
        NSTabPositionNone = 0,
        NSTabPositionTop = 1,
        NSTabPositionLeft = 2,
        NSTabPositionBottom = 3,
        NSTabPositionRight = 4,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSTabViewBorderType {
        NSTabViewBorderTypeNone = 0,
        NSTabViewBorderTypeLine = 1,
        NSTabViewBorderTypeBezel = 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSTabView")]
    pub struct NSTabView;

    #[cfg(feature = "AppKit_NSTabView")]
    unsafe impl ClassType for NSTabView {
        #[inherits(NSResponder, NSObject)]
        type Super = NSView;
    }
);

#[cfg(feature = "AppKit_NSTabView")]
unsafe impl NSAccessibility for NSTabView {}

#[cfg(feature = "AppKit_NSTabView")]
unsafe impl NSAccessibilityElementProtocol for NSTabView {}

#[cfg(feature = "AppKit_NSTabView")]
unsafe impl NSAnimatablePropertyContainer for NSTabView {}

#[cfg(feature = "AppKit_NSTabView")]
unsafe impl NSAppearanceCustomization for NSTabView {}

#[cfg(feature = "AppKit_NSTabView")]
unsafe impl NSCoding for NSTabView {}

#[cfg(feature = "AppKit_NSTabView")]
unsafe impl NSDraggingDestination for NSTabView {}

#[cfg(feature = "AppKit_NSTabView")]
unsafe impl NSObjectProtocol for NSTabView {}

#[cfg(feature = "AppKit_NSTabView")]
unsafe impl NSUserInterfaceItemIdentification for NSTabView {}

extern_methods!(
    #[cfg(feature = "AppKit_NSTabView")]
    unsafe impl NSTabView {
        #[cfg(feature = "AppKit_NSTabViewItem")]
        #[method(selectTabViewItem:)]
        pub unsafe fn selectTabViewItem(&self, tab_view_item: Option<&NSTabViewItem>);

        #[method(selectTabViewItemAtIndex:)]
        pub unsafe fn selectTabViewItemAtIndex(&self, index: NSInteger);

        #[method(selectTabViewItemWithIdentifier:)]
        pub unsafe fn selectTabViewItemWithIdentifier(&self, identifier: &Object);

        #[method(takeSelectedTabViewItemFromSender:)]
        pub unsafe fn takeSelectedTabViewItemFromSender(&self, sender: Option<&Object>);

        #[method(selectFirstTabViewItem:)]
        pub unsafe fn selectFirstTabViewItem(&self, sender: Option<&Object>);

        #[method(selectLastTabViewItem:)]
        pub unsafe fn selectLastTabViewItem(&self, sender: Option<&Object>);

        #[method(selectNextTabViewItem:)]
        pub unsafe fn selectNextTabViewItem(&self, sender: Option<&Object>);

        #[method(selectPreviousTabViewItem:)]
        pub unsafe fn selectPreviousTabViewItem(&self, sender: Option<&Object>);

        #[cfg(feature = "AppKit_NSTabViewItem")]
        /**
          return nil if none are selected
        */
        #[method_id(@__retain_semantics Other selectedTabViewItem)]
        pub unsafe fn selectedTabViewItem(&self) -> Option<Id<NSTabViewItem>>;

        #[cfg(feature = "AppKit_NSFont")]
        /**
          returns font used for all tab labels.
        */
        #[method_id(@__retain_semantics Other font)]
        pub unsafe fn font(&self) -> Id<NSFont>;

        #[cfg(feature = "AppKit_NSFont")]
        /**
          returns font used for all tab labels.
        */
        #[method(setFont:)]
        pub unsafe fn setFont(&self, font: &NSFont);

        /**
          Use tabPosition and tabViewBorderType instead. Setting this will also set the tabPosition and tabViewBorderType. Setting tabPosition or tabViewBorderType will affect tabViewType
        */
        #[method(tabViewType)]
        pub unsafe fn tabViewType(&self) -> NSTabViewType;

        /**
          Use tabPosition and tabViewBorderType instead. Setting this will also set the tabPosition and tabViewBorderType. Setting tabPosition or tabViewBorderType will affect tabViewType
        */
        #[method(setTabViewType:)]
        pub unsafe fn setTabViewType(&self, tab_view_type: NSTabViewType);

        #[method(tabPosition)]
        pub unsafe fn tabPosition(&self) -> NSTabPosition;

        #[method(setTabPosition:)]
        pub unsafe fn setTabPosition(&self, tab_position: NSTabPosition);

        /**
          This will only be respected if NSTabPosition is NSTabPositionNone.
        */
        #[method(tabViewBorderType)]
        pub unsafe fn tabViewBorderType(&self) -> NSTabViewBorderType;

        /**
          This will only be respected if NSTabPosition is NSTabPositionNone.
        */
        #[method(setTabViewBorderType:)]
        pub unsafe fn setTabViewBorderType(&self, tab_view_border_type: NSTabViewBorderType);

        #[cfg(all(feature = "AppKit_NSTabViewItem", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other tabViewItems)]
        pub unsafe fn tabViewItems(&self) -> Id<NSArray<NSTabViewItem>>;

        #[cfg(all(feature = "AppKit_NSTabViewItem", feature = "Foundation_NSArray"))]
        #[method(setTabViewItems:)]
        pub unsafe fn setTabViewItems(&self, tab_view_items: &NSArray<NSTabViewItem>);

        #[method(allowsTruncatedLabels)]
        pub unsafe fn allowsTruncatedLabels(&self) -> bool;

        #[method(setAllowsTruncatedLabels:)]
        pub unsafe fn setAllowsTruncatedLabels(&self, allows_truncated_labels: bool);

        /**
          returns the minimum size of the tab view
        */
        #[method(minimumSize)]
        pub unsafe fn minimumSize(&self) -> NSSize;

        /**
          only relevant for borderless tab view type
        */
        #[method(drawsBackground)]
        pub unsafe fn drawsBackground(&self) -> bool;

        /**
          only relevant for borderless tab view type
        */
        #[method(setDrawsBackground:)]
        pub unsafe fn setDrawsBackground(&self, draws_background: bool);

        #[method(controlSize)]
        pub unsafe fn controlSize(&self) -> NSControlSize;

        #[method(setControlSize:)]
        pub unsafe fn setControlSize(&self, control_size: NSControlSize);

        #[cfg(feature = "AppKit_NSTabViewItem")]
        #[method(addTabViewItem:)]
        pub unsafe fn addTabViewItem(&self, tab_view_item: &NSTabViewItem);

        #[cfg(feature = "AppKit_NSTabViewItem")]
        #[method(insertTabViewItem:atIndex:)]
        pub unsafe fn insertTabViewItem_atIndex(
            &self,
            tab_view_item: &NSTabViewItem,
            index: NSInteger,
        );

        #[cfg(feature = "AppKit_NSTabViewItem")]
        #[method(removeTabViewItem:)]
        pub unsafe fn removeTabViewItem(&self, tab_view_item: &NSTabViewItem);

        /**
          Delegate
        */
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<ProtocolObject<dyn NSTabViewDelegate>>>;

        /**
          Delegate
        */
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&ProtocolObject<dyn NSTabViewDelegate>>);

        #[cfg(feature = "AppKit_NSTabViewItem")]
        #[method_id(@__retain_semantics Other tabViewItemAtPoint:)]
        pub unsafe fn tabViewItemAtPoint(&self, point: NSPoint) -> Option<Id<NSTabViewItem>>;

        /**
          Return the rect available for a "page".
        */
        #[method(contentRect)]
        pub unsafe fn contentRect(&self) -> NSRect;

        /**
          Query
        */
        #[method(numberOfTabViewItems)]
        pub unsafe fn numberOfTabViewItems(&self) -> NSInteger;

        #[cfg(feature = "AppKit_NSTabViewItem")]
        #[method(indexOfTabViewItem:)]
        pub unsafe fn indexOfTabViewItem(&self, tab_view_item: &NSTabViewItem) -> NSInteger;

        #[cfg(feature = "AppKit_NSTabViewItem")]
        #[method_id(@__retain_semantics Other tabViewItemAtIndex:)]
        pub unsafe fn tabViewItemAtIndex(&self, index: NSInteger) -> Id<NSTabViewItem>;

        #[method(indexOfTabViewItemWithIdentifier:)]
        pub unsafe fn indexOfTabViewItemWithIdentifier(&self, identifier: &Object) -> NSInteger;

        #[deprecated = "The controlTint property is not respected on 10.14 and later."]
        #[method(controlTint)]
        pub unsafe fn controlTint(&self) -> NSControlTint;

        #[deprecated = "The controlTint property is not respected on 10.14 and later."]
        #[method(setControlTint:)]
        pub unsafe fn setControlTint(&self, control_tint: NSControlTint);
    }
);

extern_protocol!(
    /**
     ================================================================================
        NSTabViewDelegate protocol
    ================================================================================
    */
    pub unsafe trait NSTabViewDelegate: NSObjectProtocol {
        #[cfg(all(feature = "AppKit_NSTabView", feature = "AppKit_NSTabViewItem"))]
        #[optional]
        #[method(tabView:shouldSelectTabViewItem:)]
        unsafe fn tabView_shouldSelectTabViewItem(
            &self,
            tab_view: &NSTabView,
            tab_view_item: Option<&NSTabViewItem>,
        ) -> bool;

        #[cfg(all(feature = "AppKit_NSTabView", feature = "AppKit_NSTabViewItem"))]
        #[optional]
        #[method(tabView:willSelectTabViewItem:)]
        unsafe fn tabView_willSelectTabViewItem(
            &self,
            tab_view: &NSTabView,
            tab_view_item: Option<&NSTabViewItem>,
        );

        #[cfg(all(feature = "AppKit_NSTabView", feature = "AppKit_NSTabViewItem"))]
        #[optional]
        #[method(tabView:didSelectTabViewItem:)]
        unsafe fn tabView_didSelectTabViewItem(
            &self,
            tab_view: &NSTabView,
            tab_view_item: Option<&NSTabViewItem>,
        );

        #[cfg(feature = "AppKit_NSTabView")]
        #[optional]
        #[method(tabViewDidChangeNumberOfTabViewItems:)]
        unsafe fn tabViewDidChangeNumberOfTabViewItems(&self, tab_view: &NSTabView);
    }

    unsafe impl ProtocolType for dyn NSTabViewDelegate {}
);

extern_methods!(
    /// Methods declared on superclass `NSView`
    #[cfg(feature = "AppKit_NSTabView")]
    unsafe impl NSTabView {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Option<Allocated<Self>>, frame_rect: NSRect) -> Id<Self>;
    }
);
