//! This package enables you to work with the Arvile calendar.
extern crate chrono;
use chrono::{Datelike, Utc};

/// The data structure to represent a date.
#[derive(Debug, Copy, Clone)]
pub struct Date {
    date: chrono::Date<Utc>,
}

impl Date {
    /// Return date as chronos::Date
    pub fn into_chrono(self) -> chrono::Date<Utc> {
        self.date.clone()
    }

    /// Retrieve the number of the current Arvile day in current month
    pub fn get_dom(self) -> u32 {
        let dow = self.date.ordinal() % 14;
        if dow == 0 {
            return 14;
        }
        return dow;
    }
}

// trait implementations
impl From<chrono::Date<Utc>> for Date {
    fn from(item: chrono::Date<Utc>) -> Self {
        Date { date: item }
    }
}

impl From<&chrono::Date<Utc>> for Date {
    fn from(item: &chrono::Date<Utc>) -> Self {
        Date { date: *item }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn get_test_data() -> (Vec<chrono::Date<Utc>>, Vec<Date>) {
        let utc_dates = vec![
            Utc.ymd(2002, 1, 1),
            Utc.ymd(2001, 2, 18),
            Utc.ymd(2013, 1, 26),
            Utc.ymd(2002, 3, 4),
            Utc.ymd(2024, 1, 29),
            Utc.ymd(2003, 12, 31),
            Utc.ymd(2020, 1, 14),
        ];
        let mut arvile_dates = Vec::<Date>::new();
        for i in &utc_dates {
            arvile_dates.push(crate::Date::from(i));
        }
        return (utc_dates, arvile_dates);
    }

    #[test]
    fn from_to_chrono() {
        let dt1 = Utc.ymd(1984, 2, 2);
        let av1 = crate::Date::from(dt1);
        assert_eq!(av1.into_chrono(), dt1);
    }

    #[test]
    fn day_of_month() {
        let (_, adates) = get_test_data();
        let dom = vec![1, 7, 12, 7, 1, 1, 14];
        for i in 0..adates.len() {
            assert_eq!(adates[i].get_dom(), dom[i]);
        }
    }
}
