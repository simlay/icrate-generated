//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSDictionaryControllerKeyValuePair")]
    /**
      This key-value pair object allows controls to be bound, for example, to arrangedObjects.localizedKey, arrangedObjects.key, arrangedObjects.value, and arrangedObjects.explicitlyIncluded of the controller. Mutating a key-value-pair object immediately results in the corresponding change in the content dictionary of the controller.
    */
    pub struct NSDictionaryControllerKeyValuePair;

    #[cfg(feature = "AppKit_NSDictionaryControllerKeyValuePair")]
    unsafe impl ClassType for NSDictionaryControllerKeyValuePair {
        type Super = NSObject;
    }
);

#[cfg(feature = "AppKit_NSDictionaryControllerKeyValuePair")]
/**
  This key-value pair object allows controls to be bound, for example, to arrangedObjects.localizedKey, arrangedObjects.key, arrangedObjects.value, and arrangedObjects.explicitlyIncluded of the controller. Mutating a key-value-pair object immediately results in the corresponding change in the content dictionary of the controller.
*/
unsafe impl NSObjectProtocol for NSDictionaryControllerKeyValuePair {}

extern_methods!(
    /**
      This key-value pair object allows controls to be bound, for example, to arrangedObjects.localizedKey, arrangedObjects.key, arrangedObjects.value, and arrangedObjects.explicitlyIncluded of the controller. Mutating a key-value-pair object immediately results in the corresponding change in the content dictionary of the controller.
    */
    #[cfg(feature = "AppKit_NSDictionaryControllerKeyValuePair")]
    unsafe impl NSDictionaryControllerKeyValuePair {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other key)]
        pub unsafe fn key(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setKey:)]
        pub unsafe fn setKey(&self, key: Option<&NSString>);

        #[method_id(@__retain_semantics Other value)]
        pub unsafe fn value(&self) -> Option<Id<Object>>;

        #[method(setValue:)]
        pub unsafe fn setValue(&self, value: Option<&Object>);

        #[cfg(feature = "Foundation_NSString")]
        /**
          The localized key from the NSDictionaryController’s localizedKeyDictionary
        */
        #[method_id(@__retain_semantics Other localizedKey)]
        pub unsafe fn localizedKey(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        /**
          The localized key from the NSDictionaryController’s localizedKeyDictionary
        */
        #[method(setLocalizedKey:)]
        pub unsafe fn setLocalizedKey(&self, localized_key: Option<&NSString>);

        /**
          YES if the key is in the NSDictionaryController’s includedKeys, otherwise NO
        */
        #[method(isExplicitlyIncluded)]
        pub unsafe fn isExplicitlyIncluded(&self) -> bool;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSDictionaryController")]
    pub struct NSDictionaryController;

    #[cfg(feature = "AppKit_NSDictionaryController")]
    unsafe impl ClassType for NSDictionaryController {
        #[inherits(NSObjectController, NSController, NSObject)]
        type Super = NSArrayController;
    }
);

#[cfg(feature = "AppKit_NSDictionaryController")]
unsafe impl NSCoding for NSDictionaryController {}

#[cfg(feature = "AppKit_NSDictionaryController")]
unsafe impl NSEditor for NSDictionaryController {}

#[cfg(feature = "AppKit_NSDictionaryController")]
unsafe impl NSEditorRegistration for NSDictionaryController {}

#[cfg(feature = "AppKit_NSDictionaryController")]
unsafe impl NSObjectProtocol for NSDictionaryController {}

extern_methods!(
    #[cfg(feature = "AppKit_NSDictionaryController")]
    unsafe impl NSDictionaryController {
        #[cfg(feature = "AppKit_NSDictionaryControllerKeyValuePair")]
        #[method_id(@__retain_semantics New newObject)]
        pub unsafe fn newObject(&self) -> Id<NSDictionaryControllerKeyValuePair>;

        #[cfg(feature = "Foundation_NSString")]
        /**
          Initial key and value are assigned to newly inserted entries. The initial key will be copied, the initial value will be retained when inserted into the dictionary (not copied), and must implement NSCoding if the dictionary controller gets archived.
        */
        #[method_id(@__retain_semantics Other initialKey)]
        pub unsafe fn initialKey(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        /**
          Initial key and value are assigned to newly inserted entries. The initial key will be copied, the initial value will be retained when inserted into the dictionary (not copied), and must implement NSCoding if the dictionary controller gets archived.
        */
        #[method(setInitialKey:)]
        pub unsafe fn setInitialKey(&self, initial_key: &NSString);

        #[method_id(@__retain_semantics Other initialValue)]
        pub unsafe fn initialValue(&self) -> Id<Object>;

        #[method(setInitialValue:)]
        pub unsafe fn setInitialValue(&self, initial_value: &Object);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        /**
          Included keys are always represented by a key-value pair in the display array, whether present in the underlying dictionary or not. Excluded keys are always suppressed in the display array.
        */
        #[method_id(@__retain_semantics Other includedKeys)]
        pub unsafe fn includedKeys(&self) -> Id<NSArray<NSString>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        /**
          Included keys are always represented by a key-value pair in the display array, whether present in the underlying dictionary or not. Excluded keys are always suppressed in the display array.
        */
        #[method(setIncludedKeys:)]
        pub unsafe fn setIncludedKeys(&self, included_keys: &NSArray<NSString>);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other excludedKeys)]
        pub unsafe fn excludedKeys(&self) -> Id<NSArray<NSString>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method(setExcludedKeys:)]
        pub unsafe fn setExcludedKeys(&self, excluded_keys: &NSArray<NSString>);

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        /**
          Localized key dictionary allows to specify a (typically localized) string for each key in the dictionary (the dictionary needs to contain the keys as keys and the localized keys as values).
        */
        #[method_id(@__retain_semantics Other localizedKeyDictionary)]
        pub unsafe fn localizedKeyDictionary(&self) -> Id<NSDictionary<NSString, NSString>>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        /**
          Localized key dictionary allows to specify a (typically localized) string for each key in the dictionary (the dictionary needs to contain the keys as keys and the localized keys as values).
        */
        #[method(setLocalizedKeyDictionary:)]
        pub unsafe fn setLocalizedKeyDictionary(
            &self,
            localized_key_dictionary: &NSDictionary<NSString, NSString>,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other localizedKeyTable)]
        pub unsafe fn localizedKeyTable(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setLocalizedKeyTable:)]
        pub unsafe fn setLocalizedKeyTable(&self, localized_key_table: Option<&NSString>);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObjectController`
    #[cfg(feature = "AppKit_NSDictionaryController")]
    unsafe impl NSDictionaryController {
        #[method_id(@__retain_semantics Init initWithContent:)]
        pub unsafe fn initWithContent(
            this: Option<Allocated<Self>>,
            content: Option<&Object>,
        ) -> Id<Self>;
    }
);
