//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Automator::*;
use crate::Foundation::*;
use crate::OSAKit::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum AMLogLevel {
        AMLogLevelDebug = 0,
        AMLogLevelInfo = 1,
        AMLogLevelWarn = 2,
        AMLogLevelError = 3,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Automator_AMAction")]
    pub struct AMAction;

    #[cfg(feature = "Automator_AMAction")]
    unsafe impl ClassType for AMAction {
        type Super = NSObject;
    }
);

#[cfg(feature = "Automator_AMAction")]
unsafe impl NSObjectProtocol for AMAction {}

extern_methods!(
    #[cfg(feature = "Automator_AMAction")]
    unsafe impl AMAction {
        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Init initWithDefinition:fromArchive:)]
        pub unsafe fn initWithDefinition_fromArchive(
            this: Option<Allocated<Self>>,
            dict: Option<&NSDictionary<NSString, Object>>,
            archived: bool,
        ) -> Option<Id<Self>>;

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSURL"))]
        #[method_id(@__retain_semantics Init initWithContentsOfURL:error:_)]
        pub unsafe fn initWithContentsOfURL_error(
            this: Option<Allocated<Self>>,
            file_url: &NSURL,
        ) -> Result<Id<Self>, Id<NSError>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Id<NSString>;

        #[method(ignoresInput)]
        pub unsafe fn ignoresInput(&self) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other selectedInputType)]
        pub unsafe fn selectedInputType(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setSelectedInputType:)]
        pub unsafe fn setSelectedInputType(&self, selected_input_type: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other selectedOutputType)]
        pub unsafe fn selectedOutputType(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setSelectedOutputType:)]
        pub unsafe fn setSelectedOutputType(&self, selected_output_type: Option<&NSString>);

        #[method(progressValue)]
        pub unsafe fn progressValue(&self) -> CGFloat;

        #[method(setProgressValue:)]
        pub unsafe fn setProgressValue(&self, progress_value: CGFloat);

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[deprecated]
        #[method_id(@__retain_semantics Other runWithInput:fromAction:error:)]
        pub unsafe fn runWithInput_fromAction_error(
            &self,
            input: Option<&Object>,
            an_action: Option<&AMAction>,
            error_info: Option<&mut Option<Id<NSDictionary<NSString, Object>>>>,
        ) -> Option<Id<Object>>;

        #[cfg(feature = "Foundation_NSError")]
        #[method_id(@__retain_semantics Other runWithInput:error:_)]
        pub unsafe fn runWithInput_error(
            &self,
            input: Option<&Object>,
        ) -> Result<Id<Object>, Id<NSError>>;

        #[method(runAsynchronouslyWithInput:)]
        pub unsafe fn runAsynchronouslyWithInput(&self, input: Option<&Object>);

        #[method(willFinishRunning)]
        pub unsafe fn willFinishRunning(&self);

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[deprecated]
        #[method(didFinishRunningWithError:)]
        pub unsafe fn didFinishRunningWithError(
            &self,
            error_info: Option<&NSDictionary<NSString, Object>>,
        );

        #[cfg(feature = "Foundation_NSError")]
        #[method(finishRunningWithError:)]
        pub unsafe fn finishRunningWithError(&self, error: Option<&NSError>);

        #[method_id(@__retain_semantics Other output)]
        pub unsafe fn output(&self) -> Option<Id<Object>>;

        #[method(setOutput:)]
        pub unsafe fn setOutput(&self, output: Option<&Object>);

        #[method(stop)]
        pub unsafe fn stop(&self);

        #[method(reset)]
        pub unsafe fn reset(&self);

        #[cfg(all(
            feature = "Foundation_NSMutableDictionary",
            feature = "Foundation_NSString"
        ))]
        #[method(writeToDictionary:)]
        pub unsafe fn writeToDictionary(&self, dictionary: &NSMutableDictionary<NSString, Object>);

        #[method(opened)]
        pub unsafe fn opened(&self);

        #[method(activated)]
        pub unsafe fn activated(&self);

        #[method(closed)]
        pub unsafe fn closed(&self);

        #[method(updateParameters)]
        pub unsafe fn updateParameters(&self);

        #[method(parametersUpdated)]
        pub unsafe fn parametersUpdated(&self);

        #[method(isStopped)]
        pub unsafe fn isStopped(&self) -> bool;
    }
);
