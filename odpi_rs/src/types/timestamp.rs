use odpic_sys::*;

#[derive(Clone, Copy, Debug)]
pub struct Timestamp {
    pub year: i16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub fsecond: u32,
    pub tz_hour_offset: i8,
    pub tz_minute_offset: i8,
}

impl From<Timestamp> for dpiTimestamp {
    fn from(value: Timestamp) -> dpiTimestamp {
        dpiTimestamp {
            year: value.year,
            month: value.month,
            day: value.day,
            hour: value.hour,
            minute: value.minute,
            second: value.second,
            fsecond: value.fsecond,
            tzHourOffset: value.tz_hour_offset,
            tzMinuteOffset: value.tz_minute_offset,
        }
    }
}

impl From<dpiTimestamp> for Timestamp {
    fn from(value: dpiTimestamp) -> Timestamp {
        Timestamp {
            year: value.year,
            month: value.month,
            day: value.day,
            hour: value.hour,
            minute: value.minute,
            second: value.second,
            fsecond: value.fsecond,
            tz_hour_offset: value.tzHourOffset,
            tz_minute_offset: value.tzMinuteOffset,
        }
    }
}
