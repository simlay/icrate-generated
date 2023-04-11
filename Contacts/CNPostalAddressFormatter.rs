//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Contacts::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    /**
      @abstract The formatting styles for postal addresses.
    */
    pub enum CNPostalAddressFormatterStyle {
        CNPostalAddressFormatterStyleMailingAddress = 0,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Contacts_CNPostalAddressFormatter")]
    /**
      @abstract Formats a postal address.

     @discussion This formatter handles international formatting of a postal address.
    */
    pub struct CNPostalAddressFormatter;

    #[cfg(feature = "Contacts_CNPostalAddressFormatter")]
    unsafe impl ClassType for CNPostalAddressFormatter {
        #[inherits(NSObject)]
        type Super = NSFormatter;
    }
);

#[cfg(feature = "Contacts_CNPostalAddressFormatter")]
/**
  @abstract Formats a postal address.

 @discussion This formatter handles international formatting of a postal address.
*/
unsafe impl NSCoding for CNPostalAddressFormatter {}

#[cfg(feature = "Contacts_CNPostalAddressFormatter")]
/**
  @abstract Formats a postal address.

 @discussion This formatter handles international formatting of a postal address.
*/
unsafe impl NSObjectProtocol for CNPostalAddressFormatter {}

extern_methods!(
    /**
      @abstract Formats a postal address.

     @discussion This formatter handles international formatting of a postal address.
    */
    #[cfg(feature = "Contacts_CNPostalAddressFormatter")]
    unsafe impl CNPostalAddressFormatter {
        #[cfg(all(feature = "Contacts_CNPostalAddress", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other stringFromPostalAddress:style:)]
        pub unsafe fn stringFromPostalAddress_style(
            postal_address: &CNPostalAddress,
            style: CNPostalAddressFormatterStyle,
        ) -> Id<NSString>;

        #[cfg(all(
            feature = "Contacts_CNPostalAddress",
            feature = "Foundation_NSAttributedString",
            feature = "Foundation_NSDictionary"
        ))]
        #[method_id(@__retain_semantics Other attributedStringFromPostalAddress:style:withDefaultAttributes:)]
        pub unsafe fn attributedStringFromPostalAddress_style_withDefaultAttributes(
            postal_address: &CNPostalAddress,
            style: CNPostalAddressFormatterStyle,
            attributes: &NSDictionary,
        ) -> Id<NSAttributedString>;

        /**
          @abstract The style for a postal address formatter instance.

         @discussion The default value is CNPostalAddressFormatterStyleMailingAddress.
        */
        #[method(style)]
        pub unsafe fn style(&self) -> CNPostalAddressFormatterStyle;

        /**
          @abstract The style for a postal address formatter instance.

         @discussion The default value is CNPostalAddressFormatterStyleMailingAddress.
        */
        #[method(setStyle:)]
        pub unsafe fn setStyle(&self, style: CNPostalAddressFormatterStyle);

        #[cfg(all(feature = "Contacts_CNPostalAddress", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other stringFromPostalAddress:)]
        pub unsafe fn stringFromPostalAddress(
            &self,
            postal_address: &CNPostalAddress,
        ) -> Id<NSString>;

        #[cfg(all(
            feature = "Contacts_CNPostalAddress",
            feature = "Foundation_NSAttributedString",
            feature = "Foundation_NSDictionary"
        ))]
        #[method_id(@__retain_semantics Other attributedStringFromPostalAddress:withDefaultAttributes:)]
        pub unsafe fn attributedStringFromPostalAddress_withDefaultAttributes(
            &self,
            postal_address: &CNPostalAddress,
            attributes: &NSDictionary,
        ) -> Id<NSAttributedString>;
    }
);

extern_static!(CNPostalAddressPropertyAttribute: &'static NSString);

extern_static!(CNPostalAddressLocalizedPropertyNameAttribute: &'static NSString);
