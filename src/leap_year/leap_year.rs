
fn is_leap_year(year: u32) -> bool {
    match year {
        year if (year % 400) == 0 => true,
        year if (year % 100) == 0 => false,
        year if (year % 4) == 0 => true,
        _ => false
    }
}

#[cfg(test)]
mod tests {
    use crate::leap_year::leap_year::is_leap_year;

    #[test]
    fn if_is_a_number_isnt_enough_to_be_a_leap_year() {
        let result = is_leap_year(1234);
        assert_eq!(result, false);
    }

    #[test]
    fn if_is_divisible_by_400_is_a_leap_year() {
        let result = is_leap_year(2800);
        assert_eq!(result, true);
    }

    #[test]
    fn if_is_divisible_by_100_isnt_a_leap_year() {
        let result = is_leap_year(1800);
        assert_eq!(result, false);
    }

    #[test]
    fn if_is_divisible_by_4_is_a_leap_year() {
        let result = is_leap_year(2020);
        assert_eq!(result, true);
    }

}