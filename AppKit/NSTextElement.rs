//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSTextElement")]
    #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
    pub struct NSTextElement;

    #[cfg(feature = "AppKit_NSTextElement")]
    unsafe impl ClassType for NSTextElement {
        type Super = NSObject;
    }
);

#[cfg(feature = "AppKit_NSTextElement")]
unsafe impl NSObjectProtocol for NSTextElement {}

extern_methods!(
    #[cfg(feature = "AppKit_NSTextElement")]
    unsafe impl NSTextElement {
        #[cfg(feature = "AppKit_NSTextContentManager")]
        #[method_id(@__retain_semantics Init initWithTextContentManager:)]
        pub unsafe fn initWithTextContentManager(
            this: Option<Allocated<Self>>,
            text_content_manager: Option<&NSTextContentManager>,
        ) -> Id<Self>;

        #[cfg(feature = "AppKit_NSTextContentManager")]
        #[method_id(@__retain_semantics Other textContentManager)]
        pub unsafe fn textContentManager(&self) -> Option<Id<NSTextContentManager>>;

        #[cfg(feature = "AppKit_NSTextContentManager")]
        #[method(setTextContentManager:)]
        pub unsafe fn setTextContentManager(
            &self,
            text_content_manager: Option<&NSTextContentManager>,
        );

        #[cfg(feature = "AppKit_NSTextRange")]
        #[method_id(@__retain_semantics Other elementRange)]
        pub unsafe fn elementRange(&self) -> Option<Id<NSTextRange>>;

        #[cfg(feature = "AppKit_NSTextRange")]
        #[method(setElementRange:)]
        pub unsafe fn setElementRange(&self, element_range: Option<&NSTextRange>);

        #[cfg(feature = "Foundation_NSArray")]
        #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
        #[method_id(@__retain_semantics Other childElements)]
        pub unsafe fn childElements(&self) -> Id<NSArray<NSTextElement>>;

        #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
        #[method_id(@__retain_semantics Other parentElement)]
        pub unsafe fn parentElement(&self) -> Option<Id<NSTextElement>>;

        #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
        #[method(isRepresentedElement)]
        pub unsafe fn isRepresentedElement(&self) -> bool;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSTextParagraph")]
    #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
    pub struct NSTextParagraph;

    #[cfg(feature = "AppKit_NSTextParagraph")]
    unsafe impl ClassType for NSTextParagraph {
        #[inherits(NSObject)]
        type Super = NSTextElement;
    }
);

#[cfg(feature = "AppKit_NSTextParagraph")]
unsafe impl NSObjectProtocol for NSTextParagraph {}

extern_methods!(
    #[cfg(feature = "AppKit_NSTextParagraph")]
    unsafe impl NSTextParagraph {
        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method_id(@__retain_semantics Init initWithAttributedString:)]
        pub unsafe fn initWithAttributedString(
            this: Option<Allocated<Self>>,
            attributed_string: Option<&NSAttributedString>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method_id(@__retain_semantics Other attributedString)]
        pub unsafe fn attributedString(&self) -> Id<NSAttributedString>;

        #[cfg(feature = "AppKit_NSTextRange")]
        #[method_id(@__retain_semantics Other paragraphContentRange)]
        pub unsafe fn paragraphContentRange(&self) -> Option<Id<NSTextRange>>;

        #[cfg(feature = "AppKit_NSTextRange")]
        #[method_id(@__retain_semantics Other paragraphSeparatorRange)]
        pub unsafe fn paragraphSeparatorRange(&self) -> Option<Id<NSTextRange>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSTextElement`
    #[cfg(feature = "AppKit_NSTextParagraph")]
    unsafe impl NSTextParagraph {
        #[cfg(feature = "AppKit_NSTextContentManager")]
        #[method_id(@__retain_semantics Init initWithTextContentManager:)]
        pub unsafe fn initWithTextContentManager(
            this: Option<Allocated<Self>>,
            text_content_manager: Option<&NSTextContentManager>,
        ) -> Id<Self>;
    }
);
