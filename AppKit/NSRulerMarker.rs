//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSRulerMarker")]
    #[cfg(not(any(target_os = "ios")))]
    pub struct NSRulerMarker;

    #[cfg(feature = "AppKit_NSRulerMarker")]
    unsafe impl ClassType for NSRulerMarker {
        type Super = NSObject;
    }
);

#[cfg(feature = "AppKit_NSRulerMarker")]
unsafe impl NSCoding for NSRulerMarker {}

#[cfg(feature = "AppKit_NSRulerMarker")]
unsafe impl NSObjectProtocol for NSRulerMarker {}

extern_methods!(
    #[cfg(feature = "AppKit_NSRulerMarker")]
    unsafe impl NSRulerMarker {
        #[cfg(all(feature = "AppKit_NSImage", feature = "AppKit_NSRulerView"))]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Init initWithRulerView:markerLocation:image:imageOrigin:)]
        pub unsafe fn initWithRulerView_markerLocation_image_imageOrigin(
            this: Option<Allocated<Self>>,
            ruler: &NSRulerView,
            location: CGFloat,
            image: &NSImage,
            image_origin: NSPoint,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Option<Allocated<Self>>, coder: &NSCoder) -> Id<Self>;

        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[cfg(feature = "AppKit_NSRulerView")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other ruler)]
        pub unsafe fn ruler(&self) -> Option<Id<NSRulerView>>;

        #[cfg(not(any(target_os = "ios")))]
        #[method(markerLocation)]
        pub unsafe fn markerLocation(&self) -> CGFloat;

        #[cfg(not(any(target_os = "ios")))]
        #[method(setMarkerLocation:)]
        pub unsafe fn setMarkerLocation(&self, marker_location: CGFloat);

        #[cfg(feature = "AppKit_NSImage")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other image)]
        pub unsafe fn image(&self) -> Id<NSImage>;

        #[cfg(feature = "AppKit_NSImage")]
        #[cfg(not(any(target_os = "ios")))]
        #[method(setImage:)]
        pub unsafe fn setImage(&self, image: &NSImage);

        #[cfg(not(any(target_os = "ios")))]
        #[method(imageOrigin)]
        pub unsafe fn imageOrigin(&self) -> NSPoint;

        #[cfg(not(any(target_os = "ios")))]
        #[method(setImageOrigin:)]
        pub unsafe fn setImageOrigin(&self, image_origin: NSPoint);

        #[cfg(not(any(target_os = "ios")))]
        #[method(isMovable)]
        pub unsafe fn isMovable(&self) -> bool;

        #[cfg(not(any(target_os = "ios")))]
        #[method(setMovable:)]
        pub unsafe fn setMovable(&self, movable: bool);

        #[cfg(not(any(target_os = "ios")))]
        #[method(isRemovable)]
        pub unsafe fn isRemovable(&self) -> bool;

        #[cfg(not(any(target_os = "ios")))]
        #[method(setRemovable:)]
        pub unsafe fn setRemovable(&self, removable: bool);

        #[cfg(not(any(target_os = "ios")))]
        #[method(isDragging)]
        pub unsafe fn isDragging(&self) -> bool;

        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Other representedObject)]
        pub unsafe fn representedObject(&self) -> Option<Id<Object>>;

        #[cfg(not(any(target_os = "ios")))]
        #[method(setRepresentedObject:)]
        pub unsafe fn setRepresentedObject(&self, represented_object: Option<&Object>);

        #[cfg(not(any(target_os = "ios")))]
        #[method(imageRectInRuler)]
        pub unsafe fn imageRectInRuler(&self) -> NSRect;

        #[cfg(not(any(target_os = "ios")))]
        #[method(thicknessRequiredInRuler)]
        pub unsafe fn thicknessRequiredInRuler(&self) -> CGFloat;

        #[cfg(not(any(target_os = "ios")))]
        #[method(drawRect:)]
        pub unsafe fn drawRect(&self, rect: NSRect);

        #[cfg(feature = "AppKit_NSEvent")]
        #[cfg(not(any(target_os = "ios")))]
        #[method(trackMouse:adding:)]
        pub unsafe fn trackMouse_adding(&self, mouse_down_event: &NSEvent, is_adding: bool)
            -> bool;
    }
);
