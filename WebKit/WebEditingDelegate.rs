//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

ns_enum!(
    #[underlying(NSInteger)]
    #[deprecated]
    pub enum WebViewInsertAction {
        #[deprecated]
        WebViewInsertActionTyped = 0,
        #[deprecated]
        WebViewInsertActionPasted = 1,
        #[deprecated]
        WebViewInsertActionDropped = 2,
    }
);

extern_protocol!(
    #[deprecated]
    pub unsafe trait WebEditingDelegate: NSObjectProtocol {
        #[cfg(all(feature = "WebKit_DOMRange", feature = "WebKit_WebView"))]
        #[optional]
        #[method(webView:shouldBeginEditingInDOMRange:)]
        unsafe fn webView_shouldBeginEditingInDOMRange(
            &self,
            web_view: Option<&WebView>,
            range: Option<&DOMRange>,
        ) -> bool;

        #[cfg(all(feature = "WebKit_DOMRange", feature = "WebKit_WebView"))]
        #[optional]
        #[method(webView:shouldEndEditingInDOMRange:)]
        unsafe fn webView_shouldEndEditingInDOMRange(
            &self,
            web_view: Option<&WebView>,
            range: Option<&DOMRange>,
        ) -> bool;

        #[cfg(all(
            feature = "WebKit_DOMNode",
            feature = "WebKit_DOMRange",
            feature = "WebKit_WebView"
        ))]
        #[optional]
        #[method(webView:shouldInsertNode:replacingDOMRange:givenAction:)]
        unsafe fn webView_shouldInsertNode_replacingDOMRange_givenAction(
            &self,
            web_view: Option<&WebView>,
            node: Option<&DOMNode>,
            range: Option<&DOMRange>,
            action: WebViewInsertAction,
        ) -> bool;

        #[cfg(all(
            feature = "Foundation_NSString",
            feature = "WebKit_DOMRange",
            feature = "WebKit_WebView"
        ))]
        #[optional]
        #[method(webView:shouldInsertText:replacingDOMRange:givenAction:)]
        unsafe fn webView_shouldInsertText_replacingDOMRange_givenAction(
            &self,
            web_view: Option<&WebView>,
            text: Option<&NSString>,
            range: Option<&DOMRange>,
            action: WebViewInsertAction,
        ) -> bool;

        #[cfg(all(feature = "WebKit_DOMRange", feature = "WebKit_WebView"))]
        #[optional]
        #[method(webView:shouldDeleteDOMRange:)]
        unsafe fn webView_shouldDeleteDOMRange(
            &self,
            web_view: Option<&WebView>,
            range: Option<&DOMRange>,
        ) -> bool;

        #[cfg(all(feature = "WebKit_DOMRange", feature = "WebKit_WebView"))]
        #[optional]
        #[method(webView:shouldChangeSelectedDOMRange:toDOMRange:affinity:stillSelecting:)]
        unsafe fn webView_shouldChangeSelectedDOMRange_toDOMRange_affinity_stillSelecting(
            &self,
            web_view: Option<&WebView>,
            current_range: Option<&DOMRange>,
            proposed_range: Option<&DOMRange>,
            selection_affinity: NSSelectionAffinity,
            flag: bool,
        ) -> bool;

        #[cfg(all(
            feature = "WebKit_DOMCSSStyleDeclaration",
            feature = "WebKit_DOMRange",
            feature = "WebKit_WebView"
        ))]
        #[optional]
        #[method(webView:shouldApplyStyle:toElementsInDOMRange:)]
        unsafe fn webView_shouldApplyStyle_toElementsInDOMRange(
            &self,
            web_view: Option<&WebView>,
            style: Option<&DOMCSSStyleDeclaration>,
            range: Option<&DOMRange>,
        ) -> bool;

        #[cfg(all(feature = "WebKit_DOMCSSStyleDeclaration", feature = "WebKit_WebView"))]
        #[optional]
        #[method(webView:shouldChangeTypingStyle:toStyle:)]
        unsafe fn webView_shouldChangeTypingStyle_toStyle(
            &self,
            web_view: Option<&WebView>,
            current_style: Option<&DOMCSSStyleDeclaration>,
            proposed_style: Option<&DOMCSSStyleDeclaration>,
        ) -> bool;

        #[cfg(feature = "WebKit_WebView")]
        #[optional]
        #[method(webView:doCommandBySelector:)]
        unsafe fn webView_doCommandBySelector(
            &self,
            web_view: Option<&WebView>,
            selector: Option<Sel>,
        ) -> bool;

        #[cfg(feature = "Foundation_NSNotification")]
        #[optional]
        #[method(webViewDidBeginEditing:)]
        unsafe fn webViewDidBeginEditing(&self, notification: Option<&NSNotification>);

        #[cfg(feature = "Foundation_NSNotification")]
        #[optional]
        #[method(webViewDidChange:)]
        unsafe fn webViewDidChange(&self, notification: Option<&NSNotification>);

        #[cfg(feature = "Foundation_NSNotification")]
        #[optional]
        #[method(webViewDidEndEditing:)]
        unsafe fn webViewDidEndEditing(&self, notification: Option<&NSNotification>);

        #[cfg(feature = "Foundation_NSNotification")]
        #[optional]
        #[method(webViewDidChangeTypingStyle:)]
        unsafe fn webViewDidChangeTypingStyle(&self, notification: Option<&NSNotification>);

        #[cfg(feature = "Foundation_NSNotification")]
        #[optional]
        #[method(webViewDidChangeSelection:)]
        unsafe fn webViewDidChangeSelection(&self, notification: Option<&NSNotification>);

        #[cfg(all(feature = "Foundation_NSUndoManager", feature = "WebKit_WebView"))]
        #[optional]
        #[method_id(@__retain_semantics Other undoManagerForWebView:)]
        unsafe fn undoManagerForWebView(
            &self,
            web_view: Option<&WebView>,
        ) -> Option<Id<NSUndoManager>>;
    }

    unsafe impl ProtocolType for dyn WebEditingDelegate {}
);
