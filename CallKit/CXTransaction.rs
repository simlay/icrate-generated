//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CallKit::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CallKit_CXTransaction")]
    pub struct CXTransaction;

    #[cfg(feature = "CallKit_CXTransaction")]
    unsafe impl ClassType for CXTransaction {
        type Super = NSObject;
    }
);

#[cfg(feature = "CallKit_CXTransaction")]
unsafe impl NSCoding for CXTransaction {}

#[cfg(feature = "CallKit_CXTransaction")]
unsafe impl NSObjectProtocol for CXTransaction {}

#[cfg(feature = "CallKit_CXTransaction")]
unsafe impl NSSecureCoding for CXTransaction {}

extern_methods!(
    #[cfg(feature = "CallKit_CXTransaction")]
    unsafe impl CXTransaction {
        #[cfg(feature = "Foundation_NSUUID")]
        /**
          Unique ID
        */
        #[method_id(@__retain_semantics Other UUID)]
        pub unsafe fn UUID(&self) -> Id<NSUUID>;

        /**
          Whether all actions have been completed
        */
        #[method(isComplete)]
        pub unsafe fn isComplete(&self) -> bool;

        #[cfg(all(feature = "CallKit_CXAction", feature = "Foundation_NSArray"))]
        /**
          The list of actions contained by the receiver
        */
        #[method_id(@__retain_semantics Other actions)]
        pub unsafe fn actions(&self) -> Id<NSArray<CXAction>>;

        #[cfg(all(feature = "CallKit_CXAction", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Init initWithActions:)]
        pub unsafe fn initWithActions(
            this: Option<Allocated<Self>>,
            actions: &NSArray<CXAction>,
        ) -> Id<Self>;

        #[cfg(feature = "CallKit_CXAction")]
        #[method_id(@__retain_semantics Init initWithAction:)]
        pub unsafe fn initWithAction(this: Option<Allocated<Self>>, action: &CXAction) -> Id<Self>;

        #[cfg(feature = "CallKit_CXAction")]
        #[method(addAction:)]
        pub unsafe fn addAction(&self, action: &CXAction);
    }
);
