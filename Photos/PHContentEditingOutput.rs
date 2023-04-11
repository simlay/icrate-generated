//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreLocation::*;
use crate::Foundation::*;
use crate::PhotoKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "PhotoKit_PHContentEditingOutput")]
    pub struct PHContentEditingOutput;

    #[cfg(feature = "PhotoKit_PHContentEditingOutput")]
    unsafe impl ClassType for PHContentEditingOutput {
        type Super = NSObject;
    }
);

#[cfg(feature = "PhotoKit_PHContentEditingOutput")]
unsafe impl NSObjectProtocol for PHContentEditingOutput {}

extern_methods!(
    #[cfg(feature = "PhotoKit_PHContentEditingOutput")]
    unsafe impl PHContentEditingOutput {
        #[cfg(feature = "PhotoKit_PHContentEditingInput")]
        #[method_id(@__retain_semantics Init initWithContentEditingInput:)]
        pub unsafe fn initWithContentEditingInput(
            this: Option<Allocated<Self>>,
            content_editing_input: &PHContentEditingInput,
        ) -> Id<Self>;

        #[cfg(feature = "PhotoKit_PHAdjustmentData")]
        #[method_id(@__retain_semantics Other adjustmentData)]
        pub unsafe fn adjustmentData(&self) -> Option<Id<PHAdjustmentData>>;

        #[cfg(feature = "PhotoKit_PHAdjustmentData")]
        #[method(setAdjustmentData:)]
        pub unsafe fn setAdjustmentData(&self, adjustment_data: Option<&PHAdjustmentData>);

        #[cfg(feature = "Foundation_NSURL")]
        /**
          File URL where the rendered output, with adjustments baked-in, needs to be written to.
        */
        #[method_id(@__retain_semantics Other renderedContentURL)]
        pub unsafe fn renderedContentURL(&self) -> Id<NSURL>;
    }
);
