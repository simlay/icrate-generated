//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameKit_GKLeaderboardViewController")]
    #[deprecated = "Use GKGameCenterViewController instead"]
    pub struct GKLeaderboardViewController;

    #[deprecated = "Use GKGameCenterViewController instead"]
    #[cfg(feature = "GameKit_GKLeaderboardViewController")]
    unsafe impl ClassType for GKLeaderboardViewController {
        #[inherits(NSViewController, NSResponder, NSObject)]
        type Super = GKGameCenterViewController;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "GameKit_GKLeaderboardViewController")]
unsafe impl GKViewController for GKLeaderboardViewController {}

#[cfg(feature = "GameKit_GKLeaderboardViewController")]
unsafe impl NSCoding for GKLeaderboardViewController {}

#[cfg(feature = "GameKit_GKLeaderboardViewController")]
unsafe impl NSEditor for GKLeaderboardViewController {}

#[cfg(feature = "GameKit_GKLeaderboardViewController")]
unsafe impl NSObjectProtocol for GKLeaderboardViewController {}

#[cfg(feature = "GameKit_GKLeaderboardViewController")]
unsafe impl NSSeguePerforming for GKLeaderboardViewController {}

#[cfg(feature = "GameKit_GKLeaderboardViewController")]
unsafe impl NSUserInterfaceItemIdentification for GKLeaderboardViewController {}

extern_methods!(
    #[cfg(feature = "GameKit_GKLeaderboardViewController")]
    unsafe impl GKLeaderboardViewController {}
);

extern_methods!(
    /// Methods declared on superclass `NSViewController`
    #[cfg(feature = "GameKit_GKLeaderboardViewController")]
    unsafe impl GKLeaderboardViewController {
        #[cfg(feature = "Foundation_NSBundle")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Init initWithNibName:bundle:)]
        pub unsafe fn initWithNibName_bundle(
            this: Option<Allocated<Self>>,
            nib_name_or_nil: Option<&NSNibName>,
            nib_bundle_or_nil: Option<&NSBundle>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Option<Id<Self>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSResponder`
    #[cfg(feature = "GameKit_GKLeaderboardViewController")]
    unsafe impl GKLeaderboardViewController {
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "GameKit_GKLeaderboardViewController")]
    unsafe impl GKLeaderboardViewController {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_methods!(
    #[cfg(feature = "GameKit_GKLeaderboardViewController")]
    unsafe impl GKLeaderboardViewController {
        #[method(timeScope)]
        pub unsafe fn timeScope(&self) -> GKLeaderboardTimeScope;

        #[method(setTimeScope:)]
        pub unsafe fn setTimeScope(&self, time_scope: GKLeaderboardTimeScope);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other category)]
        pub unsafe fn category(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setCategory:)]
        pub unsafe fn setCategory(&self, category: Option<&NSString>);

        #[method_id(@__retain_semantics Other leaderboardDelegate)]
        pub unsafe fn leaderboardDelegate(
            &self,
        ) -> Option<Id<ProtocolObject<dyn GKLeaderboardViewControllerDelegate>>>;

        #[method(setLeaderboardDelegate:)]
        pub unsafe fn setLeaderboardDelegate(
            &self,
            leaderboard_delegate: Option<&ProtocolObject<dyn GKLeaderboardViewControllerDelegate>>,
        );
    }
);

extern_protocol!(
    pub unsafe trait GKLeaderboardViewControllerDelegate: NSObjectProtocol {
        #[cfg(feature = "GameKit_GKLeaderboardViewController")]
        #[method(leaderboardViewControllerDidFinish:)]
        unsafe fn leaderboardViewControllerDidFinish(
            &self,
            view_controller: Option<&GKLeaderboardViewController>,
        );
    }

    unsafe impl ProtocolType for dyn GKLeaderboardViewControllerDelegate {}
);
