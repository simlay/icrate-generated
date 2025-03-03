//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_DOMXPathExpression")]
    #[deprecated]
    pub struct DOMXPathExpression;

    #[cfg(feature = "WebKit_DOMXPathExpression")]
    unsafe impl ClassType for DOMXPathExpression {
        #[inherits(WebScriptObject, NSObject)]
        type Super = DOMObject;
    }
);

#[cfg(feature = "WebKit_DOMXPathExpression")]
unsafe impl NSObjectProtocol for DOMXPathExpression {}

extern_methods!(
    #[cfg(feature = "WebKit_DOMXPathExpression")]
    unsafe impl DOMXPathExpression {
        #[cfg(all(feature = "WebKit_DOMNode", feature = "WebKit_DOMXPathResult"))]
        #[method_id(@__retain_semantics Other evaluate:type:inResult:)]
        pub unsafe fn evaluate_type_inResult(
            &self,
            context_node: Option<&DOMNode>,
            r#type: c_ushort,
            in_result: Option<&DOMXPathResult>,
        ) -> Option<Id<DOMXPathResult>>;
    }
);

extern_methods!(
    /// DOMXPathExpressionDeprecated
    #[cfg(feature = "WebKit_DOMXPathExpression")]
    unsafe impl DOMXPathExpression {
        #[cfg(all(feature = "WebKit_DOMNode", feature = "WebKit_DOMXPathResult"))]
        #[deprecated]
        #[method_id(@__retain_semantics Other evaluate:::)]
        pub unsafe fn evaluate(
            &self,
            context_node: Option<&DOMNode>,
            r#type: c_ushort,
            in_result: Option<&DOMXPathResult>,
        ) -> Option<Id<DOMXPathResult>>;
    }
);
