//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

pub type NSImageName = NSString;

extern_static!(NSImageHintCTM: &'static NSImageHintKey);

extern_static!(NSImageHintInterpolation: &'static NSImageHintKey);

extern_static!(NSImageHintUserInterfaceLayoutDirection: &'static NSImageHintKey);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSImageLoadStatus {
        NSImageLoadStatusCompleted = 0,
        NSImageLoadStatusCancelled = 1,
        NSImageLoadStatusInvalidData = 2,
        NSImageLoadStatusUnexpectedEOF = 3,
        NSImageLoadStatusReadError = 4,
    }
);

ns_enum!(
    #[underlying(NSUInteger)]
    pub enum NSImageCacheMode {
        NSImageCacheDefault = 0,
        NSImageCacheAlways = 1,
        NSImageCacheBySize = 2,
        NSImageCacheNever = 3,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSImage")]
    pub struct NSImage;

    #[cfg(feature = "AppKit_NSImage")]
    unsafe impl ClassType for NSImage {
        type Super = NSObject;
    }
);

#[cfg(feature = "AppKit_NSImage")]
unsafe impl NSObjectProtocol for NSImage {}

extern_methods!(
    #[cfg(feature = "AppKit_NSImage")]
    unsafe impl NSImage {
        #[method_id(@__retain_semantics Other imageNamed:)]
        pub unsafe fn imageNamed(name: &NSImageName) -> Option<Id<NSImage>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other imageWithSystemSymbolName:accessibilityDescription:)]
        pub unsafe fn imageWithSystemSymbolName_accessibilityDescription(
            name: &NSString,
            description: Option<&NSString>,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other imageWithSystemSymbolName:variableValue:accessibilityDescription:)]
        pub unsafe fn imageWithSystemSymbolName_variableValue_accessibilityDescription(
            name: &NSString,
            value: c_double,
            description: Option<&NSString>,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other imageWithSymbolName:variableValue:)]
        pub unsafe fn imageWithSymbolName_variableValue(
            name: &NSString,
            value: c_double,
        ) -> Option<Id<Self>>;

        #[cfg(all(feature = "Foundation_NSBundle", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other imageWithSymbolName:bundle:variableValue:)]
        pub unsafe fn imageWithSymbolName_bundle_variableValue(
            name: &NSString,
            bundle: Option<&NSBundle>,
            value: c_double,
        ) -> Option<Id<Self>>;

        #[method_id(@__retain_semantics Init initWithSize:)]
        pub unsafe fn initWithSize(this: Option<Allocated<Self>>, size: NSSize) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Option<Allocated<Self>>, coder: &NSCoder) -> Id<Self>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Init initWithData:)]
        pub unsafe fn initWithData(
            this: Option<Allocated<Self>>,
            data: &NSData,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithContentsOfFile:)]
        pub unsafe fn initWithContentsOfFile(
            this: Option<Allocated<Self>>,
            file_name: &NSString,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Init initWithContentsOfURL:)]
        pub unsafe fn initWithContentsOfURL(
            this: Option<Allocated<Self>>,
            url: &NSURL,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initByReferencingFile:)]
        pub unsafe fn initByReferencingFile(
            this: Option<Allocated<Self>>,
            file_name: &NSString,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Init initByReferencingURL:)]
        pub unsafe fn initByReferencingURL(this: Option<Allocated<Self>>, url: &NSURL) -> Id<Self>;

        #[cfg(feature = "AppKit_NSPasteboard")]
        #[method_id(@__retain_semantics Init initWithPasteboard:)]
        pub unsafe fn initWithPasteboard(
            this: Option<Allocated<Self>>,
            pasteboard: &NSPasteboard,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Init initWithDataIgnoringOrientation:)]
        pub unsafe fn initWithDataIgnoringOrientation(
            this: Option<Allocated<Self>>,
            data: &NSData,
        ) -> Option<Id<Self>>;

        #[method_id(@__retain_semantics Other imageWithSize:flipped:drawingHandler:)]
        pub unsafe fn imageWithSize_flipped_drawingHandler(
            size: NSSize,
            drawing_handler_should_be_called_with_flipped_context: bool,
            drawing_handler: &Block<(NSRect,), Bool>,
        ) -> Id<Self>;

        #[method(size)]
        pub unsafe fn size(&self) -> NSSize;

        #[method(setSize:)]
        pub unsafe fn setSize(&self, size: NSSize);

        #[method(setName:)]
        pub unsafe fn setName(&self, string: Option<&NSImageName>) -> bool;

        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Option<Id<NSImageName>>;

        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Other backgroundColor)]
        pub unsafe fn backgroundColor(&self) -> Id<NSColor>;

        #[cfg(feature = "AppKit_NSColor")]
        #[method(setBackgroundColor:)]
        pub unsafe fn setBackgroundColor(&self, background_color: &NSColor);

        #[method(usesEPSOnResolutionMismatch)]
        pub unsafe fn usesEPSOnResolutionMismatch(&self) -> bool;

        #[method(setUsesEPSOnResolutionMismatch:)]
        pub unsafe fn setUsesEPSOnResolutionMismatch(&self, uses_eps_on_resolution_mismatch: bool);

        #[method(prefersColorMatch)]
        pub unsafe fn prefersColorMatch(&self) -> bool;

        #[method(setPrefersColorMatch:)]
        pub unsafe fn setPrefersColorMatch(&self, prefers_color_match: bool);

        #[method(matchesOnMultipleResolution)]
        pub unsafe fn matchesOnMultipleResolution(&self) -> bool;

        #[method(setMatchesOnMultipleResolution:)]
        pub unsafe fn setMatchesOnMultipleResolution(&self, matches_on_multiple_resolution: bool);

        #[method(matchesOnlyOnBestFittingAxis)]
        pub unsafe fn matchesOnlyOnBestFittingAxis(&self) -> bool;

        #[method(setMatchesOnlyOnBestFittingAxis:)]
        pub unsafe fn setMatchesOnlyOnBestFittingAxis(
            &self,
            matches_only_on_best_fitting_axis: bool,
        );

        #[method(drawAtPoint:fromRect:operation:fraction:)]
        pub unsafe fn drawAtPoint_fromRect_operation_fraction(
            &self,
            point: NSPoint,
            from_rect: NSRect,
            op: NSCompositingOperation,
            delta: CGFloat,
        );

        #[method(drawInRect:fromRect:operation:fraction:)]
        pub unsafe fn drawInRect_fromRect_operation_fraction(
            &self,
            rect: NSRect,
            from_rect: NSRect,
            op: NSCompositingOperation,
            delta: CGFloat,
        );

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method(drawInRect:fromRect:operation:fraction:respectFlipped:hints:)]
        pub unsafe fn drawInRect_fromRect_operation_fraction_respectFlipped_hints(
            &self,
            dst_space_portion_rect: NSRect,
            src_space_portion_rect: NSRect,
            op: NSCompositingOperation,
            requested_alpha: CGFloat,
            respect_context_is_flipped: bool,
            hints: Option<&NSDictionary<NSImageHintKey, Object>>,
        );

        #[cfg(feature = "AppKit_NSImageRep")]
        #[method(drawRepresentation:inRect:)]
        pub unsafe fn drawRepresentation_inRect(
            &self,
            image_rep: &NSImageRep,
            rect: NSRect,
        ) -> bool;

        #[method(drawInRect:)]
        pub unsafe fn drawInRect(&self, rect: NSRect);

        #[method(recache)]
        pub unsafe fn recache(&self);

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other TIFFRepresentation)]
        pub unsafe fn TIFFRepresentation(&self) -> Option<Id<NSData>>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other TIFFRepresentationUsingCompression:factor:)]
        pub unsafe fn TIFFRepresentationUsingCompression_factor(
            &self,
            comp: NSTIFFCompression,
            factor: c_float,
        ) -> Option<Id<NSData>>;

        #[cfg(all(feature = "AppKit_NSImageRep", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other representations)]
        pub unsafe fn representations(&self) -> Id<NSArray<NSImageRep>>;

        #[cfg(all(feature = "AppKit_NSImageRep", feature = "Foundation_NSArray"))]
        #[method(addRepresentations:)]
        pub unsafe fn addRepresentations(&self, image_reps: &NSArray<NSImageRep>);

        #[cfg(feature = "AppKit_NSImageRep")]
        #[method(addRepresentation:)]
        pub unsafe fn addRepresentation(&self, image_rep: &NSImageRep);

        #[cfg(feature = "AppKit_NSImageRep")]
        #[method(removeRepresentation:)]
        pub unsafe fn removeRepresentation(&self, image_rep: &NSImageRep);

        #[method(isValid)]
        pub unsafe fn isValid(&self) -> bool;

        #[deprecated = "This method is incompatible with resolution-independent drawing and should not be used. Instead, use +[NSImage imageWithSize:flipped:drawingHandler:] to create a block-based image describing the desired image drawing, or use +[NSGraphicsContext graphicsContextWithBitmapImageRep:] to manipulate specific bitmap image representations."]
        #[method(lockFocus)]
        pub unsafe fn lockFocus(&self);

        #[deprecated = "This method is incompatible with resolution-independent drawing and should not be used. Instead, use +[NSImage imageWithSize:flipped:drawingHandler:] to create a block-based image describing the desired image drawing, or use +[NSGraphicsContext graphicsContextWithBitmapImageRep:] to manipulate specific bitmap image representations."]
        #[method(lockFocusFlipped:)]
        pub unsafe fn lockFocusFlipped(&self, flipped: bool);

        #[deprecated = "This method is incompatible with resolution-independent drawing and should not be used. Instead, use +[NSImage imageWithSize:flipped:drawingHandler:] to create a block-based image describing the desired image drawing, or use +[NSGraphicsContext graphicsContextWithBitmapImageRep:] to manipulate specific bitmap image representations."]
        #[method(unlockFocus)]
        pub unsafe fn unlockFocus(&self);

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<ProtocolObject<dyn NSImageDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&ProtocolObject<dyn NSImageDelegate>>);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other imageTypes)]
        pub unsafe fn imageTypes() -> Id<NSArray<NSString>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other imageUnfilteredTypes)]
        pub unsafe fn imageUnfilteredTypes() -> Id<NSArray<NSString>>;

        #[cfg(feature = "AppKit_NSPasteboard")]
        #[method(canInitWithPasteboard:)]
        pub unsafe fn canInitWithPasteboard(pasteboard: &NSPasteboard) -> bool;

        #[method(cancelIncrementalLoad)]
        pub unsafe fn cancelIncrementalLoad(&self);

        #[method(cacheMode)]
        pub unsafe fn cacheMode(&self) -> NSImageCacheMode;

        #[method(setCacheMode:)]
        pub unsafe fn setCacheMode(&self, cache_mode: NSImageCacheMode);

        #[method(alignmentRect)]
        pub unsafe fn alignmentRect(&self) -> NSRect;

        #[method(setAlignmentRect:)]
        pub unsafe fn setAlignmentRect(&self, alignment_rect: NSRect);

        #[method(isTemplate)]
        pub unsafe fn isTemplate(&self) -> bool;

        #[method(setTemplate:)]
        pub unsafe fn setTemplate(&self, template: bool);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other accessibilityDescription)]
        pub unsafe fn accessibilityDescription(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setAccessibilityDescription:)]
        pub unsafe fn setAccessibilityDescription(
            &self,
            accessibility_description: Option<&NSString>,
        );

        #[cfg(all(
            feature = "AppKit_NSGraphicsContext",
            feature = "AppKit_NSImageRep",
            feature = "Foundation_NSDictionary"
        ))]
        #[method_id(@__retain_semantics Other bestRepresentationForRect:context:hints:)]
        pub unsafe fn bestRepresentationForRect_context_hints(
            &self,
            rect: NSRect,
            reference_context: Option<&NSGraphicsContext>,
            hints: Option<&NSDictionary<NSImageHintKey, Object>>,
        ) -> Option<Id<NSImageRep>>;

        #[cfg(all(
            feature = "AppKit_NSGraphicsContext",
            feature = "Foundation_NSDictionary"
        ))]
        #[method(hitTestRect:withImageDestinationRect:context:hints:flipped:)]
        pub unsafe fn hitTestRect_withImageDestinationRect_context_hints_flipped(
            &self,
            test_rect_dest_space: NSRect,
            image_rect_dest_space: NSRect,
            context: Option<&NSGraphicsContext>,
            hints: Option<&NSDictionary<NSImageHintKey, Object>>,
            flipped: bool,
        ) -> bool;

        #[method(recommendedLayerContentsScale:)]
        pub unsafe fn recommendedLayerContentsScale(
            &self,
            preferred_contents_scale: CGFloat,
        ) -> CGFloat;

        #[method_id(@__retain_semantics Other layerContentsForContentsScale:)]
        pub unsafe fn layerContentsForContentsScale(
            &self,
            layer_contents_scale: CGFloat,
        ) -> Id<Object>;

        #[method(capInsets)]
        pub unsafe fn capInsets(&self) -> NSEdgeInsets;

        #[method(setCapInsets:)]
        pub unsafe fn setCapInsets(&self, cap_insets: NSEdgeInsets);

        #[method(resizingMode)]
        pub unsafe fn resizingMode(&self) -> NSImageResizingMode;

        #[method(setResizingMode:)]
        pub unsafe fn setResizingMode(&self, resizing_mode: NSImageResizingMode);

        #[cfg(feature = "AppKit_NSImageSymbolConfiguration")]
        #[method_id(@__retain_semantics Other imageWithSymbolConfiguration:)]
        pub unsafe fn imageWithSymbolConfiguration(
            &self,
            configuration: &NSImageSymbolConfiguration,
        ) -> Option<Id<NSImage>>;

        #[cfg(feature = "AppKit_NSImageSymbolConfiguration")]
        #[method_id(@__retain_semantics Other symbolConfiguration)]
        pub unsafe fn symbolConfiguration(&self) -> Id<NSImageSymbolConfiguration>;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSImage")]
    unsafe impl NSImage {}
);

