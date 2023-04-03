//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSBrowserCell")]
    #[cfg(not(any(target_os = "ios")))]
    pub struct NSBrowserCell;

    #[cfg(feature = "AppKit_NSBrowserCell")]
    unsafe impl ClassType for NSBrowserCell {
        #[inherits(NSObject)]
        type Super = NSCell;
    }
);

#[cfg(feature = "AppKit_NSBrowserCell")]
unsafe impl NSAccessibility for NSBrowserCell {}

#[cfg(feature = "AppKit_NSBrowserCell")]
unsafe impl NSAccessibilityElementProtocol for NSBrowserCell {}

#[cfg(feature = "AppKit_NSBrowserCell")]
unsafe impl NSCoding for NSBrowserCell {}

#[cfg(feature = "AppKit_NSBrowserCell")]
unsafe impl NSObjectProtocol for NSBrowserCell {}

#[cfg(feature = "AppKit_NSBrowserCell")]
unsafe impl NSUserInterfaceItemIdentification for NSBrowserCell {}

extern_methods!(
    #[cfg(feature = "AppKit_NSBrowserCell")]
    unsafe impl NSBrowserCell {
        #[cfg(feature = "Foundation_NSString")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Init initTextCell:)]
        pub unsafe fn initTextCell(this: Option<Allocated<Self>>, string: &NSString) -> Id<Self>;

        #[cfg(feature = "AppKit_NSImage")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Init initImageCell:)]
        pub unsafe fn initImageCell(
            this: Option<Allocated<Self>>,
            image: Option<&NSImage>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Option<Allocated<Self>>, coder: &NSCoder) -> Id<Self>;

        #[cfg(feature = "AppKit_NSImage")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other branchImage)]
        pub unsafe fn branchImage() -> Option<Id<NSImage>>;

        #[cfg(feature = "AppKit_NSImage")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other highlightedBranchImage)]
        pub unsafe fn highlightedBranchImage() -> Option<Id<NSImage>>;

        #[cfg(all(feature = "AppKit_NSColor", feature = "AppKit_NSView"))]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other highlightColorInView:)]
        pub unsafe fn highlightColorInView(&self, control_view: &NSView) -> Option<Id<NSColor>>;

        #[cfg(not(any(target_os = "ios")))]
        #[method(isLeaf)]
        pub unsafe fn isLeaf(&self) -> bool;

        #[cfg(not(any(target_os = "ios")))]
        #[method(setLeaf:)]
        pub unsafe fn setLeaf(&self, leaf: bool);

        #[cfg(not(any(target_os = "ios")))]
        #[method(isLoaded)]
        pub unsafe fn isLoaded(&self) -> bool;

        #[cfg(not(any(target_os = "ios")))]
        #[method(setLoaded:)]
        pub unsafe fn setLoaded(&self, loaded: bool);

        #[cfg(not(any(target_os = "ios")))]
        #[method(reset)]
        pub unsafe fn reset(&self);

        #[cfg(not(any(target_os = "ios")))]
        #[method(set)]
        pub unsafe fn set(&self);

        #[cfg(feature = "AppKit_NSImage")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other image)]
        pub unsafe fn image(&self) -> Option<Id<NSImage>>;

        #[cfg(feature = "AppKit_NSImage")]
        #[cfg(not(any(target_os = "ios")))]
        #[method(setImage:)]
        pub unsafe fn setImage(&self, image: Option<&NSImage>);

        #[cfg(feature = "AppKit_NSImage")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other alternateImage)]
        pub unsafe fn alternateImage(&self) -> Option<Id<NSImage>>;

        #[cfg(feature = "AppKit_NSImage")]
        #[cfg(not(any(target_os = "ios")))]
        #[method(setAlternateImage:)]
        pub unsafe fn setAlternateImage(&self, alternate_image: Option<&NSImage>);
    }
);
