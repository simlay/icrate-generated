//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::FileProvider::*;
use crate::Foundation::*;
use crate::UniformTypeIdentifiers::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "FileProvider_NSFileProviderRequest")]
    pub struct NSFileProviderRequest;

    #[cfg(feature = "FileProvider_NSFileProviderRequest")]
    unsafe impl ClassType for NSFileProviderRequest {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "FileProvider_NSFileProviderRequest")]
unsafe impl NSObjectProtocol for NSFileProviderRequest {}

extern_methods!(
    #[cfg(feature = "FileProvider_NSFileProviderRequest")]
    unsafe impl NSFileProviderRequest {
        #[method(isSystemRequest)]
        pub unsafe fn isSystemRequest(&self) -> bool;

        #[method(isFileViewerRequest)]
        pub unsafe fn isFileViewerRequest(&self) -> bool;

        #[cfg(feature = "Foundation_NSURL")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other requestingExecutable)]
        pub unsafe fn requestingExecutable(&self) -> Option<Id<NSURL>>;

        #[cfg(feature = "FileProvider_NSFileProviderDomainVersion")]
        #[method_id(@__retain_semantics Other domainVersion)]
        pub unsafe fn domainVersion(&self) -> Option<Id<NSFileProviderDomainVersion>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "FileProvider_NSFileProviderRequest")]
    unsafe impl NSFileProviderRequest {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
