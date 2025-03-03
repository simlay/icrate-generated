//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_enum!(
    #[underlying(c_uint)]
    #[deprecated]
    pub enum __anonymous__ {
        #[deprecated]
        DOM_ELEMENT_NODE = 1,
        #[deprecated]
        DOM_ATTRIBUTE_NODE = 2,
        #[deprecated]
        DOM_TEXT_NODE = 3,
        #[deprecated]
        DOM_CDATA_SECTION_NODE = 4,
        #[deprecated]
        DOM_ENTITY_REFERENCE_NODE = 5,
        #[deprecated]
        DOM_ENTITY_NODE = 6,
        #[deprecated]
        DOM_PROCESSING_INSTRUCTION_NODE = 7,
        #[deprecated]
        DOM_COMMENT_NODE = 8,
        #[deprecated]
        DOM_DOCUMENT_NODE = 9,
        #[deprecated]
        DOM_DOCUMENT_TYPE_NODE = 10,
        #[deprecated]
        DOM_DOCUMENT_FRAGMENT_NODE = 11,
        #[deprecated]
        DOM_NOTATION_NODE = 12,
        #[deprecated]
        DOM_DOCUMENT_POSITION_DISCONNECTED = 0x01,
        #[deprecated]
        DOM_DOCUMENT_POSITION_PRECEDING = 0x02,
        #[deprecated]
        DOM_DOCUMENT_POSITION_FOLLOWING = 0x04,
        #[deprecated]
        DOM_DOCUMENT_POSITION_CONTAINS = 0x08,
        #[deprecated]
        DOM_DOCUMENT_POSITION_CONTAINED_BY = 0x10,
        #[deprecated]
        DOM_DOCUMENT_POSITION_IMPLEMENTATION_SPECIFIC = 0x20,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMNode")]
    #[deprecated]
    pub struct DOMNode;

    #[cfg(feature = "WebKit_DOMNode")]
    unsafe impl ClassType for DOMNode {
        #[inherits(WebScriptObject, NSObject)]
        type Super = DOMObject;
    }
);

#[cfg(feature = "WebKit_DOMNode")]
unsafe impl DOMEventTarget for DOMNode {}

#[cfg(feature = "WebKit_DOMNode")]
unsafe impl NSObjectProtocol for DOMNode {}

