//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_static!(DOMXPathException: Option<&'static NSString>);

extern_enum!(
    #[underlying(c_uint)]
    pub enum DOMXPathExceptionCode {
        #[deprecated]
        DOM_INVALID_EXPRESSION_ERR = 51,
        #[deprecated]
        DOM_TYPE_ERR = 52,
    }
);
