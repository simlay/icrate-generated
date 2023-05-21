//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_WebBackForwardList")]
    #[deprecated]
    pub struct WebBackForwardList;

    #[deprecated]
    #[cfg(feature = "WebKit_WebBackForwardList")]
    unsafe impl ClassType for WebBackForwardList {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "WebKit_WebBackForwardList")]
unsafe impl NSObjectProtocol for WebBackForwardList {}

extern_methods!(
    #[cfg(feature = "WebKit_WebBackForwardList")]
    unsafe impl WebBackForwardList {
        #[cfg(feature = "WebKit_WebHistoryItem")]
        #[method(addItem:)]
        pub unsafe fn addItem(&self, item: Option<&WebHistoryItem>);

        #[method(goBack)]
        pub unsafe fn goBack(&self);

        #[method(goForward)]
        pub unsafe fn goForward(&self);

        #[cfg(feature = "WebKit_WebHistoryItem")]
        #[method(goToItem:)]
        pub unsafe fn goToItem(&self, item: Option<&WebHistoryItem>);

        #[cfg(feature = "WebKit_WebHistoryItem")]
        #[method_id(@__retain_semantics Other backItem)]
        pub unsafe fn backItem(&self) -> Option<Id<WebHistoryItem>>;

        #[cfg(feature = "WebKit_WebHistoryItem")]
        #[method_id(@__retain_semantics Other currentItem)]
        pub unsafe fn currentItem(&self) -> Option<Id<WebHistoryItem>>;

        #[cfg(feature = "WebKit_WebHistoryItem")]
        #[method_id(@__retain_semantics Other forwardItem)]
        pub unsafe fn forwardItem(&self) -> Option<Id<WebHistoryItem>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other backListWithLimit:)]
        pub unsafe fn backListWithLimit(&self, limit: c_int) -> Option<Id<NSArray>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other forwardListWithLimit:)]
        pub unsafe fn forwardListWithLimit(&self, limit: c_int) -> Option<Id<NSArray>>;

        #[method(capacity)]
        pub unsafe fn capacity(&self) -> c_int;

        #[method(setCapacity:)]
        pub unsafe fn setCapacity(&self, capacity: c_int);

        #[method(backListCount)]
        pub unsafe fn backListCount(&self) -> c_int;

        #[method(forwardListCount)]
        pub unsafe fn forwardListCount(&self) -> c_int;

        #[cfg(feature = "WebKit_WebHistoryItem")]
        #[method(containsItem:)]
        pub unsafe fn containsItem(&self, item: Option<&WebHistoryItem>) -> bool;

        #[cfg(feature = "WebKit_WebHistoryItem")]
        #[method_id(@__retain_semantics Other itemAtIndex:)]
        pub unsafe fn itemAtIndex(&self, index: c_int) -> Option<Id<WebHistoryItem>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "WebKit_WebBackForwardList")]
    unsafe impl WebBackForwardList {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    /// WebBackForwardListDeprecated
    #[cfg(feature = "WebKit_WebBackForwardList")]
    unsafe impl WebBackForwardList {
        #[method(setPageCacheSize:)]
        pub unsafe fn setPageCacheSize(&self, size: NSUInteger);

        #[method(pageCacheSize)]
        pub unsafe fn pageCacheSize(&self) -> NSUInteger;
    }
);
