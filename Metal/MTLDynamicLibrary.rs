//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;
use crate::Metal::*;

extern_static!(MTLDynamicLibraryDomain: &'static NSErrorDomain);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum MTLDynamicLibraryError {
        MTLDynamicLibraryErrorNone = 0,
        MTLDynamicLibraryErrorInvalidFile = 1,
        MTLDynamicLibraryErrorCompilationFailure = 2,
        MTLDynamicLibraryErrorUnresolvedInstallName = 3,
        MTLDynamicLibraryErrorDependencyLoadFailure = 4,
        MTLDynamicLibraryErrorUnsupported = 5,
    }
);

extern_protocol!(
    /**
     @protocol MTLDynamicLibrary
    @abstract A container for the binary representation of code compiled for a MTLDevice.
    @discussion MTLDynamicLibrary can be created in two ways:
    1) MTLDevice newDynamicLibrary:error:
    This method takes an MTLLibrary (which has .type set to MTLLibraryTypeDynamic) and then compiles the code in the MTLLibrary for the current device.
    2) MTLDevice newDynamicLibraryWithURL:error
    This method loads from a file a previously serialized MTLDynamicLibrary. If the dynamic library containg compiled code for the current device, that code is loaded.
    Otherwise, as a fallback, the MTLLibrary contents used to create the MTLDynamicLibrary are compiled for the current device similar to path #1 above.
    This path may also be taken if the driver for the current device has been updated or has otherwise become incompatible with the compiled code.
    Either way, if a MTLDynamicLibrary is successfully created, it contains compiled code for the current device.
    That code may be used via MTLComputePipelineDescriptor .preloadedLibraries to allow the code to be loaded into a MTLComputePipelineState
    It may also be used as an argument to MTLCompileOptions .libraries so that another MTLLibrary is linked against the code in this MTLDynamicLibrary.
    Such library dependencies are encoded into the resulting MTLLibrary by embedding the install name of the MTLDynamicLibrary.
    When creating a MTLComputePipelineState from a function in that MTLLibrary, the embedded install names are used to load MTLDynamicLibrary instances via path #2 (possibly falling back to #1 as well).
    If an embedded install name could not be used to load a MTLDynamicLibrary from the path indicated by the install name, the creation of the MTLComputePipelineState fails.
    The set of both the implictly loaded MTLDynamicLibrary and the MTLDynamicLibrary specified with .preloadedLibraries are used to resolve any unresolved symbols in the source MTLLibrary (or in other MTLDynamicLibrary).
    If any unresolved symbols remain after searching the set, the creation of the MTLComputePipelineState fails.
    Otherwise, the MTLComputePipelineState creation succeeds, and the set of MTLDynamicLibraries used are retained by the MTLComputePipelineState.
    */
    pub unsafe trait MTLDynamicLibrary: NSObjectProtocol {
        #[cfg(feature = "Foundation_NSString")]
        /**
         @property label
        @abstract A string to help identify this object.
        */
        #[method_id(@__retain_semantics Other label)]
        fn label(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        /**
         @property label
        @abstract A string to help identify this object.
        */
        #[method(setLabel:)]
        fn setLabel(&self, label: Option<&NSString>);

        /**
         @property device
        @abstract The device this resource was created against.  This resource can only be used with this device.
        */
        #[method_id(@__retain_semantics Other device)]
        fn device(&self) -> Id<ProtocolObject<dyn MTLDevice>>;

        #[cfg(feature = "Foundation_NSString")]
        /**
         @property installName
        @abstract The installName of this dynamic library. Can not be nil.
        */
        #[method_id(@__retain_semantics Other installName)]
        fn installName(&self) -> Id<NSString>;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
        #[method(serializeToURL:error:_)]
        fn serializeToURL_error(&self, url: &NSURL) -> Result<(), Id<NSError>>;
    }

    unsafe impl ProtocolType for dyn MTLDynamicLibrary {}
);
