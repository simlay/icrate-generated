//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

typed_extensible_enum!(
    pub type NSSharingServiceName = NSString;
);

extern_static!(NSSharingServiceNameComposeEmail: &'static NSSharingServiceName);

extern_static!(NSSharingServiceNameComposeMessage: &'static NSSharingServiceName);

extern_static!(NSSharingServiceNameSendViaAirDrop: &'static NSSharingServiceName);

extern_static!(NSSharingServiceNameAddToSafariReadingList: &'static NSSharingServiceName);

extern_static!(NSSharingServiceNameAddToIPhoto: &'static NSSharingServiceName);

extern_static!(NSSharingServiceNameAddToAperture: &'static NSSharingServiceName);

extern_static!(NSSharingServiceNameUseAsDesktopPicture: &'static NSSharingServiceName);

extern_static!(NSSharingServiceNamePostOnFacebook: &'static NSSharingServiceName);

extern_static!(NSSharingServiceNamePostOnTwitter: &'static NSSharingServiceName);

extern_static!(NSSharingServiceNamePostOnSinaWeibo: &'static NSSharingServiceName);

extern_static!(NSSharingServiceNamePostOnTencentWeibo: &'static NSSharingServiceName);

extern_static!(NSSharingServiceNamePostOnLinkedIn: &'static NSSharingServiceName);

extern_static!(NSSharingServiceNameUseAsTwitterProfileImage: &'static NSSharingServiceName);

extern_static!(NSSharingServiceNameUseAsFacebookProfileImage: &'static NSSharingServiceName);

extern_static!(NSSharingServiceNameUseAsLinkedInProfileImage: &'static NSSharingServiceName);

extern_static!(NSSharingServiceNamePostImageOnFlickr: &'static NSSharingServiceName);

extern_static!(NSSharingServiceNamePostVideoOnVimeo: &'static NSSharingServiceName);

extern_static!(NSSharingServiceNamePostVideoOnYouku: &'static NSSharingServiceName);

extern_static!(NSSharingServiceNamePostVideoOnTudou: &'static NSSharingServiceName);

extern_static!(NSSharingServiceNameCloudSharing: &'static NSSharingServiceName);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSSharingService")]
    #[cfg(not(any(target_os = "ios")))]
    pub struct NSSharingService;

    #[cfg(feature = "AppKit_NSSharingService")]
    unsafe impl ClassType for NSSharingService {
        type Super = NSObject;
    }
);

#[cfg(feature = "AppKit_NSSharingService")]
unsafe impl NSObjectProtocol for NSSharingService {}

extern_methods!(
    #[cfg(feature = "AppKit_NSSharingService")]
    unsafe impl NSSharingService {
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<ProtocolObject<dyn NSSharingServiceDelegate>>>;

        #[cfg(not(any(target_os = "ios")))]
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn NSSharingServiceDelegate>>,
        );

        #[cfg(feature = "Foundation_NSString")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Id<NSString>;

        #[cfg(feature = "AppKit_NSImage")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other image)]
        pub unsafe fn image(&self) -> Id<NSImage>;

        #[cfg(feature = "AppKit_NSImage")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other alternateImage)]
        pub unsafe fn alternateImage(&self) -> Option<Id<NSImage>>;

        #[cfg(feature = "Foundation_NSString")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other menuItemTitle)]
        pub unsafe fn menuItemTitle(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[cfg(not(any(target_os = "ios")))]
        #[method(setMenuItemTitle:)]
        pub unsafe fn setMenuItemTitle(&self, menu_item_title: &NSString);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other recipients)]
        pub unsafe fn recipients(&self) -> Option<Id<NSArray<NSString>>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSString"))]
        #[cfg(not(any(target_os = "ios")))]
        #[method(setRecipients:)]
        pub unsafe fn setRecipients(&self, recipients: Option<&NSArray<NSString>>);

        #[cfg(feature = "Foundation_NSString")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other subject)]
        pub unsafe fn subject(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[cfg(not(any(target_os = "ios")))]
        #[method(setSubject:)]
        pub unsafe fn setSubject(&self, subject: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other messageBody)]
        pub unsafe fn messageBody(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSURL")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other permanentLink)]
        pub unsafe fn permanentLink(&self) -> Option<Id<NSURL>>;

        #[cfg(feature = "Foundation_NSString")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other accountName)]
        pub unsafe fn accountName(&self) -> Option<Id<NSString>>;

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSURL"))]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other attachmentFileURLs)]
        pub unsafe fn attachmentFileURLs(&self) -> Option<Id<NSArray<NSURL>>>;

        #[cfg(feature = "Foundation_NSArray")]
        #[deprecated = "Use -[NSSharingServicePicker standardShareMenuItem] instead."]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other sharingServicesForItems:)]
        pub unsafe fn sharingServicesForItems(items: &NSArray) -> Id<NSArray<NSSharingService>>;

        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other sharingServiceNamed:)]
        pub unsafe fn sharingServiceNamed(
            service_name: &NSSharingServiceName,
        ) -> Option<Id<NSSharingService>>;

        #[cfg(all(feature = "AppKit_NSImage", feature = "Foundation_NSString"))]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Init initWithTitle:image:alternateImage:handler:)]
        pub unsafe fn initWithTitle_image_alternateImage_handler(
            this: Option<Allocated<Self>>,
            title: &NSString,
            image: &NSImage,
            alternate_image: Option<&NSImage>,
            block: &Block<(), ()>,
        ) -> Id<Self>;

        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[cfg(feature = "Foundation_NSArray")]
        #[cfg(not(any(target_os = "ios")))]
        #[method(canPerformWithItems:)]
        pub unsafe fn canPerformWithItems(&self, items: Option<&NSArray>) -> bool;

        #[cfg(feature = "Foundation_NSArray")]
        #[cfg(not(any(target_os = "ios")))]
        #[method(performWithItems:)]
        pub unsafe fn performWithItems(&self, items: &NSArray);
    }
);

ns_enum!(
    #[underlying(NSInteger)]
    #[cfg(not(any(target_os = "ios")))]
    pub enum NSSharingContentScope {
        #[cfg(not(any(target_os = "ios")))]
        NSSharingContentScopeItem = 0,
        #[cfg(not(any(target_os = "ios")))]
        NSSharingContentScopePartial = 1,
        #[cfg(not(any(target_os = "ios")))]
        NSSharingContentScopeFull = 2,
    }
);

extern_protocol!(
    #[cfg(not(any(target_os = "ios")))]
    pub unsafe trait NSSharingServiceDelegate: NSObjectProtocol {
        #[cfg(all(feature = "AppKit_NSSharingService", feature = "Foundation_NSArray"))]
        #[cfg(not(any(target_os = "ios")))]
        #[optional]
        #[method(sharingService:willShareItems:)]
        unsafe fn sharingService_willShareItems(
            &self,
            sharing_service: &NSSharingService,
            items: &NSArray,
        );

        #[cfg(all(
            feature = "AppKit_NSSharingService",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError"
        ))]
        #[cfg(not(any(target_os = "ios")))]
        #[optional]
        #[method(sharingService:didFailToShareItems:error:)]
        unsafe fn sharingService_didFailToShareItems_error(
            &self,
            sharing_service: &NSSharingService,
            items: &NSArray,
            error: &NSError,
        );

        #[cfg(all(feature = "AppKit_NSSharingService", feature = "Foundation_NSArray"))]
        #[cfg(not(any(target_os = "ios")))]
        #[optional]
        #[method(sharingService:didShareItems:)]
        unsafe fn sharingService_didShareItems(
            &self,
            sharing_service: &NSSharingService,
            items: &NSArray,
        );

        #[cfg(feature = "AppKit_NSSharingService")]
        #[cfg(not(any(target_os = "ios")))]
        #[optional]
        #[method(sharingService:sourceFrameOnScreenForShareItem:)]
        unsafe fn sharingService_sourceFrameOnScreenForShareItem(
            &self,
            sharing_service: &NSSharingService,
            item: &Object,
        ) -> NSRect;

        #[cfg(all(feature = "AppKit_NSImage", feature = "AppKit_NSSharingService"))]
        #[cfg(not(any(target_os = "ios")))]
        #[optional]
        #[method_id(@__retain_semantics Other sharingService:transitionImageForShareItem:contentRect:)]
        unsafe fn sharingService_transitionImageForShareItem_contentRect(
            &self,
            sharing_service: &NSSharingService,
            item: &Object,
            content_rect: NonNull<NSRect>,
        ) -> Option<Id<NSImage>>;

        #[cfg(all(
            feature = "AppKit_NSSharingService",
            feature = "AppKit_NSWindow",
            feature = "Foundation_NSArray"
        ))]
        #[cfg(not(any(target_os = "ios")))]
        #[optional]
        #[method_id(@__retain_semantics Other sharingService:sourceWindowForShareItems:sharingContentScope:)]
        unsafe fn sharingService_sourceWindowForShareItems_sharingContentScope(
            &self,
            sharing_service: &NSSharingService,
            items: &NSArray,
            sharing_content_scope: NonNull<NSSharingContentScope>,
        ) -> Option<Id<NSWindow>>;

        #[cfg(all(feature = "AppKit_NSSharingService", feature = "AppKit_NSView"))]
        #[cfg(not(any(target_os = "ios")))]
        #[optional]
        #[method_id(@__retain_semantics Other anchoringViewForSharingService:showRelativeToRect:preferredEdge:)]
        unsafe fn anchoringViewForSharingService_showRelativeToRect_preferredEdge(
            &self,
            sharing_service: &NSSharingService,
            positioning_rect: NonNull<NSRect>,
            preferred_edge: NonNull<NSRectEdge>,
        ) -> Option<Id<NSView>>;
    }

    unsafe impl ProtocolType for dyn NSSharingServiceDelegate {}
);

ns_options!(
    #[underlying(NSUInteger)]
    #[cfg(not(any(target_os = "ios")))]
    pub enum NSCloudKitSharingServiceOptions {
        #[cfg(not(any(target_os = "ios")))]
        NSCloudKitSharingServiceStandard = 0,
        #[cfg(not(any(target_os = "ios")))]
        NSCloudKitSharingServiceAllowPublic = 1 << 0,
        #[cfg(not(any(target_os = "ios")))]
        NSCloudKitSharingServiceAllowPrivate = 1 << 1,
        #[cfg(not(any(target_os = "ios")))]
        NSCloudKitSharingServiceAllowReadOnly = 1 << 4,
        #[cfg(not(any(target_os = "ios")))]
        NSCloudKitSharingServiceAllowReadWrite = 1 << 5,
    }
);

extern_protocol!(
    #[cfg(not(any(target_os = "ios")))]
    pub unsafe trait NSCloudSharingServiceDelegate: NSSharingServiceDelegate {
        #[cfg(all(
            feature = "AppKit_NSSharingService",
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError"
        ))]
        #[cfg(not(any(target_os = "ios")))]
        #[optional]
        #[method(sharingService:didCompleteForItems:error:)]
        unsafe fn sharingService_didCompleteForItems_error(
            &self,
            sharing_service: &NSSharingService,
            items: &NSArray,
            error: Option<&NSError>,
        );

        #[cfg(all(
            feature = "AppKit_NSSharingService",
            feature = "Foundation_NSItemProvider"
        ))]
        #[cfg(not(any(target_os = "ios")))]
        #[optional]
        #[method(optionsForSharingService:shareProvider:)]
        unsafe fn optionsForSharingService_shareProvider(
            &self,
            cloud_kit_sharing_service: &NSSharingService,
            provider: &NSItemProvider,
        ) -> NSCloudKitSharingServiceOptions;
    }

    unsafe impl ProtocolType for dyn NSCloudSharingServiceDelegate {}
);

extern_methods!(
    /// NSCloudKitSharing
    #[cfg(feature = "Foundation_NSItemProvider")]
    unsafe impl NSItemProvider {}
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSSharingServicePicker")]
    #[cfg(not(any(target_os = "ios")))]
    pub struct NSSharingServicePicker;

    #[cfg(feature = "AppKit_NSSharingServicePicker")]
    unsafe impl ClassType for NSSharingServicePicker {
        type Super = NSObject;
    }
);

#[cfg(feature = "AppKit_NSSharingServicePicker")]
unsafe impl NSObjectProtocol for NSSharingServicePicker {}

extern_methods!(
    #[cfg(feature = "AppKit_NSSharingServicePicker")]
    unsafe impl NSSharingServicePicker {
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Id<ProtocolObject<dyn NSSharingServicePickerDelegate>>>;

        #[cfg(not(any(target_os = "ios")))]
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn NSSharingServicePickerDelegate>>,
        );

        #[cfg(feature = "Foundation_NSArray")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Init initWithItems:)]
        pub unsafe fn initWithItems(this: Option<Allocated<Self>>, items: &NSArray) -> Id<Self>;

        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[cfg(feature = "AppKit_NSView")]
        #[cfg(not(any(target_os = "ios")))]
        #[method(showRelativeToRect:ofView:preferredEdge:)]
        pub unsafe fn showRelativeToRect_ofView_preferredEdge(
            &self,
            rect: NSRect,
            view: &NSView,
            preferred_edge: NSRectEdge,
        );

        #[cfg(not(any(target_os = "ios")))]
        #[method(close)]
        pub unsafe fn close(&self);

        #[cfg(feature = "AppKit_NSMenuItem")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other standardShareMenuItem)]
        pub unsafe fn standardShareMenuItem(&self) -> Id<NSMenuItem>;
    }
);

extern_protocol!(
    #[cfg(not(any(target_os = "ios")))]
    pub unsafe trait NSSharingServicePickerDelegate: NSObjectProtocol {
        #[cfg(all(
            feature = "AppKit_NSSharingService",
            feature = "AppKit_NSSharingServicePicker",
            feature = "Foundation_NSArray"
        ))]
        #[cfg(not(any(target_os = "ios")))]
        #[optional]
        #[method_id(@__retain_semantics Other sharingServicePicker:sharingServicesForItems:proposedSharingServices:)]
        unsafe fn sharingServicePicker_sharingServicesForItems_proposedSharingServices(
            &self,
            sharing_service_picker: &NSSharingServicePicker,
            items: &NSArray,
            proposed_services: &NSArray<NSSharingService>,
        ) -> Id<NSArray<NSSharingService>>;

        #[cfg(all(
            feature = "AppKit_NSSharingService",
            feature = "AppKit_NSSharingServicePicker"
        ))]
        #[cfg(not(any(target_os = "ios")))]
        #[optional]
        #[method_id(@__retain_semantics Other sharingServicePicker:delegateForSharingService:)]
        unsafe fn sharingServicePicker_delegateForSharingService(
            &self,
            sharing_service_picker: &NSSharingServicePicker,
            sharing_service: &NSSharingService,
        ) -> Option<Id<ProtocolObject<dyn NSSharingServiceDelegate>>>;

        #[cfg(all(
            feature = "AppKit_NSSharingService",
            feature = "AppKit_NSSharingServicePicker"
        ))]
        #[cfg(not(any(target_os = "ios")))]
        #[optional]
        #[method(sharingServicePicker:didChooseSharingService:)]
        unsafe fn sharingServicePicker_didChooseSharingService(
            &self,
            sharing_service_picker: &NSSharingServicePicker,
            service: Option<&NSSharingService>,
        );
    }

    unsafe impl ProtocolType for dyn NSSharingServicePickerDelegate {}
);