#[cfg(feature = "AppKit_NSImage")]
unsafe impl NSPasteboardReading for NSImage {}

#[cfg(feature = "AppKit_NSImage")]
unsafe impl NSPasteboardWriting for NSImage {}

#[cfg(feature = "AppKit_NSImage")]
unsafe impl NSSecureCoding for NSImage {}

extern_methods!(
    #[cfg(feature = "AppKit_NSImage")]
    unsafe impl NSImage {}
);

#[cfg(feature = "AppKit_NSImage")]
unsafe impl NSItemProviderReading for NSImage {}

#[cfg(feature = "AppKit_NSImage")]
unsafe impl NSItemProviderWriting for NSImage {}

extern_protocol!(
    pub unsafe trait NSImageDelegate: NSObjectProtocol {
        #[cfg(feature = "AppKit_NSImage")]
        #[optional]
        #[method_id(@__retain_semantics Other imageDidNotDraw:inRect:)]
        unsafe fn imageDidNotDraw_inRect(
            &self,
            sender: &NSImage,
            rect: NSRect,
        ) -> Option<Id<NSImage>>;

        #[cfg(all(feature = "AppKit_NSImage", feature = "AppKit_NSImageRep"))]
        #[optional]
        #[method(image:willLoadRepresentation:)]
        unsafe fn image_willLoadRepresentation(&self, image: &NSImage, rep: &NSImageRep);

        #[cfg(all(feature = "AppKit_NSImage", feature = "AppKit_NSImageRep"))]
        #[optional]
        #[method(image:didLoadRepresentationHeader:)]
        unsafe fn image_didLoadRepresentationHeader(&self, image: &NSImage, rep: &NSImageRep);

        #[cfg(all(feature = "AppKit_NSImage", feature = "AppKit_NSImageRep"))]
        #[optional]
        #[method(image:didLoadPartOfRepresentation:withValidRows:)]
        unsafe fn image_didLoadPartOfRepresentation_withValidRows(
            &self,
            image: &NSImage,
            rep: &NSImageRep,
            rows: NSInteger,
        );

        #[cfg(all(feature = "AppKit_NSImage", feature = "AppKit_NSImageRep"))]
        #[optional]
        #[method(image:didLoadRepresentation:withStatus:)]
        unsafe fn image_didLoadRepresentation_withStatus(
            &self,
            image: &NSImage,
            rep: &NSImageRep,
            status: NSImageLoadStatus,
        );
    }

    unsafe impl ProtocolType for dyn NSImageDelegate {}
);

