//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_static!(WKWebsiteDataTypeFetchCache: &'static NSString);

extern_static!(WKWebsiteDataTypeDiskCache: &'static NSString);

extern_static!(WKWebsiteDataTypeMemoryCache: &'static NSString);

extern_static!(WKWebsiteDataTypeOfflineWebApplicationCache: &'static NSString);

extern_static!(WKWebsiteDataTypeCookies: &'static NSString);

extern_static!(WKWebsiteDataTypeSessionStorage: &'static NSString);

extern_static!(WKWebsiteDataTypeLocalStorage: &'static NSString);

extern_static!(WKWebsiteDataTypeWebSQLDatabases: &'static NSString);

extern_static!(WKWebsiteDataTypeIndexedDBDatabases: &'static NSString);

extern_static!(WKWebsiteDataTypeServiceWorkerRegistrations: &'static NSString);

extern_static!(WKWebsiteDataTypeFileSystem: &'static NSString);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_WKWebsiteDataRecord")]
    pub struct WKWebsiteDataRecord;

    #[cfg(feature = "WebKit_WKWebsiteDataRecord")]
    unsafe impl ClassType for WKWebsiteDataRecord {
        type Super = NSObject;
    }
);

#[cfg(feature = "WebKit_WKWebsiteDataRecord")]
unsafe impl NSObjectProtocol for WKWebsiteDataRecord {}

extern_methods!(
    #[cfg(feature = "WebKit_WKWebsiteDataRecord")]
    unsafe impl WKWebsiteDataRecord {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other displayName)]
        pub unsafe fn displayName(&self) -> Id<NSString>;

        #[cfg(all(feature = "Foundation_NSSet", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other dataTypes)]
        pub unsafe fn dataTypes(&self) -> Id<NSSet<NSString>>;
    }
);
