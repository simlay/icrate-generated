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

extern_methods!(
    #[cfg(feature = "AppKit_NSImage")]
    unsafe impl NSImage {
        #[method_id(@__retain_semantics Other imageNamed:)]
        pub unsafe fn imageNamed(name: &NSImageName) -> Option<Id<NSImage, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other imageWithSystemSymbolName:accessibilityDescription:)]
        pub unsafe fn imageWithSystemSymbolName_accessibilityDescription(
            name: &NSString,
            description: Option<&NSString>,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other imageWithSystemSymbolName:variableValue:accessibilityDescription:)]
        pub unsafe fn imageWithSystemSymbolName_variableValue_accessibilityDescription(
            name: &NSString,
            value: c_double,
            description: Option<&NSString>,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other imageWithSymbolName:variableValue:)]
        pub unsafe fn imageWithSymbolName_variableValue(
            name: &NSString,
            value: c_double,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(all(feature = "Foundation_NSBundle", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other imageWithSymbolName:bundle:variableValue:)]
        pub unsafe fn imageWithSymbolName_bundle_variableValue(
            name: &NSString,
            bundle: Option<&NSBundle>,
            value: c_double,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Init initWithSize:)]
        pub unsafe fn initWithSize(this: Option<Allocated<Self>>, size: NSSize)
            -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Init initWithData:)]
        pub unsafe fn initWithData(
            this: Option<Allocated<Self>>,
            data: &NSData,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithContentsOfFile:)]
        pub unsafe fn initWithContentsOfFile(
            this: Option<Allocated<Self>>,
            fileName: &NSString,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Init initWithContentsOfURL:)]
        pub unsafe fn initWithContentsOfURL(
            this: Option<Allocated<Self>>,
            url: &NSURL,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initByReferencingFile:)]
        pub unsafe fn initByReferencingFile(
            this: Option<Allocated<Self>>,
            fileName: &NSString,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Init initByReferencingURL:)]
        pub unsafe fn initByReferencingURL(
            this: Option<Allocated<Self>>,
            url: &NSURL,
        ) -> Id<Self, Shared>;

        #[cfg(feature = "AppKit_NSPasteboard")]
        #[method_id(@__retain_semantics Init initWithPasteboard:)]
        pub unsafe fn initWithPasteboard(
            this: Option<Allocated<Self>>,
            pasteboard: &NSPasteboard,
        ) -> Option<Id<Self, Shared>>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Init initWithDataIgnoringOrientation:)]
        pub unsafe fn initWithDataIgnoringOrientation(
            this: Option<Allocated<Self>>,
            data: &NSData,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Other imageWithSize:flipped:drawingHandler:)]
        pub unsafe fn imageWithSize_flipped_drawingHandler(
            size: NSSize,
            drawingHandlerShouldBeCalledWithFlippedContext: bool,
            drawingHandler: &Block<(NSRect,), Bool>,
        ) -> Id<Self, Shared>;

        #[method(size)]
        pub unsafe fn size(&self) -> NSSize;

        #[method(setSize:)]
        pub unsafe fn setSize(&self, size: NSSize);

        #[method(setName:)]
        pub unsafe fn setName(&self, string: Option<&NSImageName>) -> bool;

        #[method_id(@__retain_semantics Other name)]
        pub unsafe fn name(&self) -> Option<Id<NSImageName, Shared>>;

        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Other backgroundColor)]
        pub unsafe fn backgroundColor(&self) -> Id<NSColor, Shared>;

        #[cfg(feature = "AppKit_NSColor")]
        #[method(setBackgroundColor:)]
        pub unsafe fn setBackgroundColor(&self, backgroundColor: &NSColor);

        #[method(usesEPSOnResolutionMismatch)]
        pub unsafe fn usesEPSOnResolutionMismatch(&self) -> bool;

        #[method(setUsesEPSOnResolutionMismatch:)]
        pub unsafe fn setUsesEPSOnResolutionMismatch(&self, usesEPSOnResolutionMismatch: bool);

        #[method(prefersColorMatch)]
        pub unsafe fn prefersColorMatch(&self) -> bool;

        #[method(setPrefersColorMatch:)]
        pub unsafe fn setPrefersColorMatch(&self, prefersColorMatch: bool);

        #[method(matchesOnMultipleResolution)]
        pub unsafe fn matchesOnMultipleResolution(&self) -> bool;

        #[method(setMatchesOnMultipleResolution:)]
        pub unsafe fn setMatchesOnMultipleResolution(&self, matchesOnMultipleResolution: bool);

        #[method(matchesOnlyOnBestFittingAxis)]
        pub unsafe fn matchesOnlyOnBestFittingAxis(&self) -> bool;

        #[method(setMatchesOnlyOnBestFittingAxis:)]
        pub unsafe fn setMatchesOnlyOnBestFittingAxis(&self, matchesOnlyOnBestFittingAxis: bool);

        #[method(drawAtPoint:fromRect:operation:fraction:)]
        pub unsafe fn drawAtPoint_fromRect_operation_fraction(
            &self,
            point: NSPoint,
            fromRect: NSRect,
            op: NSCompositingOperation,
            delta: CGFloat,
        );

        #[method(drawInRect:fromRect:operation:fraction:)]
        pub unsafe fn drawInRect_fromRect_operation_fraction(
            &self,
            rect: NSRect,
            fromRect: NSRect,
            op: NSCompositingOperation,
            delta: CGFloat,
        );

        #[cfg(feature = "Foundation_NSDictionary")]
        #[method(drawInRect:fromRect:operation:fraction:respectFlipped:hints:)]
        pub unsafe fn drawInRect_fromRect_operation_fraction_respectFlipped_hints(
            &self,
            dstSpacePortionRect: NSRect,
            srcSpacePortionRect: NSRect,
            op: NSCompositingOperation,
            requestedAlpha: CGFloat,
            respectContextIsFlipped: bool,
            hints: Option<&NSDictionary<NSImageHintKey, Object>>,
        );

        #[cfg(feature = "AppKit_NSImageRep")]
        #[method(drawRepresentation:inRect:)]
        pub unsafe fn drawRepresentation_inRect(&self, imageRep: &NSImageRep, rect: NSRect)
            -> bool;

        #[method(drawInRect:)]
        pub unsafe fn drawInRect(&self, rect: NSRect);

        #[method(recache)]
        pub unsafe fn recache(&self);

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other TIFFRepresentation)]
        pub unsafe fn TIFFRepresentation(&self) -> Option<Id<NSData, Shared>>;

        #[cfg(feature = "Foundation_NSData")]
        #[method_id(@__retain_semantics Other TIFFRepresentationUsingCompression:factor:)]
        pub unsafe fn TIFFRepresentationUsingCompression_factor(
            &self,
            comp: NSTIFFCompression,
            factor: c_float,
        ) -> Option<Id<NSData, Shared>>;

        #[cfg(all(feature = "AppKit_NSImageRep", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other representations)]
        pub unsafe fn representations(&self) -> Id<NSArray<NSImageRep>, Shared>;

        #[cfg(all(feature = "AppKit_NSImageRep", feature = "Foundation_NSArray"))]
        #[method(addRepresentations:)]
        pub unsafe fn addRepresentations(&self, imageReps: &NSArray<NSImageRep>);

        #[cfg(feature = "AppKit_NSImageRep")]
        #[method(addRepresentation:)]
        pub unsafe fn addRepresentation(&self, imageRep: &NSImageRep);

        #[cfg(feature = "AppKit_NSImageRep")]
        #[method(removeRepresentation:)]
        pub unsafe fn removeRepresentation(&self, imageRep: &NSImageRep);

        #[method(isValid)]
        pub unsafe fn isValid(&self) -> bool;

        #[method(lockFocus)]
        pub unsafe fn lockFocus(&self);

        #[method(lockFocusFlipped:)]
        pub unsafe fn lockFocusFlipped(&self, flipped: bool);

        #[method(unlockFocus)]
        pub unsafe fn unlockFocus(&self);

        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSImageDelegate, Shared>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSImageDelegate>);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other imageTypes)]
        pub unsafe fn imageTypes() -> Id<NSArray<NSString>, Shared>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other imageUnfilteredTypes)]
        pub unsafe fn imageUnfilteredTypes() -> Id<NSArray<NSString>, Shared>;

        #[cfg(feature = "AppKit_NSPasteboard")]
        #[method(canInitWithPasteboard:)]
        pub unsafe fn canInitWithPasteboard(pasteboard: &NSPasteboard) -> bool;

        #[method(cancelIncrementalLoad)]
        pub unsafe fn cancelIncrementalLoad(&self);

        #[method(cacheMode)]
        pub unsafe fn cacheMode(&self) -> NSImageCacheMode;

        #[method(setCacheMode:)]
        pub unsafe fn setCacheMode(&self, cacheMode: NSImageCacheMode);

        #[method(alignmentRect)]
        pub unsafe fn alignmentRect(&self) -> NSRect;

        #[method(setAlignmentRect:)]
        pub unsafe fn setAlignmentRect(&self, alignmentRect: NSRect);

        #[method(isTemplate)]
        pub unsafe fn isTemplate(&self) -> bool;

        #[method(setTemplate:)]
        pub unsafe fn setTemplate(&self, template: bool);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other accessibilityDescription)]
        pub unsafe fn accessibilityDescription(&self) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setAccessibilityDescription:)]
        pub unsafe fn setAccessibilityDescription(
            &self,
            accessibilityDescription: Option<&NSString>,
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
            referenceContext: Option<&NSGraphicsContext>,
            hints: Option<&NSDictionary<NSImageHintKey, Object>>,
        ) -> Option<Id<NSImageRep, Shared>>;

        #[cfg(all(
            feature = "AppKit_NSGraphicsContext",
            feature = "Foundation_NSDictionary"
        ))]
        #[method(hitTestRect:withImageDestinationRect:context:hints:flipped:)]
        pub unsafe fn hitTestRect_withImageDestinationRect_context_hints_flipped(
            &self,
            testRectDestSpace: NSRect,
            imageRectDestSpace: NSRect,
            context: Option<&NSGraphicsContext>,
            hints: Option<&NSDictionary<NSImageHintKey, Object>>,
            flipped: bool,
        ) -> bool;

        #[method(recommendedLayerContentsScale:)]
        pub unsafe fn recommendedLayerContentsScale(
            &self,
            preferredContentsScale: CGFloat,
        ) -> CGFloat;

        #[method_id(@__retain_semantics Other layerContentsForContentsScale:)]
        pub unsafe fn layerContentsForContentsScale(
            &self,
            layerContentsScale: CGFloat,
        ) -> Id<Object, Shared>;

        #[method(capInsets)]
        pub unsafe fn capInsets(&self) -> NSEdgeInsets;

        #[method(setCapInsets:)]
        pub unsafe fn setCapInsets(&self, capInsets: NSEdgeInsets);

        #[method(resizingMode)]
        pub unsafe fn resizingMode(&self) -> NSImageResizingMode;

        #[method(setResizingMode:)]
        pub unsafe fn setResizingMode(&self, resizingMode: NSImageResizingMode);

        #[cfg(feature = "AppKit_NSImageSymbolConfiguration")]
        #[method_id(@__retain_semantics Other imageWithSymbolConfiguration:)]
        pub unsafe fn imageWithSymbolConfiguration(
            &self,
            configuration: &NSImageSymbolConfiguration,
        ) -> Option<Id<NSImage, Shared>>;

        #[cfg(feature = "AppKit_NSImageSymbolConfiguration")]
        #[method_id(@__retain_semantics Other symbolConfiguration)]
        pub unsafe fn symbolConfiguration(&self) -> Id<NSImageSymbolConfiguration, Shared>;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSImage")]
    unsafe impl NSImage {}
);

