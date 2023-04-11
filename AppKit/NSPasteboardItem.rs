//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSPasteboardItem")]
    /**
      An NSPasteboard can contain multiple items.  Any object that implements the NSPasteboardWriting and NSPasteboardReading protocols can be written and read on the pasteboard directly.  This allows common pasteboard classes such as URLs, colors, images, strings, attributed strings, and sounds to be written and read without an intermediary object.  The custom classes of an application can also implement these protocols for use with the pasteboard.

    Sometimes, however, an application needs more fine-grained access to the types and data of a particular pasteboard item, or in the case of a delegate or subclass, needs a way to inspect and change what has already been put on the pasteboard.  In these cases, it is appropriate to use pasteboard items.

    There are three main uses for an NSPasteboardItem:
    1. Providing data on the pasteboard: create one or more pasteboard items, set data or data providers for types, and write to the pasteboard
    2. Customizing data already on the pasteboard: as a delegate or subclass, retrieve the pasteboard items currently on the pasteboard, Read the existing types and data and set new data and data providers for types as needed.
    3. Retrieving data from the pasteboard: Retrieve the pasteboard items from the pasteboard.  Read the data for types.

    A pasteboard item can be associated with a single pasteboard.  When you create a pasteboard item, it can be written to any pasteboard.  When you pass in a pasteboard item to -writeObjects:, that pasteboard item becomes bound to the pasteboard it was written to.  When you retrieve pasteboard items using -pasteboardItems or -readObjectsForClasses:options:, the returned pasteboard items are associated with the messaged pasteboard.  Passing a pasteboard item that is aready associated with a pasteboard into -writeObjects: causes an exception to be raised.

    Pasteboard items are intended to be used during a single pasteboard interaction, not held onto and used repeatedly.  A pasteboard item is only valid until the owner of the pasteboard changes.  If a pasteboard item is stale because the pasteboard owner has changed, it will return nil or NO values from its methods.

    The pasteboard item API is very similar to the NSPasteboard API for a single item.  One important difference is that NSPasteboardItem expects strings which are valid UTI strings.  If a type is specified that is not a valid UTI string, the method call will fail.  Similarly, all reported types are UTIs.  As of 10.6, NSPasteboard.h declares a number of NSPasteboardType constants which can be used to provide the correct UTI for common pasteboard types.
    */
    pub struct NSPasteboardItem;

    #[cfg(feature = "AppKit_NSPasteboardItem")]
    unsafe impl ClassType for NSPasteboardItem {
        type Super = NSObject;
    }
);

#[cfg(feature = "AppKit_NSPasteboardItem")]
/**
  An NSPasteboard can contain multiple items.  Any object that implements the NSPasteboardWriting and NSPasteboardReading protocols can be written and read on the pasteboard directly.  This allows common pasteboard classes such as URLs, colors, images, strings, attributed strings, and sounds to be written and read without an intermediary object.  The custom classes of an application can also implement these protocols for use with the pasteboard.

Sometimes, however, an application needs more fine-grained access to the types and data of a particular pasteboard item, or in the case of a delegate or subclass, needs a way to inspect and change what has already been put on the pasteboard.  In these cases, it is appropriate to use pasteboard items.

There are three main uses for an NSPasteboardItem:
1. Providing data on the pasteboard: create one or more pasteboard items, set data or data providers for types, and write to the pasteboard
2. Customizing data already on the pasteboard: as a delegate or subclass, retrieve the pasteboard items currently on the pasteboard, Read the existing types and data and set new data and data providers for types as needed.
3. Retrieving data from the pasteboard: Retrieve the pasteboard items from the pasteboard.  Read the data for types.

A pasteboard item can be associated with a single pasteboard.  When you create a pasteboard item, it can be written to any pasteboard.  When you pass in a pasteboard item to -writeObjects:, that pasteboard item becomes bound to the pasteboard it was written to.  When you retrieve pasteboard items using -pasteboardItems or -readObjectsForClasses:options:, the returned pasteboard items are associated with the messaged pasteboard.  Passing a pasteboard item that is aready associated with a pasteboard into -writeObjects: causes an exception to be raised.

Pasteboard items are intended to be used during a single pasteboard interaction, not held onto and used repeatedly.  A pasteboard item is only valid until the owner of the pasteboard changes.  If a pasteboard item is stale because the pasteboard owner has changed, it will return nil or NO values from its methods.

The pasteboard item API is very similar to the NSPasteboard API for a single item.  One important difference is that NSPasteboardItem expects strings which are valid UTI strings.  If a type is specified that is not a valid UTI string, the method call will fail.  Similarly, all reported types are UTIs.  As of 10.6, NSPasteboard.h declares a number of NSPasteboardType constants which can be used to provide the correct UTI for common pasteboard types.
*/
unsafe impl NSObjectProtocol for NSPasteboardItem {}

