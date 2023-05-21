//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_static!(WebHistoryItemsAddedNotification: Option<&'static NSString>);

extern_static!(WebHistoryItemsRemovedNotification: Option<&'static NSString>);

extern_static!(WebHistoryAllItemsRemovedNotification: Option<&'static NSString>);

extern_static!(WebHistoryLoadedNotification: Option<&'static NSString>);

extern_static!(WebHistorySavedNotification: Option<&'static NSString>);

extern_static!(WebHistoryItemsKey: Option<&'static NSString>);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_WebHistory")]
    #[deprecated]
    pub struct WebHistory;

    #[deprecated]
    #[cfg(feature = "WebKit_WebHistory")]
    unsafe impl ClassType for WebHistory {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "WebKit_WebHistory")]
unsafe impl NSObjectProtocol for WebHistory {}

extern_methods!(
    #[cfg(feature = "WebKit_WebHistory")]
    unsafe impl WebHistory {
        #[method_id(@__retain_semantics Other optionalSharedHistory)]
        pub unsafe fn optionalSharedHistory() -> Option<Id<WebHistory>>;

        #[method(setOptionalSharedHistory:)]
        pub unsafe fn setOptionalSharedHistory(history: Option<&WebHistory>);

        #[cfg(feature = "Foundation_NSArray")]
        #[method(addItems:)]
        pub unsafe fn addItems(&self, new_items: Option<&NSArray>);

        #[cfg(feature = "Foundation_NSArray")]
        #[method(removeItems:)]
        pub unsafe fn removeItems(&self, items: Option<&NSArray>);

        #[method(removeAllItems)]
        pub unsafe fn removeAllItems(&self);

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other orderedLastVisitedDays)]
        pub unsafe fn orderedLastVisitedDays(&self) -> Id<NSArray>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSCalendarDate"))]
        #[method_id(@__retain_semantics Other orderedItemsLastVisitedOnDay:)]
        pub unsafe fn orderedItemsLastVisitedOnDay(
            &self,
            calendar_date: Option<&NSCalendarDate>,
        ) -> Option<Id<NSArray>>;

        #[cfg(all(feature = "Foundation_NSURL", feature = "WebKit_WebHistoryItem"))]
        #[method_id(@__retain_semantics Other itemForURL:)]
        pub unsafe fn itemForURL(&self, url: Option<&NSURL>) -> Option<Id<WebHistoryItem>>;

        #[method(historyItemLimit)]
        pub unsafe fn historyItemLimit(&self) -> c_int;

        #[method(setHistoryItemLimit:)]
        pub unsafe fn setHistoryItemLimit(&self, history_item_limit: c_int);

        #[method(historyAgeInDaysLimit)]
        pub unsafe fn historyAgeInDaysLimit(&self) -> c_int;

        #[method(setHistoryAgeInDaysLimit:)]
        pub unsafe fn setHistoryAgeInDaysLimit(&self, history_age_in_days_limit: c_int);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "WebKit_WebHistory")]
    unsafe impl WebHistory {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
