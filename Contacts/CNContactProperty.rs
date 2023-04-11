//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Contacts::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Contacts_CNContactProperty")]
    /**
      @abstract Contains related information for a specific contact property.

     @discussion CNContactProperty is used by the CNContactPicker to return the user's selected property.
    */
    pub struct CNContactProperty;

    #[cfg(feature = "Contacts_CNContactProperty")]
    unsafe impl ClassType for CNContactProperty {
        type Super = NSObject;
    }
);

#[cfg(feature = "Contacts_CNContactProperty")]
/**
  @abstract Contains related information for a specific contact property.

 @discussion CNContactProperty is used by the CNContactPicker to return the user's selected property.
*/
unsafe impl NSCoding for CNContactProperty {}

#[cfg(feature = "Contacts_CNContactProperty")]
/**
  @abstract Contains related information for a specific contact property.

 @discussion CNContactProperty is used by the CNContactPicker to return the user's selected property.
*/
unsafe impl NSObjectProtocol for CNContactProperty {}

#[cfg(feature = "Contacts_CNContactProperty")]
/**
  @abstract Contains related information for a specific contact property.

 @discussion CNContactProperty is used by the CNContactPicker to return the user's selected property.
*/
unsafe impl NSSecureCoding for CNContactProperty {}

extern_methods!(
    /**
      @abstract Contains related information for a specific contact property.

     @discussion CNContactProperty is used by the CNContactPicker to return the user's selected property.
    */
    #[cfg(feature = "Contacts_CNContactProperty")]
    unsafe impl CNContactProperty {
        #[cfg(feature = "Contacts_CNContact")]
        #[method_id(@__retain_semantics Other contact)]
        pub unsafe fn contact(&self) -> Id<CNContact>;

        #[cfg(feature = "Foundation_NSString")]
        /**
          @abstract The key of the contact property, as defined in CNContact.h.
        */
        #[method_id(@__retain_semantics Other key)]
        pub unsafe fn key(&self) -> Id<NSString>;

        /**
          @abstract The value of the property.
        */
        #[method_id(@__retain_semantics Other value)]
        pub unsafe fn value(&self) -> Option<Id<Object>>;

        #[cfg(feature = "Foundation_NSString")]
        /**
          @abstract The identifier of the labeled value if the property is an array of labeled values, otherwise is nil.
        */
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        /**
          @abstract The label of the labeled value if the property is an array of labeled values, otherwise is nil.
        */
        #[method_id(@__retain_semantics Other label)]
        pub unsafe fn label(&self) -> Option<Id<NSString>>;
    }
);
