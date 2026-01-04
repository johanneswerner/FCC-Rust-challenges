/// Determines if a given year is a leap year.
///
/// A year is a leap year if:
/// - It is divisible by 4, AND
/// - Either not divisible by 100 OR divisible by 400
///
/// # Examples
/// ```
/// # use c147_leap_year::is_leap_year;
/// assert_eq!(is_leap_year(2024), true);
/// assert_eq!(is_leap_year(2100), false);
/// assert_eq!(is_leap_year(2000), true);
/// ```
pub const fn is_leap_year(year: u32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leap_year_2024() {
        assert!(is_leap_year(2024));
    }

    #[test]
    fn test_non_leap_2023() {
        assert!(!is_leap_year(2023));
    }

    #[test]
    fn test_non_leap_2100() {
        assert!(!is_leap_year(2100));
    }

    #[test]
    fn test_leap_2000() {
        assert!(is_leap_year(2000));
    }

    #[test]
    fn test_non_leap_1999() {
        assert!(!is_leap_year(1999));
    }

    #[test]
    fn test_leap_2040() {
        assert!(is_leap_year(2040));
    }

    #[test]
    fn test_non_leap_2026() {
        assert!(!is_leap_year(2026));
    }
}

