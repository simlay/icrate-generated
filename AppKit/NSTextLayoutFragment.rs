//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_options!(
    #[underlying(NSUInteger)]
    #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
    pub enum NSTextLayoutFragmentEnumerationOptions {
        #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
        NSTextLayoutFragmentEnumerationOptionsNone = 0,
        #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
        NSTextLayoutFragmentEnumerationOptionsReverse = 1 << 0,
        #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
        NSTextLayoutFragmentEnumerationOptionsEstimatesSize = 1 << 1,
        #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
        NSTextLayoutFragmentEnumerationOptionsEnsuresLayout = 1 << 2,
        #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
        NSTextLayoutFragmentEnumerationOptionsEnsuresExtraLineFragment = 1 << 3,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
    pub enum NSTextLayoutFragmentState {
        #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
        NSTextLayoutFragmentStateNone = 0,
        #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
        NSTextLayoutFragmentStateEstimatedUsageBounds = 1,
        #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
        NSTextLayoutFragmentStateCalculatedUsageBounds = 2,
        #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
        NSTextLayoutFragmentStateLayoutAvailable = 3,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSTextLayoutFragment")]
    #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
    pub struct NSTextLayoutFragment;

    #[cfg(feature = "AppKit_NSTextLayoutFragment")]
    unsafe impl ClassType for NSTextLayoutFragment {
        type Super = NSObject;
    }
);

#[cfg(feature = "AppKit_NSTextLayoutFragment")]
unsafe impl NSCoding for NSTextLayoutFragment {}

#[cfg(feature = "AppKit_NSTextLayoutFragment")]
unsafe impl NSObjectProtocol for NSTextLayoutFragment {}

#[cfg(feature = "AppKit_NSTextLayoutFragment")]
unsafe impl NSSecureCoding for NSTextLayoutFragment {}

extern_methods!(
    #[cfg(feature = "AppKit_NSTextLayoutFragment")]
    unsafe impl NSTextLayoutFragment {
        #[cfg(all(feature = "AppKit_NSTextElement", feature = "AppKit_NSTextRange"))]
        #[method_id(@__retain_semantics Init initWithTextElement:range:)]
        pub unsafe fn initWithTextElement_range(
            this: Option<Allocated<Self>>,
            text_element: &NSTextElement,
            range_in_element: Option<&NSTextRange>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self>>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[cfg(feature = "AppKit_NSTextLayoutManager")]
        #[method_id(@__retain_semantics Other textLayoutManager)]
        pub unsafe fn textLayoutManager(&self) -> Option<Id<NSTextLayoutManager>>;

        #[cfg(feature = "AppKit_NSTextElement")]
        #[method_id(@__retain_semantics Other textElement)]
        pub unsafe fn textElement(&self) -> Option<Id<NSTextElement>>;

        #[cfg(feature = "AppKit_NSTextRange")]
        #[method_id(@__retain_semantics Other rangeInElement)]
        pub unsafe fn rangeInElement(&self) -> Id<NSTextRange>;

        #[cfg(all(feature = "AppKit_NSTextLineFragment", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other textLineFragments)]
        pub unsafe fn textLineFragments(&self) -> Id<NSArray<NSTextLineFragment>>;

        #[cfg(feature = "Foundation_NSOperationQueue")]
        #[method_id(@__retain_semantics Other layoutQueue)]
        pub unsafe fn layoutQueue(&self) -> Option<Id<NSOperationQueue>>;

        #[cfg(feature = "Foundation_NSOperationQueue")]
        #[method(setLayoutQueue:)]
        pub unsafe fn setLayoutQueue(&self, layout_queue: Option<&NSOperationQueue>);

        #[method(state)]
        pub unsafe fn state(&self) -> NSTextLayoutFragmentState;

        #[method(invalidateLayout)]
        pub unsafe fn invalidateLayout(&self);

        #[method(layoutFragmentFrame)]
        pub unsafe fn layoutFragmentFrame(&self) -> CGRect;

        #[method(renderingSurfaceBounds)]
        pub unsafe fn renderingSurfaceBounds(&self) -> CGRect;

        #[method(leadingPadding)]
        pub unsafe fn leadingPadding(&self) -> CGFloat;

        #[method(trailingPadding)]
        pub unsafe fn trailingPadding(&self) -> CGFloat;

        #[method(topMargin)]
        pub unsafe fn topMargin(&self) -> CGFloat;

        #[method(bottomMargin)]
        pub unsafe fn bottomMargin(&self) -> CGFloat;

        #[cfg(all(
            feature = "AppKit_NSTextAttachmentViewProvider",
            feature = "Foundation_NSArray"
        ))]
        #[method_id(@__retain_semantics Other textAttachmentViewProviders)]
        pub unsafe fn textAttachmentViewProviders(
            &self,
        ) -> Id<NSArray<NSTextAttachmentViewProvider>>;

        #[method(frameForTextAttachmentAtLocation:)]
        pub unsafe fn frameForTextAttachmentAtLocation(
            &self,
            location: &ProtocolObject<dyn NSTextLocation>,
        ) -> CGRect;
    }
);
