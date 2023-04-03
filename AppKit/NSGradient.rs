//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_options!(
    #[underlying(NSUInteger)]
    #[cfg(not(any(target_os = "ios")))]
    pub enum NSGradientDrawingOptions {
        #[cfg(not(any(target_os = "ios")))]
        NSGradientDrawsBeforeStartingLocation = 1 << 0,
        #[cfg(not(any(target_os = "ios")))]
        NSGradientDrawsAfterEndingLocation = 1 << 1,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSGradient")]
    #[cfg(not(any(target_os = "ios")))]
    pub struct NSGradient;

    #[cfg(feature = "AppKit_NSGradient")]
    unsafe impl ClassType for NSGradient {
        type Super = NSObject;
    }
);

#[cfg(feature = "AppKit_NSGradient")]
unsafe impl NSCoding for NSGradient {}

#[cfg(feature = "AppKit_NSGradient")]
unsafe impl NSObjectProtocol for NSGradient {}

#[cfg(feature = "AppKit_NSGradient")]
unsafe impl NSSecureCoding for NSGradient {}

extern_methods!(
    #[cfg(feature = "AppKit_NSGradient")]
    unsafe impl NSGradient {
        #[cfg(feature = "AppKit_NSColor")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Init initWithStartingColor:endingColor:)]
        pub unsafe fn initWithStartingColor_endingColor(
            this: Option<Allocated<Self>>,
            starting_color: &NSColor,
            ending_color: &NSColor,
        ) -> Option<Id<Self>>;

        #[cfg(all(feature = "AppKit_NSColor", feature = "Foundation_NSArray"))]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Init initWithColors:)]
        pub unsafe fn initWithColors(
            this: Option<Allocated<Self>>,
            color_array: &NSArray<NSColor>,
        ) -> Option<Id<Self>>;

        #[cfg(all(
            feature = "AppKit_NSColor",
            feature = "AppKit_NSColorSpace",
            feature = "Foundation_NSArray"
        ))]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Init initWithColors:atLocations:colorSpace:)]
        pub unsafe fn initWithColors_atLocations_colorSpace(
            this: Option<Allocated<Self>>,
            color_array: &NSArray<NSColor>,
            locations: *mut CGFloat,
            color_space: &NSColorSpace,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Option<Allocated<Self>>, coder: &NSCoder) -> Id<Self>;

        #[cfg(not(any(target_os = "ios")))]
        #[method(drawFromPoint:toPoint:options:)]
        pub unsafe fn drawFromPoint_toPoint_options(
            &self,
            starting_point: NSPoint,
            ending_point: NSPoint,
            options: NSGradientDrawingOptions,
        );

        #[cfg(not(any(target_os = "ios")))]
        #[method(drawInRect:angle:)]
        pub unsafe fn drawInRect_angle(&self, rect: NSRect, angle: CGFloat);

        #[cfg(feature = "AppKit_NSBezierPath")]
        #[cfg(not(any(target_os = "ios")))]
        #[method(drawInBezierPath:angle:)]
        pub unsafe fn drawInBezierPath_angle(&self, path: &NSBezierPath, angle: CGFloat);

        #[cfg(not(any(target_os = "ios")))]
        #[method(drawFromCenter:radius:toCenter:radius:options:)]
        pub unsafe fn drawFromCenter_radius_toCenter_radius_options(
            &self,
            start_center: NSPoint,
            start_radius: CGFloat,
            end_center: NSPoint,
            end_radius: CGFloat,
            options: NSGradientDrawingOptions,
        );

        #[cfg(not(any(target_os = "ios")))]
        #[method(drawInRect:relativeCenterPosition:)]
        pub unsafe fn drawInRect_relativeCenterPosition(
            &self,
            rect: NSRect,
            relative_center_position: NSPoint,
        );

        #[cfg(feature = "AppKit_NSBezierPath")]
        #[cfg(not(any(target_os = "ios")))]
        #[method(drawInBezierPath:relativeCenterPosition:)]
        pub unsafe fn drawInBezierPath_relativeCenterPosition(
            &self,
            path: &NSBezierPath,
            relative_center_position: NSPoint,
        );

        #[cfg(feature = "AppKit_NSColorSpace")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other colorSpace)]
        pub unsafe fn colorSpace(&self) -> Id<NSColorSpace>;

        #[cfg(not(any(target_os = "ios")))]
        #[method(numberOfColorStops)]
        pub unsafe fn numberOfColorStops(&self) -> NSInteger;

        #[cfg(feature = "AppKit_NSColor")]
        #[cfg(not(any(target_os = "ios")))]
        #[method(getColor:location:atIndex:)]
        pub unsafe fn getColor_location_atIndex(
            &self,
            color: Option<&mut Id<NSColor>>,
            location: *mut CGFloat,
            index: NSInteger,
        );

        #[cfg(feature = "AppKit_NSColor")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other interpolatedColorAtLocation:)]
        pub unsafe fn interpolatedColorAtLocation(&self, location: CGFloat) -> Id<NSColor>;
    }
);
