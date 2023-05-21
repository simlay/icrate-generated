//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CloudKit::*;
use crate::CoreLocation::*;
use crate::Foundation::*;

#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
ns_options!(
    #[underlying(NSUInteger)]
    pub enum CKSharingParticipantAccessOption {
        #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
        CKSharingParticipantAccessOptionAnyoneWithLink = 1 << 0,
        #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
        CKSharingParticipantAccessOptionSpecifiedRecipientsOnly = 1 << 1,
        #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
        CKSharingParticipantAccessOptionAny = CKSharingParticipantAccessOptionAnyoneWithLink
            | CKSharingParticipantAccessOptionSpecifiedRecipientsOnly,
    }
);

#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
ns_options!(
    #[underlying(NSUInteger)]
    pub enum CKSharingParticipantPermissionOption {
        #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
        CKSharingParticipantPermissionOptionReadOnly = 1 << 0,
        #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
        CKSharingParticipantPermissionOptionReadWrite = 1 << 1,
        #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
        CKSharingParticipantPermissionOptionAny = CKSharingParticipantPermissionOptionReadOnly
            | CKSharingParticipantPermissionOptionReadWrite,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CloudKit_CKAllowedSharingOptions")]
    #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
    pub struct CKAllowedSharingOptions;

    #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
    #[cfg(feature = "CloudKit_CKAllowedSharingOptions")]
    unsafe impl ClassType for CKAllowedSharingOptions {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "CloudKit_CKAllowedSharingOptions")]
#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
unsafe impl NSCoding for CKAllowedSharingOptions {}

#[cfg(feature = "CloudKit_CKAllowedSharingOptions")]
#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
unsafe impl NSCopying for CKAllowedSharingOptions {}

#[cfg(feature = "CloudKit_CKAllowedSharingOptions")]
#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
unsafe impl NSObjectProtocol for CKAllowedSharingOptions {}

#[cfg(feature = "CloudKit_CKAllowedSharingOptions")]
#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
unsafe impl NSSecureCoding for CKAllowedSharingOptions {}

#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
extern_methods!(
    #[cfg(feature = "CloudKit_CKAllowedSharingOptions")]
    #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
    unsafe impl CKAllowedSharingOptions {
        #[method_id(@__retain_semantics Init initWithAllowedParticipantPermissionOptions:allowedParticipantAccessOptions:)]
        pub unsafe fn initWithAllowedParticipantPermissionOptions_allowedParticipantAccessOptions(
            this: Option<Allocated<Self>>,
            allowed_participant_permission_options: CKSharingParticipantPermissionOption,
            allowed_participant_access_options: CKSharingParticipantAccessOption,
        ) -> Id<Self>;

        #[method(allowedParticipantPermissionOptions)]
        pub unsafe fn allowedParticipantPermissionOptions(
            &self,
        ) -> CKSharingParticipantPermissionOption;

        #[method(setAllowedParticipantPermissionOptions:)]
        pub unsafe fn setAllowedParticipantPermissionOptions(
            &self,
            allowed_participant_permission_options: CKSharingParticipantPermissionOption,
        );

        #[method(allowedParticipantAccessOptions)]
        pub unsafe fn allowedParticipantAccessOptions(&self) -> CKSharingParticipantAccessOption;

        #[method(setAllowedParticipantAccessOptions:)]
        pub unsafe fn setAllowedParticipantAccessOptions(
            &self,
            allowed_participant_access_options: CKSharingParticipantAccessOption,
        );

        #[method_id(@__retain_semantics Other standardOptions)]
        pub unsafe fn standardOptions() -> Id<CKAllowedSharingOptions>;
    }
);

#[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "CloudKit_CKAllowedSharingOptions")]
    #[cfg(not(any(target_os = "tvos", target_os = "watchos")))]
    unsafe impl CKAllowedSharingOptions {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
