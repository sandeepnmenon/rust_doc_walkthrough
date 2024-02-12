mod median_mode;
#[cfg(test)]
mod median_mode_tests {
    use super::median_mode::median_mode;

    #[test]
    fn test_even_len() {
        let arr = vec![3, 1, 2, 1];
        let (median, mode) = median_mode(&arr);

        assert_eq!(median, 1.5);
        assert_eq!(mode, 1);
    }

    #[test]
    fn test_odd_len() {
        let arr = vec![3, 4, 2, 1, 3];
        let (median, mode) = median_mode(&arr);

        assert_eq!(median, 3.0);
        assert_eq!(mode, 3);
    }
}

