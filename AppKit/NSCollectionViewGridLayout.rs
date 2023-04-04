//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSCollectionViewGridLayout")]
    pub struct NSCollectionViewGridLayout;

    #[cfg(feature = "AppKit_NSCollectionViewGridLayout")]
    unsafe impl ClassType for NSCollectionViewGridLayout {
        #[inherits(NSObject)]
        type Super = NSCollectionViewLayout;
    }
);

#[cfg(feature = "AppKit_NSCollectionViewGridLayout")]
unsafe impl NSCoding for NSCollectionViewGridLayout {}

#[cfg(feature = "AppKit_NSCollectionViewGridLayout")]
unsafe impl NSObjectProtocol for NSCollectionViewGridLayout {}

extern_methods!(
    #[cfg(feature = "AppKit_NSCollectionViewGridLayout")]
    unsafe impl NSCollectionViewGridLayout {
        #[method(margins)]
        pub unsafe fn margins(&self) -> NSEdgeInsets;

        #[method(setMargins:)]
        pub unsafe fn setMargins(&self, margins: NSEdgeInsets);

        #[method(minimumInteritemSpacing)]
        pub unsafe fn minimumInteritemSpacing(&self) -> CGFloat;

        #[method(setMinimumInteritemSpacing:)]
        pub unsafe fn setMinimumInteritemSpacing(&self, minimum_interitem_spacing: CGFloat);

        #[method(minimumLineSpacing)]
        pub unsafe fn minimumLineSpacing(&self) -> CGFloat;

        #[method(setMinimumLineSpacing:)]
        pub unsafe fn setMinimumLineSpacing(&self, minimum_line_spacing: CGFloat);

        #[method(maximumNumberOfRows)]
        pub unsafe fn maximumNumberOfRows(&self) -> NSUInteger;

        #[method(setMaximumNumberOfRows:)]
        pub unsafe fn setMaximumNumberOfRows(&self, maximum_number_of_rows: NSUInteger);

        #[method(maximumNumberOfColumns)]
        pub unsafe fn maximumNumberOfColumns(&self) -> NSUInteger;

        #[method(setMaximumNumberOfColumns:)]
        pub unsafe fn setMaximumNumberOfColumns(&self, maximum_number_of_columns: NSUInteger);

        #[method(minimumItemSize)]
        pub unsafe fn minimumItemSize(&self) -> NSSize;

        #[method(setMinimumItemSize:)]
        pub unsafe fn setMinimumItemSize(&self, minimum_item_size: NSSize);

        #[method(maximumItemSize)]
        pub unsafe fn maximumItemSize(&self) -> NSSize;

        #[method(setMaximumItemSize:)]
        pub unsafe fn setMaximumItemSize(&self, maximum_item_size: NSSize);

        #[cfg(all(feature = "AppKit_NSColor", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other backgroundColors)]
        pub unsafe fn backgroundColors(&self) -> Id<NSArray<NSColor>>;

        #[cfg(all(feature = "AppKit_NSColor", feature = "Foundation_NSArray"))]
        #[method(setBackgroundColors:)]
        pub unsafe fn setBackgroundColors(&self, background_colors: Option<&NSArray<NSColor>>);
    }
);
