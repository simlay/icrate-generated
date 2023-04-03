//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSAccessibilityCustomAction")]
    #[cfg(not(any(target_os = "ios")))]
    pub struct NSAccessibilityCustomAction;

    #[cfg(feature = "AppKit_NSAccessibilityCustomAction")]
    unsafe impl ClassType for NSAccessibilityCustomAction {
        type Super = NSObject;
    }
);

#[cfg(feature = "AppKit_NSAccessibilityCustomAction")]
unsafe impl NSObjectProtocol for NSAccessibilityCustomAction {}

extern_methods!(
    #[cfg(feature = "AppKit_NSAccessibilityCustomAction")]
    unsafe impl NSAccessibilityCustomAction {
        #[cfg(feature = "Foundation_NSString")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Init initWithName:handler:)]
        pub unsafe fn initWithName_handler(
            this: Option<Allocated<Self>>,
            name: &NSString,
            handler: Option<&Block<(), Bool>>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Init initWithName:target:selector:)]
        pub unsafe fn initWithName_target_selector(
            this: Option<Allocated<Self>>,
            name: &NSString,
            target: &NSObject,
            selector: Sel,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[cfg(not(any(target_os = "ios")))]
        #[method(setName:)]
        pub unsafe fn setName(&self, name: &NSString);

        #[cfg(not(any(target_os = "ios")))]
        #[method(handler)]
        pub unsafe fn handler(&self) -> *mut Block<(), Bool>;

        #[cfg(not(any(target_os = "ios")))]
        #[method(setHandler:)]
        pub unsafe fn setHandler(&self, handler: Option<&Block<(), Bool>>);

        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other target)]
        pub unsafe fn target(&self) -> Option<Id<NSObject>>;

        #[cfg(not(any(target_os = "ios")))]
        #[method(setTarget:)]
        pub unsafe fn setTarget(&self, target: Option<&NSObject>);

        #[cfg(not(any(target_os = "ios")))]
        #[method(selector)]
        pub unsafe fn selector(&self) -> Option<Sel>;

        #[cfg(not(any(target_os = "ios")))]
        #[method(setSelector:)]
        pub unsafe fn setSelector(&self, selector: Option<Sel>);
    }
);