#[cfg(feature = "AppKit_NSPasteboardItem")]
/**
  An NSPasteboard can contain multiple items.  Any object that implements the NSPasteboardWriting and NSPasteboardReading protocols can be written and read on the pasteboard directly.  This allows common pasteboard classes such as URLs, colors, images, strings, attributed strings, and sounds to be written and read without an intermediary object.  The custom classes of an application can also implement these protocols for use with the pasteboard.

Sometimes, however, an application needs more fine-grained access to the types and data of a particular pasteboard item, or in the case of a delegate or subclass, needs a way to inspect and change what has already been put on the pasteboard.  In these cases, it is appropriate to use pasteboard items.

There are three main uses for an NSPasteboardItem:
1. Providing data on the pasteboard: create one or more pasteboard items, set data or data providers for types, and write to the pasteboard
2. Customizing data already on the pasteboard: as a delegate or subclass, retrieve the pasteboard items currently on the pasteboard, Read the existing types and data and set new data and data providers for types as needed.
3. Retrieving data from the pasteboard: Retrieve the pasteboard items from the pasteboard.  Read the data for types.

A pasteboard item can be associated with a single pasteboard.  When you create a pasteboard item, it can be written to any pasteboard.  When you pass in a pasteboard item to -writeObjects:, that pasteboard item becomes bound to the pasteboard it was written to.  When you retrieve pasteboard items using -pasteboardItems or -readObjectsForClasses:options:, the returned pasteboard items are associated with the messaged pasteboard.  Passing a pasteboard item that is aready associated with a pasteboard into -writeObjects: causes an exception to be raised.

Pasteboard items are intended to be used during a single pasteboard interaction, not held onto and used repeatedly.  A pasteboard item is only valid until the owner of the pasteboard changes.  If a pasteboard item is stale because the pasteboard owner has changed, it will return nil or NO values from its methods.

The pasteboard item API is very similar to the NSPasteboard API for a single item.  One important difference is that NSPasteboardItem expects strings which are valid UTI strings.  If a type is specified that is not a valid UTI string, the method call will fail.  Similarly, all reported types are UTIs.  As of 10.6, NSPasteboard.h declares a number of NSPasteboardType constants which can be used to provide the correct UTI for common pasteboard types.
*/
unsafe impl NSPasteboardReading for NSPasteboardItem {}

#[cfg(feature = "AppKit_NSPasteboardItem")]
/**
  An NSPasteboard can contain multiple items.  Any object that implements the NSPasteboardWriting and NSPasteboardReading protocols can be written and read on the pasteboard directly.  This allows common pasteboard classes such as URLs, colors, images, strings, attributed strings, and sounds to be written and read without an intermediary object.  The custom classes of an application can also implement these protocols for use with the pasteboard.

Sometimes, however, an application needs more fine-grained access to the types and data of a particular pasteboard item, or in the case of a delegate or subclass, needs a way to inspect and change what has already been put on the pasteboard.  In these cases, it is appropriate to use pasteboard items.

There are three main uses for an NSPasteboardItem:
1. Providing data on the pasteboard: create one or more pasteboard items, set data or data providers for types, and write to the pasteboard
2. Customizing data already on the pasteboard: as a delegate or subclass, retrieve the pasteboard items currently on the pasteboard, Read the existing types and data and set new data and data providers for types as needed.
3. Retrieving data from the pasteboard: Retrieve the pasteboard items from the pasteboard.  Read the data for types.

A pasteboard item can be associated with a single pasteboard.  When you create a pasteboard item, it can be written to any pasteboard.  When you pass in a pasteboard item to -writeObjects:, that pasteboard item becomes bound to the pasteboard it was written to.  When you retrieve pasteboard items using -pasteboardItems or -readObjectsForClasses:options:, the returned pasteboard items are associated with the messaged pasteboard.  Passing a pasteboard item that is aready associated with a pasteboard into -writeObjects: causes an exception to be raised.

Pasteboard items are intended to be used during a single pasteboard interaction, not held onto and used repeatedly.  A pasteboard item is only valid until the owner of the pasteboard changes.  If a pasteboard item is stale because the pasteboard owner has changed, it will return nil or NO values from its methods.

The pasteboard item API is very similar to the NSPasteboard API for a single item.  One important difference is that NSPasteboardItem expects strings which are valid UTI strings.  If a type is specified that is not a valid UTI string, the method call will fail.  Similarly, all reported types are UTIs.  As of 10.6, NSPasteboard.h declares a number of NSPasteboardType constants which can be used to provide the correct UTI for common pasteboard types.
*/
unsafe impl NSPasteboardWriting for NSPasteboardItem {}

