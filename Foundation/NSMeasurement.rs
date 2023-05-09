//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

__inner_extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "Foundation_NSMeasurement")]
    pub struct NSMeasurement<UnitType: Message = Object> {
        __superclass: NSObject,
        _inner0: PhantomData<*mut UnitType>,
        notunwindsafe: PhantomData<&'static mut ()>,
    }

    #[cfg(feature = "Foundation_NSMeasurement")]
    unsafe impl<UnitType: Message> ClassType for NSMeasurement<UnitType> {
        type Super = NSObject;
        type Mutability = InteriorMutable;

        fn as_super(&self) -> &Self::Super {
            &self.__superclass
        }

        fn as_super_mut(&mut self) -> &mut Self::Super {
            &mut self.__superclass
        }
    }
);

#[cfg(feature = "Foundation_NSMeasurement")]
unsafe impl<UnitType: Message + NSCoding> NSCoding for NSMeasurement<UnitType> {}

#[cfg(feature = "Foundation_NSMeasurement")]
unsafe impl<UnitType: IsIdCloneable> NSCopying for NSMeasurement<UnitType> {}

#[cfg(feature = "Foundation_NSMeasurement")]
unsafe impl<UnitType: Message> NSObjectProtocol for NSMeasurement<UnitType> {}

#[cfg(feature = "Foundation_NSMeasurement")]
unsafe impl<UnitType: Message + NSSecureCoding> NSSecureCoding for NSMeasurement<UnitType> {}

extern_methods!(
    #[cfg(feature = "Foundation_NSMeasurement")]
    unsafe impl<UnitType: Message> NSMeasurement<UnitType> {
        #[method_id(@__retain_semantics Other unit)]
        pub unsafe fn unit(&self) -> Id<UnitType>;

        #[method(doubleValue)]
        pub unsafe fn doubleValue(&self) -> c_double;

        #[cfg(not(any(
            target_os = "ios",
            target_os = "macos",
            target_os = "tvos",
            target_os = "watchos"
        )))]
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Option<Allocated<Self>>) -> Id<Self>;

        #[method_id(@__retain_semantics Init initWithDoubleValue:unit:)]
        pub unsafe fn initWithDoubleValue_unit(
            this: Option<Allocated<Self>>,
            double_value: c_double,
            unit: &UnitType,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSUnit")]
        #[method(canBeConvertedToUnit:)]
        pub unsafe fn canBeConvertedToUnit(&self, unit: &NSUnit) -> bool;

        #[cfg(feature = "Foundation_NSUnit")]
        #[method_id(@__retain_semantics Other measurementByConvertingToUnit:)]
        pub unsafe fn measurementByConvertingToUnit(&self, unit: &NSUnit) -> Id<NSMeasurement>;

        #[method_id(@__retain_semantics Other measurementByAddingMeasurement:)]
        pub unsafe fn measurementByAddingMeasurement(
            &self,
            measurement: &NSMeasurement<UnitType>,
        ) -> Id<NSMeasurement<UnitType>>;

        #[method_id(@__retain_semantics Other measurementBySubtractingMeasurement:)]
        pub unsafe fn measurementBySubtractingMeasurement(
            &self,
            measurement: &NSMeasurement<UnitType>,
        ) -> Id<NSMeasurement<UnitType>>;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    #[cfg(feature = "Foundation_NSMeasurement")]
    unsafe impl<UnitType: Message> NSMeasurement<UnitType> {
        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Id<Self>;
    }
);
