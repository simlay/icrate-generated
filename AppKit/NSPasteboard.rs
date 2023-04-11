//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

typed_extensible_enum!(
    pub type NSPasteboardType = NSString;
);

extern_static!(NSPasteboardTypeString: &'static NSPasteboardType);

extern_static!(NSPasteboardTypePDF: &'static NSPasteboardType);

extern_static!(NSPasteboardTypeTIFF: &'static NSPasteboardType);

extern_static!(NSPasteboardTypePNG: &'static NSPasteboardType);

extern_static!(NSPasteboardTypeRTF: &'static NSPasteboardType);

extern_static!(NSPasteboardTypeRTFD: &'static NSPasteboardType);

extern_static!(NSPasteboardTypeHTML: &'static NSPasteboardType);

extern_static!(NSPasteboardTypeTabularText: &'static NSPasteboardType);

extern_static!(NSPasteboardTypeFont: &'static NSPasteboardType);

extern_static!(NSPasteboardTypeRuler: &'static NSPasteboardType);

extern_static!(NSPasteboardTypeColor: &'static NSPasteboardType);

extern_static!(NSPasteboardTypeSound: &'static NSPasteboardType);

extern_static!(NSPasteboardTypeMultipleTextSelection: &'static NSPasteboardType);

extern_static!(NSPasteboardTypeTextFinderOptions: &'static NSPasteboardType);

extern_static!(NSPasteboardTypeURL: &'static NSPasteboardType);

extern_static!(NSPasteboardTypeFileURL: &'static NSPasteboardType);

typed_extensible_enum!(
    pub type NSPasteboardName = NSString;
);

extern_static!(NSPasteboardNameGeneral: &'static NSPasteboardName);

extern_static!(NSPasteboardNameFont: &'static NSPasteboardName);

extern_static!(NSPasteboardNameRuler: &'static NSPasteboardName);

extern_static!(NSPasteboardNameFind: &'static NSPasteboardName);

extern_static!(NSPasteboardNameDrag: &'static NSPasteboardName);

ns_options!(
    #[underlying(NSUInteger)]
    /**
      Options for prepareForNewContentsWithOptions:
    */
    pub enum NSPasteboardContentsOptions {
        NSPasteboardContentsCurrentHostOnly = 1 << 0,
    }
);

typed_enum!(
    pub type NSPasteboardReadingOptionKey = NSString;
);

extern_static!(NSPasteboardURLReadingFileURLsOnlyKey: &'static NSPasteboardReadingOptionKey);

extern_static!(
    NSPasteboardURLReadingContentsConformToTypesKey: &'static NSPasteboardReadingOptionKey
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSPasteboard")]
    pub struct NSPasteboard;

    #[cfg(feature = "AppKit_NSPasteboard")]
    unsafe impl ClassType for NSPasteboard {
        type Super = NSObject;
    }
);

#[cfg(feature = "AppKit_NSPasteboard")]
unsafe impl NSObjectProtocol for NSPasteboard {}

extern_methods!(
    #[cfg(feature = "AppKit_NSPasteboard")]
    unsafe impl NSPasteboard {
        #[method_id(@__retain_semantics Other generalPasteboard)]
        pub unsafe fn generalPasteboard() -> Id<NSPasteboard>;

        #[method_id(@__retain_semantics Other pasteboardWithName:)]
        pub unsafe fn pasteboardWithName(name: &NSPasteboardName) -> Id<NSPasteboard>;

        #[method_id(@__retain_semantics Other pasteboardWithUniqueName)]
        pub unsafe fn pasteboardWithUniqueName() -> Id<NSPasteboard>;

        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Id<NSPasteboardName>;

        #[method(changeCount)]
        pub unsafe fn changeCount(&self) -> NSInteger;

        #[method(prepareForNewContentsWithOptions:)]
        pub unsafe fn prepareForNewContentsWithOptions(
            &self,
            options: NSPasteboardContentsOptions,
        ) -> NSInteger;

        #[method(clearContents)]
        pub unsafe fn clearContents(&self) -> NSInteger;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(writeObjects:)]
        pub unsafe fn writeObjects(
            &self,
            objects: &NSArray<ProtocolObject<dyn NSPasteboardWriting>>,
        ) -> bool;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSDictionary"))]
        #[method_id(@__retain_semantics Other readObjectsForClasses:options:)]
        pub unsafe fn readObjectsForClasses_options(
            &self,
            class_array: &NSArray<TodoClass>,
            options: Option<&NSDictionary<NSPasteboardReadingOptionKey, Object>>,
        ) -> Option<Id<NSArray>>;

        #[cfg(all(feature = "AppKit_NSPasteboardItem", feature = "Foundation_NSArray"))]
        /**
          Returns all pasteboard items.  Returns nil if there is an error retrieving pasteboard items.
        */
        #[method_id(@__retain_semantics Other pasteboardItems)]
        pub unsafe fn pasteboardItems(&self) -> Option<Id<NSArray<NSPasteboardItem>>>;

        #[cfg(feature = "AppKit_NSPasteboardItem")]
        #[method(indexOfPasteboardItem:)]
        pub unsafe fn indexOfPasteboardItem(
            &self,
            pasteboard_item: &NSPasteboardItem,
        ) -> NSUInteger;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method(canReadItemWithDataConformingToTypes:)]
        pub unsafe fn canReadItemWithDataConformingToTypes(
            &self,
            types: &NSArray<NSString>,
        ) -> bool;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSDictionary"))]
        #[method(canReadObjectForClasses:options:)]
        pub unsafe fn canReadObjectForClasses_options(
            &self,
            class_array: &NSArray<TodoClass>,
            options: Option<&NSDictionary<NSPasteboardReadingOptionKey, Object>>,
        ) -> bool;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(declareTypes:owner:)]
        pub unsafe fn declareTypes_owner(
            &self,
            new_types: &NSArray<NSPasteboardType>,
            new_owner: Option<&Object>,
        ) -> NSInteger;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(addTypes:owner:)]
        pub unsafe fn addTypes_owner(
            &self,
            new_types: &NSArray<NSPasteboardType>,
            new_owner: Option<&Object>,
        ) -> NSInteger;

        #[cfg(feature = "Foundation_NSArray")]
        /**
          These methods provide information about the types available from the entire pasteboard.
        */
        #[method_id(@__retain_semantics Other types)]
        pub unsafe fn types(&self) -> Option<Id<NSArray<NSPasteboardType>>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other availableTypeFromArray:)]
        pub unsafe fn availableTypeFromArray(
            &self,
            types: &NSArray<NSPasteboardType>,
        ) -> Option<Id<NSPasteboardType>>;

        #[cfg(feature = "Foundation_NSData")]
        #[method(setData:forType:)]
        pub unsafe fn setData_forType(
            &self,
            data: Option<&NSData>,
            data_type: &NSPasteboardType,
        ) -> bool;

        #[method(setPropertyList:forType:)]
        pub unsafe fn setPropertyList_forType(
            &self,
            plist: &Object,
            data_type: &NSPasteboardType,
        ) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setString:forType:)]
        pub unsafe fn setString_forType(
            &self,
            string: &NSString,
            data_type: &NSPasteboardType,
        ) -> bool;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other dataForType:)]
        pub unsafe fn dataForType(&self, data_type: &NSPasteboardType) -> Option<Id<NSData>>;

        #[method_id(@__retain_semantics Other propertyListForType:)]
        pub unsafe fn propertyListForType(
            &self,
            data_type: &NSPasteboardType,
        ) -> Option<Id<Object>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other stringForType:)]
        pub unsafe fn stringForType(&self, data_type: &NSPasteboardType) -> Option<Id<NSString>>;
    }
);

extern_methods!(
    /// FilterServices
    #[cfg(feature = "AppKit_NSPasteboard")]
    unsafe impl NSPasteboard {
        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other typesFilterableTo:)]
        pub unsafe fn typesFilterableTo(r#type: &NSPasteboardType)
            -> Id<NSArray<NSPasteboardType>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other pasteboardByFilteringFile:)]
        pub unsafe fn pasteboardByFilteringFile(filename: &NSString) -> Id<NSPasteboard>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other pasteboardByFilteringData:ofType:)]
        pub unsafe fn pasteboardByFilteringData_ofType(
            data: &NSData,
            r#type: &NSPasteboardType,
        ) -> Id<NSPasteboard>;

        #[method_id(@__retain_semantics Other pasteboardByFilteringTypesInPasteboard:)]
        pub unsafe fn pasteboardByFilteringTypesInPasteboard(
            pboard: &NSPasteboard,
        ) -> Id<NSPasteboard>;
    }
);

extern_protocol!(
    pub unsafe trait NSPasteboardTypeOwner: NSObjectProtocol {
        #[cfg(feature = "AppKit_NSPasteboard")]
        #[method(pasteboard:provideDataForType:)]
        unsafe fn pasteboard_provideDataForType(
            &self,
            sender: &NSPasteboard,
            r#type: &NSPasteboardType,
        );

        #[cfg(feature = "AppKit_NSPasteboard")]
        #[optional]
        #[method(pasteboardChangedOwner:)]
        unsafe fn pasteboardChangedOwner(&self, sender: &NSPasteboard);
    }

    unsafe impl ProtocolType for dyn NSPasteboardTypeOwner {}
);

ns_options!(
    #[underlying(NSUInteger)]
    /**
      The NSPasteboardWriting protocol enables instances of a class to be used with the -writeObjects: method of NSPasteboard.  The Cocoa framework classes NSString, NSAttributedString, NSURL, NSColor, NSSound, NSImage, and NSPasteboardItem implement this protocol.  The protocol can also be implemented by custom application classes for use with -writeObjects:
    */
    pub enum NSPasteboardWritingOptions {
        NSPasteboardWritingPromised = 1 << 9,
    }
);

extern_protocol!(
    pub unsafe trait NSPasteboardWriting: NSObjectProtocol {
        #[cfg(all(feature = "AppKit_NSPasteboard", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other writableTypesForPasteboard:)]
        unsafe fn writableTypesForPasteboard(
            &self,
            pasteboard: &NSPasteboard,
        ) -> Id<NSArray<NSPasteboardType>>;

        #[cfg(feature = "AppKit_NSPasteboard")]
        #[optional]
        #[method(writingOptionsForType:pasteboard:)]
        unsafe fn writingOptionsForType_pasteboard(
            &self,
            r#type: &NSPasteboardType,
            pasteboard: &NSPasteboard,
        ) -> NSPasteboardWritingOptions;

        #[method_id(@__retain_semantics Other pasteboardPropertyListForType:)]
        unsafe fn pasteboardPropertyListForType(
            &self,
            r#type: &NSPasteboardType,
        ) -> Option<Id<Object>>;
    }

    unsafe impl ProtocolType for dyn NSPasteboardWriting {}
);

ns_options!(
    #[underlying(NSUInteger)]
    /**
       NSPasteboardReadingOptions specify how data is read from the pasteboard.  You can specify only one option from this list.  If you do not specify an option, the default NSPasteboardReadingAsData is used.  The first three options specify how and if pasteboard data should be pre-processed by the pasteboard before being passed to -initWithPasteboardPropertyList:ofType.  The fourth option, NSPasteboardReadingAsKeyedArchive, should be used when the data on the pasteboard is a keyed archive of this class.  Using this option, a keyed unarchiver will be used and -initWithCoder: will be called to initialize the new instance.
    */
    pub enum NSPasteboardReadingOptions {
        NSPasteboardReadingAsData = 0,
        NSPasteboardReadingAsString = 1 << 0,
        NSPasteboardReadingAsPropertyList = 1 << 1,
        NSPasteboardReadingAsKeyedArchive = 1 << 2,
    }
);

extern_protocol!(
    pub unsafe trait NSPasteboardReading: NSObjectProtocol {
        #[cfg(all(feature = "AppKit_NSPasteboard", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other readableTypesForPasteboard:)]
        unsafe fn readableTypesForPasteboard(
            pasteboard: &NSPasteboard,
        ) -> Id<NSArray<NSPasteboardType>>;

        #[cfg(feature = "AppKit_NSPasteboard")]
        #[optional]
        #[method(readingOptionsForType:pasteboard:)]
        unsafe fn readingOptionsForType_pasteboard(
            r#type: &NSPasteboardType,
            pasteboard: &NSPasteboard,
        ) -> NSPasteboardReadingOptions;

        #[optional]
        #[method_id(@__retain_semantics Init initWithPasteboardPropertyList:ofType:)]
        unsafe fn initWithPasteboardPropertyList_ofType(
            this: Option<Allocated<Self>>,
            property_list: &Object,
            r#type: &NSPasteboardType,
        ) -> Option<Id<Self>>;
    }

    unsafe impl ProtocolType for dyn NSPasteboardReading {}
);

extern_methods!(
    /**
      The recommended approach for writing URLs to the pasteboard is as follows:

    NSArray *arrayOfURLs; // assume this exists
    NSPasteboard *pasteboard = [NSPasteboard generalPasteboard]; // get pasteboard
    [pasteboard clearContents]; // clear pasteboard to take ownership
    [pasteboard writeObjects:arrayOfURLs]; // write the URLs

    The recommended approach for reading URLs is as follows:

    NSPasteboard *pasteboard = [NSPasteboard generalPasteboard]; // get pasteboard
    NSArray *classArray = [NSArray arrayWithObject:[NSURL class]]; // types of objects you are looking for
    NSArray *arrayOfURLs = [pasteboard readObjectsForClasses:classArray options:nil]; // read objects of those classes

    To read only file URLs, use the NSPasteboardURLReadingFileURLsOnlyKey option in a dictionary provided to -readObjectsForClasses:options:.
    NSDictionary *options = [NSDictionary dictionaryWithObject:[NSNumber numberWithBool:YES] forKey:NSPasteboardURLReadingFileURLsOnlyKey];

    To read only URLs with particular content types, use the NSPasteboardURLReadingContentsConformToTypesKey option in a dictionary provided to -readObjectsForClasses:options:.  In the sample below, only URLs whose content types are images will be returned.
    NSDictionary *options = [NSDictionary dictionaryWithObject:[NSImage imageTypes] forKey:NSPasteboardURLReadingContentsConformToTypesKey];

    To read only file URLs with particular content types, combine the two options.
    */
    /// NSPasteboardSupport
    #[cfg(feature = "Foundation_NSURL")]
    unsafe impl NSURL {
        #[cfg(feature = "AppKit_NSPasteboard")]
        #[method_id(@__retain_semantics Other URLFromPasteboard:)]
        pub unsafe fn URLFromPasteboard(paste_board: &NSPasteboard) -> Option<Id<NSURL>>;

        #[cfg(feature = "AppKit_NSPasteboard")]
        #[method(writeToPasteboard:)]
        pub unsafe fn writeToPasteboard(&self, paste_board: &NSPasteboard);
    }
);

#[cfg(feature = "Foundation_NSURL")]
/**
  The recommended approach for writing URLs to the pasteboard is as follows:

NSArray *arrayOfURLs; // assume this exists
NSPasteboard *pasteboard = [NSPasteboard generalPasteboard]; // get pasteboard
[pasteboard clearContents]; // clear pasteboard to take ownership
[pasteboard writeObjects:arrayOfURLs]; // write the URLs

The recommended approach for reading URLs is as follows:

NSPasteboard *pasteboard = [NSPasteboard generalPasteboard]; // get pasteboard
NSArray *classArray = [NSArray arrayWithObject:[NSURL class]]; // types of objects you are looking for
NSArray *arrayOfURLs = [pasteboard readObjectsForClasses:classArray options:nil]; // read objects of those classes

To read only file URLs, use the NSPasteboardURLReadingFileURLsOnlyKey option in a dictionary provided to -readObjectsForClasses:options:.
NSDictionary *options = [NSDictionary dictionaryWithObject:[NSNumber numberWithBool:YES] forKey:NSPasteboardURLReadingFileURLsOnlyKey];

To read only URLs with particular content types, use the NSPasteboardURLReadingContentsConformToTypesKey option in a dictionary provided to -readObjectsForClasses:options:.  In the sample below, only URLs whose content types are images will be returned.
NSDictionary *options = [NSDictionary dictionaryWithObject:[NSImage imageTypes] forKey:NSPasteboardURLReadingContentsConformToTypesKey];

To read only file URLs with particular content types, combine the two options.
*/
unsafe impl NSPasteboardReading for NSURL {}

#[cfg(feature = "Foundation_NSURL")]
/**
  The recommended approach for writing URLs to the pasteboard is as follows:

NSArray *arrayOfURLs; // assume this exists
NSPasteboard *pasteboard = [NSPasteboard generalPasteboard]; // get pasteboard
[pasteboard clearContents]; // clear pasteboard to take ownership
[pasteboard writeObjects:arrayOfURLs]; // write the URLs

The recommended approach for reading URLs is as follows:

NSPasteboard *pasteboard = [NSPasteboard generalPasteboard]; // get pasteboard
NSArray *classArray = [NSArray arrayWithObject:[NSURL class]]; // types of objects you are looking for
NSArray *arrayOfURLs = [pasteboard readObjectsForClasses:classArray options:nil]; // read objects of those classes

To read only file URLs, use the NSPasteboardURLReadingFileURLsOnlyKey option in a dictionary provided to -readObjectsForClasses:options:.
NSDictionary *options = [NSDictionary dictionaryWithObject:[NSNumber numberWithBool:YES] forKey:NSPasteboardURLReadingFileURLsOnlyKey];

To read only URLs with particular content types, use the NSPasteboardURLReadingContentsConformToTypesKey option in a dictionary provided to -readObjectsForClasses:options:.  In the sample below, only URLs whose content types are images will be returned.
NSDictionary *options = [NSDictionary dictionaryWithObject:[NSImage imageTypes] forKey:NSPasteboardURLReadingContentsConformToTypesKey];

To read only file URLs with particular content types, combine the two options.
*/
unsafe impl NSPasteboardWriting for NSURL {}

extern_methods!(
    /**
      NSString Pasteboard Support
    */
    /// NSPasteboardSupport
    #[cfg(feature = "Foundation_NSString")]
    unsafe impl NSString {}
);

#[cfg(feature = "Foundation_NSString")]
/**
  NSString Pasteboard Support
*/
unsafe impl NSPasteboardReading for NSString {}

#[cfg(feature = "Foundation_NSString")]
/**
  NSString Pasteboard Support
*/
unsafe impl NSPasteboardWriting for NSString {}

extern_methods!(
    /**
      The file contents pboard type allowed you to synthesize a pboard type for a file's contents based on the file's extension.  Using the UTI of a file to represent its contents now replaces this functionality.
    */
    /// NSFileContents
    #[cfg(feature = "AppKit_NSPasteboard")]
    unsafe impl NSPasteboard {
        #[cfg(feature = "Foundation_NSString")]
        #[method(writeFileContents:)]
        pub unsafe fn writeFileContents(&self, filename: &NSString) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other readFileContentsType:toFile:)]
        pub unsafe fn readFileContentsType_toFile(
            &self,
            r#type: Option<&NSPasteboardType>,
            filename: &NSString,
        ) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSFileWrapper")]
        #[method(writeFileWrapper:)]
        pub unsafe fn writeFileWrapper(&self, wrapper: &NSFileWrapper) -> bool;

        #[cfg(feature = "Foundation_NSFileWrapper")]
        #[method_id(@__retain_semantics Other readFileWrapper)]
        pub unsafe fn readFileWrapper(&self) -> Option<Id<NSFileWrapper>>;
    }
);

extern_static!(NSFileContentsPboardType: &'static NSPasteboardType);

extern_fn!(
    #[cfg(feature = "Foundation_NSString")]
    pub unsafe fn NSCreateFilenamePboardType(file_type: &NSString) -> *mut NSPasteboardType;
);

extern_fn!(
    #[cfg(feature = "Foundation_NSString")]
    pub unsafe fn NSCreateFileContentsPboardType(file_type: &NSString) -> *mut NSPasteboardType;
);

extern_fn!(
    #[cfg(feature = "Foundation_NSString")]
    pub unsafe fn NSGetFileType(pboard_type: &NSPasteboardType) -> *mut NSString;
);

extern_fn!(
    #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
    pub unsafe fn NSGetFileTypes(
        pboard_types: &NSArray<NSPasteboardType>,
    ) -> *mut NSArray<NSString>;
);

extern_static!(NSStringPboardType: &'static NSPasteboardType);

extern_static!(NSFilenamesPboardType: &'static NSPasteboardType);

extern_static!(NSTIFFPboardType: &'static NSPasteboardType);

extern_static!(NSRTFPboardType: &'static NSPasteboardType);

extern_static!(NSTabularTextPboardType: &'static NSPasteboardType);

extern_static!(NSFontPboardType: &'static NSPasteboardType);

extern_static!(NSRulerPboardType: &'static NSPasteboardType);

extern_static!(NSColorPboardType: &'static NSPasteboardType);

extern_static!(NSRTFDPboardType: &'static NSPasteboardType);

extern_static!(NSHTMLPboardType: &'static NSPasteboardType);

extern_static!(NSURLPboardType: &'static NSPasteboardType);

extern_static!(NSPDFPboardType: &'static NSPasteboardType);

extern_static!(NSMultipleTextSelectionPboardType: &'static NSPasteboardType);

extern_static!(NSPostScriptPboardType: &'static NSPasteboardType);

extern_static!(NSVCardPboardType: &'static NSPasteboardType);

extern_static!(NSInkTextPboardType: &'static NSPasteboardType);

extern_static!(NSFilesPromisePboardType: &'static NSPasteboardType);

extern_static!(NSPasteboardTypeFindPanelSearchOptions: &'static NSPasteboardType);

extern_static!(NSGeneralPboard: &'static NSPasteboardName);

extern_static!(NSFontPboard: &'static NSPasteboardName);

extern_static!(NSRulerPboard: &'static NSPasteboardName);

extern_static!(NSFindPboard: &'static NSPasteboardName);

extern_static!(NSDragPboard: &'static NSPasteboardName);

extern_static!(NSPICTPboardType: &'static NSPasteboardType);
