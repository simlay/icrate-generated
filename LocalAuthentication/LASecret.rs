//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::LocalAuthentication::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "LocalAuthentication_LASecret")]
    #[cfg(not(any(target_os = "watchos")))]
    pub struct LASecret;

    #[cfg(not(any(target_os = "watchos")))]
    #[cfg(feature = "LocalAuthentication_LASecret")]
    unsafe impl ClassType for LASecret {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "LocalAuthentication_LASecret")]
#[cfg(not(any(target_os = "watchos")))]
unsafe impl NSObjectProtocol for LASecret {}

#[cfg(not(any(target_os = "watchos")))]
extern_methods!(
    #[cfg(feature = "LocalAuthentication_LASecret")]
    #[cfg(not(any(target_os = "watchos")))]
    unsafe impl LASecret {
        #[cfg(all(feature = "Foundation_NSData", feature = "Foundation_NSError"))]
        #[method(loadDataWithCompletion:)]
        pub unsafe fn loadDataWithCompletion(
            &self,
            handler: &Block<(*mut NSData, *mut NSError), ()>,
        );

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);
