//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSGradientDrawingOptions {
        NSGradientDrawsBeforeStartingLocation = 1 << 0,
        NSGradientDrawsAfterEndingLocation = 1 << 1,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSGradient")]
    /**
      An NSGradient defines a transition between colors.  The transition is defined over a range from 0.0 to 1.0 inclusive.  A gradient typically contains a color at location 0.0, and one at location 1.0 with additional colors assigned to locations between 0.0 and 1.0.

    An NSGradient is a drawing primitive that can draw itself as a linear or radial gradient.  The color value at location 0.0 is considered the starting color, the color value at location 1.0 is considered the ending color.  A primitive drawing method is provided for both linear and radial gradients.  Each primitive drawing method provides flexibility in specifying the location of its respective gradient.  These primitive drawing methods perform no clipping before drawing.

    In addition, convenience drawing methods take a rectangle or path, and automatically calculate staring and ending locations and perform clipping to provide a convenient means for drawing gradient fills.  Two of the convenience methods are demonstrated below:


    The following code will fill a rectangle with a 45 degree linear gradient, from black to white:

    NSRect rect; // assume this exists
    NSGradient *gradient = [[NSGradient alloc] initWithStartingColor: [NSColor blackColor] endingColor: [NSColor whiteColor]];
    [gradient drawInRect: rect angle: 45.0];
    [gradient release];

    The following code will fill an arbitrary bezier path with a radial gradient, from the center of the path's bounding box, that transitions through three colors, evenly distributed in the gradient from 0.0 to 1.0.

    NSBezierPath *path // assume this exists
    NSArray *colorArray = [NSArray arrayWithObjects: [NSColor blueColor], [NSColor yellowColor], [NSColor orangeColor], nil];
    NSGradient *gradient = [[NSGradient alloc] initWithColors: colorArray];
    [gradient drawInBezierPath: path relativeCenterPosition: NSZeroPoint];
    [gradient release];

    An NSGradient has a color space.  When initialized, all colors provided are converted to that color space, and interpolation of colors occurs using the components of that color space.  The designated initializer takes a color space argument, all other initializers use the default generic RGB color space.
    */
    pub struct NSGradient;

    #[cfg(feature = "AppKit_NSGradient")]
    unsafe impl ClassType for NSGradient {
        type Super = NSObject;
    }
);

#[cfg(feature = "AppKit_NSGradient")]
/**
  An NSGradient defines a transition between colors.  The transition is defined over a range from 0.0 to 1.0 inclusive.  A gradient typically contains a color at location 0.0, and one at location 1.0 with additional colors assigned to locations between 0.0 and 1.0.

An NSGradient is a drawing primitive that can draw itself as a linear or radial gradient.  The color value at location 0.0 is considered the starting color, the color value at location 1.0 is considered the ending color.  A primitive drawing method is provided for both linear and radial gradients.  Each primitive drawing method provides flexibility in specifying the location of its respective gradient.  These primitive drawing methods perform no clipping before drawing.

In addition, convenience drawing methods take a rectangle or path, and automatically calculate staring and ending locations and perform clipping to provide a convenient means for drawing gradient fills.  Two of the convenience methods are demonstrated below:


The following code will fill a rectangle with a 45 degree linear gradient, from black to white:

NSRect rect; // assume this exists
NSGradient *gradient = [[NSGradient alloc] initWithStartingColor: [NSColor blackColor] endingColor: [NSColor whiteColor]];
[gradient drawInRect: rect angle: 45.0];
[gradient release];

The following code will fill an arbitrary bezier path with a radial gradient, from the center of the path's bounding box, that transitions through three colors, evenly distributed in the gradient from 0.0 to 1.0.

NSBezierPath *path // assume this exists
NSArray *colorArray = [NSArray arrayWithObjects: [NSColor blueColor], [NSColor yellowColor], [NSColor orangeColor], nil];
NSGradient *gradient = [[NSGradient alloc] initWithColors: colorArray];
[gradient drawInBezierPath: path relativeCenterPosition: NSZeroPoint];
[gradient release];

An NSGradient has a color space.  When initialized, all colors provided are converted to that color space, and interpolation of colors occurs using the components of that color space.  The designated initializer takes a color space argument, all other initializers use the default generic RGB color space.
*/
unsafe impl NSCoding for NSGradient {}

