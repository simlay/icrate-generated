//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AutomaticAssessmentConfiguration::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AutomaticAssessmentConfiguration_AEAssessmentSession")]
    pub struct AEAssessmentSession;

    #[cfg(feature = "AutomaticAssessmentConfiguration_AEAssessmentSession")]
    unsafe impl ClassType for AEAssessmentSession {
        type Super = NSObject;
    }
);

#[cfg(feature = "AutomaticAssessmentConfiguration_AEAssessmentSession")]
unsafe impl NSObjectProtocol for AEAssessmentSession {}

extern_methods!(
    #[cfg(feature = "AutomaticAssessmentConfiguration_AEAssessmentSession")]
    unsafe impl AEAssessmentSession {
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Id<ProtocolObject<dyn AEAssessmentSessionDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn AEAssessmentSessionDelegate>>,
        );

        #[cfg(feature = "AutomaticAssessmentConfiguration_AEAssessmentConfiguration")]
        #[method_id(@__retain_semantics Other configuration)]
        pub unsafe fn configuration(&self) -> Id<AEAssessmentConfiguration>;

        #[method(isActive)]
        pub unsafe fn isActive(&self) -> bool;

        #[cfg(feature = "AutomaticAssessmentConfiguration_AEAssessmentConfiguration")]
        #[method_id(@__retain_semantics Init initWithConfiguration:)]
        pub unsafe fn initWithConfiguration(
            this: Option<Allocated<Self>>,
            configuration: &AEAssessmentConfiguration,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method(begin)]
        pub unsafe fn begin(&self);

        #[method(end)]
        pub unsafe fn end(&self);

        #[cfg(feature = "AutomaticAssessmentConfiguration_AEAssessmentConfiguration")]
        #[cfg(not(any(target_os = "ios")))]
        #[method(updateToConfiguration:)]
        pub unsafe fn updateToConfiguration(&self, configuration: &AEAssessmentConfiguration);
    }
);
