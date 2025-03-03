//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSDistantObject")]
    #[deprecated = "Use NSXPCConnection instead"]
    pub struct NSDistantObject;

    #[cfg(feature = "Foundation_NSDistantObject")]
    unsafe impl ClassType for NSDistantObject {
        type Super = NSProxy;
    }
);

#[cfg(feature = "Foundation_NSDistantObject")]
unsafe impl NSCoding for NSDistantObject {}

#[cfg(feature = "Foundation_NSDistantObject")]
unsafe impl NSObjectProtocol for NSDistantObject {}

extern_methods!(
    #[cfg(feature = "Foundation_NSDistantObject")]
    unsafe impl NSDistantObject {
        #[cfg(feature = "Foundation_NSConnection")]
        #[method_id(@__retain_semantics Other proxyWithTarget:connection:)]
        pub unsafe fn proxyWithTarget_connection(
            target: &Object,
            connection: &NSConnection,
        ) -> Option<Id<Object>>;

        #[cfg(feature = "Foundation_NSConnection")]
        #[method_id(@__retain_semantics Init initWithTarget:connection:)]
        pub unsafe fn initWithTarget_connection(
            this: Option<Allocated<Self>>,
            target: &Object,
            connection: &NSConnection,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSConnection")]
        #[method_id(@__retain_semantics Other proxyWithLocal:connection:)]
        pub unsafe fn proxyWithLocal_connection(
            target: &Object,
            connection: &NSConnection,
        ) -> Id<Object>;

        #[cfg(feature = "Foundation_NSConnection")]
        #[method_id(@__retain_semantics Init initWithLocal:connection:)]
        pub unsafe fn initWithLocal_connection(
            this: Option<Allocated<Self>>,
            target: &Object,
            connection: &NSConnection,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            in_coder: &NSCoder,
        ) -> Option<Id<Self>>;

        #[method(setProtocolForProxy:)]
        pub unsafe fn setProtocolForProxy(&self, proto: Option<&Protocol>);

        #[cfg(feature = "Foundation_NSConnection")]
        #[method_id(@__retain_semantics Other connectionForProxy)]
        pub unsafe fn connectionForProxy(&self) -> Id<NSConnection>;
    }
);
