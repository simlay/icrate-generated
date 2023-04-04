//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#![allow(unused_imports)]
#![allow(deprecated)]
#[path = "ExceptionHandlingDefines.rs"]
mod __ExceptionHandlingDefines;
#[path = "NSExceptionHandler.rs"]
mod __NSExceptionHandler;

pub use self::__NSExceptionHandler::NSUncaughtSystemExceptionException;

pub use self::__NSExceptionHandler::NSUncaughtRuntimeErrorException;

pub use self::__NSExceptionHandler::NSStackTraceKey;

pub use self::__NSExceptionHandler::NSExceptionHandlerResume;

pub use self::__NSExceptionHandler::NSLogUncaughtExceptionMask;

pub use self::__NSExceptionHandler::NSHandleUncaughtExceptionMask;

pub use self::__NSExceptionHandler::NSLogUncaughtSystemExceptionMask;

pub use self::__NSExceptionHandler::NSHandleUncaughtSystemExceptionMask;

pub use self::__NSExceptionHandler::NSLogUncaughtRuntimeErrorMask;

pub use self::__NSExceptionHandler::NSHandleUncaughtRuntimeErrorMask;

pub use self::__NSExceptionHandler::NSLogTopLevelExceptionMask;

pub use self::__NSExceptionHandler::NSHandleTopLevelExceptionMask;

pub use self::__NSExceptionHandler::NSLogOtherExceptionMask;

pub use self::__NSExceptionHandler::NSHandleOtherExceptionMask;

pub use self::__NSExceptionHandler::NSHangOnUncaughtExceptionMask;

pub use self::__NSExceptionHandler::NSHangOnUncaughtSystemExceptionMask;

pub use self::__NSExceptionHandler::NSHangOnUncaughtRuntimeErrorMask;

pub use self::__NSExceptionHandler::NSHangOnTopLevelExceptionMask;

#[cfg(feature = "ExceptionHandling_NSExceptionHandler")]
pub use self::__NSExceptionHandler::NSExceptionHandler;
pub use self::__NSExceptionHandler::NSHangOnOtherExceptionMask;
