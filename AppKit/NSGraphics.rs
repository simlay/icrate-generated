//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSCompositingOperation {
        NSCompositingOperationClear = 0,
        NSCompositingOperationCopy = 1,
        NSCompositingOperationSourceOver = 2,
        NSCompositingOperationSourceIn = 3,
        NSCompositingOperationSourceOut = 4,
        NSCompositingOperationSourceAtop = 5,
        NSCompositingOperationDestinationOver = 6,
        NSCompositingOperationDestinationIn = 7,
        NSCompositingOperationDestinationOut = 8,
        NSCompositingOperationDestinationAtop = 9,
        NSCompositingOperationXOR = 10,
        NSCompositingOperationPlusDarker = 11,
        #[deprecated = "Use NSCompositingOperationSourceOver instead"]
        NSCompositingOperationHighlight = 12,
        NSCompositingOperationPlusLighter = 13,
        NSCompositingOperationMultiply = 14,
        NSCompositingOperationScreen = 15,
        NSCompositingOperationOverlay = 16,
        NSCompositingOperationDarken = 17,
        NSCompositingOperationLighten = 18,
        NSCompositingOperationColorDodge = 19,
        NSCompositingOperationColorBurn = 20,
        NSCompositingOperationSoftLight = 21,
        NSCompositingOperationHardLight = 22,
        NSCompositingOperationDifference = 23,
        NSCompositingOperationExclusion = 24,
        NSCompositingOperationHue = 25,
        NSCompositingOperationSaturation = 26,
        NSCompositingOperationColor = 27,
        NSCompositingOperationLuminosity = 28,
    }
);

#[deprecated]
extern_static!(NSCompositeClear: NSCompositingOperation = NSCompositingOperationClear);

#[deprecated]
extern_static!(NSCompositeCopy: NSCompositingOperation = NSCompositingOperationCopy);

#[deprecated]
extern_static!(NSCompositeSourceOver: NSCompositingOperation = NSCompositingOperationSourceOver);

#[deprecated]
extern_static!(NSCompositeSourceIn: NSCompositingOperation = NSCompositingOperationSourceIn);

#[deprecated]
extern_static!(NSCompositeSourceOut: NSCompositingOperation = NSCompositingOperationSourceOut);

#[deprecated]
extern_static!(NSCompositeSourceAtop: NSCompositingOperation = NSCompositingOperationSourceAtop);

#[deprecated]
extern_static!(
    NSCompositeDestinationOver: NSCompositingOperation = NSCompositingOperationDestinationOver
);

#[deprecated]
extern_static!(
    NSCompositeDestinationIn: NSCompositingOperation = NSCompositingOperationDestinationIn
);

#[deprecated]
extern_static!(
    NSCompositeDestinationOut: NSCompositingOperation = NSCompositingOperationDestinationOut
);

#[deprecated]
extern_static!(
    NSCompositeDestinationAtop: NSCompositingOperation = NSCompositingOperationDestinationAtop
);

#[deprecated]
extern_static!(NSCompositeXOR: NSCompositingOperation = NSCompositingOperationXOR);

#[deprecated]
extern_static!(NSCompositePlusDarker: NSCompositingOperation = NSCompositingOperationPlusDarker);

#[deprecated]
extern_static!(NSCompositeHighlight: NSCompositingOperation = NSCompositingOperationHighlight);

#[deprecated]
extern_static!(NSCompositePlusLighter: NSCompositingOperation = NSCompositingOperationPlusLighter);

#[deprecated]
extern_static!(NSCompositeMultiply: NSCompositingOperation = NSCompositingOperationMultiply);

#[deprecated]
extern_static!(NSCompositeScreen: NSCompositingOperation = NSCompositingOperationScreen);

#[deprecated]
extern_static!(NSCompositeOverlay: NSCompositingOperation = NSCompositingOperationOverlay);

#[deprecated]
extern_static!(NSCompositeDarken: NSCompositingOperation = NSCompositingOperationDarken);

#[deprecated]
extern_static!(NSCompositeLighten: NSCompositingOperation = NSCompositingOperationLighten);

#[deprecated]
extern_static!(NSCompositeColorDodge: NSCompositingOperation = NSCompositingOperationColorDodge);

