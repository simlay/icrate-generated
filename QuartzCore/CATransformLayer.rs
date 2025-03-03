//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CoreAnimation::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CoreAnimation_CATransformLayer")]
    pub struct CATransformLayer;

    #[cfg(feature = "CoreAnimation_CATransformLayer")]
    unsafe impl ClassType for CATransformLayer {
        #[inherits(NSObject)]
        type Super = CALayer;
    }
);

#[cfg(feature = "CoreAnimation_CATransformLayer")]
unsafe impl CAMediaTiming for CATransformLayer {}

#[cfg(feature = "CoreAnimation_CATransformLayer")]
unsafe impl NSCoding for CATransformLayer {}

#[cfg(feature = "CoreAnimation_CATransformLayer")]
unsafe impl NSObjectProtocol for CATransformLayer {}

#[cfg(feature = "CoreAnimation_CATransformLayer")]
unsafe impl NSSecureCoding for CATransformLayer {}

extern_methods!(
    #[cfg(feature = "CoreAnimation_CATransformLayer")]
    unsafe impl CATransformLayer {}
);

extern_methods!(
    /// Methods declared on superclass `CALayer`
    #[cfg(feature = "CoreAnimation_CATransformLayer")]
    unsafe impl CATransformLayer {
        #[method_id(@__retain_semantics Other layer)]
        pub unsafe fn layer() -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithLayer:)]
        pub unsafe fn initWithLayer(this: Option<Allocated<Self>>, layer: &Object) -> Id<Self>;
    }
);
