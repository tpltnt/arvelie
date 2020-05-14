//! This package enables you to work with the Arvile calendar.
extern crate chrono;
use chrono::Utc;

/// The data structure to represent a date.
#[derive(Debug)]
pub struct Date {
    date: chrono::Date<Utc>,
}

impl Date {
    /// Return date as chronos::Date
    pub fn into_chrono(self) -> chrono::Date<Utc> {
        self.date.clone()
    }
}

// trait implementation
impl From<chrono::Date<Utc>> for Date {
    fn from(item: chrono::Date<Utc>) -> Self {
        Date { date: item }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn from_to_chrono() {
        let dt1 = Utc.ymd(1984, 2, 2);
        let av1 = crate::Date::from(dt1);
        assert_eq!(av1.into_chrono(), dt1);
    }
}
