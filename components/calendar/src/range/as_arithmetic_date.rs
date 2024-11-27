use core::num::NonZero;
use crate::calendar_arithmetic::{ArithmeticDate, CalendarArithmetic};

pub(in super) trait AsArithmeticDate<C: CalendarArithmetic> {
    const MIN_DAYS_IN_MONTH: NonZero<u8>;
    
    fn year_info_for_year(year: i32) -> C::YearInfo;
    
    fn to_arithmetic_date(self) -> ArithmeticDate<C>;
    fn from_arithmetic_date(arithmetic_date: ArithmeticDate<C>) -> Self;

}


impl AsArithmeticDate<crate::Iso> for crate::iso::IsoDateInner {
    // Safety: 28 is not 0
    const MIN_DAYS_IN_MONTH: NonZero<u8> = unsafe { NonZero::new_unchecked(28) };

    fn to_arithmetic_date(self) -> ArithmeticDate<crate::Iso> { self.0 }
    fn from_arithmetic_date(arithmetic_date: ArithmeticDate<crate::Iso>) -> Self { Self(arithmetic_date) }
    fn year_info_for_year(_year: i32) -> <crate::Iso as CalendarArithmetic>::YearInfo { () }
}
