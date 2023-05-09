//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_enum!(
    #[underlying(c_int)]
    pub enum __anonymous__ {
        NSOutlineViewDropOnItemIndex = -1,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSOutlineView")]
    pub struct NSOutlineView;

    #[cfg(feature = "AppKit_NSOutlineView")]
    unsafe impl ClassType for NSOutlineView {
        #[inherits(NSControl, NSView, NSResponder, NSObject)]
        type Super = NSTableView;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "AppKit_NSOutlineView")]
unsafe impl NSAccessibility for NSOutlineView {}

#[cfg(feature = "AppKit_NSOutlineView")]
unsafe impl NSAccessibilityElementProtocol for NSOutlineView {}

#[cfg(feature = "AppKit_NSOutlineView")]
unsafe impl NSAccessibilityGroup for NSOutlineView {}

#[cfg(feature = "AppKit_NSOutlineView")]
unsafe impl NSAccessibilityOutline for NSOutlineView {}

#[cfg(feature = "AppKit_NSOutlineView")]
unsafe impl NSAccessibilityTable for NSOutlineView {}

#[cfg(feature = "AppKit_NSOutlineView")]
unsafe impl NSAnimatablePropertyContainer for NSOutlineView {}

#[cfg(feature = "AppKit_NSOutlineView")]
unsafe impl NSAppearanceCustomization for NSOutlineView {}

#[cfg(feature = "AppKit_NSOutlineView")]
unsafe impl NSCoding for NSOutlineView {}

#[cfg(feature = "AppKit_NSOutlineView")]
unsafe impl NSDraggingDestination for NSOutlineView {}

#[cfg(feature = "AppKit_NSOutlineView")]
unsafe impl NSDraggingSource for NSOutlineView {}

#[cfg(feature = "AppKit_NSOutlineView")]
unsafe impl NSObjectProtocol for NSOutlineView {}

#[cfg(feature = "AppKit_NSOutlineView")]
unsafe impl NSTextDelegate for NSOutlineView {}

#[cfg(feature = "AppKit_NSOutlineView")]
unsafe impl NSTextViewDelegate for NSOutlineView {}

#[cfg(feature = "AppKit_NSOutlineView")]
unsafe impl NSUserInterfaceItemIdentification for NSOutlineView {}

#[cfg(feature = "AppKit_NSOutlineView")]
unsafe impl NSUserInterfaceValidations for NSOutlineView {}

