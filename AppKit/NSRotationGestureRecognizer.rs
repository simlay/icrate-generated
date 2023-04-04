//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSRotationGestureRecognizer")]
    pub struct NSRotationGestureRecognizer;

    #[cfg(feature = "AppKit_NSRotationGestureRecognizer")]
    unsafe impl ClassType for NSRotationGestureRecognizer {
        #[inherits(NSObject)]
        type Super = NSGestureRecognizer;
    }
);

#[cfg(feature = "AppKit_NSRotationGestureRecognizer")]
unsafe impl NSCoding for NSRotationGestureRecognizer {}

#[cfg(feature = "AppKit_NSRotationGestureRecognizer")]
unsafe impl NSObjectProtocol for NSRotationGestureRecognizer {}

extern_methods!(
    #[cfg(feature = "AppKit_NSRotationGestureRecognizer")]
    unsafe impl NSRotationGestureRecognizer {
        #[method(rotation)]
        pub unsafe fn rotation(&self) -> CGFloat;

        #[method(setRotation:)]
        pub unsafe fn setRotation(&self, rotation: CGFloat);

        #[method(rotationInDegrees)]
        pub unsafe fn rotationInDegrees(&self) -> CGFloat;

        #[method(setRotationInDegrees:)]
        pub unsafe fn setRotationInDegrees(&self, rotation_in_degrees: CGFloat);
    }
);

extern_methods!(
    /// Methods declared on superclass `NSGestureRecognizer`
    #[cfg(feature = "AppKit_NSRotationGestureRecognizer")]
    unsafe impl NSRotationGestureRecognizer {
        #[method_id(@__retain_semantics Init initWithTarget:action:)]
        pub unsafe fn initWithTarget_action(
            this: Option<Allocated<Self>>,
            target: Option<&Object>,
            action: Option<Sel>,
        ) -> Id<Self>;
    }
);
