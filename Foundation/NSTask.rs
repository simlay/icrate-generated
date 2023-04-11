//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSTaskTerminationReason {
        NSTaskTerminationReasonExit = 1,
        NSTaskTerminationReasonUncaughtSignal = 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSTask")]
    pub struct NSTask;

    #[cfg(feature = "Foundation_NSTask")]
    unsafe impl ClassType for NSTask {
        type Super = NSObject;
    }
);

#[cfg(feature = "Foundation_NSTask")]
unsafe impl NSObjectProtocol for NSTask {}

extern_methods!(
    #[cfg(feature = "Foundation_NSTask")]
    unsafe impl NSTask {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[cfg(feature = "Foundation_NSURL")]
        /**
          these methods can only be set before a launch
        */
        #[method_id(@__retain_semantics Other executableURL)]
        pub unsafe fn executableURL(&self) -> Option<Id<NSURL>>;

        #[cfg(feature = "Foundation_NSURL")]
        /**
          these methods can only be set before a launch
        */
        #[method(setExecutableURL:)]
        pub unsafe fn setExecutableURL(&self, executable_url: Option<&NSURL>);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other arguments)]
        pub unsafe fn arguments(&self) -> Option<Id<NSArray<NSString>>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method(setArguments:)]
        pub unsafe fn setArguments(&self, arguments: Option<&NSArray<NSString>>);

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        /**
          if not set, use current
        */
        #[method_id(@__retain_semantics Other environment)]
        pub unsafe fn environment(&self) -> Option<Id<NSDictionary<NSString, NSString>>>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        /**
          if not set, use current
        */
        #[method(setEnvironment:)]
        pub unsafe fn setEnvironment(&self, environment: Option<&NSDictionary<NSString, NSString>>);

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other currentDirectoryURL)]
        pub unsafe fn currentDirectoryURL(&self) -> Option<Id<NSURL>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method(setCurrentDirectoryURL:)]
        pub unsafe fn setCurrentDirectoryURL(&self, current_directory_url: Option<&NSURL>);

        /**
          standard I/O channels; could be either an NSFileHandle or an NSPipe
        */
        #[method_id(@__retain_semantics Other standardInput)]
        pub unsafe fn standardInput(&self) -> Option<Id<Object>>;

        /**
          standard I/O channels; could be either an NSFileHandle or an NSPipe
        */
        #[method(setStandardInput:)]
        pub unsafe fn setStandardInput(&self, standard_input: Option<&Object>);

        #[method_id(@__retain_semantics Other standardOutput)]
        pub unsafe fn standardOutput(&self) -> Option<Id<Object>>;

        #[method(setStandardOutput:)]
        pub unsafe fn setStandardOutput(&self, standard_output: Option<&Object>);

        #[method_id(@__retain_semantics Other standardError)]
        pub unsafe fn standardError(&self) -> Option<Id<Object>>;

        #[method(setStandardError:)]
        pub unsafe fn setStandardError(&self, standard_error: Option<&Object>);

        #[cfg(feature = "Foundation_NSError")]
        #[method(launchAndReturnError:_)]
        pub unsafe fn launchAndReturnError(&self) -> Result<(), Id<NSError>>;

        #[method(interrupt)]
        pub unsafe fn interrupt(&self);

        #[method(terminate)]
        pub unsafe fn terminate(&self);

        #[method(suspend)]
        pub unsafe fn suspend(&self) -> bool;

        #[method(resume)]
        pub unsafe fn resume(&self) -> bool;

        /**
          status
        */
        #[method(processIdentifier)]
        pub unsafe fn processIdentifier(&self) -> c_int;

        #[method(isRunning)]
        pub unsafe fn isRunning(&self) -> bool;

        #[method(terminationStatus)]
        pub unsafe fn terminationStatus(&self) -> c_int;

        #[method(terminationReason)]
        pub unsafe fn terminationReason(&self) -> NSTaskTerminationReason;

        /**
         A block to be invoked when the process underlying the NSTask terminates.  Setting the block to nil is valid, and stops the previous block from being invoked, as long as it hasn't started in any way.  The NSTask is passed as the argument to the block so the block does not have to capture, and thus retain, it.  The block is copied when set.  Only one termination handler block can be set at any time.  The execution context in which the block is invoked is undefined.  If the NSTask has already finished, the block is executed immediately/soon (not necessarily on the current thread).  If a terminationHandler is set on an NSTask, the NSTaskDidTerminateNotification notification is not posted for that task.  Also note that -waitUntilExit won't wait until the terminationHandler has been fully executed.  You cannot use this property in a concrete subclass of NSTask which hasn't been updated to include an implementation of the storage and use of it.
        */
        #[method(terminationHandler)]
        pub unsafe fn terminationHandler(&self) -> *mut Block<(NonNull<NSTask>,), ()>;

        /**
         A block to be invoked when the process underlying the NSTask terminates.  Setting the block to nil is valid, and stops the previous block from being invoked, as long as it hasn't started in any way.  The NSTask is passed as the argument to the block so the block does not have to capture, and thus retain, it.  The block is copied when set.  Only one termination handler block can be set at any time.  The execution context in which the block is invoked is undefined.  If the NSTask has already finished, the block is executed immediately/soon (not necessarily on the current thread).  If a terminationHandler is set on an NSTask, the NSTaskDidTerminateNotification notification is not posted for that task.  Also note that -waitUntilExit won't wait until the terminationHandler has been fully executed.  You cannot use this property in a concrete subclass of NSTask which hasn't been updated to include an implementation of the storage and use of it.
        */
        #[method(setTerminationHandler:)]
        pub unsafe fn setTerminationHandler(
            &self,
            termination_handler: Option<&Block<(NonNull<NSTask>,), ()>>,
        );

        /**
          read-only after the task is launched
        */
        #[method(qualityOfService)]
        pub unsafe fn qualityOfService(&self) -> NSQualityOfService;

        /**
          read-only after the task is launched
        */
        #[method(setQualityOfService:)]
        pub unsafe fn setQualityOfService(&self, quality_of_service: NSQualityOfService);
    }
);

