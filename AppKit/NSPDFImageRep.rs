//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSPDFImageRep")]
    pub struct NSPDFImageRep;

    #[cfg(feature = "AppKit_NSPDFImageRep")]
    unsafe impl ClassType for NSPDFImageRep {
        #[inherits(NSObject)]
        type Super = NSImageRep;
    }
);

#[cfg(feature = "AppKit_NSPDFImageRep")]
unsafe impl NSCoding for NSPDFImageRep {}

#[cfg(feature = "AppKit_NSPDFImageRep")]
unsafe impl NSObjectProtocol for NSPDFImageRep {}

extern_methods!(
    #[cfg(feature = "AppKit_NSPDFImageRep")]
    unsafe impl NSPDFImageRep {
        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other imageRepWithData:)]
        pub unsafe fn imageRepWithData(pdf_data: &NSData) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Init initWithData:)]
        pub unsafe fn initWithData(
            this: Option<Allocated<Self>>,
            pdf_data: &NSData,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other PDFRepresentation)]
        pub unsafe fn PDFRepresentation(&self) -> Id<NSData>;

        #[method(bounds)]
        pub unsafe fn bounds(&self) -> NSRect;

        #[method(currentPage)]
        pub unsafe fn currentPage(&self) -> NSInteger;

        #[method(setCurrentPage:)]
        pub unsafe fn setCurrentPage(&self, current_page: NSInteger);

        #[method(pageCount)]
        pub unsafe fn pageCount(&self) -> NSInteger;
    }
);
