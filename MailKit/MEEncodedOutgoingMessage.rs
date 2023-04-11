//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::MailKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MailKit_MEEncodedOutgoingMessage")]
    pub struct MEEncodedOutgoingMessage;

    #[cfg(feature = "MailKit_MEEncodedOutgoingMessage")]
    unsafe impl ClassType for MEEncodedOutgoingMessage {
        type Super = NSObject;
    }
);

#[cfg(feature = "MailKit_MEEncodedOutgoingMessage")]
unsafe impl NSCoding for MEEncodedOutgoingMessage {}

#[cfg(feature = "MailKit_MEEncodedOutgoingMessage")]
unsafe impl NSObjectProtocol for MEEncodedOutgoingMessage {}

#[cfg(feature = "MailKit_MEEncodedOutgoingMessage")]
unsafe impl NSSecureCoding for MEEncodedOutgoingMessage {}

extern_methods!(
    #[cfg(feature = "MailKit_MEEncodedOutgoingMessage")]
    unsafe impl MEEncodedOutgoingMessage {
        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Init initWithRawData:isSigned:isEncrypted:)]
        pub unsafe fn initWithRawData_isSigned_isEncrypted(
            this: Option<Allocated<Self>>,
            raw_data: &NSData,
            is_signed: bool,
            is_encrypted: bool,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSData")]
        /**
          @brief The full encoded RFC822 message including headers and body.
        */
        #[method_id(@__retain_semantics Other rawData)]
        pub unsafe fn rawData(&self) -> Id<NSData>;

        /**
          @brief Whether or not the encoded message is signed
        */
        #[method(isSigned)]
        pub unsafe fn isSigned(&self) -> bool;

        /**
          @brief Whether or not the encoded message is encrypted
        */
        #[method(isEncrypted)]
        pub unsafe fn isEncrypted(&self) -> bool;
    }
);