extern_methods!(
    #[cfg(feature = "AppKit_NSOutlineView")]
    unsafe impl NSOutlineView {
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<ProtocolObject<dyn NSOutlineViewDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn NSOutlineViewDelegate>>,
        );

        #[method_id(@__retain_semantics Other dataSource)]
        pub unsafe fn dataSource(&self) -> Option<Id<ProtocolObject<dyn NSOutlineViewDataSource>>>;

        #[method(setDataSource:)]
        pub unsafe fn setDataSource(
            &self,
            data_source: Option<&ProtocolObject<dyn NSOutlineViewDataSource>>,
        );

        #[cfg(feature = "AppKit_NSTableColumn")]
        #[method_id(@__retain_semantics Other outlineTableColumn)]
        pub unsafe fn outlineTableColumn(&self) -> Option<Id<NSTableColumn>>;

        #[cfg(feature = "AppKit_NSTableColumn")]
        #[method(setOutlineTableColumn:)]
        pub unsafe fn setOutlineTableColumn(&self, outline_table_column: Option<&NSTableColumn>);

        #[method(isExpandable:)]
        pub unsafe fn isExpandable(&self, item: Option<&Object>) -> bool;

        #[method(numberOfChildrenOfItem:)]
        pub unsafe fn numberOfChildrenOfItem(&self, item: Option<&Object>) -> NSInteger;

        #[method_id(@__retain_semantics Other child:ofItem:)]
        pub unsafe fn child_ofItem(
            &self,
            index: NSInteger,
            item: Option<&Object>,
        ) -> Option<Id<Object>>;

        #[method(expandItem:expandChildren:)]
        pub unsafe fn expandItem_expandChildren(
            &self,
            item: Option<&Object>,
            expand_children: bool,
        );

        #[method(expandItem:)]
        pub unsafe fn expandItem(&self, item: Option<&Object>);

        #[method(collapseItem:collapseChildren:)]
        pub unsafe fn collapseItem_collapseChildren(
            &self,
            item: Option<&Object>,
            collapse_children: bool,
        );

        #[method(collapseItem:)]
        pub unsafe fn collapseItem(&self, item: Option<&Object>);

        #[method(reloadItem:reloadChildren:)]
        pub unsafe fn reloadItem_reloadChildren(
            &self,
            item: Option<&Object>,
            reload_children: bool,
        );

        #[method(reloadItem:)]
        pub unsafe fn reloadItem(&self, item: Option<&Object>);

        #[method_id(@__retain_semantics Other parentForItem:)]
        pub unsafe fn parentForItem(&self, item: Option<&Object>) -> Option<Id<Object>>;

        #[method(childIndexForItem:)]
        pub unsafe fn childIndexForItem(&self, item: &Object) -> NSInteger;

        #[method_id(@__retain_semantics Other itemAtRow:)]
        pub unsafe fn itemAtRow(&self, row: NSInteger) -> Option<Id<Object>>;

        #[method(rowForItem:)]
        pub unsafe fn rowForItem(&self, item: Option<&Object>) -> NSInteger;

        #[method(levelForItem:)]
        pub unsafe fn levelForItem(&self, item: Option<&Object>) -> NSInteger;

        #[method(levelForRow:)]
        pub unsafe fn levelForRow(&self, row: NSInteger) -> NSInteger;

        #[method(isItemExpanded:)]
        pub unsafe fn isItemExpanded(&self, item: Option<&Object>) -> bool;

        #[method(indentationPerLevel)]
        pub unsafe fn indentationPerLevel(&self) -> CGFloat;

        #[method(setIndentationPerLevel:)]
        pub unsafe fn setIndentationPerLevel(&self, indentation_per_level: CGFloat);

        #[method(indentationMarkerFollowsCell)]
        pub unsafe fn indentationMarkerFollowsCell(&self) -> bool;

        #[method(setIndentationMarkerFollowsCell:)]
        pub unsafe fn setIndentationMarkerFollowsCell(&self, indentation_marker_follows_cell: bool);

        #[method(autoresizesOutlineColumn)]
        pub unsafe fn autoresizesOutlineColumn(&self) -> bool;

        #[method(setAutoresizesOutlineColumn:)]
        pub unsafe fn setAutoresizesOutlineColumn(&self, autoresizes_outline_column: bool);

        #[method(frameOfOutlineCellAtRow:)]
        pub unsafe fn frameOfOutlineCellAtRow(&self, row: NSInteger) -> NSRect;

        #[method(setDropItem:dropChildIndex:)]
        pub unsafe fn setDropItem_dropChildIndex(&self, item: Option<&Object>, index: NSInteger);

        #[method(shouldCollapseAutoExpandedItemsForDeposited:)]
        pub unsafe fn shouldCollapseAutoExpandedItemsForDeposited(&self, deposited: bool) -> bool;

        #[method(autosaveExpandedItems)]
        pub unsafe fn autosaveExpandedItems(&self) -> bool;

        #[method(setAutosaveExpandedItems:)]
        pub unsafe fn setAutosaveExpandedItems(&self, autosave_expanded_items: bool);

        #[cfg(feature = "Foundation_NSIndexSet")]
        #[method(insertItemsAtIndexes:inParent:withAnimation:)]
        pub unsafe fn insertItemsAtIndexes_inParent_withAnimation(
            &self,
            indexes: &NSIndexSet,
            parent: Option<&Object>,
            animation_options: NSTableViewAnimationOptions,
        );

        #[cfg(feature = "Foundation_NSIndexSet")]
        #[method(removeItemsAtIndexes:inParent:withAnimation:)]
        pub unsafe fn removeItemsAtIndexes_inParent_withAnimation(
            &self,
            indexes: &NSIndexSet,
            parent: Option<&Object>,
            animation_options: NSTableViewAnimationOptions,
        );

        #[method(moveItemAtIndex:inParent:toIndex:inParent:)]
        pub unsafe fn moveItemAtIndex_inParent_toIndex_inParent(
            &self,
            from_index: NSInteger,
            old_parent: Option<&Object>,
            to_index: NSInteger,
            new_parent: Option<&Object>,
        );

        #[cfg(feature = "Foundation_NSIndexSet")]
        #[method(insertRowsAtIndexes:withAnimation:)]
        pub unsafe fn insertRowsAtIndexes_withAnimation(
            &self,
            indexes: &NSIndexSet,
            animation_options: NSTableViewAnimationOptions,
        );

        #[cfg(feature = "Foundation_NSIndexSet")]
        #[method(removeRowsAtIndexes:withAnimation:)]
        pub unsafe fn removeRowsAtIndexes_withAnimation(
            &self,
            indexes: &NSIndexSet,
            animation_options: NSTableViewAnimationOptions,
        );

        #[method(moveRowAtIndex:toIndex:)]
        pub unsafe fn moveRowAtIndex_toIndex(&self, old_index: NSInteger, new_index: NSInteger);

        #[method(userInterfaceLayoutDirection)]
        pub unsafe fn userInterfaceLayoutDirection(&self) -> NSUserInterfaceLayoutDirection;

        #[method(setUserInterfaceLayoutDirection:)]
        pub unsafe fn setUserInterfaceLayoutDirection(
            &self,
            user_interface_layout_direction: NSUserInterfaceLayoutDirection,
        );

        #[method(stronglyReferencesItems)]
        pub unsafe fn stronglyReferencesItems(&self) -> bool;

        #[method(setStronglyReferencesItems:)]
        pub unsafe fn setStronglyReferencesItems(&self, strongly_references_items: bool);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSTableView`
    #[cfg(feature = "AppKit_NSOutlineView")]
    unsafe impl NSOutlineView {
        #[method_id(@__retain_semantics Init initWithFrame:)]
        pub unsafe fn initWithFrame(this: Option<Allocated<Self>>, frame_rect: NSRect) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(feature = "AppKit_NSOutlineView")]
    unsafe impl NSOutlineView {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "AppKit_NSOutlineView")]
    unsafe impl NSOutlineView {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_protocol!(
    pub unsafe trait NSOutlineViewDataSource: NSObjectProtocol {
        #[cfg(feature = "AppKit_NSOutlineView")]
        #[optional]
        #[method(outlineView:numberOfChildrenOfItem:)]
        unsafe fn outlineView_numberOfChildrenOfItem(
            &self,
            outline_view: &NSOutlineView,
            item: Option<&Object>,
        ) -> NSInteger;

        #[cfg(feature = "AppKit_NSOutlineView")]
        #[optional]
        #[method_id(@__retain_semantics Other outlineView:child:ofItem:)]
        unsafe fn outlineView_child_ofItem(
            &self,
            outline_view: &NSOutlineView,
            index: NSInteger,
            item: Option<&Object>,
        ) -> Id<Object>;

        #[cfg(feature = "AppKit_NSOutlineView")]
        #[optional]
        #[method(outlineView:isItemExpandable:)]
        unsafe fn outlineView_isItemExpandable(
            &self,
            outline_view: &NSOutlineView,
            item: &Object,
        ) -> bool;

        #[cfg(all(feature = "AppKit_NSOutlineView", feature = "AppKit_NSTableColumn"))]
        #[optional]
        #[method_id(@__retain_semantics Other outlineView:objectValueForTableColumn:byItem:)]
        unsafe fn outlineView_objectValueForTableColumn_byItem(
            &self,
            outline_view: &NSOutlineView,
            table_column: Option<&NSTableColumn>,
            item: Option<&Object>,
        ) -> Option<Id<Object>>;

        #[cfg(all(feature = "AppKit_NSOutlineView", feature = "AppKit_NSTableColumn"))]
        #[optional]
        #[method(outlineView:setObjectValue:forTableColumn:byItem:)]
        unsafe fn outlineView_setObjectValue_forTableColumn_byItem(
            &self,
            outline_view: &NSOutlineView,
            object: Option<&Object>,
            table_column: Option<&NSTableColumn>,
            item: Option<&Object>,
        );

        #[cfg(feature = "AppKit_NSOutlineView")]
        #[optional]
        #[method_id(@__retain_semantics Other outlineView:itemForPersistentObject:)]
        unsafe fn outlineView_itemForPersistentObject(
            &self,
            outline_view: &NSOutlineView,
            object: &Object,
        ) -> Option<Id<Object>>;

        #[cfg(feature = "AppKit_NSOutlineView")]
        #[optional]
        #[method_id(@__retain_semantics Other outlineView:persistentObjectForItem:)]
        unsafe fn outlineView_persistentObjectForItem(
            &self,
            outline_view: &NSOutlineView,
            item: Option<&Object>,
        ) -> Option<Id<Object>>;

        #[cfg(all(
            feature = "AppKit_NSOutlineView",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSSortDescriptor"
        ))]
        #[optional]
        #[method(outlineView:sortDescriptorsDidChange:)]
        unsafe fn outlineView_sortDescriptorsDidChange(
            &self,
            outline_view: &NSOutlineView,
            old_descriptors: &NSArray<NSSortDescriptor>,
        );

        #[cfg(feature = "AppKit_NSOutlineView")]
        #[optional]
        #[method_id(@__retain_semantics Other outlineView:pasteboardWriterForItem:)]
        unsafe fn outlineView_pasteboardWriterForItem(
            &self,
            outline_view: &NSOutlineView,
            item: &Object,
        ) -> Option<Id<ProtocolObject<dyn NSPasteboardWriting>>>;

        #[cfg(all(
            feature = "AppKit_NSDraggingSession",
            feature = "AppKit_NSOutlineView",
            feature = "Foundation_NSArray"
        ))]
        #[optional]
        #[method(outlineView:draggingSession:willBeginAtPoint:forItems:)]
        unsafe fn outlineView_draggingSession_willBeginAtPoint_forItems(
            &self,
            outline_view: &NSOutlineView,
            session: &NSDraggingSession,
            screen_point: NSPoint,
            dragged_items: &NSArray,
        );

        #[cfg(all(feature = "AppKit_NSDraggingSession", feature = "AppKit_NSOutlineView"))]
        #[optional]
        #[method(outlineView:draggingSession:endedAtPoint:operation:)]
        unsafe fn outlineView_draggingSession_endedAtPoint_operation(
            &self,
            outline_view: &NSOutlineView,
            session: &NSDraggingSession,
            screen_point: NSPoint,
            operation: NSDragOperation,
        );

        #[cfg(all(
            feature = "AppKit_NSOutlineView",
            feature = "AppKit_NSPasteboard",
            feature = "Foundation_NSArray"
        ))]
        #[deprecated = "Use -outlineView:pasteboardWriterForItem: instead"]
        #[optional]
        #[method(outlineView:writeItems:toPasteboard:)]
        unsafe fn outlineView_writeItems_toPasteboard(
            &self,
            outline_view: &NSOutlineView,
            items: &NSArray,
            pasteboard: &NSPasteboard,
        ) -> bool;

        #[cfg(feature = "AppKit_NSOutlineView")]
        #[optional]
        #[method(outlineView:updateDraggingItemsForDrag:)]
        unsafe fn outlineView_updateDraggingItemsForDrag(
            &self,
            outline_view: &NSOutlineView,
            dragging_info: &ProtocolObject<dyn NSDraggingInfo>,
        );

        #[cfg(feature = "AppKit_NSOutlineView")]
        #[optional]
        #[method(outlineView:validateDrop:proposedItem:proposedChildIndex:)]
        unsafe fn outlineView_validateDrop_proposedItem_proposedChildIndex(
            &self,
            outline_view: &NSOutlineView,
            info: &ProtocolObject<dyn NSDraggingInfo>,
            item: Option<&Object>,
            index: NSInteger,
        ) -> NSDragOperation;

        #[cfg(feature = "AppKit_NSOutlineView")]
        #[optional]
        #[method(outlineView:acceptDrop:item:childIndex:)]
        unsafe fn outlineView_acceptDrop_item_childIndex(
            &self,
            outline_view: &NSOutlineView,
            info: &ProtocolObject<dyn NSDraggingInfo>,
            item: Option<&Object>,
            index: NSInteger,
        ) -> bool;

        #[cfg(all(
            feature = "AppKit_NSOutlineView",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURL"
        ))]
        #[deprecated = "Use NSFilePromiseReceiver objects instead"]
        #[optional]
        #[method_id(@__retain_semantics Other outlineView:namesOfPromisedFilesDroppedAtDestination:forDraggedItems:)]
        unsafe fn outlineView_namesOfPromisedFilesDroppedAtDestination_forDraggedItems(
            &self,
            outline_view: &NSOutlineView,
            drop_destination: &NSURL,
            items: &NSArray,
        ) -> Id<NSArray<NSString>>;
    }

    unsafe impl ProtocolType for dyn NSOutlineViewDataSource {}
);

extern_protocol!(
    pub unsafe trait NSOutlineViewDelegate: NSControlTextEditingDelegate {
        #[cfg(all(
            feature = "AppKit_NSOutlineView",
            feature = "AppKit_NSTableColumn",
            feature = "AppKit_NSView"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other outlineView:viewForTableColumn:item:)]
        unsafe fn outlineView_viewForTableColumn_item(
            &self,
            outline_view: &NSOutlineView,
            table_column: Option<&NSTableColumn>,
            item: &Object,
        ) -> Option<Id<NSView>>;

        #[cfg(all(feature = "AppKit_NSOutlineView", feature = "AppKit_NSTableRowView"))]
        #[optional]
        #[method_id(@__retain_semantics Other outlineView:rowViewForItem:)]
        unsafe fn outlineView_rowViewForItem(
            &self,
            outline_view: &NSOutlineView,
            item: &Object,
        ) -> Option<Id<NSTableRowView>>;

        #[cfg(all(feature = "AppKit_NSOutlineView", feature = "AppKit_NSTableRowView"))]
        #[optional]
        #[method(outlineView:didAddRowView:forRow:)]
        unsafe fn outlineView_didAddRowView_forRow(
            &self,
            outline_view: &NSOutlineView,
            row_view: &NSTableRowView,
            row: NSInteger,
        );

        #[cfg(all(feature = "AppKit_NSOutlineView", feature = "AppKit_NSTableRowView"))]
        #[optional]
        #[method(outlineView:didRemoveRowView:forRow:)]
        unsafe fn outlineView_didRemoveRowView_forRow(
            &self,
            outline_view: &NSOutlineView,
            row_view: &NSTableRowView,
            row: NSInteger,
        );

        #[cfg(all(feature = "AppKit_NSOutlineView", feature = "AppKit_NSTableColumn"))]
        #[optional]
        #[method(outlineView:willDisplayCell:forTableColumn:item:)]
        unsafe fn outlineView_willDisplayCell_forTableColumn_item(
            &self,
            outline_view: &NSOutlineView,
            cell: &Object,
            table_column: Option<&NSTableColumn>,
            item: &Object,
        );

        #[cfg(all(feature = "AppKit_NSOutlineView", feature = "AppKit_NSTableColumn"))]
        #[optional]
        #[method(outlineView:shouldEditTableColumn:item:)]
        unsafe fn outlineView_shouldEditTableColumn_item(
            &self,
            outline_view: &NSOutlineView,
            table_column: Option<&NSTableColumn>,
            item: &Object,
        ) -> bool;

        #[cfg(feature = "AppKit_NSOutlineView")]
        #[optional]
        #[method(selectionShouldChangeInOutlineView:)]
        unsafe fn selectionShouldChangeInOutlineView(&self, outline_view: &NSOutlineView) -> bool;

        #[cfg(feature = "AppKit_NSOutlineView")]
        #[optional]
        #[method(outlineView:shouldSelectItem:)]
        unsafe fn outlineView_shouldSelectItem(
            &self,
            outline_view: &NSOutlineView,
            item: &Object,
        ) -> bool;

        #[cfg(all(feature = "AppKit_NSOutlineView", feature = "Foundation_NSIndexSet"))]
        #[optional]
        #[method_id(@__retain_semantics Other outlineView:selectionIndexesForProposedSelection:)]
        unsafe fn outlineView_selectionIndexesForProposedSelection(
            &self,
            outline_view: &NSOutlineView,
            proposed_selection_indexes: &NSIndexSet,
        ) -> Id<NSIndexSet>;

        #[cfg(all(feature = "AppKit_NSOutlineView", feature = "AppKit_NSTableColumn"))]
        #[optional]
        #[method(outlineView:shouldSelectTableColumn:)]
        unsafe fn outlineView_shouldSelectTableColumn(
            &self,
            outline_view: &NSOutlineView,
            table_column: Option<&NSTableColumn>,
        ) -> bool;

        #[cfg(all(feature = "AppKit_NSOutlineView", feature = "AppKit_NSTableColumn"))]
        #[optional]
        #[method(outlineView:mouseDownInHeaderOfTableColumn:)]
        unsafe fn outlineView_mouseDownInHeaderOfTableColumn(
            &self,
            outline_view: &NSOutlineView,
            table_column: &NSTableColumn,
        );

        #[cfg(all(feature = "AppKit_NSOutlineView", feature = "AppKit_NSTableColumn"))]
        #[optional]
        #[method(outlineView:didClickTableColumn:)]
        unsafe fn outlineView_didClickTableColumn(
            &self,
            outline_view: &NSOutlineView,
            table_column: &NSTableColumn,
        );

        #[cfg(all(feature = "AppKit_NSOutlineView", feature = "AppKit_NSTableColumn"))]
        #[optional]
        #[method(outlineView:didDragTableColumn:)]
        unsafe fn outlineView_didDragTableColumn(
            &self,
            outline_view: &NSOutlineView,
            table_column: &NSTableColumn,
        );

        #[cfg(all(
            feature = "AppKit_NSCell",
            feature = "AppKit_NSOutlineView",
            feature = "AppKit_NSTableColumn",
            feature = "Foundation_NSString"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other outlineView:toolTipForCell:rect:tableColumn:item:mouseLocation:)]
        unsafe fn outlineView_toolTipForCell_rect_tableColumn_item_mouseLocation(
            &self,
            outline_view: &NSOutlineView,
            cell: &NSCell,
            rect: NSRectPointer,
            table_column: Option<&NSTableColumn>,
            item: &Object,
            mouse_location: NSPoint,
        ) -> Id<NSString>;

        #[cfg(feature = "AppKit_NSOutlineView")]
        #[optional]
        #[method(outlineView:heightOfRowByItem:)]
        unsafe fn outlineView_heightOfRowByItem(
            &self,
            outline_view: &NSOutlineView,
            item: &Object,
        ) -> CGFloat;

        #[cfg(all(
            feature = "AppKit_NSOutlineView",
            feature = "AppKit_NSTintConfiguration"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other outlineView:tintConfigurationForItem:)]
        unsafe fn outlineView_tintConfigurationForItem(
            &self,
            outline_view: &NSOutlineView,
            item: &Object,
        ) -> Option<Id<NSTintConfiguration>>;

        #[cfg(all(
            feature = "AppKit_NSOutlineView",
            feature = "AppKit_NSTableColumn",
            feature = "Foundation_NSString"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other outlineView:typeSelectStringForTableColumn:item:)]
        unsafe fn outlineView_typeSelectStringForTableColumn_item(
            &self,
            outline_view: &NSOutlineView,
            table_column: Option<&NSTableColumn>,
            item: &Object,
        ) -> Option<Id<NSString>>;

        #[cfg(all(feature = "AppKit_NSOutlineView", feature = "Foundation_NSString"))]
        #[optional]
        #[method_id(@__retain_semantics Other outlineView:nextTypeSelectMatchFromItem:toItem:forString:)]
        unsafe fn outlineView_nextTypeSelectMatchFromItem_toItem_forString(
            &self,
            outline_view: &NSOutlineView,
            start_item: &Object,
            end_item: &Object,
            search_string: &NSString,
        ) -> Option<Id<Object>>;

        #[cfg(all(
            feature = "AppKit_NSEvent",
            feature = "AppKit_NSOutlineView",
            feature = "Foundation_NSString"
        ))]
        #[optional]
        #[method(outlineView:shouldTypeSelectForEvent:withCurrentSearchString:)]
        unsafe fn outlineView_shouldTypeSelectForEvent_withCurrentSearchString(
            &self,
            outline_view: &NSOutlineView,
            event: &NSEvent,
            search_string: Option<&NSString>,
        ) -> bool;

        #[cfg(all(feature = "AppKit_NSOutlineView", feature = "AppKit_NSTableColumn"))]
        #[optional]
        #[method(outlineView:shouldShowCellExpansionForTableColumn:item:)]
        unsafe fn outlineView_shouldShowCellExpansionForTableColumn_item(
            &self,
            outline_view: &NSOutlineView,
            table_column: Option<&NSTableColumn>,
            item: &Object,
        ) -> bool;

        #[cfg(all(
            feature = "AppKit_NSCell",
            feature = "AppKit_NSOutlineView",
            feature = "AppKit_NSTableColumn"
        ))]
        #[optional]
        #[method(outlineView:shouldTrackCell:forTableColumn:item:)]
        unsafe fn outlineView_shouldTrackCell_forTableColumn_item(
            &self,
            outline_view: &NSOutlineView,
            cell: &NSCell,
            table_column: Option<&NSTableColumn>,
            item: &Object,
        ) -> bool;

        #[cfg(all(
            feature = "AppKit_NSCell",
            feature = "AppKit_NSOutlineView",
            feature = "AppKit_NSTableColumn"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other outlineView:dataCellForTableColumn:item:)]
        unsafe fn outlineView_dataCellForTableColumn_item(
            &self,
            outline_view: &NSOutlineView,
            table_column: Option<&NSTableColumn>,
            item: &Object,
        ) -> Option<Id<NSCell>>;

        #[cfg(feature = "AppKit_NSOutlineView")]
        #[optional]
        #[method(outlineView:isGroupItem:)]
        unsafe fn outlineView_isGroupItem(
            &self,
            outline_view: &NSOutlineView,
            item: &Object,
        ) -> bool;

        #[cfg(feature = "AppKit_NSOutlineView")]
        #[optional]
        #[method(outlineView:shouldExpandItem:)]
        unsafe fn outlineView_shouldExpandItem(
            &self,
            outline_view: &NSOutlineView,
            item: &Object,
        ) -> bool;

        #[cfg(feature = "AppKit_NSOutlineView")]
        #[optional]
        #[method(outlineView:shouldCollapseItem:)]
        unsafe fn outlineView_shouldCollapseItem(
            &self,
            outline_view: &NSOutlineView,
            item: &Object,
        ) -> bool;

        #[cfg(all(feature = "AppKit_NSOutlineView", feature = "AppKit_NSTableColumn"))]
        #[optional]
        #[method(outlineView:willDisplayOutlineCell:forTableColumn:item:)]
        unsafe fn outlineView_willDisplayOutlineCell_forTableColumn_item(
            &self,
            outline_view: &NSOutlineView,
            cell: &Object,
            table_column: Option<&NSTableColumn>,
            item: &Object,
        );

        #[cfg(feature = "AppKit_NSOutlineView")]
        #[optional]
        #[method(outlineView:sizeToFitWidthOfColumn:)]
        unsafe fn outlineView_sizeToFitWidthOfColumn(
            &self,
            outline_view: &NSOutlineView,
            column: NSInteger,
        ) -> CGFloat;

        #[cfg(feature = "AppKit_NSOutlineView")]
        #[optional]
        #[method(outlineView:shouldReorderColumn:toColumn:)]
        unsafe fn outlineView_shouldReorderColumn_toColumn(
            &self,
            outline_view: &NSOutlineView,
            column_index: NSInteger,
            new_column_index: NSInteger,
        ) -> bool;

        #[cfg(feature = "AppKit_NSOutlineView")]
        #[optional]
        #[method(outlineView:shouldShowOutlineCellForItem:)]
        unsafe fn outlineView_shouldShowOutlineCellForItem(
            &self,
            outline_view: &NSOutlineView,
            item: &Object,
        ) -> bool;

        #[cfg(feature = "Foundation_NSNotification")]
        #[optional]
        #[method(outlineViewSelectionDidChange:)]
        unsafe fn outlineViewSelectionDidChange(&self, notification: &NSNotification);

        #[cfg(feature = "Foundation_NSNotification")]
        #[optional]
        #[method(outlineViewColumnDidMove:)]
        unsafe fn outlineViewColumnDidMove(&self, notification: &NSNotification);

        #[cfg(feature = "Foundation_NSNotification")]
        #[optional]
        #[method(outlineViewColumnDidResize:)]
        unsafe fn outlineViewColumnDidResize(&self, notification: &NSNotification);

        #[cfg(feature = "Foundation_NSNotification")]
        #[optional]
        #[method(outlineViewSelectionIsChanging:)]
        unsafe fn outlineViewSelectionIsChanging(&self, notification: &NSNotification);

        #[cfg(feature = "Foundation_NSNotification")]
        #[optional]
        #[method(outlineViewItemWillExpand:)]
        unsafe fn outlineViewItemWillExpand(&self, notification: &NSNotification);

        #[cfg(feature = "Foundation_NSNotification")]
        #[optional]
        #[method(outlineViewItemDidExpand:)]
        unsafe fn outlineViewItemDidExpand(&self, notification: &NSNotification);

        #[cfg(feature = "Foundation_NSNotification")]
        #[optional]
        #[method(outlineViewItemWillCollapse:)]
        unsafe fn outlineViewItemWillCollapse(&self, notification: &NSNotification);

        #[cfg(feature = "Foundation_NSNotification")]
        #[optional]
        #[method(outlineViewItemDidCollapse:)]
        unsafe fn outlineViewItemDidCollapse(&self, notification: &NSNotification);
    }

    unsafe impl ProtocolType for dyn NSOutlineViewDelegate {}
);

extern_static!(NSOutlineViewDisclosureButtonKey: &'static NSUserInterfaceItemIdentifier);

extern_static!(NSOutlineViewShowHideButtonKey: &'static NSUserInterfaceItemIdentifier);

extern_static!(NSOutlineViewSelectionDidChangeNotification: &'static NSNotificationName);

extern_static!(NSOutlineViewColumnDidMoveNotification: &'static NSNotificationName);

extern_static!(NSOutlineViewColumnDidResizeNotification: &'static NSNotificationName);

extern_static!(NSOutlineViewSelectionIsChangingNotification: &'static NSNotificationName);

extern_static!(NSOutlineViewItemWillExpandNotification: &'static NSNotificationName);

extern_static!(NSOutlineViewItemDidExpandNotification: &'static NSNotificationName);

extern_static!(NSOutlineViewItemWillCollapseNotification: &'static NSNotificationName);

extern_static!(NSOutlineViewItemDidCollapseNotification: &'static NSNotificationName);
