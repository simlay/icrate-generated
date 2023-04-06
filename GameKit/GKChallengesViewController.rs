//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameKit_GKChallengesViewController")]
    #[deprecated]
    pub struct GKChallengesViewController;

    #[deprecated]
    #[cfg(feature = "GameKit_GKChallengesViewController")]
    unsafe impl ClassType for GKChallengesViewController {
        #[inherits(NSResponder, NSObject)]
        type Super = NSViewController;
    }
);

#[cfg(feature = "GameKit_GKChallengesViewController")]
unsafe impl GKViewController for GKChallengesViewController {}

#[cfg(feature = "GameKit_GKChallengesViewController")]
unsafe impl NSCoding for GKChallengesViewController {}

#[cfg(feature = "GameKit_GKChallengesViewController")]
unsafe impl NSEditor for GKChallengesViewController {}

#[cfg(feature = "GameKit_GKChallengesViewController")]
unsafe impl NSObjectProtocol for GKChallengesViewController {}

#[cfg(feature = "GameKit_GKChallengesViewController")]
unsafe impl NSSeguePerforming for GKChallengesViewController {}

#[cfg(feature = "GameKit_GKChallengesViewController")]
unsafe impl NSUserInterfaceItemIdentification for GKChallengesViewController {}

extern_methods!(
    #[cfg(feature = "GameKit_GKChallengesViewController")]
    unsafe impl GKChallengesViewController {
        #[method_id(@__retain_semantics Other challengeDelegate)]
        pub unsafe fn challengeDelegate(
            &self,
        ) -> Option<Id<ProtocolObject<dyn GKChallengesViewControllerDelegate>>>;

        #[method(setChallengeDelegate:)]
        pub unsafe fn setChallengeDelegate(
            &self,
            challenge_delegate: Option<&ProtocolObject<dyn GKChallengesViewControllerDelegate>>,
        );
    }
);

extern_protocol!(
    pub unsafe trait GKChallengesViewControllerDelegate {
        #[cfg(feature = "GameKit_GKChallengesViewController")]
        #[method(challengesViewControllerDidFinish:)]
        unsafe fn challengesViewControllerDidFinish(
            &self,
            view_controller: Option<&GKChallengesViewController>,
        );
    }

    unsafe impl ProtocolType for dyn GKChallengesViewControllerDelegate {}
);

extern_methods!(
    /// Methods declared on superclass `NSViewController`
    #[cfg(feature = "GameKit_GKChallengesViewController")]
    unsafe impl GKChallengesViewController {
        #[cfg(feature = "Foundation_NSBundle")]
        #[method_id(@__retain_semantics Init initWithNibName:bundle:)]
        pub unsafe fn initWithNibName_bundle(
            this: Option<Allocated<Self>>,
            nib_name_or_nil: Option<&NSNibName>,
            nib_bundle_or_nil: Option<&NSBundle>,
        ) -> Id<Self>;
    }
);
