//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSTextLineFragment")]
    /**
      NSTextLineFragment represents a single textual layout and rendering unit inside NSTextLayoutFragment.
    */
    pub struct NSTextLineFragment;

    #[cfg(feature = "AppKit_NSTextLineFragment")]
    unsafe impl ClassType for NSTextLineFragment {
        type Super = NSObject;
    }
);

#[cfg(feature = "AppKit_NSTextLineFragment")]
/**
  NSTextLineFragment represents a single textual layout and rendering unit inside NSTextLayoutFragment.
*/
unsafe impl NSCoding for NSTextLineFragment {}

#[cfg(feature = "AppKit_NSTextLineFragment")]
/**
  NSTextLineFragment represents a single textual layout and rendering unit inside NSTextLayoutFragment.
*/
unsafe impl NSObjectProtocol for NSTextLineFragment {}

#[cfg(feature = "AppKit_NSTextLineFragment")]
/**
  NSTextLineFragment represents a single textual layout and rendering unit inside NSTextLayoutFragment.
*/
unsafe impl NSSecureCoding for NSTextLineFragment {}

extern_methods!(
    /**
      NSTextLineFragment represents a single textual layout and rendering unit inside NSTextLayoutFragment.
    */
    #[cfg(feature = "AppKit_NSTextLineFragment")]
    unsafe impl NSTextLineFragment {
        #[cfg(feature = "Foundation_NSAttributedString")]
        #[method_id(@__retain_semantics Init initWithAttributedString:range:)]
        pub unsafe fn initWithAttributedString_range(
            this: Option<Allocated<Self>>,
            attributed_string: &NSAttributedString,
            range: NSRange,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            a_decoder: &NSCoder,
        ) -> Option<Id<Self>>;

        #[cfg(all(feature = "Foundation_NSDictionary", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Init initWithString:attributes:range:)]
        pub unsafe fn initWithString_attributes_range(
            this: Option<Allocated<Self>>,
            string: &NSString,
            attributes: &NSDictionary<NSAttributedStringKey, Object>,
            range: NSRange,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[cfg(feature = "Foundation_NSAttributedString")]
        /**
          The source attributed string
        */
        #[method_id(@__retain_semantics Other attributedString)]
        pub unsafe fn attributedString(&self) -> Id<NSAttributedString>;

        /**
          The string range for the source attributed string corresponding to this line fragment
        */
        #[method(characterRange)]
        pub unsafe fn characterRange(&self) -> NSRange;

        /**
          The typographic bounds specifying the dimensions of the line fragment for laying out line fragments to each other. The origin value is offset from the beginning of the line fragment group belonging to the parent layout fragment.
        */
        #[method(typographicBounds)]
        pub unsafe fn typographicBounds(&self) -> CGRect;

        /**
          Rendering origin for the left most glyph in the line fragment coordinate system
        */
        #[method(glyphOrigin)]
        pub unsafe fn glyphOrigin(&self) -> CGPoint;

        #[method(locationForCharacterAtIndex:)]
        pub unsafe fn locationForCharacterAtIndex(&self, index: NSInteger) -> CGPoint;

        #[method(characterIndexForPoint:)]
        pub unsafe fn characterIndexForPoint(&self, point: CGPoint) -> NSInteger;

        #[method(fractionOfDistanceThroughGlyphForPoint:)]
        pub unsafe fn fractionOfDistanceThroughGlyphForPoint(&self, point: CGPoint) -> CGFloat;
    }
);
