//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    #[cfg(not(any(target_os = "ios")))]
    pub enum NSPathStyle {
        #[cfg(not(any(target_os = "ios")))]
        NSPathStyleStandard = 0,
        #[cfg(not(any(target_os = "ios")))]
        NSPathStylePopUp = 2,
        #[deprecated]
        #[cfg(not(any(target_os = "ios")))]
        NSPathStyleNavigationBar = 1,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSPathCell")]
    #[cfg(not(any(target_os = "ios")))]
    pub struct NSPathCell;

    #[cfg(feature = "AppKit_NSPathCell")]
    unsafe impl ClassType for NSPathCell {
        #[inherits(NSCell, NSObject)]
        type Super = NSActionCell;
    }
);

#[cfg(feature = "AppKit_NSPathCell")]
unsafe impl NSAccessibility for NSPathCell {}

#[cfg(feature = "AppKit_NSPathCell")]
unsafe impl NSAccessibilityElementProtocol for NSPathCell {}

#[cfg(feature = "AppKit_NSPathCell")]
unsafe impl NSCoding for NSPathCell {}

#[cfg(feature = "AppKit_NSPathCell")]
unsafe impl NSMenuItemValidation for NSPathCell {}

#[cfg(feature = "AppKit_NSPathCell")]
unsafe impl NSObjectProtocol for NSPathCell {}

#[cfg(feature = "AppKit_NSPathCell")]
unsafe impl NSOpenSavePanelDelegate for NSPathCell {}

#[cfg(feature = "AppKit_NSPathCell")]
unsafe impl NSUserInterfaceItemIdentification for NSPathCell {}

