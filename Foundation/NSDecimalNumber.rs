//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_static!(NSDecimalNumberExactnessException: &'static NSExceptionName);

extern_static!(NSDecimalNumberOverflowException: &'static NSExceptionName);

extern_static!(NSDecimalNumberUnderflowException: &'static NSExceptionName);

extern_static!(NSDecimalNumberDivideByZeroException: &'static NSExceptionName);

extern_protocol!(
    pub unsafe trait NSDecimalNumberBehaviors {
        #[method(roundingMode)]
        unsafe fn roundingMode(&self) -> NSRoundingMode;

        #[method(scale)]
        unsafe fn scale(&self) -> c_short;

        #[cfg(feature = "Foundation_NSDecimalNumber")]
        #[method_id(@__retain_semantics Other exceptionDuringOperation:error:leftOperand:rightOperand:)]
        unsafe fn exceptionDuringOperation_error_leftOperand_rightOperand(
            &self,
            operation: Sel,
            error: NSCalculationError,
            left_operand: &NSDecimalNumber,
            right_operand: Option<&NSDecimalNumber>,
        ) -> Option<Id<NSDecimalNumber>>;
    }

    unsafe impl ProtocolType for dyn NSDecimalNumberBehaviors {}
);

extern_class!(
    #[derive(Debug, PartialEq, Hash)]
    #[cfg(feature = "Foundation_NSDecimalNumber")]
    /**
         Rounding and Exception behavior
    */
    pub struct NSDecimalNumber;

    #[cfg(feature = "Foundation_NSDecimalNumber")]
    unsafe impl ClassType for NSDecimalNumber {
        #[inherits(NSValue, NSObject)]
        type Super = NSNumber;
    }
);

#[cfg(feature = "Foundation_NSDecimalNumber")]
/**
     Rounding and Exception behavior
*/
unsafe impl NSCoding for NSDecimalNumber {}

#[cfg(feature = "Foundation_NSDecimalNumber")]
/**
     Rounding and Exception behavior
*/
unsafe impl NSObjectProtocol for NSDecimalNumber {}

#[cfg(feature = "Foundation_NSDecimalNumber")]
/**
     Rounding and Exception behavior
*/
unsafe impl NSSecureCoding for NSDecimalNumber {}

extern_methods!(
    /**
         Rounding and Exception behavior
    */
    #[cfg(feature = "Foundation_NSDecimalNumber")]
    unsafe impl NSDecimalNumber {
        #[method_id(@__retain_semantics Init initWithMantissa:exponent:isNegative:)]
        pub unsafe fn initWithMantissa_exponent_isNegative(
            this: Option<Allocated<Self>>,
            mantissa: c_ulonglong,
            exponent: c_short,
            flag: bool,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithDecimal:)]
        pub unsafe fn initWithDecimal(this: Option<Allocated<Self>>, dcm: NSDecimal) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithString:)]
        pub unsafe fn initWithString(
            this: Option<Allocated<Self>>,
            number_value: Option<&NSString>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithString:locale:)]
        pub unsafe fn initWithString_locale(
            this: Option<Allocated<Self>>,
            number_value: Option<&NSString>,
            locale: Option<&Object>,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other descriptionWithLocale:)]
        pub unsafe fn descriptionWithLocale(&self, locale: Option<&Object>) -> Id<NSString>;

        #[method(decimalValue)]
        pub unsafe fn decimalValue(&self) -> NSDecimal;

        #[method_id(@__retain_semantics Other decimalNumberWithMantissa:exponent:isNegative:)]
        pub unsafe fn decimalNumberWithMantissa_exponent_isNegative(
            mantissa: c_ulonglong,
            exponent: c_short,
            flag: bool,
        ) -> Id<NSDecimalNumber>;

        #[method_id(@__retain_semantics Other decimalNumberWithDecimal:)]
        pub unsafe fn decimalNumberWithDecimal(dcm: NSDecimal) -> Id<NSDecimalNumber>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other decimalNumberWithString:)]
        pub unsafe fn decimalNumberWithString(
            number_value: Option<&NSString>,
        ) -> Id<NSDecimalNumber>;

        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Other decimalNumberWithString:locale:)]
        pub unsafe fn decimalNumberWithString_locale(
            number_value: Option<&NSString>,
            locale: Option<&Object>,
        ) -> Id<NSDecimalNumber>;

        #[method_id(@__retain_semantics Other zero)]
        pub unsafe fn zero() -> Id<NSDecimalNumber>;

        #[method_id(@__retain_semantics Other one)]
        pub unsafe fn one() -> Id<NSDecimalNumber>;

        #[method_id(@__retain_semantics Other minimumDecimalNumber)]
        pub unsafe fn minimumDecimalNumber() -> Id<NSDecimalNumber>;

        #[method_id(@__retain_semantics Other maximumDecimalNumber)]
        pub unsafe fn maximumDecimalNumber() -> Id<NSDecimalNumber>;

        #[method_id(@__retain_semantics Other notANumber)]
        pub unsafe fn notANumber() -> Id<NSDecimalNumber>;

        #[method_id(@__retain_semantics Other decimalNumberByAdding:)]
        pub unsafe fn decimalNumberByAdding(
            &self,
            decimal_number: &NSDecimalNumber,
        ) -> Id<NSDecimalNumber>;

        #[method_id(@__retain_semantics Other decimalNumberByAdding:withBehavior:)]
        pub unsafe fn decimalNumberByAdding_withBehavior(
            &self,
            decimal_number: &NSDecimalNumber,
            behavior: Option<&ProtocolObject<dyn NSDecimalNumberBehaviors>>,
        ) -> Id<NSDecimalNumber>;

        #[method_id(@__retain_semantics Other decimalNumberBySubtracting:)]
        pub unsafe fn decimalNumberBySubtracting(
            &self,
            decimal_number: &NSDecimalNumber,
        ) -> Id<NSDecimalNumber>;

        #[method_id(@__retain_semantics Other decimalNumberBySubtracting:withBehavior:)]
        pub unsafe fn decimalNumberBySubtracting_withBehavior(
            &self,
            decimal_number: &NSDecimalNumber,
            behavior: Option<&ProtocolObject<dyn NSDecimalNumberBehaviors>>,
        ) -> Id<NSDecimalNumber>;

        #[method_id(@__retain_semantics Other decimalNumberByMultiplyingBy:)]
        pub unsafe fn decimalNumberByMultiplyingBy(
            &self,
            decimal_number: &NSDecimalNumber,
        ) -> Id<NSDecimalNumber>;

        #[method_id(@__retain_semantics Other decimalNumberByMultiplyingBy:withBehavior:)]
        pub unsafe fn decimalNumberByMultiplyingBy_withBehavior(
            &self,
            decimal_number: &NSDecimalNumber,
            behavior: Option<&ProtocolObject<dyn NSDecimalNumberBehaviors>>,
        ) -> Id<NSDecimalNumber>;

        #[method_id(@__retain_semantics Other decimalNumberByDividingBy:)]
        pub unsafe fn decimalNumberByDividingBy(
            &self,
            decimal_number: &NSDecimalNumber,
        ) -> Id<NSDecimalNumber>;

        #[method_id(@__retain_semantics Other decimalNumberByDividingBy:withBehavior:)]
        pub unsafe fn decimalNumberByDividingBy_withBehavior(
            &self,
            decimal_number: &NSDecimalNumber,
            behavior: Option<&ProtocolObject<dyn NSDecimalNumberBehaviors>>,
        ) -> Id<NSDecimalNumber>;

        #[method_id(@__retain_semantics Other decimalNumberByRaisingToPower:)]
        pub unsafe fn decimalNumberByRaisingToPower(
            &self,
            power: NSUInteger,
        ) -> Id<NSDecimalNumber>;

        #[method_id(@__retain_semantics Other decimalNumberByRaisingToPower:withBehavior:)]
        pub unsafe fn decimalNumberByRaisingToPower_withBehavior(
            &self,
            power: NSUInteger,
            behavior: Option<&ProtocolObject<dyn NSDecimalNumberBehaviors>>,
        ) -> Id<NSDecimalNumber>;

        #[method_id(@__retain_semantics Other decimalNumberByMultiplyingByPowerOf10:)]
        pub unsafe fn decimalNumberByMultiplyingByPowerOf10(
            &self,
            power: c_short,
        ) -> Id<NSDecimalNumber>;

        #[method_id(@__retain_semantics Other decimalNumberByMultiplyingByPowerOf10:withBehavior:)]
        pub unsafe fn decimalNumberByMultiplyingByPowerOf10_withBehavior(
            &self,
            power: c_short,
            behavior: Option<&ProtocolObject<dyn NSDecimalNumberBehaviors>>,
        ) -> Id<NSDecimalNumber>;

        #[method_id(@__retain_semantics Other decimalNumberByRoundingAccordingToBehavior:)]
        pub unsafe fn decimalNumberByRoundingAccordingToBehavior(
            &self,
            behavior: Option<&ProtocolObject<dyn NSDecimalNumberBehaviors>>,
        ) -> Id<NSDecimalNumber>;

        #[method(compare:)]
        pub unsafe fn compare(&self, decimal_number: &NSNumber) -> NSComparisonResult;

        /**
          compare two NSDecimalNumbers
        */
        #[method_id(@__retain_semantics Other defaultBehavior)]
        pub unsafe fn defaultBehavior() -> Id<ProtocolObject<dyn NSDecimalNumberBehaviors>>;

        /**
          compare two NSDecimalNumbers
        */
        #[method(setDefaultBehavior:)]
        pub unsafe fn setDefaultBehavior(
            default_behavior: &ProtocolObject<dyn NSDecimalNumberBehaviors>,
        );

        /**
          One behavior per thread - The default behavior is
           rounding mode: NSRoundPlain
           scale: No defined scale (full precision)
           ignore exactnessException
           raise on overflow, underflow and divide by zero.
        */
        #[method(objCType)]
        pub unsafe fn objCType(&self) -> NonNull<c_char>;

        /**
          return 'd' for double
        */
        #[method(doubleValue)]
        pub unsafe fn doubleValue(&self) -> c_double;
    }
);

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSDecimalNumberHandler")]
    pub struct NSDecimalNumberHandler;

    #[cfg(feature = "Foundation_NSDecimalNumberHandler")]
    unsafe impl ClassType for NSDecimalNumberHandler {
        type Super = NSObject;
    }
);