#[cfg(feature = "AppKit_NSGradient")]
/**
  An NSGradient defines a transition between colors.  The transition is defined over a range from 0.0 to 1.0 inclusive.  A gradient typically contains a color at location 0.0, and one at location 1.0 with additional colors assigned to locations between 0.0 and 1.0.

An NSGradient is a drawing primitive that can draw itself as a linear or radial gradient.  The color value at location 0.0 is considered the starting color, the color value at location 1.0 is considered the ending color.  A primitive drawing method is provided for both linear and radial gradients.  Each primitive drawing method provides flexibility in specifying the location of its respective gradient.  These primitive drawing methods perform no clipping before drawing.

In addition, convenience drawing methods take a rectangle or path, and automatically calculate staring and ending locations and perform clipping to provide a convenient means for drawing gradient fills.  Two of the convenience methods are demonstrated below:


The following code will fill a rectangle with a 45 degree linear gradient, from black to white:

NSRect rect; // assume this exists
NSGradient *gradient = [[NSGradient alloc] initWithStartingColor: [NSColor blackColor] endingColor: [NSColor whiteColor]];
[gradient drawInRect: rect angle: 45.0];
[gradient release];

The following code will fill an arbitrary bezier path with a radial gradient, from the center of the path's bounding box, that transitions through three colors, evenly distributed in the gradient from 0.0 to 1.0.

NSBezierPath *path // assume this exists
NSArray *colorArray = [NSArray arrayWithObjects: [NSColor blueColor], [NSColor yellowColor], [NSColor orangeColor], nil];
NSGradient *gradient = [[NSGradient alloc] initWithColors: colorArray];
[gradient drawInBezierPath: path relativeCenterPosition: NSZeroPoint];
[gradient release];

An NSGradient has a color space.  When initialized, all colors provided are converted to that color space, and interpolation of colors occurs using the components of that color space.  The designated initializer takes a color space argument, all other initializers use the default generic RGB color space.
*/
unsafe impl NSObjectProtocol for NSGradient {}

#[cfg(feature = "AppKit_NSGradient")]
/**
  An NSGradient defines a transition between colors.  The transition is defined over a range from 0.0 to 1.0 inclusive.  A gradient typically contains a color at location 0.0, and one at location 1.0 with additional colors assigned to locations between 0.0 and 1.0.

An NSGradient is a drawing primitive that can draw itself as a linear or radial gradient.  The color value at location 0.0 is considered the starting color, the color value at location 1.0 is considered the ending color.  A primitive drawing method is provided for both linear and radial gradients.  Each primitive drawing method provides flexibility in specifying the location of its respective gradient.  These primitive drawing methods perform no clipping before drawing.

In addition, convenience drawing methods take a rectangle or path, and automatically calculate staring and ending locations and perform clipping to provide a convenient means for drawing gradient fills.  Two of the convenience methods are demonstrated below:


The following code will fill a rectangle with a 45 degree linear gradient, from black to white:

NSRect rect; // assume this exists
NSGradient *gradient = [[NSGradient alloc] initWithStartingColor: [NSColor blackColor] endingColor: [NSColor whiteColor]];
[gradient drawInRect: rect angle: 45.0];
[gradient release];

The following code will fill an arbitrary bezier path with a radial gradient, from the center of the path's bounding box, that transitions through three colors, evenly distributed in the gradient from 0.0 to 1.0.

NSBezierPath *path // assume this exists
NSArray *colorArray = [NSArray arrayWithObjects: [NSColor blueColor], [NSColor yellowColor], [NSColor orangeColor], nil];
NSGradient *gradient = [[NSGradient alloc] initWithColors: colorArray];
[gradient drawInBezierPath: path relativeCenterPosition: NSZeroPoint];
[gradient release];

An NSGradient has a color space.  When initialized, all colors provided are converted to that color space, and interpolation of colors occurs using the components of that color space.  The designated initializer takes a color space argument, all other initializers use the default generic RGB color space.
*/
unsafe impl NSSecureCoding for NSGradient {}

