//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AutomaticAssessmentConfiguration::*;
use crate::Foundation::*;

extern_protocol!(
    pub unsafe trait AEAssessmentSessionDelegate: NSObjectProtocol {
        #[cfg(feature = "AutomaticAssessmentConfiguration_AEAssessmentSession")]
        #[optional]
        #[method(assessmentSessionDidBegin:)]
        unsafe fn assessmentSessionDidBegin(&self, session: &AEAssessmentSession);

        #[cfg(all(
            feature = "AutomaticAssessmentConfiguration_AEAssessmentSession",
            feature = "Foundation_NSError"
        ))]
        #[optional]
        #[method(assessmentSession:failedToBeginWithError:)]
        unsafe fn assessmentSession_failedToBeginWithError(
            &self,
            session: &AEAssessmentSession,
            error: &NSError,
        );

        #[cfg(all(
            feature = "AutomaticAssessmentConfiguration_AEAssessmentSession",
            feature = "Foundation_NSError"
        ))]
        #[optional]
        #[method(assessmentSession:wasInterruptedWithError:)]
        unsafe fn assessmentSession_wasInterruptedWithError(
            &self,
            session: &AEAssessmentSession,
            error: &NSError,
        );

        #[cfg(feature = "AutomaticAssessmentConfiguration_AEAssessmentSession")]
        #[optional]
        #[method(assessmentSessionDidEnd:)]
        unsafe fn assessmentSessionDidEnd(&self, session: &AEAssessmentSession);

        #[cfg(feature = "AutomaticAssessmentConfiguration_AEAssessmentSession")]
        #[cfg(not(any(target_os = "ios")))]
        #[optional]
        #[method(assessmentSessionDidUpdate:)]
        unsafe fn assessmentSessionDidUpdate(&self, session: &AEAssessmentSession);

        #[cfg(all(
            feature = "AutomaticAssessmentConfiguration_AEAssessmentConfiguration",
            feature = "AutomaticAssessmentConfiguration_AEAssessmentSession",
            feature = "Foundation_NSError"
        ))]
        #[cfg(not(any(target_os = "ios")))]
        #[optional]
        #[method(assessmentSession:failedToUpdateToConfiguration:error:)]
        unsafe fn assessmentSession_failedToUpdateToConfiguration_error(
            &self,
            session: &AEAssessmentSession,
            configuration: &AEAssessmentConfiguration,
            error: &NSError,
        );
    }

    unsafe impl ProtocolType for dyn AEAssessmentSessionDelegate {}
);
