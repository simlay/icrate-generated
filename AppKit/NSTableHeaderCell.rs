//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSTableHeaderCell")]
    #[cfg(not(any(target_os = "ios")))]
    pub struct NSTableHeaderCell;

    #[cfg(feature = "AppKit_NSTableHeaderCell")]
    unsafe impl ClassType for NSTableHeaderCell {
        #[inherits(NSActionCell, NSCell, NSObject)]
        type Super = NSTextFieldCell;
    }
);

#[cfg(feature = "AppKit_NSTableHeaderCell")]
unsafe impl NSAccessibility for NSTableHeaderCell {}

#[cfg(feature = "AppKit_NSTableHeaderCell")]
unsafe impl NSAccessibilityElementProtocol for NSTableHeaderCell {}

#[cfg(feature = "AppKit_NSTableHeaderCell")]
unsafe impl NSCoding for NSTableHeaderCell {}

#[cfg(feature = "AppKit_NSTableHeaderCell")]
unsafe impl NSObjectProtocol for NSTableHeaderCell {}

#[cfg(feature = "AppKit_NSTableHeaderCell")]
unsafe impl NSUserInterfaceItemIdentification for NSTableHeaderCell {}

extern_methods!(
    #[cfg(feature = "AppKit_NSTableHeaderCell")]
    unsafe impl NSTableHeaderCell {
        #[cfg(feature = "AppKit_NSView")]
        #[cfg(not(any(target_os = "ios")))]
        #[method(drawSortIndicatorWithFrame:inView:ascending:priority:)]
        pub unsafe fn drawSortIndicatorWithFrame_inView_ascending_priority(
            &self,
            cell_frame: NSRect,
            control_view: &NSView,
            ascending: bool,
            priority: NSInteger,
        );

        #[cfg(not(any(target_os = "ios")))]
        #[method(sortIndicatorRectForBounds:)]
        pub unsafe fn sortIndicatorRectForBounds(&self, rect: NSRect) -> NSRect;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSTextFieldCell`
    #[cfg(feature = "AppKit_NSTableHeaderCell")]
    unsafe impl NSTableHeaderCell {
        #[cfg(feature = "Foundation_NSString")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Init initTextCell:)]
        pub unsafe fn initTextCell(this: Option<Allocated<Self>>, string: &NSString) -> Id<Self>;

        #[cfg(feature = "AppKit_NSImage")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Init initImageCell:)]
        pub unsafe fn initImageCell(
            this: Option<Allocated<Self>>,
            image: Option<&NSImage>,
        ) -> Id<Self>;
    }
);
