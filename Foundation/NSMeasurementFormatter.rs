//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSMeasurementFormatterUnitOptions {
        NSMeasurementFormatterUnitOptionsProvidedUnit = 1 << 0,
        NSMeasurementFormatterUnitOptionsNaturalScale = 1 << 1,
        NSMeasurementFormatterUnitOptionsTemperatureWithoutUnit = 1 << 2,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSMeasurementFormatter")]
    pub struct NSMeasurementFormatter;

    #[cfg(feature = "Foundation_NSMeasurementFormatter")]
    unsafe impl ClassType for NSMeasurementFormatter {
        #[inherits(NSObject)]
        type Super = NSFormatter;
    }
);

#[cfg(feature = "Foundation_NSMeasurementFormatter")]
unsafe impl NSCoding for NSMeasurementFormatter {}

#[cfg(feature = "Foundation_NSMeasurementFormatter")]
unsafe impl NSObjectProtocol for NSMeasurementFormatter {}

#[cfg(feature = "Foundation_NSMeasurementFormatter")]
unsafe impl NSSecureCoding for NSMeasurementFormatter {}

extern_methods!(
    #[cfg(feature = "Foundation_NSMeasurementFormatter")]
    unsafe impl NSMeasurementFormatter {
        /**
         This property can be set to ensure that the formatter behaves in a way the developer expects, even if it is not standard according to the preferences of the user's locale. If not specified, unitOptions defaults to localizing according to the preferences of the locale.

        Ex:

        By default, if unitOptions is set to the empty set, the formatter will do the following:
        - kilocalories may be formatted as "C" instead of "kcal" depending on the locale.
        - kilometersPerHour may be formatted as "miles per hour" for US and UK locales but "kilometers per hour" for other locales.

        However, if NSMeasurementFormatterUnitOptionsProvidedUnit is set, the formatter will do the following:
        - kilocalories would be formatted as "kcal" in the language of the locale, even if the locale prefers "C".
        - kilometersPerHour would be formatted as "kilometers per hour" for US and UK locales even though the preference is for "miles per hour."

        Note that NSMeasurementFormatter will handle converting measurement objects to the preferred units in a particular locale.  For instance, if provided a measurement object in kilometers and the set locale is en_US, the formatter will implicitly convert the measurement object to miles and return the formatted string as the equivalent measurement in miles.

        */
        #[method(unitOptions)]
        pub unsafe fn unitOptions(&self) -> NSMeasurementFormatterUnitOptions;

        /**
         This property can be set to ensure that the formatter behaves in a way the developer expects, even if it is not standard according to the preferences of the user's locale. If not specified, unitOptions defaults to localizing according to the preferences of the locale.

        Ex:

        By default, if unitOptions is set to the empty set, the formatter will do the following:
        - kilocalories may be formatted as "C" instead of "kcal" depending on the locale.
        - kilometersPerHour may be formatted as "miles per hour" for US and UK locales but "kilometers per hour" for other locales.

        However, if NSMeasurementFormatterUnitOptionsProvidedUnit is set, the formatter will do the following:
        - kilocalories would be formatted as "kcal" in the language of the locale, even if the locale prefers "C".
        - kilometersPerHour would be formatted as "kilometers per hour" for US and UK locales even though the preference is for "miles per hour."

        Note that NSMeasurementFormatter will handle converting measurement objects to the preferred units in a particular locale.  For instance, if provided a measurement object in kilometers and the set locale is en_US, the formatter will implicitly convert the measurement object to miles and return the formatted string as the equivalent measurement in miles.

        */
        #[method(setUnitOptions:)]
        pub unsafe fn setUnitOptions(&self, unit_options: NSMeasurementFormatterUnitOptions);

        /**
         If not specified, unitStyle is set to NSFormattingUnitStyleMedium.
        */
        #[method(unitStyle)]
        pub unsafe fn unitStyle(&self) -> NSFormattingUnitStyle;

        /**
         If not specified, unitStyle is set to NSFormattingUnitStyleMedium.
        */
        #[method(setUnitStyle:)]
        pub unsafe fn setUnitStyle(&self, unit_style: NSFormattingUnitStyle);

        #[cfg(feature = "Foundation_NSLocale")]
        /**
         If not specified, locale is set to the user's current locale.
        */
        #[method_id(@__retain_semantics Other locale)]
        pub unsafe fn locale(&self) -> Id<NSLocale>;

        #[cfg(feature = "Foundation_NSLocale")]
        /**
         If not specified, locale is set to the user's current locale.
        */
        #[method(setLocale:)]
        pub unsafe fn setLocale(&self, locale: Option<&NSLocale>);

        #[cfg(feature = "Foundation_NSNumberFormatter")]
        /**
         If not specified, the number formatter is set up with NSNumberFormatterDecimalStyle.
        */
        #[method_id(@__retain_semantics Other numberFormatter)]
        pub unsafe fn numberFormatter(&self) -> Id<NSNumberFormatter>;

        #[cfg(feature = "Foundation_NSNumberFormatter")]
        /**
         If not specified, the number formatter is set up with NSNumberFormatterDecimalStyle.
        */
        #[method(setNumberFormatter:)]
        pub unsafe fn setNumberFormatter(&self, number_formatter: Option<&NSNumberFormatter>);

        #[cfg(all(feature = "Foundation_NSMeasurement", feature = "Foundation_NSString"))]
        #[method_id(@__retain_semantics Other stringFromMeasurement:)]
        pub unsafe fn stringFromMeasurement(&self, measurement: &NSMeasurement) -> Id<NSString>;

        #[cfg(all(feature = "Foundation_NSString", feature = "Foundation_NSUnit"))]
        #[method_id(@__retain_semantics Other stringFromUnit:)]
        pub unsafe fn stringFromUnit(&self, unit: &NSUnit) -> Id<NSString>;
    }
);
