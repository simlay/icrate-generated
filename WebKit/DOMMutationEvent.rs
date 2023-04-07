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
        DOM_MODIFICATION = 1,
        #[deprecated]
        DOM_ADDITION = 2,
        #[deprecated]
        DOM_REMOVAL = 3,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMMutationEvent")]
    #[deprecated]
    pub struct DOMMutationEvent;

    #[deprecated]
    #[cfg(feature = "WebKit_DOMMutationEvent")]
    unsafe impl ClassType for DOMMutationEvent {
        #[inherits(DOMObject, WebScriptObject, NSObject)]
        type Super = DOMEvent;
    }
);

#[cfg(feature = "WebKit_DOMMutationEvent")]
unsafe impl NSObjectProtocol for DOMMutationEvent {}

extern_methods!(
    #[cfg(feature = "WebKit_DOMMutationEvent")]
    unsafe impl DOMMutationEvent {
        #[cfg(feature = "WebKit_DOMNode")]
        #[method_id(@__retain_semantics Other relatedNode)]
        pub unsafe fn relatedNode(&self) -> Option<Id<DOMNode>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other prevValue)]
        pub unsafe fn prevValue(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other attrName)]
        pub unsafe fn attrName(&self) -> Id<NSString>;

        #[method(attrChange)]
        pub unsafe fn attrChange(&self) -> c_ushort;

        #[cfg(all(feature = "Foundation_NSString", feature = "WebKit_DOMNode"))]
        #[method(initMutationEvent:canBubble:cancelable:relatedNode:prevValue:newValue:attrName:attrChange:)]
        pub unsafe fn initMutationEvent_canBubble_cancelable_relatedNode_prevValue_newValue_attrName_attrChange(
            &self,
            r#type: Option<&NSString>,
            can_bubble: bool,
            cancelable: bool,
            related_node: Option<&DOMNode>,
            prev_value: Option<&NSString>,
            new_value: Option<&NSString>,
            attr_name: Option<&NSString>,
            attr_change: c_ushort,
        );
    }
);

extern_methods!(
    /// DOMMutationEventDeprecated
    #[cfg(feature = "WebKit_DOMMutationEvent")]
    unsafe impl DOMMutationEvent {
        #[cfg(all(feature = "Foundation_NSString", feature = "WebKit_DOMNode"))]
        #[deprecated]
        #[method(initMutationEvent::::::::)]
        pub unsafe fn initMutationEvent(
            &self,
            r#type: Option<&NSString>,
            can_bubble: bool,
            cancelable: bool,
            related_node: Option<&DOMNode>,
            prev_value: Option<&NSString>,
            new_value: Option<&NSString>,
            attr_name: Option<&NSString>,
            attr_change: c_ushort,
        );
    }
);
