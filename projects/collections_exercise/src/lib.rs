mod median_mode;
mod pig_latin;

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

#[cfg(test)]
mod pig_latin_tests {
    use super::pig_latin::to_pig_latin;

    #[test]
    fn default_tests() {
        let test1 = String::from("first");
        assert_eq!(to_pig_latin(&test1), String::from("irst-fay"));

        let test2 = String::from("apple");
        assert_eq!(to_pig_latin(&test2), String::from("apple-hay"));

        let test2 = String::from("first apple");
        assert_eq!(to_pig_latin(&test2), String::from("irst-fay apple-hay"));
    }
}
