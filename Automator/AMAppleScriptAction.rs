//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Automator::*;
use crate::Foundation::*;
use crate::OSAKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Automator_AMAppleScriptAction")]
    pub struct AMAppleScriptAction;

    #[cfg(feature = "Automator_AMAppleScriptAction")]
    unsafe impl ClassType for AMAppleScriptAction {
        #[inherits(AMAction, NSObject)]
        type Super = AMBundleAction;
    }
);

#[cfg(feature = "Automator_AMAppleScriptAction")]
unsafe impl NSCoding for AMAppleScriptAction {}

#[cfg(feature = "Automator_AMAppleScriptAction")]
unsafe impl NSObjectProtocol for AMAppleScriptAction {}

#[cfg(feature = "Automator_AMAppleScriptAction")]
unsafe impl NSSecureCoding for AMAppleScriptAction {}

extern_methods!(
    #[cfg(feature = "Automator_AMAppleScriptAction")]
    unsafe impl AMAppleScriptAction {
        #[cfg(feature = "OSAKit_OSAScript")]
        #[method_id(@__retain_semantics Other script)]
        pub unsafe fn script(&self) -> Option<Id<OSAScript>>;

        #[cfg(feature = "OSAKit_OSAScript")]
        #[method(setScript:)]
        pub unsafe fn setScript(&self, script: Option<&OSAScript>);
    }
);

extern_methods!(
    /// Methods declared on superclass `AMAction`
    #[cfg(feature = "Automator_AMAppleScriptAction")]
    unsafe impl AMAppleScriptAction {
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
    }
);
