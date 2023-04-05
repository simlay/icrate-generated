//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::ExtensionKit::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "ExtensionKit_EXHostViewController")]
    #[cfg(not(any(target_os = "ios", target_os = "tvos", target_os = "watchos")))]
    pub struct EXHostViewController;

    #[cfg(feature = "ExtensionKit_EXHostViewController")]
    unsafe impl ClassType for EXHostViewController {
        #[inherits(NSResponder, NSObject)]
        type Super = NSViewController;
    }
);

#[cfg(feature = "ExtensionKit_EXHostViewController")]
unsafe impl NSCoding for EXHostViewController {}

#[cfg(feature = "ExtensionKit_EXHostViewController")]
unsafe impl NSEditor for EXHostViewController {}

#[cfg(feature = "ExtensionKit_EXHostViewController")]
unsafe impl NSObjectProtocol for EXHostViewController {}

#[cfg(feature = "ExtensionKit_EXHostViewController")]
unsafe impl NSSeguePerforming for EXHostViewController {}

#[cfg(feature = "ExtensionKit_EXHostViewController")]
unsafe impl NSUserInterfaceItemIdentification for EXHostViewController {}

extern_methods!(
    #[cfg(feature = "ExtensionKit_EXHostViewController")]
    unsafe impl EXHostViewController {
        #[method_id(@__retain_semantics Other delegate)]
        pub unsafe fn delegate(
            &self,
        ) -> Option<Id<ProtocolObject<dyn EXHostViewControllerDelegate>>>;

        #[method(setDelegate:)]
        pub unsafe fn setDelegate(
            &self,
            delegate: Option<&ProtocolObject<dyn EXHostViewControllerDelegate>>,
        );

        #[cfg(feature = "AppKit_NSView")]
        #[method_id(@__retain_semantics Other placeholderView)]
        pub unsafe fn placeholderView(&self) -> Id<NSView>;

        #[cfg(feature = "AppKit_NSView")]
        #[method(setPlaceholderView:)]
        pub unsafe fn setPlaceholderView(&self, placeholder_view: &NSView);

        #[cfg(all(feature = "Foundation_NSError", feature = "Foundation_NSXPCConnection"))]
        #[method_id(@__retain_semantics Other makeXPCConnectionWithError:_)]
        pub unsafe fn makeXPCConnectionWithError(&self)
            -> Result<Id<NSXPCConnection>, Id<NSError>>;
    }
);

extern_protocol!(
    #[cfg(not(any(target_os = "ios", target_os = "tvos", target_os = "watchos")))]
    pub unsafe trait EXHostViewControllerDelegate: NSObjectProtocol {
        #[cfg(feature = "ExtensionKit_EXHostViewController")]
        #[optional]
        #[method(hostViewControllerDidActivate:)]
        unsafe fn hostViewControllerDidActivate(&self, view_controller: &EXHostViewController);

        #[cfg(all(
            feature = "ExtensionKit_EXHostViewController",
            feature = "Foundation_NSError"
        ))]
        #[optional]
        #[method(hostViewControllerWillDeactivate:error:)]
        unsafe fn hostViewControllerWillDeactivate_error(
            &self,
            view_controller: &EXHostViewController,
            error: Option<&NSError>,
        );
    }

    #[cfg(not(any(target_os = "ios", target_os = "tvos", target_os = "watchos")))]
    unsafe impl ProtocolType for dyn EXHostViewControllerDelegate {}
);

extern_methods!(
    /// Methods declared on superclass `NSViewController`
    #[cfg(feature = "ExtensionKit_EXHostViewController")]
    unsafe impl EXHostViewController {
        #[cfg(feature = "Foundation_NSBundle")]
        #[method_id(@__retain_semantics Init initWithNibName:bundle:)]
        pub unsafe fn initWithNibName_bundle(
            this: Option<Allocated<Self>>,
            nib_name_or_nil: Option<&NSNibName>,
            nib_bundle_or_nil: Option<&NSBundle>,
        ) -> Id<Self>;
    }
);
