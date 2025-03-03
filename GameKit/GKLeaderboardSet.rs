//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::GameKit::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "GameKit_GKLeaderboardSet")]
    pub struct GKLeaderboardSet;

    #[cfg(feature = "GameKit_GKLeaderboardSet")]
    unsafe impl ClassType for GKLeaderboardSet {
        type Super = NSObject;
    }
);

#[cfg(feature = "GameKit_GKLeaderboardSet")]
unsafe impl NSCoding for GKLeaderboardSet {}

#[cfg(feature = "GameKit_GKLeaderboardSet")]
unsafe impl NSObjectProtocol for GKLeaderboardSet {}

#[cfg(feature = "GameKit_GKLeaderboardSet")]
unsafe impl NSSecureCoding for GKLeaderboardSet {}

extern_methods!(
    #[cfg(feature = "GameKit_GKLeaderboardSet")]
    unsafe impl GKLeaderboardSet {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other groupIdentifier)]
        pub unsafe fn groupIdentifier(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(setIdentifier:)]
        pub unsafe fn setIdentifier(&self, identifier: Option<&NSString>);

        #[cfg(all(feature = "Foundation_NSArray", feature = "Foundation_NSError"))]
        #[method(loadLeaderboardSetsWithCompletionHandler:)]
        pub unsafe fn loadLeaderboardSetsWithCompletionHandler(
            completion_handler: Option<&Block<(*mut NSArray<GKLeaderboardSet>, *mut NSError), ()>>,
        );

        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "GameKit_GKLeaderboard"
        ))]
        #[method(loadLeaderboardsWithHandler:)]
        pub unsafe fn loadLeaderboardsWithHandler(
            &self,
            handler: &Block<(*mut NSArray<GKLeaderboard>, *mut NSError), ()>,
        );
    }
);

extern_methods!(
    /// Deprecated
    #[cfg(feature = "GameKit_GKLeaderboardSet")]
    unsafe impl GKLeaderboardSet {
        #[cfg(all(
            feature = "Foundation_NSArray",
            feature = "Foundation_NSError",
            feature = "GameKit_GKLeaderboard"
        ))]
        #[deprecated = "Use loadLeaderboardsWithHandler: instead."]
        #[method(loadLeaderboardsWithCompletionHandler:)]
        pub unsafe fn loadLeaderboardsWithCompletionHandler(
            &self,
            completion_handler: Option<&Block<(*mut NSArray<GKLeaderboard>, *mut NSError), ()>>,
        );
    }
);

extern_methods!(
    /// UI
    #[cfg(feature = "GameKit_GKLeaderboardSet")]
    unsafe impl GKLeaderboardSet {
        #[cfg(all(feature = "AppKit_NSImage", feature = "Foundation_NSError"))]
        #[method(loadImageWithCompletionHandler:)]
        pub unsafe fn loadImageWithCompletionHandler(
            &self,
            completion_handler: Option<&Block<(*mut NSImage, *mut NSError), ()>>,
        );
    }
);
