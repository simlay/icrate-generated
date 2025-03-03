//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSScanner")]
    pub struct NSScanner;

    #[cfg(feature = "Foundation_NSScanner")]
    unsafe impl ClassType for NSScanner {
        type Super = NSObject;
    }
);

#[cfg(feature = "Foundation_NSScanner")]
unsafe impl NSObjectProtocol for NSScanner {}

extern_methods!(
    #[cfg(feature = "Foundation_NSScanner")]
    unsafe impl NSScanner {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other string)]
        pub unsafe fn string(&self) -> Id<NSString>;

        #[method(scanLocation)]
        pub unsafe fn scanLocation(&self) -> NSUInteger;

        #[method(setScanLocation:)]
        pub unsafe fn setScanLocation(&self, scan_location: NSUInteger);

        #[cfg(feature = "Foundation_NSCharacterSet")]
        #[method_id(@__retain_semantics Other charactersToBeSkipped)]
        pub unsafe fn charactersToBeSkipped(&self) -> Option<Id<NSCharacterSet>>;

        #[cfg(feature = "Foundation_NSCharacterSet")]
        #[method(setCharactersToBeSkipped:)]
        pub unsafe fn setCharactersToBeSkipped(
            &self,
            characters_to_be_skipped: Option<&NSCharacterSet>,
        );

        #[method(caseSensitive)]
        pub unsafe fn caseSensitive(&self) -> bool;

        #[method(setCaseSensitive:)]
        pub unsafe fn setCaseSensitive(&self, case_sensitive: bool);

        #[method_id(@__retain_semantics Other locale)]
        pub unsafe fn locale(&self) -> Option<Id<Object>>;

        #[method(setLocale:)]
        pub unsafe fn setLocale(&self, locale: Option<&Object>);

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithString:)]
        pub unsafe fn initWithString(this: Option<Allocated<Self>>, string: &NSString) -> Id<Self>;
    }
);

extern_methods!(
    /// NSExtendedScanner
    #[cfg(feature = "Foundation_NSScanner")]
    unsafe impl NSScanner {
        #[method(scanInt:)]
        pub unsafe fn scanInt(&self, result: *mut c_int) -> bool;

        #[method(scanInteger:)]
        pub unsafe fn scanInteger(&self, result: *mut NSInteger) -> bool;

        #[method(scanLongLong:)]
        pub unsafe fn scanLongLong(&self, result: *mut c_longlong) -> bool;

        #[method(scanUnsignedLongLong:)]
        pub unsafe fn scanUnsignedLongLong(&self, result: *mut c_ulonglong) -> bool;

        #[method(scanFloat:)]
        pub unsafe fn scanFloat(&self, result: *mut c_float) -> bool;

        #[method(scanDouble:)]
        pub unsafe fn scanDouble(&self, result: *mut c_double) -> bool;

        #[method(scanHexInt:)]
        pub unsafe fn scanHexInt(&self, result: *mut c_uint) -> bool;

        #[method(scanHexLongLong:)]
        pub unsafe fn scanHexLongLong(&self, result: *mut c_ulonglong) -> bool;

        #[method(scanHexFloat:)]
        pub unsafe fn scanHexFloat(&self, result: *mut c_float) -> bool;

        #[method(scanHexDouble:)]
        pub unsafe fn scanHexDouble(&self, result: *mut c_double) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method(scanString:intoString:)]
        pub unsafe fn scanString_intoString(
            &self,
            string: &NSString,
            result: Option<&mut Option<Id<NSString>>>,
        ) -> bool;

        #[cfg(all(feature = "Foundation_NSCharacterSet", feature = "Foundation_NSString"))]
        #[method(scanCharactersFromSet:intoString:)]
        pub unsafe fn scanCharactersFromSet_intoString(
            &self,
            set: &NSCharacterSet,
            result: Option<&mut Option<Id<NSString>>>,
        ) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method(scanUpToString:intoString:)]
        pub unsafe fn scanUpToString_intoString(
            &self,
            string: &NSString,
            result: Option<&mut Option<Id<NSString>>>,
        ) -> bool;

        #[cfg(all(feature = "Foundation_NSCharacterSet", feature = "Foundation_NSString"))]
        #[method(scanUpToCharactersFromSet:intoString:)]
        pub unsafe fn scanUpToCharactersFromSet_intoString(
            &self,
            set: &NSCharacterSet,
            result: Option<&mut Option<Id<NSString>>>,
        ) -> bool;

        #[method(isAtEnd)]
        pub unsafe fn isAtEnd(&self) -> bool;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other scannerWithString:)]
        pub unsafe fn scannerWithString(string: &NSString) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other localizedScannerWithString:)]
        pub unsafe fn localizedScannerWithString(string: &NSString) -> Id<Object>;
    }
);
