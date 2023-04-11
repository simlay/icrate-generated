//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::InputMethodKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "InputMethodKit_IMKServer")]
    /**
     @class      IMKServer
    @abstract   This class manages input sessions.
    @discussion An input method should create one and only one of these objects.  An IMKServer creates an NSConnection that can be connected to by input clients.  After a connection has been made an IMKServer manages communication between the client and the input method.  For each communication session the IMKServer will create an IMKInputController class as well as delegate classes for that controller.  Each controller object then serves as a proxy for the input session on the client side.  This means that input methods do not have to concern themselves with managing client sessions.  A given controller will only receive communication from a single session.

    IMKServer's also will manage a basic candidate window for an input method.  See IMKCandidates.h to understand how to create a candidate window and associate the candidate window with the IMKServer object.
    */
    pub struct IMKServer;

    #[cfg(feature = "InputMethodKit_IMKServer")]
    unsafe impl ClassType for IMKServer {
        type Super = NSObject;
    }
);

#[cfg(feature = "InputMethodKit_IMKServer")]
/**
 @class      IMKServer
@abstract   This class manages input sessions.
@discussion An input method should create one and only one of these objects.  An IMKServer creates an NSConnection that can be connected to by input clients.  After a connection has been made an IMKServer manages communication between the client and the input method.  For each communication session the IMKServer will create an IMKInputController class as well as delegate classes for that controller.  Each controller object then serves as a proxy for the input session on the client side.  This means that input methods do not have to concern themselves with managing client sessions.  A given controller will only receive communication from a single session.

IMKServer's also will manage a basic candidate window for an input method.  See IMKCandidates.h to understand how to create a candidate window and associate the candidate window with the IMKServer object.
*/
unsafe impl NSObjectProtocol for IMKServer {}

extern_methods!(
    /**
     @class      IMKServer
    @abstract   This class manages input sessions.
    @discussion An input method should create one and only one of these objects.  An IMKServer creates an NSConnection that can be connected to by input clients.  After a connection has been made an IMKServer manages communication between the client and the input method.  For each communication session the IMKServer will create an IMKInputController class as well as delegate classes for that controller.  Each controller object then serves as a proxy for the input session on the client side.  This means that input methods do not have to concern themselves with managing client sessions.  A given controller will only receive communication from a single session.

    IMKServer's also will manage a basic candidate window for an input method.  See IMKCandidates.h to understand how to create a candidate window and associate the candidate window with the IMKServer object.
    */
    #[cfg(feature = "InputMethodKit_IMKServer")]
    unsafe impl IMKServer {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithName:bundleIdentifier:)]
        pub unsafe fn initWithName_bundleIdentifier(
            this: Option<Allocated<Self>>,
            name: Option<&NSString>,
            bundle_identifier: Option<&NSString>,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithName:controllerClass:delegateClass:)]
        pub unsafe fn initWithName_controllerClass_delegateClass(
            this: Option<Allocated<Self>>,
            name: Option<&NSString>,
            controller_class_id: Option<&Class>,
            delegate_class_id: Option<&Class>,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSBundle")]
        #[method_id(@__retain_semantics Other bundle)]
        pub unsafe fn bundle(&self) -> Option<Id<NSBundle>>;

        #[method(paletteWillTerminate)]
        pub unsafe fn paletteWillTerminate(&self) -> bool;

        #[method(lastKeyEventWasDeadKey)]
        pub unsafe fn lastKeyEventWasDeadKey(&self) -> bool;
    }
);
