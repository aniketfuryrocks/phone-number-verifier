use lazy_static::lazy_static;
use regex::Regex;

/// check's phone_number against (regex)[https://stackoverflow.com/a/56450924/8959586]
///
/// ```regex
/// ^(\+\d{1,2}\s?)?1?\-?\.?\s?\(?\d{3}\)?[\s.-]?\d{3}[\s.-]?\d{4}$
/// ```
///
pub fn verify_phone_number_with_country_code(ph: &str) -> bool {
    // prevent re-compilation of regex
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"^(\+\d{1,2}\s?)?1?\-?\.?\s?\(?\d{3}\)?[\s.-]?\d{3}[\s.-]?\d{4}$").unwrap();
    }

    RE.is_match(ph)
}

/// check's phone_number against (regex)[https://stackoverflow.com/a/56450924/8959586], but without the
/// country code
///
/// ```regex
/// ^1?\-?\.?\s?\(?\d{3}\)?[\s.-]?\d{3}[\s.-]?\d{4}$
/// ```
///
pub fn verify_phone_number_without_country_code(ph: &str) -> bool {
    // prevent re-compilation of regex
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"^1?\-?\.?\s?\(?\d{3}\)?[\s.-]?\d{3}[\s.-]?\d{4}$").unwrap();
    }

    RE.is_match(ph)
}

