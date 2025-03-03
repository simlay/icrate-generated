//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSProtocolChecker")]
    pub struct NSProtocolChecker;

    #[cfg(feature = "Foundation_NSProtocolChecker")]
    unsafe impl ClassType for NSProtocolChecker {
        type Super = NSProxy;
    }
);

#[cfg(feature = "Foundation_NSProtocolChecker")]
unsafe impl NSObjectProtocol for NSProtocolChecker {}

extern_methods!(
    #[cfg(feature = "Foundation_NSProtocolChecker")]
    unsafe impl NSProtocolChecker {
        #[method_id(@__retain_semantics Other protocol)]
        pub unsafe fn protocol(&self) -> Id<Protocol>;

        #[method_id(@__retain_semantics Other target)]
        pub unsafe fn target(&self) -> Option<Id<NSObject>>;
    }
);

extern_methods!(
    /// NSProtocolCheckerCreation
    #[cfg(feature = "Foundation_NSProtocolChecker")]
    unsafe impl NSProtocolChecker {
        #[method_id(@__retain_semantics Other protocolCheckerWithTarget:protocol:)]
        pub unsafe fn protocolCheckerWithTarget_protocol(
            an_object: &NSObject,
            a_protocol: &Protocol,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithTarget:protocol:)]
        pub unsafe fn initWithTarget_protocol(
            this: Option<Allocated<Self>>,
            an_object: &NSObject,
            a_protocol: &Protocol,
        ) -> Id<Self>;
    }
);
