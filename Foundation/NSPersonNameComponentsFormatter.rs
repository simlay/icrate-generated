//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

ns_enum!(
    #[underlying(NSInteger)]
    pub enum NSPersonNameComponentsFormatterStyle {
        NSPersonNameComponentsFormatterStyleDefault = 0,
        NSPersonNameComponentsFormatterStyleShort = 1,
        NSPersonNameComponentsFormatterStyleMedium = 2,
        NSPersonNameComponentsFormatterStyleLong = 3,
        NSPersonNameComponentsFormatterStyleAbbreviated = 4,
    }
);

ns_options!(
    #[underlying(NSUInteger)]
    pub enum NSPersonNameComponentsFormatterOptions {
        NSPersonNameComponentsFormatterPhonetic = 1 << 1,
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSPersonNameComponentsFormatter")]
    pub struct NSPersonNameComponentsFormatter;

    #[cfg(feature = "Foundation_NSPersonNameComponentsFormatter")]
    unsafe impl ClassType for NSPersonNameComponentsFormatter {
        #[inherits(NSObject)]
        type Super = NSFormatter;
    }
);

#[cfg(feature = "Foundation_NSPersonNameComponentsFormatter")]
unsafe impl NSCoding for NSPersonNameComponentsFormatter {}

#[cfg(feature = "Foundation_NSPersonNameComponentsFormatter")]
unsafe impl NSObjectProtocol for NSPersonNameComponentsFormatter {}

extern_methods!(
    #[cfg(feature = "Foundation_NSPersonNameComponentsFormatter")]
    unsafe impl NSPersonNameComponentsFormatter {
        #[method(style)]
        pub unsafe fn style(&self) -> NSPersonNameComponentsFormatterStyle;

        #[method(setStyle:)]
        pub unsafe fn setStyle(&self, style: NSPersonNameComponentsFormatterStyle);

        #[method(isPhonetic)]
        pub unsafe fn isPhonetic(&self) -> bool;

        #[method(setPhonetic:)]
        pub unsafe fn setPhonetic(&self, phonetic: bool);

        #[cfg(feature = "Foundation_NSLocale")]
        #[method_id(@__retain_semantics Other locale)]
        pub unsafe fn locale(&self) -> Id<NSLocale>;

        #[cfg(feature = "Foundation_NSLocale")]
        #[method(setLocale:)]
        pub unsafe fn setLocale(&self, locale: Option<&NSLocale>);

        #[cfg(all(
            feature = "Foundation_NSPersonNameComponents",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other localizedStringFromPersonNameComponents:style:options:)]
        pub unsafe fn localizedStringFromPersonNameComponents_style_options(
            components: &NSPersonNameComponents,
            name_format_style: NSPersonNameComponentsFormatterStyle,
            name_options: NSPersonNameComponentsFormatterOptions,
        ) -> Id<NSString>;

        #[cfg(all(
            feature = "Foundation_NSPersonNameComponents",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other stringFromPersonNameComponents:)]
        pub unsafe fn stringFromPersonNameComponents(
            &self,
            components: &NSPersonNameComponents,
        ) -> Id<NSString>;

        #[cfg(all(
            feature = "Foundation_NSAttributedString",
            feature = "Foundation_NSPersonNameComponents"
        ))]
        #[method_id(@__retain_semantics Other annotatedStringFromPersonNameComponents:)]
        pub unsafe fn annotatedStringFromPersonNameComponents(
            &self,
            components: &NSPersonNameComponents,
        ) -> Id<NSAttributedString>;

        #[cfg(all(
            feature = "Foundation_NSPersonNameComponents",
            feature = "Foundation_NSString"
        ))]
        #[method_id(@__retain_semantics Other personNameComponentsFromString:)]
        pub unsafe fn personNameComponentsFromString(
            &self,
            string: &NSString,
        ) -> Option<Id<NSPersonNameComponents>>;

        #[cfg(feature = "Foundation_NSString")]
        #[method(getObjectValue:forString:errorDescription:)]
        pub unsafe fn getObjectValue_forString_errorDescription(
            &self,
            obj: Option<&mut Option<Id<Object>>>,
            string: &NSString,
            error: Option<&mut Option<Id<NSString>>>,
        ) -> bool;
    }
);

extern_static!(NSPersonNameComponentKey: &'static NSString);

extern_static!(NSPersonNameComponentGivenName: &'static NSString);

extern_static!(NSPersonNameComponentFamilyName: &'static NSString);

extern_static!(NSPersonNameComponentMiddleName: &'static NSString);

extern_static!(NSPersonNameComponentPrefix: &'static NSString);

extern_static!(NSPersonNameComponentSuffix: &'static NSString);

extern_static!(NSPersonNameComponentNickname: &'static NSString);

extern_static!(NSPersonNameComponentDelimiter: &'static NSString);
