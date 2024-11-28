use core::num::NonZero;
use crate::calendar_arithmetic::{ArithmeticDate, CalendarArithmetic};
use crate::calendar_arithmetic::PrecomputedDataSource;

pub(in super) trait AsArithmeticDate<C: CalendarArithmetic> {
    const MIN_DAYS_IN_MONTH: NonZero<u8>;
    
    fn year_info_for_year(year: i32, calendar: &C) -> C::YearInfo;

    fn to_arithmetic_date(self) -> ArithmeticDate<C>;
    fn from_arithmetic_date(arithmetic_date: ArithmeticDate<C>) -> Self;
}

/// Generate something like this:
///```rust,ignore
/// impl AsArithmeticDate<crate::Iso> for crate::iso::IsoDateInner {
///     // Safety: 28 is not 0
///     const MIN_DAYS_IN_MONTH: NonZero<u8> = unsafe { NonZero::new_unchecked(28) };
///
///     fn to_arithmetic_date(self) -> ArithmeticDate<crate::Iso> { self.0 }
///     fn from_arithmetic_date(arithmetic_date: ArithmeticDate<crate::Iso>) -> Self { Self(arithmetic_date) }
///     fn year_info_for_year(_year: i32) -> <crate::Iso as CalendarArithmetic>::YearInfo { () }
/// }
/// ```
macro_rules! impl_as_arithmetic_date {
    (0, $cal: ty, $inner: ty) => {
        !ZERO_IS_INVALID!
    };
    ($non_zero: literal, $cal: ty, $inner: ty) => {
        impl AsArithmeticDate<$cal> for $inner {
            // Safety: `non_zero` is not 0
            const MIN_DAYS_IN_MONTH: NonZero<u8> = unsafe { NonZero::new_unchecked($non_zero) };
        
            fn to_arithmetic_date(self) -> ArithmeticDate<$cal> { self.0 }
            fn from_arithmetic_date(arithmetic_date: ArithmeticDate<$cal>) -> Self { Self(arithmetic_date) }
            fn year_info_for_year(_year: i32, _: &$cal) -> <$cal as CalendarArithmetic>::YearInfo { () }
        }
    };
}

impl_as_arithmetic_date!(28, crate::Iso, crate::iso::IsoDateInner);
impl_as_arithmetic_date!(5, crate::coptic::Coptic, crate::coptic::CopticDateInner);
impl_as_arithmetic_date!(5, crate::ethiopian::Ethiopian, crate::ethiopian::EthiopianDateInner);
impl_as_arithmetic_date!(30, crate::indian::Indian, crate::indian::IndianDateInner);
impl_as_arithmetic_date!(29, crate::islamic::IslamicCivil, crate::islamic::IslamicCivilDateInner);
impl_as_arithmetic_date!(29, crate::islamic::IslamicTabular, crate::islamic::IslamicTabularDateInner);
impl_as_arithmetic_date!(29, crate::persian::Persian, crate::persian::PersianDateInner);

impl AsArithmeticDate<crate::islamic::IslamicUmmAlQura> for crate::islamic::IslamicUmmAlQuraDateInner {
    // Safety: `29` is not 0
    const MIN_DAYS_IN_MONTH: NonZero<u8> = unsafe { NonZero::new_unchecked(29) };

    fn to_arithmetic_date(self) -> ArithmeticDate<crate::islamic::IslamicUmmAlQura> { self.0 }
    fn from_arithmetic_date(arithmetic_date: ArithmeticDate<crate::islamic::IslamicUmmAlQura>) -> Self { Self(arithmetic_date) }
    fn year_info_for_year(year: i32, calendar: &crate::islamic::IslamicUmmAlQura) -> <crate::islamic::IslamicUmmAlQura as CalendarArithmetic>::YearInfo {
        calendar
            .precomputed_data()
            .load_or_compute_info(year)
    }
}

impl AsArithmeticDate<crate::islamic::IslamicObservational> for crate::islamic::IslamicDateInner {
    // Safety: `29` is not 0
    const MIN_DAYS_IN_MONTH: NonZero<u8> = unsafe { NonZero::new_unchecked(29) };