#[deprecated]
extern_static!(NSCompositeColorBurn: NSCompositingOperation = NSCompositingOperationColorBurn);

#[deprecated]
extern_static!(NSCompositeSoftLight: NSCompositingOperation = NSCompositingOperationSoftLight);

#[deprecated]
extern_static!(NSCompositeHardLight: NSCompositingOperation = NSCompositingOperationHardLight);

#[deprecated]
extern_static!(NSCompositeDifference: NSCompositingOperation = NSCompositingOperationDifference);

#[deprecated]
extern_static!(NSCompositeExclusion: NSCompositingOperation = NSCompositingOperationExclusion);

#[deprecated]
extern_static!(NSCompositeHue: NSCompositingOperation = NSCompositingOperationHue);

#[deprecated]
extern_static!(NSCompositeSaturation: NSCompositingOperation = NSCompositingOperationSaturation);

#[deprecated]
extern_static!(NSCompositeColor: NSCompositingOperation = NSCompositingOperationColor);

#[deprecated]
extern_static!(NSCompositeLuminosity: NSCompositingOperation = NSCompositingOperationLuminosity);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSBackingStoreType {
        #[deprecated]
        NSBackingStoreRetained = 0,
        #[deprecated]
        NSBackingStoreNonretained = 1,
        NSBackingStoreBuffered = 2,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSWindowOrderingMode {
        NSWindowAbove = 1,
        NSWindowBelow = -1,
        NSWindowOut = 0,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSFocusRingPlacement {
        NSFocusRingOnly = 0,
        NSFocusRingBelow = 1,
        NSFocusRingAbove = 2,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSFocusRingType {
        NSFocusRingTypeDefault = 0,
        NSFocusRingTypeNone = 1,
        NSFocusRingTypeExterior = 2,
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSColorRenderingIntent {
        NSColorRenderingIntentDefault = 0,
        NSColorRenderingIntentAbsoluteColorimetric = 1,
        NSColorRenderingIntentRelativeColorimetric = 2,
        NSColorRenderingIntentPerceptual = 3,
        NSColorRenderingIntentSaturation = 4,
    }
);

typed_enum!(
    pub type NSColorSpaceName = NSString;
);

extern_static!(NSCalibratedWhiteColorSpace: &'static NSColorSpaceName);

extern_static!(NSCalibratedRGBColorSpace: &'static NSColorSpaceName);

extern_static!(NSDeviceWhiteColorSpace: &'static NSColorSpaceName);

extern_static!(NSDeviceRGBColorSpace: &'static NSColorSpaceName);

extern_static!(NSDeviceCMYKColorSpace: &'static NSColorSpaceName);

extern_static!(NSNamedColorSpace: &'static NSColorSpaceName);

extern_static!(NSPatternColorSpace: &'static NSColorSpaceName);

extern_static!(NSCustomColorSpace: &'static NSColorSpaceName);

#[deprecated]
extern_static!(NSCalibratedBlackColorSpace: &'static NSColorSpaceName);

#[deprecated]
extern_static!(NSDeviceBlackColorSpace: &'static NSColorSpaceName);

ns_enum!(
    #[underlying(i32)]
    pub enum NSWindowDepth {
        NSWindowDepthTwentyfourBitRGB = 0x208,
        NSWindowDepthSixtyfourBitRGB = 0x210,
        NSWindowDepthOnehundredtwentyeightBitRGB = 0x220,
    }
);

extern_fn!(
    pub unsafe fn NSBestDepth(
        color_space: &NSColorSpaceName,
        bps: NSInteger,
        bpp: NSInteger,
        planar: Bool,
        exact_match: *mut Bool,
    ) -> NSWindowDepth;
);

extern_fn!(
    pub unsafe fn NSPlanarFromDepth(depth: NSWindowDepth) -> Bool;
);

extern_fn!(
    pub unsafe fn NSColorSpaceFromDepth(depth: NSWindowDepth) -> *mut NSColorSpaceName;
);

extern_fn!(
    pub unsafe fn NSBitsPerSampleFromDepth(depth: NSWindowDepth) -> NSInteger;
);

extern_fn!(
    pub unsafe fn NSBitsPerPixelFromDepth(depth: NSWindowDepth) -> NSInteger;
);

extern_fn!(
    pub unsafe fn NSNumberOfColorComponents(color_space_name: &NSColorSpaceName) -> NSInteger;
);

extern_fn!(
    pub unsafe fn NSAvailableWindowDepths() -> NonNull<NSWindowDepth>;
);

extern_static!(NSWhite: CGFloat);

extern_static!(NSLightGray: CGFloat);

extern_static!(NSDarkGray: CGFloat);

extern_static!(NSBlack: CGFloat);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSDisplayGamut {
        NSDisplayGamutSRGB = 1,
        NSDisplayGamutP3 = 2,
    }
);

typed_extensible_enum!(
    pub type NSDeviceDescriptionKey = NSString;
);

extern_static!(NSDeviceResolution: &'static NSDeviceDescriptionKey);

extern_static!(NSDeviceColorSpaceName: &'static NSDeviceDescriptionKey);

extern_static!(NSDeviceBitsPerSample: &'static NSDeviceDescriptionKey);

extern_static!(NSDeviceIsScreen: &'static NSDeviceDescriptionKey);

extern_static!(NSDeviceIsPrinter: &'static NSDeviceDescriptionKey);

extern_static!(NSDeviceSize: &'static NSDeviceDescriptionKey);

extern_fn!(
    pub unsafe fn NSRectFill(rect: NSRect);
);

extern_fn!(
    pub unsafe fn NSRectFillList(rects: NonNull<NSRect>, count: NSInteger);
);

extern_fn!(
    pub unsafe fn NSRectFillListWithGrays(
        rects: NonNull<NSRect>,
        grays: NonNull<CGFloat>,
        num: NSInteger,
    );
);

extern_fn!(
    #[cfg(feature = "AppKit_NSColor")]
    pub unsafe fn NSRectFillListWithColors(
        rects: NonNull<NSRect>,
        colors: NonNull<NonNull<NSColor>>,
        num: NSInteger,
    );
);

extern_fn!(
    pub unsafe fn NSRectFillUsingOperation(rect: NSRect, op: NSCompositingOperation);
);

extern_fn!(
    pub unsafe fn NSRectFillListUsingOperation(
        rects: NonNull<NSRect>,
        count: NSInteger,
        op: NSCompositingOperation,
    );
);

extern_fn!(
    #[cfg(feature = "AppKit_NSColor")]
    pub unsafe fn NSRectFillListWithColorsUsingOperation(
        rects: NonNull<NSRect>,
        colors: NonNull<NonNull<NSColor>>,
        num: NSInteger,
        op: NSCompositingOperation,
    );
);

extern_fn!(
    pub unsafe fn NSFrameRect(rect: NSRect);
);

extern_fn!(
    pub unsafe fn NSFrameRectWithWidth(rect: NSRect, frame_width: CGFloat);
);

extern_fn!(
    pub unsafe fn NSFrameRectWithWidthUsingOperation(
        rect: NSRect,
        frame_width: CGFloat,
        op: NSCompositingOperation,
    );
);

extern_fn!(
    pub unsafe fn NSRectClip(rect: NSRect);
);

extern_fn!(
    pub unsafe fn NSRectClipList(rects: NonNull<NSRect>, count: NSInteger);
);

extern_fn!(
    pub unsafe fn NSDrawTiledRects(
        bounds_rect: NSRect,
        clip_rect: NSRect,
        sides: NonNull<NSRectEdge>,
        grays: NonNull<CGFloat>,
        count: NSInteger,
    ) -> NSRect;
);

extern_fn!(
    pub unsafe fn NSDrawGrayBezel(rect: NSRect, clip_rect: NSRect);
);

extern_fn!(
    pub unsafe fn NSDrawGroove(rect: NSRect, clip_rect: NSRect);
);

extern_fn!(
    pub unsafe fn NSDrawWhiteBezel(rect: NSRect, clip_rect: NSRect);
);

extern_fn!(
    pub unsafe fn NSDrawButton(rect: NSRect, clip_rect: NSRect);
);

extern_fn!(
    pub unsafe fn NSEraseRect(rect: NSRect);
);

extern_fn!(
    #[cfg(feature = "AppKit_NSColor")]
    #[deprecated = "Use -[NSBitmapImageRep colorAtX:y:] to interrogate pixel values.  If necessary, use -[NSView cacheDisplayInRect:toBitmapImageRep:] to snapshot a view hierarchy into an NSBitmapImageRep."]
    pub unsafe fn NSReadPixel(passed_point: NSPoint) -> *mut NSColor;
);

extern_fn!(
    #[deprecated]
    pub unsafe fn NSHighlightRect(rect: NSRect);
);

extern_fn!(
    pub unsafe fn NSBeep();
);

extern_fn!(
    #[cfg(feature = "Foundation_NSString")]
    #[deprecated = "Doesn't return anything useful since 10.0"]
    pub unsafe fn NSGetWindowServerMemory(
        context: NSInteger,
        virtual_memory: NonNull<NSInteger>,
        window_backing_memory: NonNull<NSInteger>,
        window_dump_string: NonNull<NonNull<NSString>>,
    ) -> NSInteger;
);

extern_fn!(
    #[cfg(feature = "AppKit_NSColor")]
    pub unsafe fn NSDrawColorTiledRects(
        bounds_rect: NSRect,
        clip_rect: NSRect,
        sides: NonNull<NSRectEdge>,
        colors: NonNull<NonNull<NSColor>>,
        count: NSInteger,
    ) -> NSRect;
);

extern_fn!(
    pub unsafe fn NSDrawDarkBezel(rect: NSRect, clip_rect: NSRect);
);

extern_fn!(
    pub unsafe fn NSDrawLightBezel(rect: NSRect, clip_rect: NSRect);
);

extern_fn!(
    pub unsafe fn NSDottedFrameRect(rect: NSRect);
);

extern_fn!(
    pub unsafe fn NSDrawWindowBackground(rect: NSRect);
);

extern_fn!(
    pub unsafe fn NSSetFocusRingStyle(placement: NSFocusRingPlacement);
);

extern_fn!(
    #[deprecated = "As of 10.11 it is not generally necessary to take explicit action to achieve visual atomicity. +[NSAnimationContext runAnimationGroup:] and other similar methods can be used when a stronger than normal need for visual atomicity is required. The NSAnimationContext methods do not suffer from the same performance problems as NSDisableScreenUpdates."]
    pub unsafe fn NSDisableScreenUpdates();
);

extern_fn!(
    #[deprecated = "As of 10.11 it is not generally necessary to take explicit action to achieve visual atomicity. +[NSAnimationContext runAnimationGroup:] and other similar methods can be used when a stronger than normal need for visual atomicity is required. The NSAnimationContext methods do not suffer from the same performance problems as NSEnableScreenUpdates."]
    pub unsafe fn NSEnableScreenUpdates();
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSAnimationEffect {
        NSAnimationEffectDisappearingItemDefault = 0,
        NSAnimationEffectPoof = 10,
    }
);

extern_fn!(
    pub unsafe fn NSShowAnimationEffect(
        animation_effect: NSAnimationEffect,
        center_location: NSPoint,
        size: NSSize,
        animation_delegate: Option<&Object>,
        did_end_selector: Option<Sel>,
        context_info: *mut c_void,
    );
);

extern_fn!(
    #[deprecated = "Use +[NSWindow windowNumbersWithOptions:] instead"]
    pub unsafe fn NSCountWindows(count: NonNull<NSInteger>);
);

extern_fn!(
    #[deprecated = "Use +[NSWindow windowNumbersWithOptions:] instead"]
    pub unsafe fn NSWindowList(size: NSInteger, list: NonNull<NSInteger>);
);

extern_fn!(
    #[deprecated = "Use +[NSWindow windowNumbersWithOptions:] instead"]
    pub unsafe fn NSCountWindowsForContext(context: NSInteger, count: NonNull<NSInteger>);
);

extern_fn!(
    #[deprecated = "Use +[NSWindow windowNumbersWithOptions:] instead"]
    pub unsafe fn NSWindowListForContext(
        context: NSInteger,
        size: NSInteger,
        list: NonNull<NSInteger>,
    );
);

extern_fn!(
    #[deprecated]
    pub unsafe fn NSCopyBits(src_g_state: NSInteger, src_rect: NSRect, dest_point: NSPoint);
);