extern_methods!(
    #[cfg(feature = "WebKit_DOMNode")]
    unsafe impl DOMNode {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other nodeName)]
        pub unsafe fn nodeName(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other nodeValue)]
        pub unsafe fn nodeValue(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setNodeValue:)]
        pub unsafe fn setNodeValue(&self, node_value: Option<&NSString>);

        #[method(nodeType)]
        pub unsafe fn nodeType(&self) -> c_ushort;

        #[method_id(@__retain_semantics Other parentNode)]
        pub unsafe fn parentNode(&self) -> Option<Id<DOMNode>>;

        #[cfg(feature = "WebKit_DOMNodeList")]
        #[method_id(@__retain_semantics Other childNodes)]
        pub unsafe fn childNodes(&self) -> Option<Id<DOMNodeList>>;

        #[method_id(@__retain_semantics Other firstChild)]
        pub unsafe fn firstChild(&self) -> Option<Id<DOMNode>>;

        #[method_id(@__retain_semantics Other lastChild)]
        pub unsafe fn lastChild(&self) -> Option<Id<DOMNode>>;

        #[method_id(@__retain_semantics Other previousSibling)]
        pub unsafe fn previousSibling(&self) -> Option<Id<DOMNode>>;

        #[method_id(@__retain_semantics Other nextSibling)]
        pub unsafe fn nextSibling(&self) -> Option<Id<DOMNode>>;

        #[cfg(feature = "WebKit_DOMDocument")]
        #[method_id(@__retain_semantics Other ownerDocument)]
        pub unsafe fn ownerDocument(&self) -> Option<Id<DOMDocument>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other namespaceURI)]
        pub unsafe fn namespaceURI(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other prefix)]
        pub unsafe fn prefix(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setPrefix:)]
        pub unsafe fn setPrefix(&self, prefix: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other localName)]
        pub unsafe fn localName(&self) -> Id<NSString>;

        #[cfg(feature = "WebKit_DOMNamedNodeMap")]
        #[method_id(@__retain_semantics Other attributes)]
        pub unsafe fn attributes(&self) -> Option<Id<DOMNamedNodeMap>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other baseURI)]
        pub unsafe fn baseURI(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other textContent)]
        pub unsafe fn textContent(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setTextContent:)]
        pub unsafe fn setTextContent(&self, text_content: Option<&NSString>);

        #[cfg(feature = "WebKit_DOMElement")]
        #[method_id(@__retain_semantics Other parentElement)]
        pub unsafe fn parentElement(&self) -> Option<Id<DOMElement>>;

        #[method(isContentEditable)]
        pub unsafe fn isContentEditable(&self) -> bool;

        #[method_id(@__retain_semantics Other insertBefore:refChild:)]
        pub unsafe fn insertBefore_refChild(
            &self,
            new_child: Option<&DOMNode>,
            ref_child: Option<&DOMNode>,
        ) -> Option<Id<DOMNode>>;

        #[method_id(@__retain_semantics Other replaceChild:oldChild:)]
        pub unsafe fn replaceChild_oldChild(
            &self,
            new_child: Option<&DOMNode>,
            old_child: Option<&DOMNode>,
        ) -> Option<Id<DOMNode>>;

        #[method_id(@__retain_semantics Other removeChild:)]
        pub unsafe fn removeChild(&self, old_child: Option<&DOMNode>) -> Option<Id<DOMNode>>;

        #[method_id(@__retain_semantics Other appendChild:)]
        pub unsafe fn appendChild(&self, new_child: Option<&DOMNode>) -> Option<Id<DOMNode>>;

        #[method(hasChildNodes)]
        pub unsafe fn hasChildNodes(&self) -> bool;

        #[method_id(@__retain_semantics Other cloneNode:)]
        pub unsafe fn cloneNode(&self, deep: bool) -> Option<Id<DOMNode>>;

        #[method(normalize)]
        pub unsafe fn normalize(&self);

        #[cfg(feature = "Foundation_NSString")]
        #[method(isSupported:version:)]
        pub unsafe fn isSupported_version(
            &self,
            feature: Option<&NSString>,
            version: Option<&NSString>,
        ) -> bool;

        #[method(hasAttributes)]
        pub unsafe fn hasAttributes(&self) -> bool;

        #[method(isSameNode:)]
        pub unsafe fn isSameNode(&self, other: Option<&DOMNode>) -> bool;

        #[method(isEqualNode:)]
        pub unsafe fn isEqualNode(&self, other: Option<&DOMNode>) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other lookupPrefix:)]
        pub unsafe fn lookupPrefix(&self, namespace_uri: Option<&NSString>)
            -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other lookupNamespaceURI:)]
        pub unsafe fn lookupNamespaceURI(&self, prefix: Option<&NSString>) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(isDefaultNamespace:)]
        pub unsafe fn isDefaultNamespace(&self, namespace_uri: Option<&NSString>) -> bool;

        #[method(compareDocumentPosition:)]
        pub unsafe fn compareDocumentPosition(&self, other: Option<&DOMNode>) -> c_ushort;

        #[method(contains:)]
        pub unsafe fn contains(&self, other: Option<&DOMNode>) -> bool;
    }
);

extern_methods!(
    /// DOMNodeDeprecated
    #[cfg(feature = "WebKit_DOMNode")]
    unsafe impl DOMNode {
        #[deprecated]
        #[method_id(@__retain_semantics Other insertBefore::)]
        pub unsafe fn insertBefore(
            &self,
            new_child: Option<&DOMNode>,
            ref_child: Option<&DOMNode>,
        ) -> Option<Id<DOMNode>>;

        #[deprecated]
        #[method_id(@__retain_semantics Other replaceChild::)]
        pub unsafe fn replaceChild(
            &self,
            new_child: Option<&DOMNode>,
            old_child: Option<&DOMNode>,
        ) -> Option<Id<DOMNode>>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(isSupported::)]
        pub unsafe fn isSupported(
            &self,
            feature: Option<&NSString>,
            version: Option<&NSString>,
        ) -> bool;
    }
);