extern_methods!(
    /// NSBundleImageExtension
    #[cfg(feature = "Foundation_NSBundle")]
    unsafe impl NSBundle {
        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other imageForResource:)]
        pub unsafe fn imageForResource(&self, name: &NSImageName) -> Option<Id<NSImage>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other pathForImageResource:)]
        pub unsafe fn pathForImageResource(&self, name: &NSImageName) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other URLForImageResource:)]
        pub unsafe fn URLForImageResource(&self, name: &NSImageName) -> Option<Id<NSURL>>;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSImage")]
    unsafe impl NSImage {
        #[cfg(all(feature = "AppKit_NSImageRep", feature = "Foundation_NSDictionary"))]
        #[deprecated = "Use -[NSImage bestRepresentationForRect:context:hints:] instead.  Any deviceDescription dictionary is also a valid hints dictionary."]
        #[method_id(@__retain_semantics Other bestRepresentationForDevice:)]
        pub unsafe fn bestRepresentationForDevice(
            &self,
            device_description: Option<&NSDictionary>,
        ) -> Option<Id<NSImageRep>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[deprecated = "Use +imageUnfilteredTypes instead"]
        #[method_id(@__retain_semantics Other imageUnfilteredFileTypes)]
        pub unsafe fn imageUnfilteredFileTypes() -> Id<NSArray<NSString>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[deprecated = "Use +imageUnfilteredTypes instead"]
        #[method_id(@__retain_semantics Other imageUnfilteredPasteboardTypes)]
        pub unsafe fn imageUnfilteredPasteboardTypes() -> Id<NSArray<NSPasteboardType>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[deprecated = "Use +imageTypes instead"]
        #[method_id(@__retain_semantics Other imageFileTypes)]
        pub unsafe fn imageFileTypes() -> Id<NSArray<NSString>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[deprecated = "Use +imageTypes instead"]
        #[method_id(@__retain_semantics Other imagePasteboardTypes)]
        pub unsafe fn imagePasteboardTypes() -> Id<NSArray<NSPasteboardType>>;

        #[deprecated = "The concept of flippedness for NSImage is deprecated.  Please see the AppKit 10.6 release notes for a discussion of why and for how to replace existing usage."]
        #[method(setFlipped:)]
        pub unsafe fn setFlipped(&self, flag: bool);

        #[deprecated = "The concept of flippedness for NSImage is deprecated.  Please see the AppKit 10.6 release notes for a discussion of why and for how to replace existing usage."]
        #[method(isFlipped)]
        pub unsafe fn isFlipped(&self) -> bool;

        #[deprecated = "Use -drawAtPoint:... or -drawInRect:... methods instead"]
        #[method(dissolveToPoint:fraction:)]
        pub unsafe fn dissolveToPoint_fraction(&self, point: NSPoint, fraction: CGFloat);

        #[deprecated = "Use -drawAtPoint:... or -drawInRect:... methods instead"]
        #[method(dissolveToPoint:fromRect:fraction:)]
        pub unsafe fn dissolveToPoint_fromRect_fraction(
            &self,
            point: NSPoint,
            rect: NSRect,
            fraction: CGFloat,
        );

        #[deprecated = "Use -drawAtPoint:... or -drawInRect:... methods instead"]
        #[method(compositeToPoint:operation:)]
        pub unsafe fn compositeToPoint_operation(&self, point: NSPoint, op: NSCompositingOperation);

        #[deprecated = "Use -drawAtPoint:... or -drawInRect:... methods instead"]
        #[method(compositeToPoint:fromRect:operation:)]
        pub unsafe fn compositeToPoint_fromRect_operation(
            &self,
            point: NSPoint,
            rect: NSRect,
            op: NSCompositingOperation,
        );

        #[deprecated = "Use -drawAtPoint:... or -drawInRect:... methods instead"]
        #[method(compositeToPoint:operation:fraction:)]
        pub unsafe fn compositeToPoint_operation_fraction(
            &self,
            point: NSPoint,
            op: NSCompositingOperation,
            delta: CGFloat,
        );

        #[deprecated = "Use -drawAtPoint:... or -drawInRect:... methods instead"]
        #[method(compositeToPoint:fromRect:operation:fraction:)]
        pub unsafe fn compositeToPoint_fromRect_operation_fraction(
            &self,
            point: NSPoint,
            rect: NSRect,
            op: NSCompositingOperation,
            delta: CGFloat,
        );

        #[cfg(feature = "AppKit_NSImageRep")]
        #[deprecated = "Create an image using +[NSImage imageWithSize:flipped:drawingHandler:], and begin your custom drawing with -[NSImageRep drawInRect:] instead."]
        #[method(lockFocusOnRepresentation:)]
        pub unsafe fn lockFocusOnRepresentation(&self, image_representation: Option<&NSImageRep>);

        #[deprecated = "You should be able to remove use of this method without any replacement.  See 10.6 AppKit release notes for details."]
        #[method(setScalesWhenResized:)]
        pub unsafe fn setScalesWhenResized(&self, flag: bool);

        #[deprecated = "You should be able to remove use of this method without any replacement.  See 10.6 AppKit release notes for details."]
        #[method(scalesWhenResized)]
        pub unsafe fn scalesWhenResized(&self) -> bool;

        #[deprecated = "You should be able to remove use of this method without any replacement.  See 10.6 AppKit release notes for details."]
        #[method(setDataRetained:)]
        pub unsafe fn setDataRetained(&self, flag: bool);

        #[deprecated = "You should be able to remove use of this method without any replacement.  See 10.6 AppKit release notes for details."]
        #[method(isDataRetained)]
        pub unsafe fn isDataRetained(&self) -> bool;

        #[deprecated = "You should be able to remove use of this method without any replacement.  See 10.6 AppKit release notes for details."]
        #[method(setCachedSeparately:)]
        pub unsafe fn setCachedSeparately(&self, flag: bool);

        #[deprecated = "You should be able to remove use of this method without any replacement.  See 10.6 AppKit release notes for details."]
        #[method(isCachedSeparately)]
        pub unsafe fn isCachedSeparately(&self) -> bool;

        #[deprecated = "You should be able to remove use of this method without any replacement.  See 10.6 AppKit release notes for details."]
        #[method(setCacheDepthMatchesImageDepth:)]
        pub unsafe fn setCacheDepthMatchesImageDepth(&self, flag: bool);

        #[deprecated = "You should be able to remove use of this method without any replacement.  See 10.6 AppKit release notes for details."]
        #[method(cacheDepthMatchesImageDepth)]
        pub unsafe fn cacheDepthMatchesImageDepth(&self) -> bool;
    }
);

