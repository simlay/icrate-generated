//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::MailKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MailKit_MEComposeSession")]
    /**
      @brief An instance of this class is associated with the lifecycle of a single mail compose window. This object associates the actions performed by the user in a mail compose window to a unique session. An instance of this class is passed to the methods in @c MEComposeSessionHandler.
    */
    pub struct MEComposeSession;

    #[cfg(feature = "MailKit_MEComposeSession")]
    unsafe impl ClassType for MEComposeSession {
        type Super = NSObject;
    }
);

#[cfg(feature = "MailKit_MEComposeSession")]
/**
  @brief An instance of this class is associated with the lifecycle of a single mail compose window. This object associates the actions performed by the user in a mail compose window to a unique session. An instance of this class is passed to the methods in @c MEComposeSessionHandler.
*/
unsafe impl NSCoding for MEComposeSession {}

#[cfg(feature = "MailKit_MEComposeSession")]
/**
  @brief An instance of this class is associated with the lifecycle of a single mail compose window. This object associates the actions performed by the user in a mail compose window to a unique session. An instance of this class is passed to the methods in @c MEComposeSessionHandler.
*/
unsafe impl NSObjectProtocol for MEComposeSession {}

#[cfg(feature = "MailKit_MEComposeSession")]
/**
  @brief An instance of this class is associated with the lifecycle of a single mail compose window. This object associates the actions performed by the user in a mail compose window to a unique session. An instance of this class is passed to the methods in @c MEComposeSessionHandler.
*/
unsafe impl NSSecureCoding for MEComposeSession {}

extern_methods!(
    /**
      @brief An instance of this class is associated with the lifecycle of a single mail compose window. This object associates the actions performed by the user in a mail compose window to a unique session. An instance of this class is passed to the methods in @c MEComposeSessionHandler.
    */
    #[cfg(feature = "MailKit_MEComposeSession")]
    unsafe impl MEComposeSession {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[cfg(feature = "Foundation_NSUUID")]
        /**
          @brief A unique identifier for the session.
        */
        #[method_id(@__retain_semantics Other sessionID)]
        pub unsafe fn sessionID(&self) -> Id<NSUUID>;

        #[cfg(feature = "MailKit_MEMessage")]
        /**
          @brief An instance of @c MEMessage that represents properties of the mail message that author is composing in this @c MEComposeSession
        */
        #[method_id(@__retain_semantics Other mailMessage)]
        pub unsafe fn mailMessage(&self) -> Id<MEMessage>;

        #[cfg(feature = "MailKit_MEComposeContext")]
        /**
          @brief An instance of @c MEComposeContext that provides additional information about the compose session.
        */
        #[method_id(@__retain_semantics Other composeContext)]
        pub unsafe fn composeContext(&self) -> Id<MEComposeContext>;

        #[method(reloadSession)]
        pub unsafe fn reloadSession(&self);
    }
);

extern_static!(MEComposeSessionErrorDomain: &'static NSErrorDomain);

ns_error_enum!(
    #[underlying(NSInteger)]
    pub enum MEComposeSessionErrorCode {
        MEComposeSessionErrorCodeInvalidRecipients = 0,
        MEComposeSessionErrorCodeInvalidHeaders = 1,
        MEComposeSessionErrorCodeInvalidBody = 2,
    }
);

extern_protocol!(
    /**
      @brief Methods in this protocol can be used by a mail app extension to keep track of new messages composed by the user and to make changes to the recipeint email address tokens.
    */
    pub unsafe trait MEComposeSessionHandler: NSObjectProtocol {
        #[cfg(feature = "MailKit_MEComposeSession")]
        #[method(mailComposeSessionDidBegin:)]
        unsafe fn mailComposeSessionDidBegin(&self, session: &MEComposeSession);

        #[cfg(feature = "MailKit_MEComposeSession")]
        #[method(mailComposeSessionDidEnd:)]
        unsafe fn mailComposeSessionDidEnd(&self, session: &MEComposeSession);

        #[cfg(all(
            feature = "MailKit_MEComposeSession",
            feature = "MailKit_MEExtensionViewController"
        ))]
        #[method_id(@__retain_semantics Other viewControllerForSession:)]
        unsafe fn viewControllerForSession(
            &self,
            session: &MEComposeSession,
        ) -> Id<MEExtensionViewController>;

        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "MailKit_MEAddressAnnotation",
            feature = "MailKit_MEComposeSession",
            feature = "MailKit_MEEmailAddress"
        ))]
        #[optional]
        #[method(session:annotateAddressesWithCompletionHandler:)]
        unsafe fn session_annotateAddressesWithCompletionHandler(
            &self,
            session: &MEComposeSession,
            completion_handler: &Block<
                (NonNull<NSDictionary<MEEmailAddress, MEAddressAnnotation>>,),
                (),
            >,
        );

        #[cfg(all(feature = "Foundation_NSError", feature = "MailKit_MEComposeSession"))]
        #[optional]
        #[method(session:canSendMessageWithCompletionHandler:)]
        unsafe fn session_canSendMessageWithCompletionHandler(
            &self,
            session: &MEComposeSession,
            completion: &Block<(*mut NSError,), ()>,
        );

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSString",
            feature = "MailKit_MEComposeSession"
        ))]
        #[optional]
        #[method_id(@__retain_semantics Other additionalHeadersForSession:)]
        unsafe fn additionalHeadersForSession(
            &self,
            session: &MEComposeSession,
        ) -> Id<NSDictionary<NSString, NSArray<NSString>>>;
    }

    unsafe impl ProtocolType for dyn MEComposeSessionHandler {}
);
