//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_protocol!(
    pub unsafe trait NSTextLocation: NSObjectProtocol {
        #[method(compare:)]
        unsafe fn compare(
            &self,
            location: &ProtocolObject<dyn NSTextLocation>,
        ) -> NSComparisonResult;
    }

    unsafe impl ProtocolType for dyn NSTextLocation {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSTextRange")]
    pub struct NSTextRange;

    #[cfg(feature = "AppKit_NSTextRange")]
    unsafe impl ClassType for NSTextRange {
        type Super = NSObject;
    }
);

#[cfg(feature = "AppKit_NSTextRange")]
unsafe impl NSObjectProtocol for NSTextRange {}

extern_methods!(
    #[cfg(feature = "AppKit_NSTextRange")]
    unsafe impl NSTextRange {
        #[method_id(@__retain_semantics Init initWithLocation:endLocation:)]
        pub unsafe fn initWithLocation_endLocation(
            this: Option<Allocated<Self>>,
            location: &ProtocolObject<dyn NSTextLocation>,
            end_location: Option<&ProtocolObject<dyn NSTextLocation>>,
        ) -> Option<Id<Self>>;

        #[method_id(@__retain_semantics Init initWithLocation:)]
        pub unsafe fn initWithLocation(
            this: Option<Allocated<Self>>,
            location: &ProtocolObject<dyn NSTextLocation>,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;

        #[method(isEmpty)]
        pub unsafe fn isEmpty(&self) -> bool;

        #[method_id(@__retain_semantics Other location)]
        pub unsafe fn location(&self) -> Id<ProtocolObject<dyn NSTextLocation>>;

        #[method_id(@__retain_semantics Other endLocation)]
        pub unsafe fn endLocation(&self) -> Id<ProtocolObject<dyn NSTextLocation>>;

        #[method(isEqualToTextRange:)]
        pub unsafe fn isEqualToTextRange(&self, text_range: &NSTextRange) -> bool;

        #[method(containsLocation:)]
        pub unsafe fn containsLocation(
            &self,
            location: &ProtocolObject<dyn NSTextLocation>,
        ) -> bool;

        #[method(containsRange:)]
        pub unsafe fn containsRange(&self, text_range: &NSTextRange) -> bool;

        #[method(intersectsWithTextRange:)]
        pub unsafe fn intersectsWithTextRange(&self, text_range: &NSTextRange) -> bool;

        #[method_id(@__retain_semantics Other textRangeByIntersectingWithTextRange:)]
        pub unsafe fn textRangeByIntersectingWithTextRange(
            &self,
            text_range: &NSTextRange,
        ) -> Option<Id<Self>>;

        #[method_id(@__retain_semantics Other textRangeByFormingUnionWithTextRange:)]
        pub unsafe fn textRangeByFormingUnionWithTextRange(
            &self,
            text_range: &NSTextRange,
        ) -> Id<Self>;
    }
);
