//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSFilePromiseReceiver")]
    pub struct NSFilePromiseReceiver;

    #[cfg(feature = "AppKit_NSFilePromiseReceiver")]
    unsafe impl ClassType for NSFilePromiseReceiver {
        type Super = NSObject;
    }
);

#[cfg(feature = "AppKit_NSFilePromiseReceiver")]
unsafe impl NSObjectProtocol for NSFilePromiseReceiver {}

#[cfg(feature = "AppKit_NSFilePromiseReceiver")]
unsafe impl NSPasteboardReading for NSFilePromiseReceiver {}

extern_methods!(
    #[cfg(feature = "AppKit_NSFilePromiseReceiver")]
    unsafe impl NSFilePromiseReceiver {
        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        /**
          A view must register what types it accepts via -registerForDraggedTypes:. Use this class method to get the file promise drag types that NSFilePromiseReceiver can accept, in order to register a view to accept promised files.
        */
        #[method_id(@__retain_semantics Other readableDraggedTypes)]
        pub unsafe fn readableDraggedTypes() -> Id<NSArray<NSString>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        /**
          The UTI of the file types promised (Note: The count of fileTypes should tell you the number of promised files, however, that is not guaranteed. Historically, some legacy file promisers only list each unique fileType once and write one or more files per type.
        */
        #[method_id(@__retain_semantics Other fileTypes)]
        pub unsafe fn fileTypes(&self) -> Id<NSArray<NSString>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        /**
          The file names of the promised files that are being written to the destination location. Note: This property returns an empty array until the file promise is called in via receivePromisedFilesAtDestination.
        Note: This is an array, because legacy promises are an array of files on one pasteboard item.
        */
        #[method_id(@__retain_semantics Other fileNames)]
        pub unsafe fn fileNames(&self) -> Id<NSArray<NSString>>;

        #[cfg(all(
            feature = "Foundation_NSDictionary",
            feature = "Foundation_NSError",
            feature = "Foundation_NSOperationQueue",
            feature = "Foundation_NSURL"
        ))]
        #[method(receivePromisedFilesAtDestination:options:operationQueue:reader:)]
        pub unsafe fn receivePromisedFilesAtDestination_options_operationQueue_reader(
            &self,
            destination_dir: &NSURL,
            options: &NSDictionary,
            operation_queue: &NSOperationQueue,
            reader: &Block<(NonNull<NSURL>, *mut NSError), ()>,
        );
    }
);