extern_methods!(
    #[cfg(feature = "AppKit_NSImage")]
    unsafe impl NSImage {}
);

extern_protocol!(
    pub struct NSImageDelegate;

    unsafe impl ProtocolType for NSImageDelegate {
        #[cfg(feature = "AppKit_NSImage")]
        #[optional]
        #[method_id(@__retain_semantics Other imageDidNotDraw:inRect:)]
        pub unsafe fn imageDidNotDraw_inRect(
            &self,
            sender: &NSImage,
            rect: NSRect,
        ) -> Option<Id<NSImage, Shared>>;

        #[cfg(all(feature = "AppKit_NSImage", feature = "AppKit_NSImageRep"))]
        #[optional]
        #[method(image:willLoadRepresentation:)]
        pub unsafe fn image_willLoadRepresentation(&self, image: &NSImage, rep: &NSImageRep);

        #[cfg(all(feature = "AppKit_NSImage", feature = "AppKit_NSImageRep"))]
        #[optional]
        #[method(image:didLoadRepresentationHeader:)]
        pub unsafe fn image_didLoadRepresentationHeader(&self, image: &NSImage, rep: &NSImageRep);

        #[cfg(all(feature = "AppKit_NSImage", feature = "AppKit_NSImageRep"))]
        #[optional]
        #[method(image:didLoadPartOfRepresentation:withValidRows:)]
        pub unsafe fn image_didLoadPartOfRepresentation_withValidRows(
            &self,
            image: &NSImage,
            rep: &NSImageRep,
            rows: NSInteger,
        );

        #[cfg(all(feature = "AppKit_NSImage", feature = "AppKit_NSImageRep"))]
        #[optional]
        #[method(image:didLoadRepresentation:withStatus:)]
        pub unsafe fn image_didLoadRepresentation_withStatus(
            &self,
            image: &NSImage,
            rep: &NSImageRep,
            status: NSImageLoadStatus,
        );
    }
);

