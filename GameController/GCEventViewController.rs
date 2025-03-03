//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameController::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameController_GCEventViewController")]
    pub struct GCEventViewController;

    #[cfg(feature = "GameController_GCEventViewController")]
    unsafe impl ClassType for GCEventViewController {
        #[inherits(NSResponder, NSObject)]
        type Super = NSViewController;
    }
);

#[cfg(feature = "GameController_GCEventViewController")]
unsafe impl NSCoding for GCEventViewController {}

#[cfg(feature = "GameController_GCEventViewController")]
unsafe impl NSEditor for GCEventViewController {}

#[cfg(feature = "GameController_GCEventViewController")]
unsafe impl NSObjectProtocol for GCEventViewController {}

#[cfg(feature = "GameController_GCEventViewController")]
unsafe impl NSSeguePerforming for GCEventViewController {}

#[cfg(feature = "GameController_GCEventViewController")]
unsafe impl NSUserInterfaceItemIdentification for GCEventViewController {}

extern_methods!(
    #[cfg(feature = "GameController_GCEventViewController")]
    unsafe impl GCEventViewController {
        #[method(controllerUserInteractionEnabled)]
        pub unsafe fn controllerUserInteractionEnabled(&self) -> bool;

        #[method(setControllerUserInteractionEnabled:)]
        pub unsafe fn setControllerUserInteractionEnabled(
            &self,
            controller_user_interaction_enabled: bool,
        );
    }
);

extern_methods!(
    /// Methods declared on superclass `NSViewController`
    #[cfg(feature = "GameController_GCEventViewController")]
    unsafe impl GCEventViewController {
        #[cfg(feature = "Foundation_NSBundle")]
        #[method_id(@__retain_semantics Init initWithNibName:bundle:)]
        pub unsafe fn initWithNibName_bundle(
            this: Option<Allocated<Self>>,
            nib_name_or_nil: Option<&NSNibName>,
            nib_bundle_or_nil: Option<&NSBundle>,
        ) -> Id<Self>;
    }
);