    fn to_arithmetic_date(self) -> ArithmeticDate<crate::islamic::IslamicObservational> { self.0 }
    fn from_arithmetic_date(arithmetic_date: ArithmeticDate<crate::islamic::IslamicObservational>) -> Self { Self(arithmetic_date) }
    fn year_info_for_year(year: i32, calendar: &crate::islamic::IslamicObservational) -> <crate::islamic::IslamicUmmAlQura as CalendarArithmetic>::YearInfo {
        calendar
            .precomputed_data()
            .load_or_compute_info(year)
    }
}

impl AsArithmeticDate<crate::Iso> for crate::gregorian::GregorianDateInner {
    // Safety: 28 is not 0
    const MIN_DAYS_IN_MONTH: NonZero<u8> = unsafe { NonZero::new_unchecked(28) };

    fn to_arithmetic_date(self) -> ArithmeticDate<crate::Iso> { self.0.0 }
    fn from_arithmetic_date(arithmetic_date: ArithmeticDate<crate::Iso>) -> Self {
        Self(crate::iso::IsoDateInner(arithmetic_date))
    }
    fn year_info_for_year(_year: i32, _: &crate::Iso) -> <crate::Iso as CalendarArithmetic>::YearInfo { () }
}
impl AsArithmeticDate<crate::Iso> for crate::roc::RocDateInner {
    // Safety: 28 is not 0
    const MIN_DAYS_IN_MONTH: NonZero<u8> = unsafe { NonZero::new_unchecked(28) };

    fn to_arithmetic_date(self) -> ArithmeticDate<crate::Iso> { self.0.0 }
    fn from_arithmetic_date(arithmetic_date: ArithmeticDate<crate::Iso>) -> Self {
        Self(crate::iso::IsoDateInner(arithmetic_date))
    }
    fn year_info_for_year(_year: i32, _: &crate::Iso) -> <crate::Iso as CalendarArithmetic>::YearInfo { () }
}

impl AsArithmeticDate<crate::hebrew::Hebrew> for crate::hebrew::HebrewDateInner {
    // Safety: 29 is not 0
    const MIN_DAYS_IN_MONTH: NonZero<u8> = unsafe { NonZero::new_unchecked(29) };

    fn to_arithmetic_date(self) -> ArithmeticDate<crate::hebrew::Hebrew> { self.0 }
    fn from_arithmetic_date(arithmetic_date: ArithmeticDate<crate::hebrew::Hebrew>) -> Self { Self(arithmetic_date) }
    fn year_info_for_year(year: i32, _: &crate::hebrew::Hebrew) -> <crate::hebrew::Hebrew as CalendarArithmetic>::YearInfo {
        crate::hebrew::HebrewYearInfo::compute(year)
    }
}

impl AsArithmeticDate<crate::chinese::Chinese> for crate::chinese::ChineseDateInner {
    // Safety: 29 is not 0
    const MIN_DAYS_IN_MONTH: NonZero<u8> = unsafe { NonZero::new_unchecked(29) };

    fn to_arithmetic_date(self) -> ArithmeticDate<crate::chinese::Chinese> { self.0.0 }
    fn from_arithmetic_date(arithmetic_date: ArithmeticDate<crate::chinese::Chinese>) -> Self {
        Self(crate::chinese_based::ChineseBasedDateInner(arithmetic_date))
    }
    fn year_info_for_year(year: i32, _: &crate::chinese::Chinese) -> <crate::chinese::Chinese as CalendarArithmetic>::YearInfo {
        crate::chinese_based::compute_cache::<calendrical_calculations::chinese_based::Chinese>(year)
    }
}

impl AsArithmeticDate<crate::dangi::Dangi> for crate::dangi::DangiDateInner {
    // Safety: 29 is not 0
    const MIN_DAYS_IN_MONTH: NonZero<u8> = unsafe { NonZero::new_unchecked(29) };

    fn to_arithmetic_date(self) -> ArithmeticDate<crate::dangi::Dangi> { self.0.0 }
    fn from_arithmetic_date(arithmetic_date: ArithmeticDate<crate::dangi::Dangi>) -> Self {
        Self(crate::chinese_based::ChineseBasedDateInner(arithmetic_date))
    }
    fn year_info_for_year(year: i32, _: &crate::dangi::Dangi) -> <crate::dangi::Dangi as CalendarArithmetic>::YearInfo {
        crate::chinese_based::compute_cache::<calendrical_calculations::chinese_based::Dangi>(year)
    }
}

//TODO:japanese
