//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#![allow(unused_imports)]
#![allow(deprecated)]
#[path = "LAAuthenticationView.rs"]
mod __LAAuthenticationView;
#[path = "LAPresentationContext.rs"]
mod __LAPresentationContext;
#[path = "LARight_UI.rs"]
mod __LARight_UI;

#[cfg(feature = "LocalAuthenticationEmbeddedUI_LAAuthenticationView")]
#[cfg(not(any(target_os = "ios")))]
pub use self::__LAAuthenticationView::LAAuthenticationView;

pub use self::__LAPresentationContext::LAPresentationContext;
