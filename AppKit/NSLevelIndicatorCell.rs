//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::CoreData::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSUInteger)]
    #[cfg(not(any(target_os = "ios")))]
    pub enum NSLevelIndicatorStyle {
        #[cfg(not(any(target_os = "ios")))]
        NSLevelIndicatorStyleRelevancy = 0,
        #[cfg(not(any(target_os = "ios")))]
        NSLevelIndicatorStyleContinuousCapacity = 1,
        #[cfg(not(any(target_os = "ios")))]
        NSLevelIndicatorStyleDiscreteCapacity = 2,
        #[cfg(not(any(target_os = "ios")))]
        NSLevelIndicatorStyleRating = 3,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "AppKit_NSLevelIndicatorCell")]
    #[cfg(not(any(target_os = "ios")))]
    pub struct NSLevelIndicatorCell;

    #[cfg(feature = "AppKit_NSLevelIndicatorCell")]
    unsafe impl ClassType for NSLevelIndicatorCell {
        #[inherits(NSCell, NSObject)]
        type Super = NSActionCell;
    }
);

#[cfg(feature = "AppKit_NSLevelIndicatorCell")]
unsafe impl NSAccessibility for NSLevelIndicatorCell {}

#[cfg(feature = "AppKit_NSLevelIndicatorCell")]
unsafe impl NSAccessibilityElementProtocol for NSLevelIndicatorCell {}

#[cfg(feature = "AppKit_NSLevelIndicatorCell")]
unsafe impl NSCoding for NSLevelIndicatorCell {}

#[cfg(feature = "AppKit_NSLevelIndicatorCell")]
unsafe impl NSObjectProtocol for NSLevelIndicatorCell {}

#[cfg(feature = "AppKit_NSLevelIndicatorCell")]
unsafe impl NSUserInterfaceItemIdentification for NSLevelIndicatorCell {}

extern_methods!(
    #[cfg(feature = "AppKit_NSLevelIndicatorCell")]
    unsafe impl NSLevelIndicatorCell {
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Init initWithLevelIndicatorStyle:)]
        pub unsafe fn initWithLevelIndicatorStyle(
            this: Option<Allocated<Self>>,
            level_indicator_style: NSLevelIndicatorStyle,
        ) -> Id<Self>;

        #[cfg(not(any(target_os = "ios")))]
        #[method(levelIndicatorStyle)]
        pub unsafe fn levelIndicatorStyle(&self) -> NSLevelIndicatorStyle;

        #[cfg(not(any(target_os = "ios")))]
        #[method(setLevelIndicatorStyle:)]
        pub unsafe fn setLevelIndicatorStyle(&self, level_indicator_style: NSLevelIndicatorStyle);

        #[cfg(not(any(target_os = "ios")))]
        #[method(minValue)]
        pub unsafe fn minValue(&self) -> c_double;

        #[cfg(not(any(target_os = "ios")))]
        #[method(setMinValue:)]
        pub unsafe fn setMinValue(&self, min_value: c_double);

        #[cfg(not(any(target_os = "ios")))]
        #[method(maxValue)]
        pub unsafe fn maxValue(&self) -> c_double;

        #[cfg(not(any(target_os = "ios")))]
        #[method(setMaxValue:)]
        pub unsafe fn setMaxValue(&self, max_value: c_double);

        #[cfg(not(any(target_os = "ios")))]
        #[method(warningValue)]
        pub unsafe fn warningValue(&self) -> c_double;

        #[cfg(not(any(target_os = "ios")))]
        #[method(setWarningValue:)]
        pub unsafe fn setWarningValue(&self, warning_value: c_double);

        #[cfg(not(any(target_os = "ios")))]
        #[method(criticalValue)]
        pub unsafe fn criticalValue(&self) -> c_double;

        #[cfg(not(any(target_os = "ios")))]
        #[method(setCriticalValue:)]
        pub unsafe fn setCriticalValue(&self, critical_value: c_double);

        #[cfg(not(any(target_os = "ios")))]
        #[method(tickMarkPosition)]
        pub unsafe fn tickMarkPosition(&self) -> NSTickMarkPosition;

        #[cfg(not(any(target_os = "ios")))]
        #[method(setTickMarkPosition:)]
        pub unsafe fn setTickMarkPosition(&self, tick_mark_position: NSTickMarkPosition);

        #[cfg(not(any(target_os = "ios")))]
        #[method(numberOfTickMarks)]
        pub unsafe fn numberOfTickMarks(&self) -> NSInteger;

        #[cfg(not(any(target_os = "ios")))]
        #[method(setNumberOfTickMarks:)]
        pub unsafe fn setNumberOfTickMarks(&self, number_of_tick_marks: NSInteger);

        #[cfg(not(any(target_os = "ios")))]
        #[method(numberOfMajorTickMarks)]
        pub unsafe fn numberOfMajorTickMarks(&self) -> NSInteger;

        #[cfg(not(any(target_os = "ios")))]
        #[method(setNumberOfMajorTickMarks:)]
        pub unsafe fn setNumberOfMajorTickMarks(&self, number_of_major_tick_marks: NSInteger);

        #[cfg(not(any(target_os = "ios")))]
        #[method(rectOfTickMarkAtIndex:)]
        pub unsafe fn rectOfTickMarkAtIndex(&self, index: NSInteger) -> NSRect;

        #[cfg(not(any(target_os = "ios")))]
        #[method(tickMarkValueAtIndex:)]
        pub unsafe fn tickMarkValueAtIndex(&self, index: NSInteger) -> c_double;
    }
);

extern_static!(
    NSRelevancyLevelIndicatorStyle: NSLevelIndicatorStyle = NSLevelIndicatorStyleRelevancy
);

extern_static!(
    NSContinuousCapacityLevelIndicatorStyle: NSLevelIndicatorStyle =
        NSLevelIndicatorStyleContinuousCapacity
);

extern_static!(
    NSDiscreteCapacityLevelIndicatorStyle: NSLevelIndicatorStyle =
        NSLevelIndicatorStyleDiscreteCapacity
);

extern_static!(NSRatingLevelIndicatorStyle: NSLevelIndicatorStyle = NSLevelIndicatorStyleRating);

extern_methods!(
    /// Methods declared on superclass `NSCell`
    #[cfg(feature = "AppKit_NSLevelIndicatorCell")]
    unsafe impl NSLevelIndicatorCell {
        #[cfg(feature = "Foundation_NSString")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Init initTextCell:)]
        pub unsafe fn initTextCell(this: Option<Allocated<Self>>, string: &NSString) -> Id<Self>;

        #[cfg(feature = "AppKit_NSImage")]
        #[cfg(not(any(target_os = "ios")))]
        #[method_id(@__retain_semantics Init initImageCell:)]
        pub unsafe fn initImageCell(
            this: Option<Allocated<Self>>,
            image: Option<&NSImage>,
        ) -> Id<Self>;
    }
);
