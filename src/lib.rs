//! This package enables you to work with the Arvile calendar.
use std::fmt;
extern crate chrono;
use chrono::{Datelike, Utc};

/// The data structure to represent a date.
#[derive(Debug, Copy, Clone)]
pub struct Date {
    date: chrono::Date<Utc>,
}

impl Date {
    /// Check if current year is leap year
    fn is_leap_year(self) -> bool {
        let year = self.date.year();
        if year % 4 != 0 {
            // definetly no leap year
            return false;
        } else {
            // possibly a leap year
            if year % 100 == 0 {
                if year % 400 == 0 {
                    // this time for sure
                    return true;
                }
                // but not when turning a century
                return false;
            }
        }
        return true;
    }

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

    /// Retrieve the number of the current Arvile month
    /// *note*: 27 encodes year day / leap day
    pub fn get_month_u32(self) -> u32 {
        let dcount = self.date.ordinal(); // days elapsed (incl. current)
        let mlen = 14; // length of a month (in days)
        let remainder = ((dcount % mlen) + mlen) % mlen; // full months + remainder = current day

        // number of full months (& off-by-one for counting)
        if 0 == remainder {
            return dcount / mlen;
        }
        return ((dcount - remainder) / mlen) + 1;
    }

    /// Retrieve the current Arvile month as string (character).
    /// *note*: '+' encodes year day / leap day
    pub fn get_month_char(self) -> char {
        let offset = self.get_month_u32();
        unsafe {
            if offset == 27 {
                return std::char::from_u32_unchecked(0x2b);
            }
            return std::char::from_u32_unchecked(0x40 + offset);
        }
    }

    /// String representation.
    pub fn to_string(&self) -> String {
        let year_long = self.date.clone().year().to_string();
        let year = &year_long[2..4];
        let mon = self.get_month_char();
        let mut d = self.get_dom();
        if mon == '+' && self.is_leap_year() {
            match d {
                1 => d = 2,
                2 => d = 1,
                _ => {}
            }
        }
        format!("{}{}{:02}", year, mon, d)
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

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}", self.to_string())
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
            Utc.ymd(2020, 1, 14),  // upper corner case
            Utc.ymd(2019, 12, 31), // year day
            Utc.ymd(2020, 12, 31), // year day in leap year
            Utc.ymd(2020, 12, 30), // leap day
        ];
        let mut arvile_dates = Vec::<Date>::new();
        for i in &utc_dates {
            arvile_dates.push(crate::Date::from(i));
        }
        return (utc_dates, arvile_dates);
    }

    #[test]
    fn is_leap_year() {
        let mut dt1 = Utc.ymd(1999, 1, 1);
        let mut av1 = crate::Date::from(dt1);
        assert_eq!(av1.is_leap_year(), false);

        dt1 = Utc.ymd(2000, 2, 2);
        av1 = crate::Date::from(dt1);
        assert_eq!(av1.is_leap_year(), true);

        dt1 = Utc.ymd(2020, 2, 2);
        av1 = crate::Date::from(dt1);
        assert_eq!(av1.is_leap_year(), true);

        dt1 = Utc.ymd(2100, 1, 1);
        av1 = crate::Date::from(dt1);
        assert_eq!(av1.is_leap_year(), false);
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
        let dom = vec![1, 7, 12, 7, 1, 1, 14, 1, 2, 1];
        for i in 0..adates.len() {
            assert_eq!(adates[i].get_dom(), dom[i]);
        }
    }

    #[test]
    fn month() {
        let (_, adates) = get_test_data();
        let m_u32 = vec![1, 4, 2, 5, 3, 27, 1, 27, 27, 27];
        let m_char = vec!['A', 'D', 'B', 'E', 'C', '+', 'A', '+', '+', '+'];
        for i in 0..adates.len() {
            assert_eq!(adates[i].get_month_u32(), m_u32[i]);
            assert_eq!(adates[i].get_month_char(), m_char[i]);
        }
    }

    #[test]
    fn to_string() {
        let (_, adates) = get_test_data();
        let a_str = vec![
            "02A01", "01D07", "13B12", "02E07", "24C01", "03+01", "20A14", "19+01", "20+01",
            "20+02",
        ];
        for i in 0..adates.len() {
            assert_eq!(adates[i].to_string(), a_str[i]);
        }
    }
}
