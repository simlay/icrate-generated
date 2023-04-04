//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

typed_enum!(
    pub type NSTextContentType = NSString;
);

extern_static!(NSTextContentTypeUsername: &'static NSTextContentType);

extern_static!(NSTextContentTypePassword: &'static NSTextContentType);

extern_static!(NSTextContentTypeOneTimeCode: &'static NSTextContentType);

extern_protocol!(
    pub unsafe trait NSTextContent {
        #[method_id(@__retain_semantics Other contentType)]
        unsafe fn contentType(&self) -> Option<Id<NSTextContentType>>;

        #[method(setContentType:)]
        unsafe fn setContentType(&self, content_type: Option<&NSTextContentType>);
    }

    unsafe impl ProtocolType for dyn NSTextContent {}
);