#[cfg(feature = "Foundation_NSDecimalNumberHandler")]
unsafe impl NSCoding for NSDecimalNumberHandler {}

#[cfg(feature = "Foundation_NSDecimalNumberHandler")]
unsafe impl NSDecimalNumberBehaviors for NSDecimalNumberHandler {}

#[cfg(feature = "Foundation_NSDecimalNumberHandler")]
unsafe impl NSObjectProtocol for NSDecimalNumberHandler {}

extern_methods!(
    #[cfg(feature = "Foundation_NSDecimalNumberHandler")]
    unsafe impl NSDecimalNumberHandler {
        #[method_id(@__retain_semantics Other defaultDecimalNumberHandler)]
        pub unsafe fn defaultDecimalNumberHandler() -> Id<NSDecimalNumberHandler>;

        #[method_id(@__retain_semantics Init initWithRoundingMode:scale:raiseOnExactness:raiseOnOverflow:raiseOnUnderflow:raiseOnDivideByZero:)]
        pub unsafe fn initWithRoundingMode_scale_raiseOnExactness_raiseOnOverflow_raiseOnUnderflow_raiseOnDivideByZero(
            this: Option<Allocated<Self>>,
            rounding_mode: NSRoundingMode,
            scale: c_short,
            exact: bool,
            overflow: bool,
            underflow: bool,
            divide_by_zero: bool,
        ) -> Id<Self>;

        #[method_id(@__retain_semantics Other decimalNumberHandlerWithRoundingMode:scale:raiseOnExactness:raiseOnOverflow:raiseOnUnderflow:raiseOnDivideByZero:)]
        pub unsafe fn decimalNumberHandlerWithRoundingMode_scale_raiseOnExactness_raiseOnOverflow_raiseOnUnderflow_raiseOnDivideByZero(
            rounding_mode: NSRoundingMode,
            scale: c_short,
            exact: bool,
            overflow: bool,
            underflow: bool,
            divide_by_zero: bool,
        ) -> Id<Self>;
    }
);

extern_methods!(
    /**
         Extensions to other classes
    */
    /// NSDecimalNumberExtensions
    #[cfg(feature = "Foundation_NSNumber")]
    unsafe impl NSNumber {
        #[method(decimalValue)]
        pub unsafe fn decimalValue(&self) -> NSDecimal;
    }
);

extern_methods!(
    /// NSDecimalNumberScanning
    #[cfg(feature = "Foundation_NSScanner")]
    unsafe impl NSScanner {
        #[method(scanDecimal:)]
        pub unsafe fn scanDecimal(&self, dcm: *mut NSDecimal) -> bool;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSValue`
    /**
         Rounding and Exception behavior
    */
    #[cfg(feature = "Foundation_NSDecimalNumber")]
    unsafe impl NSDecimalNumber {
        #[method_id(@__retain_semantics Init initWithBytes:objCType:)]
        pub unsafe fn initWithBytes_objCType(
            this: Option<Allocated<Self>>,
            value: NonNull<c_void>,
            r#type: NonNull<c_char>,
        ) -> Id<Self>;
    }
);
