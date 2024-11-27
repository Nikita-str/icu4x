use core::iter::Iterator;

use crate::{AsCalendar, Calendar, Date};
use crate::calendar_arithmetic::CalendarArithmetic;
use super::AsArithmeticDate;

type Inner<A> = <<A as AsCalendar>::Calendar as Calendar>::DateInner;

#[derive(Clone, Copy, Debug)]
enum StepType {
    WithinMonthForward(u8),
    WithinMonthBackward(u8),
}
impl StepType {
    fn new_one_day_forward() -> Self {
        Self::WithinMonthForward(1)
    }
    fn new_one_day_backward() -> Self {
        Self::WithinMonthBackward(1)
    }

    fn from_i16(step: i16) -> Self {
        const FORWARD_MAX: i16 = u8::MAX as i16;
        const FORWARD_MIN: i16 = -FORWARD_MAX;

        match step {
            1..=FORWARD_MAX => Self::WithinMonthForward(step as u8),
            FORWARD_MIN..=-1 => Self::WithinMonthBackward((-step) as u8),
            0 => panic!("step was 0"),
            _ => panic!("step is out from bound ({step})"),
        }
    }

    fn check<C: CalendarArithmetic, T: AsArithmeticDate<C>>(self) {
        match self {
            StepType::WithinMonthForward(days) if T::MIN_DAYS_IN_MONTH.get() < days => {
                panic!("WithinMonthForward: step is more than min days in month for the given calendar")
            }
            StepType::WithinMonthBackward(days) if T::MIN_DAYS_IN_MONTH.get() < days => {
                panic!("WithinMonthBackward: step is more than min days in month for the given calendar")
            }
            _ => { }
        }
    }
}

#[derive(Debug)]
pub struct ArithmeticDateRangeFromIter<A: AsCalendar + Clone> {
    cur: Inner<A>,
    step: StepType,
    first: bool,
    calenda: A,
}


impl<A: AsCalendar + CalendarArithmetic + Clone> ArithmeticDateRangeFromIter<A>
where Inner<A>: AsArithmeticDate<A>
{
    // TODO:check step less or eq than MIN in MONTH  
    fn new(start: Inner<A>, step: StepType, calenda: A) -> Self {
        StepType::check::<A, Inner<A>>(step);
        Self {
            cur: start,
            step,
            first: true,
            calenda,
        }
    }

    pub fn new_one_day_forward(start: Inner<A>, calenda: A) -> Self {
        let step = StepType::new_one_day_forward();
        Self::new(start, step, calenda)
    }
    
    pub fn new_one_day_backward(start: Inner<A>, calenda: A) -> Self {
        let step = StepType::new_one_day_backward();
        Self::new(start, step, calenda)
    }
    
    pub fn new_within_month(start: Inner<A>, calenda: A, day_step: i16) -> Self {
        let step = StepType::from_i16(day_step);
        Self::new(start, step, calenda)
    }
}

impl<A: AsCalendar + CalendarArithmetic + Clone> Iterator for ArithmeticDateRangeFromIter<A>
where Inner<A>: AsArithmeticDate<A>
{
    type Item = Date<A>;
    
    fn next(&mut self) -> Option<Self::Item> {
        let prev = Inner::<A>::to_arithmetic_date(self.cur.clone());
        if self.first {
            self.first = false;
            return Some(Date::from_raw(self.cur.clone(), self.calenda.clone()))
        }

        
        let prev_day = prev.day;
        let prev_month = prev.month;
        let prev_year = prev.year;
        let prev_days_in_month = prev.days_in_month();
        let prev_month_in_year = prev.months_in_year();
        
        let mut cur = prev;

        // let cal = self.calenda.as_calendar();
        // let prev_day = cal.day_of_month(&self.cur).0;
        // let prev_days_in_month = cal.days_in_month(&self.cur);
        // let prev_month = cal.month(&self.cur).ordinal;
        // let prev_month_in_year = cal.months_in_year(&self.cur);
        // let prev_year = cal.year(&self.cur).extended_year;

        match self.step {
            StepType::WithinMonthForward(add_days) => {
                let mut new_day = prev_day + add_days;
                if prev_days_in_month < new_day {
                    new_day -= prev_days_in_month;
                    if prev_month == prev_month_in_year {
                        if prev_year == i32::MAX {
                            return None
                        }
                        cur.year += 1;
                        cur.year_info = Inner::<A>::year_info_for_year(cur.year);
                        cur.month = 1;
                    } else {
                        cur.month += 1;
                    }
                }
                cur.day = new_day;
            },
            StepType::WithinMonthBackward(sub_days) => {
                if prev_day <= sub_days {
                    if prev_month == 1 {
                        if prev_year == i32::MIN {
                            return None
                        }
                        cur.year -= 1;
                        cur.year_info = Inner::<A>::year_info_for_year(cur.year);
                        cur.month = cur.months_in_year();
                    } else {
                        cur.month -= 1;
                    }
                    cur.day = cur.days_in_month() + prev.day - sub_days;
                } else {
                    cur.day -= sub_days;
                }
            }
        };

        self.cur = Inner::<A>::from_arithmetic_date(cur);

        Some(Date::from_raw(self.cur.clone(), self.calenda.clone()))
    }
}


#[test]
fn test_range_01() {
    use crate::Iso;
    use calendrical_calculations::rata_die::RataDie;

    let date = Date::try_new_iso(2024, 3, 4).unwrap();
    let date = *date.inner();

    let mut range_backward_1 = ArithmeticDateRangeFromIter::new_within_month(date, Iso, -1);
    let mut range_backward_3 = ArithmeticDateRangeFromIter::new_within_month(date, Iso, -3);
    let mut range_forward_1 = ArithmeticDateRangeFromIter::new_within_month(date, Iso, 1);
    let mut range_forward_3 = ArithmeticDateRangeFromIter::new_within_month(date, Iso, 3);

    let rata_die_init = Iso::fixed_from_iso(date);
    
    for i in 0..10_000 {
        let backward_must = Iso::iso_from_fixed(RataDie::new(rata_die_init.to_i64_date() - i));
        let backward = range_backward_1.next().unwrap();
        assert_eq!(backward, backward_must);
        assert_eq!(backward.inner.0.year_info, backward_must.inner.0.year_info);

        let forward_must = Iso::iso_from_fixed(RataDie::new(rata_die_init.to_i64_date() + i));
        let forward = range_forward_1.next().unwrap();
        assert_eq!(forward, forward_must);
        assert_eq!(forward.inner.0.year_info, forward_must.inner.0.year_info);
        
        if i % 3 == 0 {
            let backward_3 = range_backward_3.next().unwrap();
            assert_eq!(backward_3, backward_must);
            assert_eq!(backward_3.inner.0.year_info, backward_must.inner.0.year_info);

            let forward_3 = range_forward_3.next().unwrap();
            assert_eq!(forward_3, forward_must);
            assert_eq!(forward_3.inner.0.year_info, forward_must.inner.0.year_info);
        }
    }
}