extern_methods!(
    /**
      An NSPasteboard can contain multiple items.  Any object that implements the NSPasteboardWriting and NSPasteboardReading protocols can be written and read on the pasteboard directly.  This allows common pasteboard classes such as URLs, colors, images, strings, attributed strings, and sounds to be written and read without an intermediary object.  The custom classes of an application can also implement these protocols for use with the pasteboard.

    Sometimes, however, an application needs more fine-grained access to the types and data of a particular pasteboard item, or in the case of a delegate or subclass, needs a way to inspect and change what has already been put on the pasteboard.  In these cases, it is appropriate to use pasteboard items.

    There are three main uses for an NSPasteboardItem:
    1. Providing data on the pasteboard: create one or more pasteboard items, set data or data providers for types, and write to the pasteboard
    2. Customizing data already on the pasteboard: as a delegate or subclass, retrieve the pasteboard items currently on the pasteboard, Read the existing types and data and set new data and data providers for types as needed.
    3. Retrieving data from the pasteboard: Retrieve the pasteboard items from the pasteboard.  Read the data for types.

    A pasteboard item can be associated with a single pasteboard.  When you create a pasteboard item, it can be written to any pasteboard.  When you pass in a pasteboard item to -writeObjects:, that pasteboard item becomes bound to the pasteboard it was written to.  When you retrieve pasteboard items using -pasteboardItems or -readObjectsForClasses:options:, the returned pasteboard items are associated with the messaged pasteboard.  Passing a pasteboard item that is aready associated with a pasteboard into -writeObjects: causes an exception to be raised.

    Pasteboard items are intended to be used during a single pasteboard interaction, not held onto and used repeatedly.  A pasteboard item is only valid until the owner of the pasteboard changes.  If a pasteboard item is stale because the pasteboard owner has changed, it will return nil or NO values from its methods.

    The pasteboard item API is very similar to the NSPasteboard API for a single item.  One important difference is that NSPasteboardItem expects strings which are valid UTI strings.  If a type is specified that is not a valid UTI string, the method call will fail.  Similarly, all reported types are UTIs.  As of 10.6, NSPasteboard.h declares a number of NSPasteboardType constants which can be used to provide the correct UTI for common pasteboard types.
    */
    #[cfg(feature = "AppKit_NSPasteboardItem")]
    unsafe impl NSPasteboardItem {
        #[cfg(feature = "Foundation_NSArray")]
        /**
          Returns an array of UTI strings of the data types supported by the receiver.
        */
        #[method_id(@__retain_semantics Other types)]
        pub unsafe fn types(&self) -> Id<NSArray<NSPasteboardType>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other availableTypeFromArray:)]
        pub unsafe fn availableTypeFromArray(
            &self,
            types: &NSArray<NSPasteboardType>,
        ) -> Option<Id<NSPasteboardType>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method(setDataProvider:forTypes:)]
        pub unsafe fn setDataProvider_forTypes(
            &self,
            data_provider: &ProtocolObject<dyn NSPasteboardItemDataProvider>,
            types: &NSArray<NSPasteboardType>,
        ) -> bool;

        #[cfg(feature = "Foundation_NSData")]
        #[method(setData:forType:)]
        pub unsafe fn setData_forType(&self, data: &NSData, r#type: &NSPasteboardType) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setString:forType:)]
        pub unsafe fn setString_forType(
            &self,
            string: &NSString,
            r#type: &NSPasteboardType,
        ) -> bool;

        #[method(setPropertyList:forType:)]
        pub unsafe fn setPropertyList_forType(
            &self,
            property_list: &Object,
            r#type: &NSPasteboardType,
        ) -> bool;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other dataForType:)]
        pub unsafe fn dataForType(&self, r#type: &NSPasteboardType) -> Option<Id<NSData>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other stringForType:)]
        pub unsafe fn stringForType(&self, r#type: &NSPasteboardType) -> Option<Id<NSString>>;

        #[method_id(@__retain_semantics Other propertyListForType:)]
        pub unsafe fn propertyListForType(&self, r#type: &NSPasteboardType) -> Option<Id<Object>>;
    }
);

extern_protocol!(
    pub unsafe trait NSPasteboardItemDataProvider: NSObjectProtocol {
        #[cfg(all(feature = "AppKit_NSPasteboard", feature = "AppKit_NSPasteboardItem"))]
        #[method(pasteboard:item:provideDataForType:)]
        unsafe fn pasteboard_item_provideDataForType(
            &self,
            pasteboard: Option<&NSPasteboard>,
            item: &NSPasteboardItem,
            r#type: &NSPasteboardType,
        );

        #[cfg(feature = "AppKit_NSPasteboard")]
        #[optional]
        #[method(pasteboardFinishedWithDataProvider:)]
        unsafe fn pasteboardFinishedWithDataProvider(&self, pasteboard: &NSPasteboard);
    }

    unsafe impl ProtocolType for dyn NSPasteboardItemDataProvider {}
);
