//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::ClassKit::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "ClassKit_CLSScoreItem")]
    /**
     @abstract      CLSScoreItem represents user generated score information.
    */
    pub struct CLSScoreItem;

    #[cfg(feature = "ClassKit_CLSScoreItem")]
    unsafe impl ClassType for CLSScoreItem {
        #[inherits(CLSObject, NSObject)]
        type Super = CLSActivityItem;
    }
);

#[cfg(feature = "ClassKit_CLSScoreItem")]
/**
 @abstract      CLSScoreItem represents user generated score information.
*/
unsafe impl NSCoding for CLSScoreItem {}

#[cfg(feature = "ClassKit_CLSScoreItem")]
/**
 @abstract      CLSScoreItem represents user generated score information.
*/
unsafe impl NSObjectProtocol for CLSScoreItem {}

#[cfg(feature = "ClassKit_CLSScoreItem")]
/**
 @abstract      CLSScoreItem represents user generated score information.
*/
unsafe impl NSSecureCoding for CLSScoreItem {}

extern_methods!(
    /**
     @abstract      CLSScoreItem represents user generated score information.
    */
    #[cfg(feature = "ClassKit_CLSScoreItem")]
    unsafe impl CLSScoreItem {
        /**
         @abstract      Score out of @c maxScore.
        @discussion    Should be between zero and @c maxScore [0.0,maxScore].
        */
        #[method(score)]
        pub unsafe fn score(&self) -> c_double;

        /**
         @abstract      Score out of @c maxScore.
        @discussion    Should be between zero and @c maxScore [0.0,maxScore].
        */
        #[method(setScore:)]
        pub unsafe fn setScore(&self, score: c_double);

        /**
         @abstract      Total score possible.
        @discussion    Must be greater than zero.
        */
        #[method(maxScore)]
        pub unsafe fn maxScore(&self) -> c_double;

        /**
         @abstract      Total score possible.
        @discussion    Must be greater than zero.
        */
        #[method(setMaxScore:)]
        pub unsafe fn setMaxScore(&self, max_score: c_double);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithIdentifier:title:score:maxScore:)]
        pub unsafe fn initWithIdentifier_title_score_maxScore(
            this: Option<Allocated<Self>>,
            identifier: &NSString,
            title: &NSString,
            score: c_double,
            max_score: c_double,
        ) -> Id<Self>;
    }
);
