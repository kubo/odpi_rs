use odpic_sys::*;

#[derive(Clone, Copy, Debug)]
pub struct IntervalYM {
    pub years: i32,
    pub months: i32,
}

impl From<IntervalYM> for dpiIntervalYM {
    fn from(value: IntervalYM) -> dpiIntervalYM {
        dpiIntervalYM {
            years: value.years,
            months: value.months,
        }
    }
}

impl From<dpiIntervalYM> for IntervalYM {
    fn from(value: dpiIntervalYM) -> IntervalYM {
        IntervalYM {
            years: value.years,
            months: value.months,
        }
    }
}
