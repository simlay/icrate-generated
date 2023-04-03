//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::ClassKit::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "ClassKit_CLSActivityItem")]
    #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
    pub struct CLSActivityItem;

    #[cfg(feature = "ClassKit_CLSActivityItem")]
    unsafe impl ClassType for CLSActivityItem {
        #[inherits(NSObject)]
        type Super = CLSObject;
    }
);

#[cfg(feature = "ClassKit_CLSActivityItem")]
unsafe impl NSCoding for CLSActivityItem {}

#[cfg(feature = "ClassKit_CLSActivityItem")]
unsafe impl NSObjectProtocol for CLSActivityItem {}

#[cfg(feature = "ClassKit_CLSActivityItem")]
unsafe impl NSSecureCoding for CLSActivityItem {}

extern_methods!(
    #[cfg(feature = "ClassKit_CLSActivityItem")]
    unsafe impl CLSActivityItem {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: &NSString);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Id<NSString>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);
