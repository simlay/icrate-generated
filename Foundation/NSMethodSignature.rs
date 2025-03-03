//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSMethodSignature")]
    pub struct NSMethodSignature;

    #[cfg(feature = "Foundation_NSMethodSignature")]
    unsafe impl ClassType for NSMethodSignature {
        type Super = NSObject;
    }
);

#[cfg(feature = "Foundation_NSMethodSignature")]
unsafe impl NSObjectProtocol for NSMethodSignature {}

extern_methods!(
    #[cfg(feature = "Foundation_NSMethodSignature")]
    unsafe impl NSMethodSignature {
        #[method_id(@__retain_semantics Other signatureWithObjCTypes:)]
        pub unsafe fn signatureWithObjCTypes(
            types: NonNull<c_char>,
        ) -> Option<Id<NSMethodSignature>>;

        #[method(numberOfArguments)]
        pub unsafe fn numberOfArguments(&self) -> NSUInteger;

        #[method(getArgumentTypeAtIndex:)]
        pub unsafe fn getArgumentTypeAtIndex(&self, idx: NSUInteger) -> NonNull<c_char>;

        #[method(frameLength)]
        pub unsafe fn frameLength(&self) -> NSUInteger;

        #[method(isOneway)]
        pub unsafe fn isOneway(&self) -> bool;

        #[method(methodReturnType)]
        pub unsafe fn methodReturnType(&self) -> NonNull<c_char>;

        #[method(methodReturnLength)]
        pub unsafe fn methodReturnLength(&self) -> NSUInteger;
    }
);
