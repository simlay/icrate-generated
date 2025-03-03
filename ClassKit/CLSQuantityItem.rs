//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::ClassKit::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "ClassKit_CLSQuantityItem")]
    pub struct CLSQuantityItem;

    #[cfg(feature = "ClassKit_CLSQuantityItem")]
    unsafe impl ClassType for CLSQuantityItem {
        #[inherits(CLSObject, NSObject)]
        type Super = CLSActivityItem;
    }
);

#[cfg(feature = "ClassKit_CLSQuantityItem")]
unsafe impl NSCoding for CLSQuantityItem {}

#[cfg(feature = "ClassKit_CLSQuantityItem")]
unsafe impl NSObjectProtocol for CLSQuantityItem {}

#[cfg(feature = "ClassKit_CLSQuantityItem")]
unsafe impl NSSecureCoding for CLSQuantityItem {}

extern_methods!(
    #[cfg(feature = "ClassKit_CLSQuantityItem")]
    unsafe impl CLSQuantityItem {
        #[method(quantity)]
        pub unsafe fn quantity(&self) -> c_double;

        #[method(setQuantity:)]
        pub unsafe fn setQuantity(&self, quantity: c_double);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithIdentifier:title:)]
        pub unsafe fn initWithIdentifier_title(
            this: Option<Allocated<Self>>,
            identifier: &NSString,
            title: &NSString,
        ) -> Id<Self>;
    }
);
