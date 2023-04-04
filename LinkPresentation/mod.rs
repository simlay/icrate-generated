//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#![allow(unused_imports)]
#![allow(deprecated)]
#[path = "LPError.rs"]
mod __LPError;
#[path = "LPFoundation.rs"]
mod __LPFoundation;
#[path = "LPLinkMetadata.rs"]
mod __LPLinkMetadata;
#[path = "LPLinkView.rs"]
mod __LPLinkView;
#[path = "LPMetadataProvider.rs"]
mod __LPMetadataProvider;

pub use self::__LPError::LPErrorDomain;

pub use self::__LPError::LPErrorCode;

pub use self::__LPError::LPErrorUnknown;

pub use self::__LPError::LPErrorMetadataFetchFailed;

pub use self::__LPError::LPErrorMetadataFetchCancelled;

pub use self::__LPError::LPErrorMetadataFetchTimedOut;
#[cfg(feature = "LinkPresentation_LPLinkMetadata")]
pub use self::__LPLinkMetadata::LPLinkMetadata;
#[cfg(feature = "LinkPresentation_LPLinkView")]
pub use self::__LPLinkView::LPLinkView;
#[cfg(feature = "LinkPresentation_LPMetadataProvider")]
#[cfg(not(any(target_os = "tvos")))]
pub use self::__LPMetadataProvider::LPMetadataProvider;
