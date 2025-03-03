//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSSegmentedCell")]
    pub struct NSSegmentedCell;

    #[cfg(feature = "AppKit_NSSegmentedCell")]
    unsafe impl ClassType for NSSegmentedCell {
        #[inherits(NSCell, NSObject)]
        type Super = NSActionCell;
    }
);

#[cfg(feature = "AppKit_NSSegmentedCell")]
unsafe impl NSAccessibility for NSSegmentedCell {}

#[cfg(feature = "AppKit_NSSegmentedCell")]
unsafe impl NSAccessibilityElementProtocol for NSSegmentedCell {}

#[cfg(feature = "AppKit_NSSegmentedCell")]
unsafe impl NSCoding for NSSegmentedCell {}

#[cfg(feature = "AppKit_NSSegmentedCell")]
unsafe impl NSObjectProtocol for NSSegmentedCell {}

#[cfg(feature = "AppKit_NSSegmentedCell")]
unsafe impl NSUserInterfaceItemIdentification for NSSegmentedCell {}

extern_methods!(
    #[cfg(feature = "AppKit_NSSegmentedCell")]
    unsafe impl NSSegmentedCell {
        #[method(segmentCount)]
        pub unsafe fn segmentCount(&self) -> NSInteger;

        #[method(setSegmentCount:)]
        pub unsafe fn setSegmentCount(&self, segment_count: NSInteger);

        #[method(selectedSegment)]
        pub unsafe fn selectedSegment(&self) -> NSInteger;

        #[method(setSelectedSegment:)]
        pub unsafe fn setSelectedSegment(&self, selected_segment: NSInteger);

        #[method(selectSegmentWithTag:)]
        pub unsafe fn selectSegmentWithTag(&self, tag: NSInteger) -> bool;

        #[method(makeNextSegmentKey)]
        pub unsafe fn makeNextSegmentKey(&self);

        #[method(makePreviousSegmentKey)]
        pub unsafe fn makePreviousSegmentKey(&self);

        #[method(trackingMode)]
        pub unsafe fn trackingMode(&self) -> NSSegmentSwitchTracking;

        #[method(setTrackingMode:)]
        pub unsafe fn setTrackingMode(&self, tracking_mode: NSSegmentSwitchTracking);

        #[method(setWidth:forSegment:)]
        pub unsafe fn setWidth_forSegment(&self, width: CGFloat, segment: NSInteger);

        #[method(widthForSegment:)]
        pub unsafe fn widthForSegment(&self, segment: NSInteger) -> CGFloat;

        #[cfg(feature = "AppKit_NSImage")]
        #[method(setImage:forSegment:)]
        pub unsafe fn setImage_forSegment(&self, image: Option<&NSImage>, segment: NSInteger);

        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other imageForSegment:)]
        pub unsafe fn imageForSegment(&self, segment: NSInteger) -> Option<Id<NSImage>>;

        #[method(setImageScaling:forSegment:)]
        pub unsafe fn setImageScaling_forSegment(
            &self,
            scaling: NSImageScaling,
            segment: NSInteger,
        );

        #[method(imageScalingForSegment:)]
        pub unsafe fn imageScalingForSegment(&self, segment: NSInteger) -> NSImageScaling;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setLabel:forSegment:)]
        pub unsafe fn setLabel_forSegment(&self, label: &NSString, segment: NSInteger);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other labelForSegment:)]
        pub unsafe fn labelForSegment(&self, segment: NSInteger) -> Option<Id<NSString>>;

        #[method(setSelected:forSegment:)]
        pub unsafe fn setSelected_forSegment(&self, selected: bool, segment: NSInteger);

        #[method(isSelectedForSegment:)]
        pub unsafe fn isSelectedForSegment(&self, segment: NSInteger) -> bool;

        #[method(setEnabled:forSegment:)]
        pub unsafe fn setEnabled_forSegment(&self, enabled: bool, segment: NSInteger);

        #[method(isEnabledForSegment:)]
        pub unsafe fn isEnabledForSegment(&self, segment: NSInteger) -> bool;

        #[cfg(feature = "AppKit_NSMenu")]
        #[method(setMenu:forSegment:)]
        pub unsafe fn setMenu_forSegment(&self, menu: Option<&NSMenu>, segment: NSInteger);

        #[cfg(feature = "AppKit_NSMenu")]
        #[method_id(@__retain_semantics Other menuForSegment:)]
        pub unsafe fn menuForSegment(&self, segment: NSInteger) -> Option<Id<NSMenu>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setToolTip:forSegment:)]
        pub unsafe fn setToolTip_forSegment(&self, tool_tip: Option<&NSString>, segment: NSInteger);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other toolTipForSegment:)]
        pub unsafe fn toolTipForSegment(&self, segment: NSInteger) -> Option<Id<NSString>>;

        #[method(setTag:forSegment:)]
        pub unsafe fn setTag_forSegment(&self, tag: NSInteger, segment: NSInteger);

        #[method(tagForSegment:)]
        pub unsafe fn tagForSegment(&self, segment: NSInteger) -> NSInteger;

        #[method(segmentStyle)]
        pub unsafe fn segmentStyle(&self) -> NSSegmentStyle;

        #[method(setSegmentStyle:)]
        pub unsafe fn setSegmentStyle(&self, segment_style: NSSegmentStyle);

        #[cfg(feature = "AppKit_NSView")]
        #[method(drawSegment:inFrame:withView:)]
        pub unsafe fn drawSegment_inFrame_withView(
            &self,
            segment: NSInteger,
            frame: NSRect,
            control_view: &NSView,
        );
    }
);

extern_methods!(
    /// NSSegmentBackgroundStyle
    #[cfg(feature = "AppKit_NSSegmentedCell")]
    unsafe impl NSSegmentedCell {
        #[method(interiorBackgroundStyleForSegment:)]
        pub unsafe fn interiorBackgroundStyleForSegment(
            &self,
            segment: NSInteger,
        ) -> NSBackgroundStyle;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSCell`
    #[cfg(feature = "AppKit_NSSegmentedCell")]
    unsafe impl NSSegmentedCell {
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
