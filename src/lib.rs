//! This package enables you to work with the Arvile calendar.
extern crate chrono;
use chrono::prelude::*;

/// The data structure to represent a date.
#[derive(Debug)]
pub struct Date {
    date: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
