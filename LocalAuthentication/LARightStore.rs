//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::LocalAuthentication::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "LocalAuthentication_LARightStore")]
    #[cfg(not(any(target_os = "watchos")))]
    pub struct LARightStore;

    #[cfg(not(any(target_os = "watchos")))]
    #[cfg(feature = "LocalAuthentication_LARightStore")]
    unsafe impl ClassType for LARightStore {
        type Super = NSObject;
    }
);

#[cfg(feature = "LocalAuthentication_LARightStore")]
#[cfg(not(any(target_os = "watchos")))]
unsafe impl NSObjectProtocol for LARightStore {}

#[cfg(not(any(target_os = "watchos")))]
extern_methods!(
    #[cfg(feature = "LocalAuthentication_LARightStore")]
    unsafe impl LARightStore {
        #[method_id(@__retain_semantics Other sharedStore)]
        pub unsafe fn sharedStore() -> Id<LARightStore>;

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "LocalAuthentication_LAPersistedRight"
        ))]
        #[method(rightForIdentifier:completion:)]
        pub unsafe fn rightForIdentifier_completion(
            &self,
            identifier: &NSString,
            handler: &Block<(*mut LAPersistedRight, *mut NSError), ()>,
        );

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "LocalAuthentication_LAPersistedRight",
            feature = "LocalAuthentication_LARight"
        ))]
        #[method(saveRight:identifier:completion:)]
        pub unsafe fn saveRight_identifier_completion(
            &self,
            right: &LARight,
            identifier: &NSString,
            handler: &Block<(*mut LAPersistedRight, *mut NSError), ()>,
        );

        #[cfg(all(
            feature = "Foundation_NSData",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "LocalAuthentication_LAPersistedRight",
            feature = "LocalAuthentication_LARight"
        ))]
        #[method(saveRight:identifier:secret:completion:)]
        pub unsafe fn saveRight_identifier_secret_completion(
            &self,
            right: &LARight,
            identifier: &NSString,
            secret: &NSData,
            handler: &Block<(*mut LAPersistedRight, *mut NSError), ()>,
        );

        #[cfg(all(
            feature = "Foundation_NSError",
            feature = "LocalAuthentication_LAPersistedRight"
        ))]
        #[method(removeRight:completion:)]
        pub unsafe fn removeRight_completion(
            &self,
            right: &LAPersistedRight,
            handler: &Block<(*mut NSError,), ()>,
        );

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSString"))]
        #[method(removeRightForIdentifier:completion:)]
        pub unsafe fn removeRightForIdentifier_completion(
            &self,
            identifier: &NSString,
            handler: &Block<(*mut NSError,), ()>,
        );

        #[cfg(feature = "Foundation_NSError")]
        #[method(removeAllRightsWithCompletion:)]
        pub unsafe fn removeAllRightsWithCompletion(&self, handler: &Block<(*mut NSError,), ()>);

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);
