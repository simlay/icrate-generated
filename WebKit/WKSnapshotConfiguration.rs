//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::WebKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "WebKit_WKSnapshotConfiguration")]
    pub struct WKSnapshotConfiguration;

    #[cfg(feature = "WebKit_WKSnapshotConfiguration")]
    unsafe impl ClassType for WKSnapshotConfiguration {
        type Super = NSObject;
    }
);

#[cfg(feature = "WebKit_WKSnapshotConfiguration")]
unsafe impl NSObjectProtocol for WKSnapshotConfiguration {}

extern_methods!(
    #[cfg(feature = "WebKit_WKSnapshotConfiguration")]
    unsafe impl WKSnapshotConfiguration {
        /**
          @abstract The rect to snapshot in view coordinates.
        @discussion This rect should be contained within WKWebView's bounds. If the rect is set to the
        null rect, the view's bounds will be used. The initial value is the null rect.
        */
        #[method(rect)]
        pub unsafe fn rect(&self) -> CGRect;

        /**
          @abstract The rect to snapshot in view coordinates.
        @discussion This rect should be contained within WKWebView's bounds. If the rect is set to the
        null rect, the view's bounds will be used. The initial value is the null rect.
        */
        #[method(setRect:)]
        pub unsafe fn setRect(&self, rect: CGRect);

        #[cfg(feature = "Foundation_NSNumber")]
        /**
          @abstract Specify a custom width to control the size of image you get back. The height will be
        computed to maintain the aspect ratio established by rect.
        @discussion snapshotWidth represents the width in points. If the snapshotWidth is nil, rect's
        width will be used.
        */
        #[method_id(@__retain_semantics Other snapshotWidth)]
        pub unsafe fn snapshotWidth(&self) -> Option<Id<NSNumber>>;

        #[cfg(feature = "Foundation_NSNumber")]
        /**
          @abstract Specify a custom width to control the size of image you get back. The height will be
        computed to maintain the aspect ratio established by rect.
        @discussion snapshotWidth represents the width in points. If the snapshotWidth is nil, rect's
        width will be used.
        */
        #[method(setSnapshotWidth:)]
        pub unsafe fn setSnapshotWidth(&self, snapshot_width: Option<&NSNumber>);

        /**
          @abstract A Boolean value that specifies whether the snapshot should be taken after recent
        changes have been incorporated. The value NO will capture the screen in its current state,
        which might not include recent changes.
        @discussion The default value is YES.
        */
        #[method(afterScreenUpdates)]
        pub unsafe fn afterScreenUpdates(&self) -> bool;

        /**
          @abstract A Boolean value that specifies whether the snapshot should be taken after recent
        changes have been incorporated. The value NO will capture the screen in its current state,
        which might not include recent changes.
        @discussion The default value is YES.
        */
        #[method(setAfterScreenUpdates:)]
        pub unsafe fn setAfterScreenUpdates(&self, after_screen_updates: bool);
    }
);