extern_methods!(
    /// NSBundleImageExtension
    #[cfg(feature = "Foundation_NSBundle")]
    unsafe impl NSBundle {
        #[cfg(feature = "AppKit_NSImage")]
        #[method_id(@__retain_semantics Other imageForResource:)]
        pub unsafe fn imageForResource(&self, name: &NSImageName) -> Option<Id<NSImage, Shared>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other pathForImageResource:)]
        pub unsafe fn pathForImageResource(
            &self,
            name: &NSImageName,
        ) -> Option<Id<NSString, Shared>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[method_id(@__retain_semantics Other URLForImageResource:)]
        pub unsafe fn URLForImageResource(&self, name: &NSImageName) -> Option<Id<NSURL, Shared>>;
    }
);

extern_methods!(
    #[cfg(feature = "AppKit_NSImage")]
    unsafe impl NSImage {
        #[cfg(all(feature = "AppKit_NSImageRep", feature = "Foundation_NSDictionary"))]
        #[method_id(@__retain_semantics Other bestRepresentationForDevice:)]
        pub unsafe fn bestRepresentationForDevice(
            &self,
            deviceDescription: Option<&NSDictionary>,
        ) -> Option<Id<NSImageRep, Shared>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other imageUnfilteredFileTypes)]
        pub unsafe fn imageUnfilteredFileTypes() -> Id<NSArray<NSString>, Shared>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other imageUnfilteredPasteboardTypes)]
        pub unsafe fn imageUnfilteredPasteboardTypes() -> Id<NSArray<NSPasteboardType>, Shared>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other imageFileTypes)]
        pub unsafe fn imageFileTypes() -> Id<NSArray<NSString>, Shared>;

        #[cfg(feature = "Foundation_NSArray")]
        #[method_id(@__retain_semantics Other imagePasteboardTypes)]
        pub unsafe fn imagePasteboardTypes() -> Id<NSArray<NSPasteboardType>, Shared>;

        #[method(setFlipped:)]
        pub unsafe fn setFlipped(&self, flag: bool);

        #[method(isFlipped)]
        pub unsafe fn isFlipped(&self) -> bool;

        #[method(dissolveToPoint:fraction:)]
        pub unsafe fn dissolveToPoint_fraction(&self, point: NSPoint, fraction: CGFloat);

        #[method(dissolveToPoint:fromRect:fraction:)]
        pub unsafe fn dissolveToPoint_fromRect_fraction(
            &self,
            point: NSPoint,
            rect: NSRect,
            fraction: CGFloat,
        );

        #[method(compositeToPoint:operation:)]
        pub unsafe fn compositeToPoint_operation(&self, point: NSPoint, op: NSCompositingOperation);

        #[method(compositeToPoint:fromRect:operation:)]
        pub unsafe fn compositeToPoint_fromRect_operation(
            &self,
            point: NSPoint,
            rect: NSRect,
            op: NSCompositingOperation,
        );

        #[method(compositeToPoint:operation:fraction:)]
        pub unsafe fn compositeToPoint_operation_fraction(
            &self,
            point: NSPoint,
            op: NSCompositingOperation,
            delta: CGFloat,
        );

        #[method(compositeToPoint:fromRect:operation:fraction:)]
        pub unsafe fn compositeToPoint_fromRect_operation_fraction(
            &self,
            point: NSPoint,
            rect: NSRect,
            op: NSCompositingOperation,
            delta: CGFloat,
        );

        #[cfg(feature = "AppKit_NSImageRep")]
        #[method(lockFocusOnRepresentation:)]
        pub unsafe fn lockFocusOnRepresentation(&self, imageRepresentation: Option<&NSImageRep>);

        #[method(setScalesWhenResized:)]
        pub unsafe fn setScalesWhenResized(&self, flag: bool);

        #[method(scalesWhenResized)]
        pub unsafe fn scalesWhenResized(&self) -> bool;

        #[method(setDataRetained:)]
        pub unsafe fn setDataRetained(&self, flag: bool);

        #[method(isDataRetained)]
        pub unsafe fn isDataRetained(&self) -> bool;

        #[method(setCachedSeparately:)]
        pub unsafe fn setCachedSeparately(&self, flag: bool);

        #[method(isCachedSeparately)]
        pub unsafe fn isCachedSeparately(&self) -> bool;

        #[method(setCacheDepthMatchesImageDepth:)]
        pub unsafe fn setCacheDepthMatchesImageDepth(&self, flag: bool);

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

