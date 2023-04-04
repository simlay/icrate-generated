//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#![allow(unused_imports)]
#![allow(deprecated)]
#[path = "OSALanguage.rs"]
mod __OSALanguage;
#[path = "OSALanguageInstance.rs"]
mod __OSALanguageInstance;
#[path = "OSAScript.rs"]
mod __OSAScript;
#[path = "OSAScriptController.rs"]
mod __OSAScriptController;
#[path = "OSAScriptView.rs"]
mod __OSAScriptView;

pub use self::__OSALanguage::OSALanguageFeatures;

pub use self::__OSALanguage::OSASupportsCompiling;

pub use self::__OSALanguage::OSASupportsGetSource;

pub use self::__OSALanguage::OSASupportsAECoercion;

pub use self::__OSALanguage::OSASupportsAESending;

pub use self::__OSALanguage::OSASupportsRecording;

pub use self::__OSALanguage::OSASupportsConvenience;

pub use self::__OSALanguage::OSASupportsDialects;

#[cfg(feature = "OSAKit_OSALanguage")]
pub use self::__OSALanguage::OSALanguage;
pub use self::__OSALanguage::OSASupportsEventHandling;
#[cfg(feature = "OSAKit_OSALanguageInstance")]
pub use self::__OSALanguageInstance::OSALanguageInstance;

pub use self::__OSAScript::OSAScriptErrorMessageKey;

pub use self::__OSAScript::OSAScriptErrorBriefMessageKey;

pub use self::__OSAScript::OSAScriptErrorNumberKey;

pub use self::__OSAScript::OSAScriptErrorPartialResultKey;

pub use self::__OSAScript::OSAScriptErrorOffendingObjectKey;

pub use self::__OSAScript::OSAScriptErrorExpectedTypeKey;

pub use self::__OSAScript::OSAScriptErrorAppAddressKey;

pub use self::__OSAScript::OSAScriptErrorAppNameKey;

pub use self::__OSAScript::OSAScriptErrorRangeKey;

pub use self::__OSAScript::OSAScriptErrorMessage;

pub use self::__OSAScript::OSAScriptErrorNumber;

pub use self::__OSAScript::OSAScriptErrorAppName;

pub use self::__OSAScript::OSAScriptErrorBriefMessage;

pub use self::__OSAScript::OSAScriptErrorRange;

pub use self::__OSAScript::OSAStorageScriptType;

pub use self::__OSAScript::OSAStorageScriptBundleType;

pub use self::__OSAScript::OSAStorageApplicationType;

pub use self::__OSAScript::OSAStorageApplicationBundleType;

pub use self::__OSAScript::OSAStorageTextType;

pub use self::__OSAScript::OSAStorageOptions;

pub use self::__OSAScript::OSANull;

pub use self::__OSAScript::OSAPreventGetSource;

pub use self::__OSAScript::OSACompileIntoContext;

pub use self::__OSAScript::OSADontSetScriptLocation;

pub use self::__OSAScript::OSAStayOpenApplet;

#[cfg(feature = "OSAKit_OSAScript")]
pub use self::__OSAScript::OSAScript;
pub use self::__OSAScript::OSAShowStartupScreen;

pub use self::__OSAScriptController::OSAScriptState;

pub use self::__OSAScriptController::OSAScriptStopped;

pub use self::__OSAScriptController::OSAScriptRunning;

#[cfg(feature = "OSAKit_OSAScriptController")]
pub use self::__OSAScriptController::OSAScriptController;
pub use self::__OSAScriptController::OSAScriptRecording;
#[cfg(feature = "OSAKit_OSAScriptView")]
pub use self::__OSAScriptView::OSAScriptView;
