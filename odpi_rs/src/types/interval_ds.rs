use odpic_sys::*;

#[derive(Clone, Copy, Debug)]
pub struct IntervalDS {
    pub days: i32,
    pub hours: i32,
    pub minutes: i32,
    pub seconds: i32,
    pub fseconds: i32,
}

impl From<IntervalDS> for dpiIntervalDS {
    fn from(value: IntervalDS) -> dpiIntervalDS {
        dpiIntervalDS {
            days: value.days,
            hours: value.hours,
            minutes: value.minutes,
            seconds: value.seconds,
            fseconds: value.fseconds,
        }
    }
}

impl From<dpiIntervalDS> for IntervalDS {
    fn from(value: dpiIntervalDS) -> IntervalDS {
        IntervalDS {
            days: value.days,
            hours: value.hours,
            minutes: value.minutes,
            seconds: value.seconds,
            fseconds: value.fseconds,
        }
    }
}
