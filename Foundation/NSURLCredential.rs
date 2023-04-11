//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSUInteger)]
    /**
     @enum NSURLCredentialPersistence
    @abstract Constants defining how long a credential will be kept around
    @constant NSURLCredentialPersistenceNone This credential won't be saved.
    @constant NSURLCredentialPersistenceForSession This credential will only be stored for this session.
    @constant NSURLCredentialPersistencePermanent This credential will be stored permanently. Note: Whereas in Mac OS X any application can access any credential provided the user gives permission, in iPhone OS an application can access only its own credentials.
    @constant NSURLCredentialPersistenceSynchronizable This credential will be stored permanently. Additionally, this credential will be distributed to other devices based on the owning AppleID.
    Note: Whereas in Mac OS X any application can access any credential provided the user gives permission, on iOS an application can
    access only its own credentials.
    */
    pub enum NSURLCredentialPersistence {
        NSURLCredentialPersistenceNone = 0,
        NSURLCredentialPersistenceForSession = 1,
        NSURLCredentialPersistencePermanent = 2,
        NSURLCredentialPersistenceSynchronizable = 3,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSURLCredential")]
    /**
     @class NSURLCredential
    @discussion This class is an immutable object representing an authentication credential.  The actual type of the credential is determined by the constructor called in the categories declared below.
    */
    pub struct NSURLCredential;

    #[cfg(feature = "Foundation_NSURLCredential")]
    unsafe impl ClassType for NSURLCredential {
        type Super = NSObject;
    }
);

#[cfg(feature = "Foundation_NSURLCredential")]
/**
 @class NSURLCredential
@discussion This class is an immutable object representing an authentication credential.  The actual type of the credential is determined by the constructor called in the categories declared below.
*/
unsafe impl NSCoding for NSURLCredential {}

#[cfg(feature = "Foundation_NSURLCredential")]
/**
 @class NSURLCredential
@discussion This class is an immutable object representing an authentication credential.  The actual type of the credential is determined by the constructor called in the categories declared below.
*/
unsafe impl NSObjectProtocol for NSURLCredential {}

#[cfg(feature = "Foundation_NSURLCredential")]
/**
 @class NSURLCredential
@discussion This class is an immutable object representing an authentication credential.  The actual type of the credential is determined by the constructor called in the categories declared below.
*/
unsafe impl NSSecureCoding for NSURLCredential {}

extern_methods!(
    /**
     @class NSURLCredential
    @discussion This class is an immutable object representing an authentication credential.  The actual type of the credential is determined by the constructor called in the categories declared below.
    */
    #[cfg(feature = "Foundation_NSURLCredential")]
    unsafe impl NSURLCredential {
        /**
         @abstract Determine whether this credential is or should be stored persistently
        @result A value indicating whether this credential is stored permanently, per session or not at all.
        */
        #[method(persistence)]
        pub unsafe fn persistence(&self) -> NSURLCredentialPersistence;
    }
);

extern_methods!(
    /**
     @discussion This category defines the methods available to an NSURLCredential created to represent an internet password credential.  These are most commonly used for resources that require a username and password combination.
    */
    /// NSInternetPassword
    #[cfg(feature = "Foundation_NSURLCredential")]
    unsafe impl NSURLCredential {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithUser:password:persistence:)]
        pub unsafe fn initWithUser_password_persistence(
            this: Option<Allocated<Self>>,
            user: &NSString,
            password: &NSString,
            persistence: NSURLCredentialPersistence,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other credentialWithUser:password:persistence:)]
        pub unsafe fn credentialWithUser_password_persistence(
            user: &NSString,
            password: &NSString,
            persistence: NSURLCredentialPersistence,
        ) -> Id<NSURLCredential>;

        #[cfg(feature = "Foundation_NSString")]
        /**
         @abstract Get the username
        @result The user string
        */
        #[method_id(@__retain_semantics Other user)]
        pub unsafe fn user(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        /**
         @abstract Get the password
        @result The password string
        @discussion This method might actually attempt to retrieve the
        password from an external store, possible resulting in prompting,
        so do not call it unless needed.
        */
        #[method_id(@__retain_semantics Other password)]
        pub unsafe fn password(&self) -> Option<Id<NSString>>;

        /**
         @abstract Find out if this credential has a password, without trying to get it
        @result YES if this credential has a password, otherwise NO
        @discussion If this credential's password is actually kept in an
        external store, the password method may return nil even if this
        method returns YES, since getting the password may fail, or the
        user may refuse access.
        */
        #[method(hasPassword)]
        pub unsafe fn hasPassword(&self) -> bool;
    }
);

extern_methods!(
    /**
     @discussion This category defines the methods available to an NSURLCredential created to represent a client certificate credential.  Client certificates are commonly stored on the users computer in the keychain and must be presented to the server during a handshake.
    */
    /// NSClientCertificate
    #[cfg(feature = "Foundation_NSURLCredential")]
    unsafe impl NSURLCredential {
        #[cfg(feature = "Foundation_NSArray")]
        /**
         @abstract Returns an NSArray of SecCertificateRef objects representing the client certificate for this credential, if this credential was created with an identity and certificate.
        @result an NSArray of SecCertificateRef or NULL if this is a username/password credential
        */
        #[method_id(@__retain_semantics Other certificates)]
        pub unsafe fn certificates(&self) -> Id<NSArray>;
    }
);

extern_methods!(
    /// NSServerTrust
    #[cfg(feature = "Foundation_NSURLCredential")]
    unsafe impl NSURLCredential {}
);