extern_methods!(
    #[cfg(feature = "AppKit_NSPathCell")]
    unsafe impl NSPathCell {
        #[cfg(not(any(target_os = "ios")))]
        #[method(pathStyle)]
        pub unsafe fn pathStyle(&self) -> NSPathStyle;

        #[cfg(not(any(target_os = "ios")))]
        #[method(setPathStyle:)]
        pub unsafe fn setPathStyle(&self, path_style: NSPathStyle);

        #[cfg(feature = "Foundation_NSURL")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other URL)]
        pub unsafe fn URL(&self) -> Option<Id<NSURL>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[cfg(not(any(target_os = "ios")))]
        #[method(setURL:)]
        pub unsafe fn setURL(&self, url: Option<&NSURL>);

        #[cfg(not(any(target_os = "ios")))]
        #[method(setObjectValue:)]
        pub unsafe fn setObjectValue(&self, obj: Option<&Object>);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other allowedTypes)]
        pub unsafe fn allowedTypes(&self) -> Option<Id<NSArray<NSString>>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[cfg(not(any(target_os = "ios")))]
        #[method(setAllowedTypes:)]
        pub unsafe fn setAllowedTypes(&self, allowed_types: Option<&NSArray<NSString>>);

        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<ProtocolObject<dyn NSPathCellDelegate>>>;

        #[cfg(not(any(target_os = "ios")))]
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&ProtocolObject<dyn NSPathCellDelegate>>);

        #[cfg(not(any(target_os = "ios")))]
        #[method(pathComponentCellClass)]
        pub unsafe fn pathComponentCellClass() -> &'static Class;

        #[cfg(all(feature = "AppKit_NSPathComponentCell", feature = "Foundation_NSArray"))]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other pathComponentCells)]
        pub unsafe fn pathComponentCells(&self) -> Id<NSArray<NSPathComponentCell>>;

        #[cfg(all(feature = "AppKit_NSPathComponentCell", feature = "Foundation_NSArray"))]
        #[cfg(not(any(target_os = "ios")))]
        #[method(setPathComponentCells:)]
        pub unsafe fn setPathComponentCells(
            &self,
            path_component_cells: &NSArray<NSPathComponentCell>,
        );

        #[cfg(all(feature = "AppKit_NSPathComponentCell", feature = "AppKit_NSView"))]
        #[cfg(not(any(target_os = "ios")))]
        #[method(rectOfPathComponentCell:withFrame:inView:)]
        pub unsafe fn rectOfPathComponentCell_withFrame_inView(
            &self,
            cell: &NSPathComponentCell,
            frame: NSRect,
            view: &NSView,
        ) -> NSRect;

        #[cfg(all(feature = "AppKit_NSPathComponentCell", feature = "AppKit_NSView"))]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other pathComponentCellAtPoint:withFrame:inView:)]
        pub unsafe fn pathComponentCellAtPoint_withFrame_inView(
            &self,
            point: NSPoint,
            frame: NSRect,
            view: &NSView,
        ) -> Option<Id<NSPathComponentCell>>;

        #[cfg(feature = "AppKit_NSPathComponentCell")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other clickedPathComponentCell)]
        pub unsafe fn clickedPathComponentCell(&self) -> Option<Id<NSPathComponentCell>>;

        #[cfg(all(feature = "AppKit_NSEvent", feature = "AppKit_NSView"))]
        #[cfg(not(any(target_os = "ios")))]
        #[method(mouseEntered:withFrame:inView:)]
        pub unsafe fn mouseEntered_withFrame_inView(
            &self,
            event: &NSEvent,
            frame: NSRect,
            view: &NSView,
        );

        #[cfg(all(feature = "AppKit_NSEvent", feature = "AppKit_NSView"))]
        #[cfg(not(any(target_os = "ios")))]
        #[method(mouseExited:withFrame:inView:)]
        pub unsafe fn mouseExited_withFrame_inView(
            &self,
            event: &NSEvent,
            frame: NSRect,
            view: &NSView,
        );

        #[cfg(not(any(target_os = "ios")))]
        #[method(doubleAction)]
        pub unsafe fn doubleAction(&self) -> Option<Sel>;

        #[cfg(not(any(target_os = "ios")))]
        #[method(setDoubleAction:)]
        pub unsafe fn setDoubleAction(&self, double_action: Option<Sel>);

        #[cfg(feature = "AppKit_NSColor")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other backgroundColor)]
        pub unsafe fn backgroundColor(&self) -> Option<Id<NSColor>>;

        #[cfg(feature = "AppKit_NSColor")]
        #[cfg(not(any(target_os = "ios")))]
        #[method(setBackgroundColor:)]
        pub unsafe fn setBackgroundColor(&self, background_color: Option<&NSColor>);

        #[cfg(feature = "Foundation_NSString")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other placeholderString)]
        pub unsafe fn placeholderString(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[cfg(not(any(target_os = "ios")))]
        #[method(setPlaceholderString:)]
        pub unsafe fn setPlaceholderString(&self, placeholder_string: Option<&NSString>);

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other placeholderAttributedString)]
        pub unsafe fn placeholderAttributedString(&self) -> Option<Id<NSAttributedString>>;

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[cfg(not(any(target_os = "ios")))]
        #[method(setPlaceholderAttributedString:)]
        pub unsafe fn setPlaceholderAttributedString(
            &self,
            placeholder_attributed_string: Option<&NSAttributedString>,
        );
    }
);

extern_protocol!(
    #[cfg(not(any(target_os = "ios")))]
    pub unsafe trait NSPathCellDelegate: NSObjectProtocol {
        #[cfg(all(feature = "AppKit_NSOpenPanel", feature = "AppKit_NSPathCell"))]
        #[cfg(not(any(target_os = "ios")))]
        #[optional]
        #[method(pathCell:willDisplayOpenPanel:)]
        unsafe fn pathCell_willDisplayOpenPanel(
            &self,
            path_cell: &NSPathCell,
            open_panel: &NSOpenPanel,
        );

        #[cfg(all(feature = "AppKit_NSMenu", feature = "AppKit_NSPathCell"))]
        #[cfg(not(any(target_os = "ios")))]
        #[optional]
        #[method(pathCell:willPopUpMenu:)]
        unsafe fn pathCell_willPopUpMenu(&self, path_cell: &NSPathCell, menu: &NSMenu);
    }

    unsafe impl ProtocolType for dyn NSPathCellDelegate {}
);

extern_methods!(
    /// Methods declared on superclass `NSCell`
    #[cfg(feature = "AppKit_NSPathCell")]
    unsafe impl NSPathCell {
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
