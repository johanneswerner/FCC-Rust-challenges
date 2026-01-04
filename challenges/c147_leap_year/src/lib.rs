// Empty function signature - implement later
pub fn is_leap_year(year: u32) -> bool {

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

