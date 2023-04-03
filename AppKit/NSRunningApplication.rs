//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_options!(
    #[underlying(NSUInteger)]
    #[cfg(not(any(target_os = "ios")))]
    pub enum NSApplicationActivationOptions {
        #[cfg(not(any(target_os = "ios")))]
        NSApplicationActivateAllWindows = 1 << 0,
        #[cfg(not(any(target_os = "ios")))]
        NSApplicationActivateIgnoringOtherApps = 1 << 1,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    #[cfg(not(any(target_os = "ios")))]
    pub enum NSApplicationActivationPolicy {
        #[cfg(not(any(target_os = "ios")))]
        NSApplicationActivationPolicyRegular = 0,
        #[cfg(not(any(target_os = "ios")))]
        NSApplicationActivationPolicyAccessory = 1,
        #[cfg(not(any(target_os = "ios")))]
        NSApplicationActivationPolicyProhibited = 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSRunningApplication")]
    #[cfg(not(any(target_os = "ios")))]
    pub struct NSRunningApplication;

    #[cfg(feature = "AppKit_NSRunningApplication")]
    unsafe impl ClassType for NSRunningApplication {
        type Super = NSObject;
    }
);

#[cfg(feature = "AppKit_NSRunningApplication")]
unsafe impl NSObjectProtocol for NSRunningApplication {}

extern_methods!(
    #[cfg(feature = "AppKit_NSRunningApplication")]
    unsafe impl NSRunningApplication {
        #[cfg(not(any(target_os = "ios")))]
        #[method(isTerminated)]
        pub unsafe fn isTerminated(&self) -> bool;

        #[cfg(not(any(target_os = "ios")))]
        #[method(isFinishedLaunching)]
        pub unsafe fn isFinishedLaunching(&self) -> bool;

        #[cfg(not(any(target_os = "ios")))]
        #[method(isHidden)]
        pub unsafe fn isHidden(&self) -> bool;

        #[cfg(not(any(target_os = "ios")))]
        #[method(isActive)]
        pub unsafe fn isActive(&self) -> bool;

        #[cfg(not(any(target_os = "ios")))]
        #[method(ownsMenuBar)]
        pub unsafe fn ownsMenuBar(&self) -> bool;

        #[cfg(not(any(target_os = "ios")))]
        #[method(activationPolicy)]
        pub unsafe fn activationPolicy(&self) -> NSApplicationActivationPolicy;

        #[cfg(feature = "Foundation_NSString")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other localizedName)]
        pub unsafe fn localizedName(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other bundleIdentifier)]
        pub unsafe fn bundleIdentifier(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other bundleURL)]
        pub unsafe fn bundleURL(&self) -> Option<Id<NSURL>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other executableURL)]
        pub unsafe fn executableURL(&self) -> Option<Id<NSURL>>;

        #[cfg(feature = "Foundation_NSDate")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other launchDate)]
        pub unsafe fn launchDate(&self) -> Option<Id<NSDate>>;

        #[cfg(feature = "AppKit_NSImage")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other icon)]
        pub unsafe fn icon(&self) -> Option<Id<NSImage>>;

        #[cfg(not(any(target_os = "ios")))]
        #[method(executableArchitecture)]
        pub unsafe fn executableArchitecture(&self) -> NSInteger;

        #[cfg(not(any(target_os = "ios")))]
        #[method(hide)]
        pub unsafe fn hide(&self) -> bool;

        #[cfg(not(any(target_os = "ios")))]
        #[method(unhide)]
        pub unsafe fn unhide(&self) -> bool;

        #[cfg(not(any(target_os = "ios")))]
        #[method(activateWithOptions:)]
        pub unsafe fn activateWithOptions(&self, options: NSApplicationActivationOptions) -> bool;

        #[cfg(not(any(target_os = "ios")))]
        #[method(terminate)]
        pub unsafe fn terminate(&self) -> bool;

        #[cfg(not(any(target_os = "ios")))]
        #[method(forceTerminate)]
        pub unsafe fn forceTerminate(&self) -> bool;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other runningApplicationsWithBundleIdentifier:)]
        pub unsafe fn runningApplicationsWithBundleIdentifier(
            bundle_identifier: &NSString,
        ) -> Id<NSArray<NSRunningApplication>>;

        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other currentApplication)]
        pub unsafe fn currentApplication() -> Id<NSRunningApplication>;

        #[cfg(not(any(target_os = "ios")))]
        #[method(terminateAutomaticallyTerminableApplications)]
        pub unsafe fn terminateAutomaticallyTerminableApplications();
    }
);

extern_methods!(
    /// NSWorkspaceRunningApplications
    #[cfg(feature = "AppKit_NSWorkspace")]
    unsafe impl NSWorkspace {
        #[cfg(all(
            feature = "AppKit_NSRunningApplication",
            feature = "Foundation_NSArray"
        ))]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other runningApplications)]
        pub unsafe fn runningApplications(&self) -> Id<NSArray<NSRunningApplication>>;
    }
);
