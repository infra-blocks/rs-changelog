use chrono::Datelike;

/// A date without a timezone.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NaiveDate(chrono::NaiveDate);

impl NaiveDate {
    /// Creates a new `NaiveDate` from the year, month, and day.
    ///
    /// Returns `None` if the date is invalid.
    pub fn from_ymd(year: i32, month: u32, day: u32) -> Option<Self> {
        chrono::NaiveDate::from_ymd_opt(year, month, day).map(|d| Self(d))
    }

    /// Creates a new `NaiveDate` from a string in the format `YYYY-MM-DD`.
    ///
    /// Returns `None` if the date is invalid or if the string is not in the correct format.
    pub fn from_ymd_str(date: &str) -> Option<Self> {
        chrono::NaiveDate::parse_from_str(date, "%Y-%m-%d")
            .ok()
            .map(|d| Self(d))
    }

    /// Returns the year component of the date.
    pub fn year(&self) -> i32 {
        self.0.year()
    }

    /// Returns the month component of the date.
    pub fn month(&self) -> u32 {
        self.0.month()
    }

    /// Returns the day component of the date.
    pub fn day(&self) -> u32 {
        self.0.day()
    }

    /// Formats the date as a `YYYY-MM-DD` string.
    pub fn to_ymd_string(&self) -> String {
        format!("{}-{:02}-{:02}", self.year(), self.month(), self.day())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod from_ymd {
        use super::*;

        #[test]
        fn works_for_valid_date() {
            let date = NaiveDate::from_ymd(2021, 1, 1).unwrap();
            assert_eq!(date.year(), 2021);
            assert_eq!(date.month(), 1);
            assert_eq!(date.day(), 1);
        }

        #[test]
        fn fails_for_invalid_date() {
            assert!(NaiveDate::from_ymd(2021, 13, 1).is_none());
        }
    }

    mod from_ymd_str {
        use super::*;

        #[test]
        fn works_for_valid_date() {
            let date = NaiveDate::from_ymd_str("2021-01-01").unwrap();
            assert_eq!(date.year(), 2021);
            assert_eq!(date.month(), 1);
            assert_eq!(date.day(), 1);
        }

        #[test]
        fn fails_for_invalid_date() {
            assert!(NaiveDate::from_ymd_str("2021-13-01").is_none());
        }
    }

    mod to_ymd_string {
        use super::*;

        #[test]
        fn works() {
            let date = NaiveDate::from_ymd(2021, 1, 1).unwrap();
            assert_eq!(date.to_ymd_string(), "2021-01-01");
        }
    }
}
