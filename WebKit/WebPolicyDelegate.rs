//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum WebNavigationType {
        #[deprecated]
        WebNavigationTypeLinkClicked = 0,
        #[deprecated]
        WebNavigationTypeFormSubmitted = 1,
        #[deprecated]
        WebNavigationTypeBackForward = 2,
        #[deprecated]
        WebNavigationTypeReload = 3,
        #[deprecated]
        WebNavigationTypeFormResubmitted = 4,
        #[deprecated]
        WebNavigationTypeOther = 5,
    }
);

extern_static!(WebActionNavigationTypeKey: Option<&'static NSString>);

extern_static!(WebActionElementKey: Option<&'static NSString>);

extern_static!(WebActionButtonKey: Option<&'static NSString>);

extern_static!(WebActionModifierFlagsKey: Option<&'static NSString>);

extern_static!(WebActionOriginalURLKey: Option<&'static NSString>);

extern_protocol!(
    pub unsafe trait WebPolicyDecisionListener: NSObjectProtocol {
        #[method(use)]
        unsafe fn r#use(&self);

        #[method(download)]
        unsafe fn download(&self);

        #[method(ignore)]
        unsafe fn ignore(&self);
    }

    unsafe impl ProtocolType for dyn WebPolicyDecisionListener {}
);

extern_protocol!(
    pub unsafe trait WebPolicyDelegate: NSObjectProtocol {
        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSURLRequest",
            feature = "WebKit_WebFrame",
            feature = "WebKit_WebView"
        ))]
        #[optional]
        #[method(webView:decidePolicyForNavigationAction:request:frame:decisionListener:)]
        unsafe fn webView_decidePolicyForNavigationAction_request_frame_decisionListener(
            &self,
            web_view: Option<&WebView>,
            action_information: Option<&NSDictionary>,
            request: Option<&NSURLRequest>,
            frame: Option<&WebFrame>,
            listener: Option<&ProtocolObject<dyn WebPolicyDecisionListener>>,
        );

        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURLRequest",
            feature = "WebKit_WebView"
        ))]
        #[optional]
        #[method(webView:decidePolicyForNewWindowAction:request:newFrameName:decisionListener:)]
        unsafe fn webView_decidePolicyForNewWindowAction_request_newFrameName_decisionListener(
            &self,
            web_view: Option<&WebView>,
            action_information: Option<&NSDictionary>,
            request: Option<&NSURLRequest>,
            frame_name: Option<&NSString>,
            listener: Option<&ProtocolObject<dyn WebPolicyDecisionListener>>,
        );

        #[cfg(all(
            feature = "Foundation_NSString",
            feature = "Foundation_NSURLRequest",
            feature = "WebKit_WebFrame",
            feature = "WebKit_WebView"
        ))]
        #[optional]
        #[method(webView:decidePolicyForMIMEType:request:frame:decisionListener:)]
        unsafe fn webView_decidePolicyForMIMEType_request_frame_decisionListener(
            &self,
            web_view: Option<&WebView>,
            r#type: Option<&NSString>,
            request: Option<&NSURLRequest>,
            frame: Option<&WebFrame>,
            listener: Option<&ProtocolObject<dyn WebPolicyDecisionListener>>,
        );

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "WebKit_WebFrame",
            feature = "WebKit_WebView"
        ))]
        #[optional]
        #[method(webView:unableToImplementPolicyWithError:frame:)]
        unsafe fn webView_unableToImplementPolicyWithError_frame(
            &self,
            web_view: Option<&WebView>,
            error: Option<&NSError>,
            frame: Option<&WebFrame>,
        );
    }

    unsafe impl ProtocolType for dyn WebPolicyDelegate {}
);