extern_static!(NSImageNameAddTemplate: &'static NSImageName);

extern_static!(NSImageNameBluetoothTemplate: &'static NSImageName);

extern_static!(NSImageNameBonjour: &'static NSImageName);

extern_static!(NSImageNameBookmarksTemplate: &'static NSImageName);

extern_static!(NSImageNameCaution: &'static NSImageName);

extern_static!(NSImageNameComputer: &'static NSImageName);

extern_static!(NSImageNameEnterFullScreenTemplate: &'static NSImageName);

extern_static!(NSImageNameExitFullScreenTemplate: &'static NSImageName);

extern_static!(NSImageNameFolder: &'static NSImageName);

extern_static!(NSImageNameFolderBurnable: &'static NSImageName);

extern_static!(NSImageNameFolderSmart: &'static NSImageName);

extern_static!(NSImageNameFollowLinkFreestandingTemplate: &'static NSImageName);

extern_static!(NSImageNameHomeTemplate: &'static NSImageName);

extern_static!(NSImageNameIChatTheaterTemplate: &'static NSImageName);

extern_static!(NSImageNameLockLockedTemplate: &'static NSImageName);

extern_static!(NSImageNameLockUnlockedTemplate: &'static NSImageName);

extern_static!(NSImageNameNetwork: &'static NSImageName);

extern_static!(NSImageNamePathTemplate: &'static NSImageName);

extern_static!(NSImageNameQuickLookTemplate: &'static NSImageName);

extern_static!(NSImageNameRefreshFreestandingTemplate: &'static NSImageName);

extern_static!(NSImageNameRefreshTemplate: &'static NSImageName);

extern_static!(NSImageNameRemoveTemplate: &'static NSImageName);

extern_static!(NSImageNameRevealFreestandingTemplate: &'static NSImageName);

extern_static!(NSImageNameShareTemplate: &'static NSImageName);

extern_static!(NSImageNameSlideshowTemplate: &'static NSImageName);

extern_static!(NSImageNameStatusAvailable: &'static NSImageName);

extern_static!(NSImageNameStatusNone: &'static NSImageName);

extern_static!(NSImageNameStatusPartiallyAvailable: &'static NSImageName);

extern_static!(NSImageNameStatusUnavailable: &'static NSImageName);

extern_static!(NSImageNameStopProgressFreestandingTemplate: &'static NSImageName);

extern_static!(NSImageNameStopProgressTemplate: &'static NSImageName);

extern_static!(NSImageNameTrashEmpty: &'static NSImageName);

extern_static!(NSImageNameTrashFull: &'static NSImageName);

extern_static!(NSImageNameActionTemplate: &'static NSImageName);

extern_static!(NSImageNameSmartBadgeTemplate: &'static NSImageName);

extern_static!(NSImageNameIconViewTemplate: &'static NSImageName);

extern_static!(NSImageNameListViewTemplate: &'static NSImageName);

extern_static!(NSImageNameColumnViewTemplate: &'static NSImageName);

extern_static!(NSImageNameFlowViewTemplate: &'static NSImageName);

extern_static!(NSImageNameInvalidDataFreestandingTemplate: &'static NSImageName);

extern_static!(NSImageNameGoForwardTemplate: &'static NSImageName);

extern_static!(NSImageNameGoBackTemplate: &'static NSImageName);

extern_static!(NSImageNameGoRightTemplate: &'static NSImageName);

extern_static!(NSImageNameGoLeftTemplate: &'static NSImageName);

extern_static!(NSImageNameRightFacingTriangleTemplate: &'static NSImageName);

extern_static!(NSImageNameLeftFacingTriangleTemplate: &'static NSImageName);

extern_static!(NSImageNameDotMac: &'static NSImageName);

extern_static!(NSImageNameMobileMe: &'static NSImageName);

extern_static!(NSImageNameMultipleDocuments: &'static NSImageName);

extern_static!(NSImageNameUserAccounts: &'static NSImageName);

extern_static!(NSImageNamePreferencesGeneral: &'static NSImageName);

extern_static!(NSImageNameAdvanced: &'static NSImageName);

extern_static!(NSImageNameInfo: &'static NSImageName);

extern_static!(NSImageNameFontPanel: &'static NSImageName);

extern_static!(NSImageNameColorPanel: &'static NSImageName);

extern_static!(NSImageNameUser: &'static NSImageName);

extern_static!(NSImageNameUserGroup: &'static NSImageName);

extern_static!(NSImageNameEveryone: &'static NSImageName);

extern_static!(NSImageNameUserGuest: &'static NSImageName);

extern_static!(NSImageNameMenuOnStateTemplate: &'static NSImageName);

extern_static!(NSImageNameMenuMixedStateTemplate: &'static NSImageName);

extern_static!(NSImageNameApplicationIcon: &'static NSImageName);

extern_static!(NSImageNameTouchBarAddDetailTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarAddTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarAlarmTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarAudioInputMuteTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarAudioInputTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarAudioOutputMuteTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarAudioOutputVolumeHighTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarAudioOutputVolumeLowTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarAudioOutputVolumeMediumTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarAudioOutputVolumeOffTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarBookmarksTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarColorPickerFill: &'static NSImageName);

extern_static!(NSImageNameTouchBarColorPickerFont: &'static NSImageName);

extern_static!(NSImageNameTouchBarColorPickerStroke: &'static NSImageName);

extern_static!(NSImageNameTouchBarCommunicationAudioTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarCommunicationVideoTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarComposeTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarDeleteTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarDownloadTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarEnterFullScreenTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarExitFullScreenTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarFastForwardTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarFolderCopyToTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarFolderMoveToTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarFolderTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarGetInfoTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarGoBackTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarGoDownTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarGoForwardTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarGoUpTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarHistoryTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarIconViewTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarListViewTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarMailTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarNewFolderTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarNewMessageTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarOpenInBrowserTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarPauseTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarPlayPauseTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarPlayTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarQuickLookTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarRecordStartTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarRecordStopTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarRefreshTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarRemoveTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarRewindTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarRotateLeftTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarRotateRightTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarSearchTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarShareTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarSidebarTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarSkipAhead15SecondsTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarSkipAhead30SecondsTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarSkipAheadTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarSkipBack15SecondsTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarSkipBack30SecondsTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarSkipBackTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarSkipToEndTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarSkipToStartTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarSlideshowTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarTagIconTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarTextBoldTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarTextBoxTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarTextCenterAlignTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarTextItalicTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarTextJustifiedAlignTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarTextLeftAlignTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarTextListTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarTextRightAlignTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarTextStrikethroughTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarTextUnderlineTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarUserAddTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarUserGroupTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarUserTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarVolumeDownTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarVolumeUpTemplate: &'static NSImageName);

extern_static!(NSImageNameTouchBarPlayheadTemplate: &'static NSImageName);

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSImageSymbolScale {
        NSImageSymbolScaleSmall = 1,
        NSImageSymbolScaleMedium = 2,
        NSImageSymbolScaleLarge = 3,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSImageSymbolConfiguration")]
    pub struct NSImageSymbolConfiguration;

    #[cfg(feature = "AppKit_NSImageSymbolConfiguration")]
    unsafe impl ClassType for NSImageSymbolConfiguration {
        type Super = NSObject;
    }
);

#[cfg(feature = "AppKit_NSImageSymbolConfiguration")]
unsafe impl NSCoding for NSImageSymbolConfiguration {}

#[cfg(feature = "AppKit_NSImageSymbolConfiguration")]
unsafe impl NSObjectProtocol for NSImageSymbolConfiguration {}

#[cfg(feature = "AppKit_NSImageSymbolConfiguration")]
unsafe impl NSSecureCoding for NSImageSymbolConfiguration {}

extern_methods!(
    #[cfg(feature = "AppKit_NSImageSymbolConfiguration")]
    unsafe impl NSImageSymbolConfiguration {
        #[method_id(@__retain_semantics Other configurationWithPointSize:weight:scale:)]
        pub unsafe fn configurationWithPointSize_weight_scale(
            point_size: CGFloat,
            weight: NSFontWeight,
            scale: NSImageSymbolScale,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Other configurationWithPointSize:weight:)]
        pub unsafe fn configurationWithPointSize_weight(
            point_size: CGFloat,
            weight: NSFontWeight,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Other configurationWithTextStyle:scale:)]
        pub unsafe fn configurationWithTextStyle_scale(
            style: &NSFontTextStyle,
            scale: NSImageSymbolScale,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Other configurationWithTextStyle:)]
        pub unsafe fn configurationWithTextStyle(style: &NSFontTextStyle) -> Id<Self>;

        #[method_id(@__retain_semantics Other configurationWithScale:)]
        pub unsafe fn configurationWithScale(scale: NSImageSymbolScale) -> Id<Self>;

        #[method_id(@__retain_semantics Other configurationPreferringMonochrome)]
        pub unsafe fn configurationPreferringMonochrome() -> Id<Self>;

        #[method_id(@__retain_semantics Other configurationPreferringHierarchical)]
        pub unsafe fn configurationPreferringHierarchical() -> Id<Self>;

        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Other configurationWithHierarchicalColor:)]
        pub unsafe fn configurationWithHierarchicalColor(hierarchical_color: &NSColor) -> Id<Self>;

        #[cfg(all(feature = "AppKit_NSColor", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other configurationWithPaletteColors:)]
        pub unsafe fn configurationWithPaletteColors(palette_colors: &NSArray<NSColor>)
            -> Id<Self>;

        #[method_id(@__retain_semantics Other configurationPreferringMulticolor)]
        pub unsafe fn configurationPreferringMulticolor() -> Id<Self>;

        #[method_id(@__retain_semantics Other configurationByApplyingConfiguration:)]
        pub unsafe fn configurationByApplyingConfiguration(
            &self,
            configuration: &NSImageSymbolConfiguration,
        ) -> Id<Self>;
    }
);