#[cfg(test)]
mod tests {
    #[test]
    fn phone_number() {
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("1-718-444-1122")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("718-444-1122")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("(718)-444-1122")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("17184441122")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("7184441122")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("718.444.1122")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("1718.444.1122")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("1-123-456-7890")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("1 123-456-7890")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("1 (123) 456-7890")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("1 123 456 7890")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("1.123.456.7890")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("+91 (123) 456-7890")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("18005551234")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("1 800 555 1234")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("+1 800 555-1234")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("+86 800 555 1234")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("1-800-555-1234")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("1 (800) 555-1234")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("(800)555-1234")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("(800) 555-1234")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("(800)5551234")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("800-555-1234")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("800.555.1234")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("18001234567")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("1 800 123 4567")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("1-800-123-4567")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("+18001234567")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("+1 800 123 4567")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("+1 (800) 123 4567")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("1(800)1234567")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("+1800 1234567")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("1.8001234567")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("1.800.123.4567")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("+1 (800) 123-4567")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("18001234567")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("1 800 123 4567")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("+1 800 123-4567")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("+86 800 123 4567")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("1-800-123-4567")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("1 (800) 123-4567")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("(800)123-4567")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("(800) 123-4567")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("(800)1234567")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("800-123-4567")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("800.123.4567")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("1231231231")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("123-1231231")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("123123-1231")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("123-123 1231")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("123 123-1231")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("123-123-1231")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("(123)123-1231")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("(123)123 1231")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("(123) 123-1231")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("(123) 123 1231")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("+99 1234567890")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("+991234567890")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("(555) 444-6789")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("555-444-6789")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("555.444.6789")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("555 444 6789")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("18005551234")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("1 800 555 1234")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("+1 800 555-1234")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("+86 800 555 1234")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("1-800-555-1234")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("1.800.555.1234")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("+1.800.555.1234")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("1 (800) 555-1234")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("(800)555-1234")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("(800) 555-1234")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("(800)5551234")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("800-555-1234")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("800.555.1234")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("(003) 555-1212")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("(103) 555-1212")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("(911) 555-1212")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("18005551234")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("1 800 555 1234")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("+86 800-555-1234")
        );
        assert_eq!(
            true,
            super::verify_phone_number_with_country_code("1 (800) 555-1234")
        );
    }

    #[test]
    fn verify_phone_number_without_country_code() {
        assert_eq!(
            true,
            super::verify_phone_number_without_country_code("1-718-444-1122")
        );
        assert_eq!(
            true,
            super::verify_phone_number_without_country_code("718-444-1122")
        );
        assert_eq!(
            true,
            super::verify_phone_number_without_country_code("(718)-444-1122")
        );
        assert_eq!(
            true,
            super::verify_phone_number_without_country_code("17184441122")
        );
        assert_eq!(
            true,
            super::verify_phone_number_without_country_code("7184441122")
        );
        assert_eq!(
            true,
            super::verify_phone_number_without_country_code("718.444.1122")
        );
        assert_eq!(
            true,
            super::verify_phone_number_without_country_code("1718.444.1122")
        );
        assert_eq!(
            true,
            super::verify_phone_number_without_country_code("1-123-456-7890")
        );
        assert_eq!(
            true,
            super::verify_phone_number_without_country_code("1 123-456-7890")
        );
        assert_eq!(
            true,
            super::verify_phone_number_without_country_code("1 (123) 456-7890")
        );
        assert_eq!(
            true,
            super::verify_phone_number_without_country_code("1 123 456 7890")
        );
        assert_eq!(
            true,
            super::verify_phone_number_without_country_code("1.123.456.7890")
        );
        assert_eq!(
            false,
            super::verify_phone_number_without_country_code("+91 (123) 456-7890")
        );
        assert_eq!(
            true,
            super::verify_phone_number_without_country_code("18005551234")
        );
        assert_eq!(
            true,
            super::verify_phone_number_without_country_code("1 800 555 1234")
        );
        assert_eq!(
            false,
            super::verify_phone_number_without_country_code("+1 800 555-1234")
        );
        assert_eq!(
            false,
            super::verify_phone_number_without_country_code("+86 800 555 1234")
        );
        assert_eq!(
            true,
            super::verify_phone_number_without_country_code("1-800-555-1234")
        );
        assert_eq!(
            true,
            super::verify_phone_number_without_country_code("1 (800) 555-1234")
        );
        assert_eq!(
            true,
            super::verify_phone_number_without_country_code("(800)555-1234")
        );
        assert_eq!(
            true,
            super::verify_phone_number_without_country_code("(800) 555-1234")
        );
        assert_eq!(
            true,
            super::verify_phone_number_without_country_code("(800)5551234")
        );
        assert_eq!(
            true,
            super::verify_phone_number_without_country_code("800-555-1234")
        );
        assert_eq!(
            true,
            super::verify_phone_number_without_country_code("800.555.1234")
        );
        assert_eq!(
            true,
            super::verify_phone_number_without_country_code("18001234567")
        );
        assert_eq!(
            true,
            super::verify_phone_number_without_country_code("1 800 123 4567")
        );
        assert_eq!(
            true,
            super::verify_phone_number_without_country_code("1-800-123-4567")
        );
        assert_eq!(
            false,
            super::verify_phone_number_without_country_code("+18001234567")
        );
        assert_eq!(
            false,
            super::verify_phone_number_without_country_code("+1 800 123 4567")
        );
        assert_eq!(
            false,
            super::verify_phone_number_without_country_code("+1 (800) 123 4567")
        );
        assert_eq!(
            true,
            super::verify_phone_number_without_country_code("1(800)1234567")
        );
        assert_eq!(
            false,
            super::verify_phone_number_without_country_code("+1800 1234567")
        );
        assert_eq!(
            true,
            super::verify_phone_number_without_country_code("1.8001234567")
        );
        assert_eq!(
            true,
            super::verify_phone_number_without_country_code("1.800.123.4567")
        );
        assert_eq!(
            false,
            super::verify_phone_number_without_country_code("+1 (800) 123-4567")
        );
        assert_eq!(
            true,
            super::verify_phone_number_without_country_code("18001234567")
        );
        assert_eq!(
            true,
            super::verify_phone_number_without_country_code("1 800 123 4567")
        );
        assert_eq!(
            false,
            super::verify_phone_number_without_country_code("+1 800 123-4567")
        );
        assert_eq!(
            false,
            super::verify_phone_number_without_country_code("+86 800 123 4567")
        );
        assert_eq!(
            true,
            super::verify_phone_number_without_country_code("1-800-123-4567")
        );
        assert_eq!(
            true,
            super::verify_phone_number_without_country_code("1 (800) 123-4567")
        );
        assert_eq!(
            true,
            super::verify_phone_number_without_country_code("(800)123-4567")
        );
        assert_eq!(
            true,
            super::verify_phone_number_without_country_code("(800) 123-4567")
        );
        assert_eq!(
            true,
            super::verify_phone_number_without_country_code("(800)1234567")
        );
        assert_eq!(
            true,
            super::verify_phone_number_without_country_code("800-123-4567")
        );
        assert_eq!(
            true,
            super::verify_phone_number_without_country_code("800.123.4567")
        );
        assert_eq!(
            true,
            super::verify_phone_number_without_country_code("1231231231")
        );
        assert_eq!(
            true,
            super::verify_phone_number_without_country_code("123-1231231")
        );
        assert_eq!(
            true,
            super::verify_phone_number_without_country_code("123123-1231")
        );
        assert_eq!(
            true,
            super::verify_phone_number_without_country_code("123-123 1231")
        );
        assert_eq!(
            true,
            super::verify_phone_number_without_country_code("123 123-1231")
        );
        assert_eq!(
            true,
            super::verify_phone_number_without_country_code("123-123-1231")
        );
        assert_eq!(
            true,
            super::verify_phone_number_without_country_code("(123)123-1231")
        );
        assert_eq!(
            true,
            super::verify_phone_number_without_country_code("(123)123 1231")
        );
        assert_eq!(
            true,
            super::verify_phone_number_without_country_code("(123) 123-1231")
        );
        assert_eq!(
            true,
            super::verify_phone_number_without_country_code("(123) 123 1231")
        );
        assert_eq!(
            false,
            super::verify_phone_number_without_country_code("+99 1234567890")
        );
        assert_eq!(
            false,
            super::verify_phone_number_without_country_code("+991234567890")
        );
        assert_eq!(
            true,
            super::verify_phone_number_without_country_code("(555) 444-6789")
        );
        assert_eq!(
            true,
            super::verify_phone_number_without_country_code("555-444-6789")
        );
        assert_eq!(
            true,
            super::verify_phone_number_without_country_code("555.444.6789")
        );
        assert_eq!(
            true,
            super::verify_phone_number_without_country_code("555 444 6789")
        );
        assert_eq!(
            true,
            super::verify_phone_number_without_country_code("18005551234")
        );
        assert_eq!(
            true,
            super::verify_phone_number_without_country_code("1 800 555 1234")
        );
        assert_eq!(
            false,
            super::verify_phone_number_without_country_code("+1 800 555-1234")
        );
        assert_eq!(
            false,
            super::verify_phone_number_without_country_code("+86 800 555 1234")
        );
        assert_eq!(
            true,
            super::verify_phone_number_without_country_code("1-800-555-1234")
        );
        assert_eq!(
            true,
            super::verify_phone_number_without_country_code("1.800.555.1234")
        );
        assert_eq!(
            false,
            super::verify_phone_number_without_country_code("+1.800.555.1234")
        );
        assert_eq!(
            true,
            super::verify_phone_number_without_country_code("1 (800) 555-1234")
        );
        assert_eq!(
            true,
            super::verify_phone_number_without_country_code("(800)555-1234")
        );
        assert_eq!(
            true,
            super::verify_phone_number_without_country_code("(800) 555-1234")
        );
        assert_eq!(
            true,
            super::verify_phone_number_without_country_code("(800)5551234")
        );
        assert_eq!(
            true,
            super::verify_phone_number_without_country_code("800-555-1234")
        );
        assert_eq!(
            true,
            super::verify_phone_number_without_country_code("800.555.1234")
        );
        assert_eq!(
            true,
            super::verify_phone_number_without_country_code("(003) 555-1212")
        );
        assert_eq!(
            true,
            super::verify_phone_number_without_country_code("(103) 555-1212")
        );
        assert_eq!(
            true,
            super::verify_phone_number_without_country_code("(911) 555-1212")
        );
        assert_eq!(
            true,
            super::verify_phone_number_without_country_code("18005551234")
        );
        assert_eq!(
            true,
            super::verify_phone_number_without_country_code("1 800 555 1234")
        );
        assert_eq!(
            false,
            super::verify_phone_number_without_country_code("+86 800-555-1234")
        );
        assert_eq!(
            true,
            super::verify_phone_number_without_country_code("1 (800) 555-1234")
        );
    }
}
