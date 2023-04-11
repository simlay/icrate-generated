//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::CloudKit::*;
use crate::CoreLocation::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "CloudKit_CKFetchShareParticipantsOperation")]
    pub struct CKFetchShareParticipantsOperation;

    #[cfg(feature = "CloudKit_CKFetchShareParticipantsOperation")]
    unsafe impl ClassType for CKFetchShareParticipantsOperation {
        #[inherits(NSOperation, NSObject)]
        type Super = CKOperation;
    }
);

#[cfg(feature = "CloudKit_CKFetchShareParticipantsOperation")]
unsafe impl NSObjectProtocol for CKFetchShareParticipantsOperation {}

extern_methods!(
    #[cfg(feature = "CloudKit_CKFetchShareParticipantsOperation")]
    unsafe impl CKFetchShareParticipantsOperation {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[cfg(all(
            feature = "CloudKit_CKUserIdentityLookupInfo",
            feature = "Foundation_NSArray"
        ))]
        #[method_id(@__retain_semantics Init initWithUserIdentityLookupInfos:)]
        pub unsafe fn initWithUserIdentityLookupInfos(
            this: Option<Allocated<Self>>,
            user_identity_lookup_infos: &NSArray<CKUserIdentityLookupInfo>,
        ) -> Id<Self>;

        #[cfg(all(
            feature = "CloudKit_CKUserIdentityLookupInfo",
            feature = "Foundation_NSArray"
        ))]
        #[method_id(@__retain_semantics Other userIdentityLookupInfos)]
        pub unsafe fn userIdentityLookupInfos(
            &self,
        ) -> Option<Id<NSArray<CKUserIdentityLookupInfo>>>;

        #[cfg(all(
            feature = "CloudKit_CKUserIdentityLookupInfo",
            feature = "Foundation_NSArray"
        ))]
        #[method(setUserIdentityLookupInfos:)]
        pub unsafe fn setUserIdentityLookupInfos(
            &self,
            user_identity_lookup_infos: Option<&NSArray<CKUserIdentityLookupInfo>>,
        );

        #[cfg(feature = "CloudKit_CKShareParticipant")]
        /**
          @abstract Called once for each share participant created from a submitted user identity lookup info.

          @discussion If the replacement callback @c perShareParticipantCompletionBlock is set, this callback block is ignored.
          Each @c CKOperation instance has a private serial queue. This queue is used for all callback block invocations.
        */
        #[deprecated = "Use perShareParticipantCompletionBlock instead, which surfaces per-share-participant errors"]
        #[method(shareParticipantFetchedBlock)]
        pub unsafe fn shareParticipantFetchedBlock(
            &self,
        ) -> *mut Block<(NonNull<CKShareParticipant>,), ()>;

        #[cfg(feature = "CloudKit_CKShareParticipant")]
        /**
          @abstract Called once for each share participant created from a submitted user identity lookup info.

          @discussion If the replacement callback @c perShareParticipantCompletionBlock is set, this callback block is ignored.
          Each @c CKOperation instance has a private serial queue. This queue is used for all callback block invocations.
        */
        #[deprecated = "Use perShareParticipantCompletionBlock instead, which surfaces per-share-participant errors"]
        #[method(setShareParticipantFetchedBlock:)]
        pub unsafe fn setShareParticipantFetchedBlock(
            &self,
            share_participant_fetched_block: Option<&Block<(NonNull<CKShareParticipant>,), ()>>,
        );

        #[cfg(all(
            feature = "CloudKit_CKShareParticipant",
            feature = "CloudKit_CKUserIdentityLookupInfo",
            feature = "Foundation_NSError"
        ))]
        /**
          @abstract Called once for each lookup info.

          @discussion Each @c CKOperation instance has a private serial queue. This queue is used for all callback block invocations.
        */
        #[method(perShareParticipantCompletionBlock)]
        pub unsafe fn perShareParticipantCompletionBlock(
            &self,
        ) -> *mut Block<
            (
                NonNull<CKUserIdentityLookupInfo>,
                *mut CKShareParticipant,
                *mut NSError,
            ),
            (),
        >;

        #[cfg(all(
            feature = "CloudKit_CKShareParticipant",
            feature = "CloudKit_CKUserIdentityLookupInfo",
            feature = "Foundation_NSError"
        ))]
        /**
          @abstract Called once for each lookup info.

          @discussion Each @c CKOperation instance has a private serial queue. This queue is used for all callback block invocations.
        */
        #[method(setPerShareParticipantCompletionBlock:)]
        pub unsafe fn setPerShareParticipantCompletionBlock(
            &self,
            per_share_participant_completion_block: Option<
                &Block<
                    (
                        NonNull<CKUserIdentityLookupInfo>,
                        *mut CKShareParticipant,
                        *mut NSError,
                    ),
                    (),
                >,
            >,
        );

        #[cfg(feature = "Foundation_NSError")]
        /**
          @abstract This block is called when the operation completes.

          @discussion The @code -[NSOperation completionBlock] @endcode will also be called if both are set.
          If the error is @c CKErrorPartialFailure, the error's userInfo dictionary contains a dictionary of lookup infos to errors keyed off of @c CKPartialErrorsByItemIDKey.  These errors are repeats of those sent back in previous @c perShareParticipantCompletionBlock invocations
          Each @c CKOperation instance has a private serial queue. This queue is used for all callback block invocations.
        */
        #[method(fetchShareParticipantsCompletionBlock)]
        pub unsafe fn fetchShareParticipantsCompletionBlock(
            &self,
        ) -> *mut Block<(*mut NSError,), ()>;

        #[cfg(feature = "Foundation_NSError")]
        /**
          @abstract This block is called when the operation completes.

          @discussion The @code -[NSOperation completionBlock] @endcode will also be called if both are set.
          If the error is @c CKErrorPartialFailure, the error's userInfo dictionary contains a dictionary of lookup infos to errors keyed off of @c CKPartialErrorsByItemIDKey.  These errors are repeats of those sent back in previous @c perShareParticipantCompletionBlock invocations
          Each @c CKOperation instance has a private serial queue. This queue is used for all callback block invocations.
        */
        #[method(setFetchShareParticipantsCompletionBlock:)]
        pub unsafe fn setFetchShareParticipantsCompletionBlock(
            &self,
            fetch_share_participants_completion_block: Option<&Block<(*mut NSError,), ()>>,
        );
    }
);