extern_methods!(
    /**
      An NSGradient defines a transition between colors.  The transition is defined over a range from 0.0 to 1.0 inclusive.  A gradient typically contains a color at location 0.0, and one at location 1.0 with additional colors assigned to locations between 0.0 and 1.0.

    An NSGradient is a drawing primitive that can draw itself as a linear or radial gradient.  The color value at location 0.0 is considered the starting color, the color value at location 1.0 is considered the ending color.  A primitive drawing method is provided for both linear and radial gradients.  Each primitive drawing method provides flexibility in specifying the location of its respective gradient.  These primitive drawing methods perform no clipping before drawing.

    In addition, convenience drawing methods take a rectangle or path, and automatically calculate staring and ending locations and perform clipping to provide a convenient means for drawing gradient fills.  Two of the convenience methods are demonstrated below:


    The following code will fill a rectangle with a 45 degree linear gradient, from black to white:

    NSRect rect; // assume this exists
    NSGradient *gradient = [[NSGradient alloc] initWithStartingColor: [NSColor blackColor] endingColor: [NSColor whiteColor]];
    [gradient drawInRect: rect angle: 45.0];
    [gradient release];

    The following code will fill an arbitrary bezier path with a radial gradient, from the center of the path's bounding box, that transitions through three colors, evenly distributed in the gradient from 0.0 to 1.0.

    NSBezierPath *path // assume this exists
    NSArray *colorArray = [NSArray arrayWithObjects: [NSColor blueColor], [NSColor yellowColor], [NSColor orangeColor], nil];
    NSGradient *gradient = [[NSGradient alloc] initWithColors: colorArray];
    [gradient drawInBezierPath: path relativeCenterPosition: NSZeroPoint];
    [gradient release];

    An NSGradient has a color space.  When initialized, all colors provided are converted to that color space, and interpolation of colors occurs using the components of that color space.  The designated initializer takes a color space argument, all other initializers use the default generic RGB color space.
    */
    #[cfg(feature = "AppKit_NSGradient")]
    unsafe impl NSGradient {
        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Init initWithStartingColor:endingColor:)]
        pub unsafe fn initWithStartingColor_endingColor(
            this: Option<Allocated<Self>>,
            starting_color: &NSColor,
            ending_color: &NSColor,
        ) -> Option<Id<Self>>;

        #[cfg(all(feature = "AppKit_NSColor", feature = "Foundation_NSArray"))]
        #[method_id(@__retain_semantics Init initWithColors:)]
        pub unsafe fn initWithColors(
            this: Option<Allocated<Self>>,
            color_array: &NSArray<NSColor>,
        ) -> Option<Id<Self>>;

        #[cfg(all(
            feature = "AppKit_NSColor",
            feature = "AppKit_NSColorSpace",
            feature = "Foundation_NSArray"
        ))]
        #[method_id(@__retain_semantics Init initWithColors:atLocations:colorSpace:)]
        pub unsafe fn initWithColors_atLocations_colorSpace(
            this: Option<Allocated<Self>>,
            color_array: &NSArray<NSColor>,
            locations: *mut CGFloat,
            color_space: &NSColorSpace,
        ) -> Option<Id<Self>>;

        #[cfg(feature = "Foundation_NSCoder")]
        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(this: Option<Allocated<Self>>, coder: &NSCoder) -> Id<Self>;

        #[method(drawFromPoint:toPoint:options:)]
        pub unsafe fn drawFromPoint_toPoint_options(
            &self,
            starting_point: NSPoint,
            ending_point: NSPoint,
            options: NSGradientDrawingOptions,
        );

        #[method(drawInRect:angle:)]
        pub unsafe fn drawInRect_angle(&self, rect: NSRect, angle: CGFloat);

        #[cfg(feature = "AppKit_NSBezierPath")]
        #[method(drawInBezierPath:angle:)]
        pub unsafe fn drawInBezierPath_angle(&self, path: &NSBezierPath, angle: CGFloat);

        #[method(drawFromCenter:radius:toCenter:radius:options:)]
        pub unsafe fn drawFromCenter_radius_toCenter_radius_options(
            &self,
            start_center: NSPoint,
            start_radius: CGFloat,
            end_center: NSPoint,
            end_radius: CGFloat,
            options: NSGradientDrawingOptions,
        );

        #[method(drawInRect:relativeCenterPosition:)]
        pub unsafe fn drawInRect_relativeCenterPosition(
            &self,
            rect: NSRect,
            relative_center_position: NSPoint,
        );

        #[cfg(feature = "AppKit_NSBezierPath")]
        #[method(drawInBezierPath:relativeCenterPosition:)]
        pub unsafe fn drawInBezierPath_relativeCenterPosition(
            &self,
            path: &NSBezierPath,
            relative_center_position: NSPoint,
        );

        #[cfg(feature = "AppKit_NSColorSpace")]
        /**
          Returns the color space of the gradient
        */
        #[method_id(@__retain_semantics Other colorSpace)]
        pub unsafe fn colorSpace(&self) -> Id<NSColorSpace>;

        /**
          The number of color stops in the color gradient
        */
        #[method(numberOfColorStops)]
        pub unsafe fn numberOfColorStops(&self) -> NSInteger;

        #[cfg(feature = "AppKit_NSColor")]
        #[method(getColor:location:atIndex:)]
        pub unsafe fn getColor_location_atIndex(
            &self,
            color: Option<&mut Id<NSColor>>,
            location: *mut CGFloat,
            index: NSInteger,
        );

        #[cfg(feature = "AppKit_NSColor")]
        #[method_id(@__retain_semantics Other interpolatedColorAtLocation:)]
        pub unsafe fn interpolatedColorAtLocation(&self, location: CGFloat) -> Id<NSColor>;
    }
);