extern_methods!(
    /// NSTaskConveniences
    #[cfg(feature = "Foundation_NSTask")]
    unsafe impl NSTask {
        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "Foundation_NSString",
            feature = "Foundation_NSURL"
        ))]
        #[method_id(@__retain_semantics Other launchedTaskWithExecutableURL:arguments:error:terminationHandler:)]
        pub unsafe fn launchedTaskWithExecutableURL_arguments_error_terminationHandler(
            url: &NSURL,
            arguments: &NSArray<NSString>,
            error: Option<&mut Option<Id<NSError>>>,
            termination_handler: Option<&Block<(NonNull<NSTask>,), ()>>,
        ) -> Option<Id<NSTask>>;

        #[method(waitUntilExit)]
        pub unsafe fn waitUntilExit(&self);
    }
);

extern_methods!(
    /// NSDeprecated
    #[cfg(feature = "Foundation_NSTask")]
    unsafe impl NSTask {
        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method_id(@__retain_semantics Other launchPath)]
        pub unsafe fn launchPath(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[deprecated]
        #[method(setLaunchPath:)]
        pub unsafe fn setLaunchPath(&self, launch_path: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        /**
          if not set, use current
        */
        #[deprecated]
        #[method_id(@__retain_semantics Other currentDirectoryPath)]
        pub unsafe fn currentDirectoryPath(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        /**
          if not set, use current
        */
        #[deprecated]
        #[method(setCurrentDirectoryPath:)]
        pub unsafe fn setCurrentDirectoryPath(&self, current_directory_path: &NSString);

        #[deprecated]
        #[method(launch)]
        pub unsafe fn launch(&self);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[deprecated]
        #[method_id(@__retain_semantics Other launchedTaskWithLaunchPath:arguments:)]
        pub unsafe fn launchedTaskWithLaunchPath_arguments(
            path: &NSString,
            arguments: &NSArray<NSString>,
        ) -> Id<NSTask>;
    }
);

extern_static!(NSTaskDidTerminateNotification: &'static NSNotificationName);
