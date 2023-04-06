//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

#[deprecated]
extern_enum!(
    #[underlying(c_uint)]
    pub enum __anonymous__ {
        #[deprecated]
        DOM_START_TO_START = 0,
        #[deprecated]
        DOM_START_TO_END = 1,
        #[deprecated]
        DOM_END_TO_END = 2,
        #[deprecated]
        DOM_END_TO_START = 3,
        #[deprecated]
        DOM_NODE_BEFORE = 0,
        #[deprecated]
        DOM_NODE_AFTER = 1,
        #[deprecated]
        DOM_NODE_BEFORE_AND_AFTER = 2,
        #[deprecated]
        DOM_NODE_INSIDE = 3,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMRange")]
    #[deprecated]
    pub struct DOMRange;

    #[deprecated]
    #[cfg(feature = "WebKit_DOMRange")]
    unsafe impl ClassType for DOMRange {
        #[inherits(WebScriptObject, NSObject)]
        type Super = DOMObject;
    }
);

#[cfg(feature = "WebKit_DOMRange")]
unsafe impl NSObjectProtocol for DOMRange {}

extern_methods!(
    #[cfg(feature = "WebKit_DOMRange")]
    unsafe impl DOMRange {
        #[cfg(feature = "WebKit_DOMNode")]
        #[method_id(@__retain_semantics Other startContainer)]
        pub unsafe fn startContainer(&self) -> Option<Id<DOMNode>>;

        #[method(startOffset)]
        pub unsafe fn startOffset(&self) -> c_int;

        #[cfg(feature = "WebKit_DOMNode")]
        #[method_id(@__retain_semantics Other endContainer)]
        pub unsafe fn endContainer(&self) -> Option<Id<DOMNode>>;

        #[method(endOffset)]
        pub unsafe fn endOffset(&self) -> c_int;

        #[method(collapsed)]
        pub unsafe fn collapsed(&self) -> bool;

        #[cfg(feature = "WebKit_DOMNode")]
        #[method_id(@__retain_semantics Other commonAncestorContainer)]
        pub unsafe fn commonAncestorContainer(&self) -> Option<Id<DOMNode>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other text)]
        pub unsafe fn text(&self) -> Id<NSString>;

        #[cfg(feature = "WebKit_DOMNode")]
        #[method(setStart:offset:)]
        pub unsafe fn setStart_offset(&self, ref_node: Option<&DOMNode>, offset: c_int);

        #[cfg(feature = "WebKit_DOMNode")]
        #[method(setEnd:offset:)]
        pub unsafe fn setEnd_offset(&self, ref_node: Option<&DOMNode>, offset: c_int);

        #[cfg(feature = "WebKit_DOMNode")]
        #[method(setStartBefore:)]
        pub unsafe fn setStartBefore(&self, ref_node: Option<&DOMNode>);

        #[cfg(feature = "WebKit_DOMNode")]
        #[method(setStartAfter:)]
        pub unsafe fn setStartAfter(&self, ref_node: Option<&DOMNode>);

        #[cfg(feature = "WebKit_DOMNode")]
        #[method(setEndBefore:)]
        pub unsafe fn setEndBefore(&self, ref_node: Option<&DOMNode>);

        #[cfg(feature = "WebKit_DOMNode")]
        #[method(setEndAfter:)]
        pub unsafe fn setEndAfter(&self, ref_node: Option<&DOMNode>);

        #[method(collapse:)]
        pub unsafe fn collapse(&self, to_start: bool);

        #[cfg(feature = "WebKit_DOMNode")]
        #[method(selectNode:)]
        pub unsafe fn selectNode(&self, ref_node: Option<&DOMNode>);

        #[cfg(feature = "WebKit_DOMNode")]
        #[method(selectNodeContents:)]
        pub unsafe fn selectNodeContents(&self, ref_node: Option<&DOMNode>);

        #[method(compareBoundaryPoints:sourceRange:)]
        pub unsafe fn compareBoundaryPoints_sourceRange(
            &self,
            how: c_ushort,
            source_range: Option<&DOMRange>,
        ) -> c_short;

        #[method(deleteContents)]
        pub unsafe fn deleteContents(&self);

        #[cfg(feature = "WebKit_DOMDocumentFragment")]
        #[method_id(@__retain_semantics Other extractContents)]
        pub unsafe fn extractContents(&self) -> Option<Id<DOMDocumentFragment>>;

        #[cfg(feature = "WebKit_DOMDocumentFragment")]
        #[method_id(@__retain_semantics Other cloneContents)]
        pub unsafe fn cloneContents(&self) -> Option<Id<DOMDocumentFragment>>;

        #[cfg(feature = "WebKit_DOMNode")]
        #[method(insertNode:)]
        pub unsafe fn insertNode(&self, new_node: Option<&DOMNode>);

        #[cfg(feature = "WebKit_DOMNode")]
        #[method(surroundContents:)]
        pub unsafe fn surroundContents(&self, new_parent: Option<&DOMNode>);

        #[method_id(@__retain_semantics Other cloneRange)]
        pub unsafe fn cloneRange(&self) -> Option<Id<DOMRange>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other toString)]
        pub unsafe fn toString(&self) -> Option<Id<NSString>>;

        #[method(detach)]
        pub unsafe fn detach(&self);

        #[cfg(all(
            feature = "Foundation_NSString",
            feature = "WebKit_DOMDocumentFragment"
        ))]
        #[method_id(@__retain_semantics Other createContextualFragment:)]
        pub unsafe fn createContextualFragment(
            &self,
            html: Option<&NSString>,
        ) -> Option<Id<DOMDocumentFragment>>;

        #[cfg(feature = "WebKit_DOMNode")]
        #[method(compareNode:)]
        pub unsafe fn compareNode(&self, ref_node: Option<&DOMNode>) -> c_short;

        #[cfg(feature = "WebKit_DOMNode")]
        #[method(intersectsNode:)]
        pub unsafe fn intersectsNode(&self, ref_node: Option<&DOMNode>) -> bool;

        #[cfg(feature = "WebKit_DOMNode")]
        #[method(comparePoint:offset:)]
        pub unsafe fn comparePoint_offset(
            &self,
            ref_node: Option<&DOMNode>,
            offset: c_int,
        ) -> c_short;

        #[cfg(feature = "WebKit_DOMNode")]
        #[method(isPointInRange:offset:)]
        pub unsafe fn isPointInRange_offset(
            &self,
            ref_node: Option<&DOMNode>,
            offset: c_int,
        ) -> bool;
    }
);

extern_methods!(
    /// DOMRangeDeprecated
    #[cfg(feature = "WebKit_DOMRange")]
    unsafe impl DOMRange {
        #[cfg(feature = "WebKit_DOMNode")]
        #[deprecated]
        #[method(setStart::)]
        pub unsafe fn setStart(&self, ref_node: Option<&DOMNode>, offset: c_int);

        #[cfg(feature = "WebKit_DOMNode")]
        #[deprecated]
        #[method(setEnd::)]
        pub unsafe fn setEnd(&self, ref_node: Option<&DOMNode>, offset: c_int);

        #[deprecated]
        #[method(compareBoundaryPoints::)]
        pub unsafe fn compareBoundaryPoints(
            &self,
            how: c_ushort,
            source_range: Option<&DOMRange>,
        ) -> c_short;
    }
);