extern_methods!(
    #[cfg(feature = "AppKit_NSImageSymbolConfiguration")]
    unsafe impl NSImageSymbolConfiguration {
        #[method_id(@__retain_semantics Other configurationWithPointSize:weight:scale:)]
        pub unsafe fn configurationWithPointSize_weight_scale(
            pointSize: CGFloat,
            weight: NSFontWeight,
            scale: NSImageSymbolScale,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other configurationWithPointSize:weight:)]
        pub unsafe fn configurationWithPointSize_weight(
            pointSize: CGFloat,
            weight: NSFontWeight,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other configurationWithTextStyle:scale:)]
        pub unsafe fn configurationWithTextStyle_scale(
            style: &NSFontTextStyle,
            scale: NSImageSymbolScale,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other configurationWithTextStyle:)]
        pub unsafe fn configurationWithTextStyle(style: &NSFontTextStyle) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other configurationWithScale:)]
        pub unsafe fn configurationWithScale(scale: NSImageSymbolScale) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other configurationPreferringMonochrome)]
        pub unsafe fn configurationPreferringMonochrome() -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other configurationPreferringHierarchical)]
        pub unsafe fn configurationPreferringHierarchical() -> Id<Self, Shared>;

        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Other configurationWithHierarchicalColor:)]
        pub unsafe fn configurationWithHierarchicalColor(
            hierarchicalColor: &NSColor,
        ) -> Id<Self, Shared>;

        #[cfg(all(feature = "AppKit_NSColor", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Other configurationWithPaletteColors:)]
        pub unsafe fn configurationWithPaletteColors(
            paletteColors: &NSArray<NSColor>,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other configurationPreferringMulticolor)]
        pub unsafe fn configurationPreferringMulticolor() -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Other configurationByApplyingConfiguration:)]
        pub unsafe fn configurationByApplyingConfiguration(
            &self,
            configuration: &NSImageSymbolConfiguration,
        ) -> Id<Self, Shared>;
    }
);
