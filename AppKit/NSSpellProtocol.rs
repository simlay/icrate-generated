//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_protocol!(
    #[cfg(not(any(target_os = "ios")))]
    pub unsafe trait NSChangeSpelling {
        #[cfg(not(any(target_os = "ios")))]
        #[method(changeSpelling:)]
        unsafe fn changeSpelling(&self, sender: Option<&Object>);
    }

    unsafe impl ProtocolType for dyn NSChangeSpelling {}
);

extern_protocol!(
    #[cfg(not(any(target_os = "ios")))]
    pub unsafe trait NSIgnoreMisspelledWords {
        #[cfg(not(any(target_os = "ios")))]
        #[method(ignoreSpelling:)]
        unsafe fn ignoreSpelling(&self, sender: Option<&Object>);
    }

    unsafe impl ProtocolType for dyn NSIgnoreMisspelledWords {}
);
