//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Accessibility::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Accessibility_AXBrailleMap")]
    /**
      An AXBrailleMap object represents a connected two-dimensional braille display.
    A display is comprised of a grid of pins that can be raised and lowered.
    This is useful for representing graphics, images, and other visual data to VoiceOver users.
    */
    pub struct AXBrailleMap;

    #[cfg(feature = "Accessibility_AXBrailleMap")]
    unsafe impl ClassType for AXBrailleMap {
        type Super = NSObject;
    }
);

#[cfg(feature = "Accessibility_AXBrailleMap")]
/**
  An AXBrailleMap object represents a connected two-dimensional braille display.
A display is comprised of a grid of pins that can be raised and lowered.
This is useful for representing graphics, images, and other visual data to VoiceOver users.
*/
unsafe impl NSCoding for AXBrailleMap {}

#[cfg(feature = "Accessibility_AXBrailleMap")]
/**
  An AXBrailleMap object represents a connected two-dimensional braille display.
A display is comprised of a grid of pins that can be raised and lowered.
This is useful for representing graphics, images, and other visual data to VoiceOver users.
*/
unsafe impl NSObjectProtocol for AXBrailleMap {}

#[cfg(feature = "Accessibility_AXBrailleMap")]
/**
  An AXBrailleMap object represents a connected two-dimensional braille display.
A display is comprised of a grid of pins that can be raised and lowered.
This is useful for representing graphics, images, and other visual data to VoiceOver users.
*/
unsafe impl NSSecureCoding for AXBrailleMap {}

extern_methods!(
    /**
      An AXBrailleMap object represents a connected two-dimensional braille display.
    A display is comprised of a grid of pins that can be raised and lowered.
    This is useful for representing graphics, images, and other visual data to VoiceOver users.
    */
    #[cfg(feature = "Accessibility_AXBrailleMap")]
    unsafe impl AXBrailleMap {
        /**
          Indicates the number of dots in each dimension. This size may change if the user zooms in the content.
        */
        #[method(dimensions)]
        pub unsafe fn dimensions(&self) -> CGSize;

        #[method(setHeight:atPoint:)]
        pub unsafe fn setHeight_atPoint(&self, status: c_float, point: CGPoint);

        #[method(heightAtPoint:)]
        pub unsafe fn heightAtPoint(&self, point: CGPoint) -> c_float;

        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);

extern_protocol!(
    /**
      Implement one of the following methods in order to provide data for a braille map to be rendered.
    */
    pub unsafe trait AXBrailleMapRenderer: NSObjectProtocol {
        /**
          If the element displays a region that should be rendered into the braille map automatically,
         specify it here (relative to the bounds of the object). VoiceOver will snapshot that region of the element
         and convert to a braille map internally.
        */
        #[optional]
        #[method(accessibilityBrailleMapRenderRegion)]
        unsafe fn accessibilityBrailleMapRenderRegion(&self) -> CGRect;

        /**
          If the element displays a region that should be rendered into the braille map automatically,
         specify it here (relative to the bounds of the object). VoiceOver will snapshot that region of the element
         and convert to a braille map internally.
        */
        #[optional]
        #[method(setAccessibilityBrailleMapRenderRegion:)]
        unsafe fn setAccessibilityBrailleMapRenderRegion(
            &self,
            accessibility_braille_map_render_region: CGRect,
        );

        #[cfg(feature = "Accessibility_AXBrailleMap")]
        /**
          This handler is called to ask the element to update the values of the braille map on-demand.
        */
        #[optional]
        #[method(accessibilityBrailleMapRenderer)]
        unsafe fn accessibilityBrailleMapRenderer(
            &self,
        ) -> NonNull<Block<(NonNull<AXBrailleMap>,), ()>>;

        #[cfg(feature = "Accessibility_AXBrailleMap")]
        /**
          This handler is called to ask the element to update the values of the braille map on-demand.
        */
        #[optional]
        #[method(setAccessibilityBrailleMapRenderer:)]
        unsafe fn setAccessibilityBrailleMapRenderer(
            &self,
            accessibility_braille_map_renderer: &Block<(NonNull<AXBrailleMap>,), ()>,
        );
    }

    unsafe impl ProtocolType for dyn AXBrailleMapRenderer {}
);
