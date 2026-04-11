#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Not valid: The length should be 9, [X/Y/Z] + [7 numbers] + [Check digit]. Got {0} characters.")]
    BadLength(usize),
    #[error("Not valid: The first character should be a 'X', 'Y' or 'Z'. Got {0}")]
    NieBadPrefix(char),
    #[error("Not valid: Char '{0}' at position {1} is not a number")]
    ExpectedNumeric(char, usize),
    #[error("Not valid: The check digit should be '{0}' instead of '{1}'")]
    InvalidCheckNumber(char, char),
}

const NIF_LETTERS: [char; 23] = [
    'T', 'R', 'W', 'A', 'G', 'M', 'Y', 'F', 'P', 'D', 'X', 'B', 'N', 'J', 'Z', 'S', 'Q', 'V',
    'H', 'L', 'C', 'K', 'E',
];

#[inline]
pub fn check_nif_digit(val: u32) -> char {
    // Calculates the check digit based on the remainder.
    let remainder = val % 23;
    NIF_LETTERS[remainder as usize]
}

// Deprecated since 0.1.4: use `check_nif_digit` instead.
// Kept for backwards compatibility with versions <= 0.1.3.
#[deprecated(since = "0.1.4", note = "use `check_nif_digit` instead")]
pub use check_nif_digit as check_digit_nif;

pub fn is_valid_nie(v: &str) -> Result<(), Error> {
    // Validate a NIE - Número de Identidad de Extranjero (NIE)
    // https://es.wikipedia.org/wiki/N%C3%BAmero_de_identidad_de_extranjero
    // Only considers validity under INT/2058/2008 regulation.
    if v.len() != 9 {
        return Err(Error::BadLength(v.len()));
    }

    let first_letter = v.chars().next().unwrap();

    let value = match first_letter.to_ascii_lowercase() {
        'x' => 0,
        'y' => 1,
        'z' => 2,
        _ => return Err(Error::NieBadPrefix(first_letter)),
    };

    let numbers = &v[1..v.len()];
    is_valid_nif(&format!("{}{}", value, numbers))
}

pub fn is_valid_nif(v: &str) -> Result<(), Error> {
    // Validate a NIF - Número de Identificación Fiscal (NIF)
    // https://es.wikipedia.org/wiki/N%C3%BAmero_de_identificaci%C3%B3n_fiscal
    if v.len() != 9 {
        return Err(Error::BadLength(v.len()));
    }

    let letter = v.chars().last().unwrap().to_ascii_uppercase();
    let numbers = &v[..v.len() - 1];

    for (index, c) in numbers.chars().enumerate() {
        if !c.is_ascii_digit() {
            return Err(Error::ExpectedNumeric(c, index + 1));
        }
    }

    let n = numbers.parse::<u32>().unwrap();
    let check_digit = check_nif_digit(n);
    if check_digit == letter {
        return Ok(());
    }

    Err(Error::InvalidCheckNumber(check_digit, letter))
}

#[cfg(test)]
mod tests {
    use crate::check_nif_digit;
    use crate::is_valid_nie;
    use crate::is_valid_nif;

    #[test]
    fn test_valid_check_nif_digit() {
        assert_eq!(check_nif_digit(54878787), 'M');
        assert_eq!(check_nif_digit(98457021), 'R');
        assert_eq!(check_nif_digit(47894211), 'T');
        assert_eq!(check_nif_digit(00000000), 'T');
    }

    #[test]
    fn test_invalid_check_nif_digit() {
        assert_ne!(check_nif_digit(54878787), 'X');
        assert_ne!(check_nif_digit(98457021), 'T');
        assert_ne!(check_nif_digit(47894211), 'V');
        assert_ne!(check_nif_digit(00000000), 'D');
    }

    #[test]
    fn test_valid_nif() {
        is_valid_nif("24591177Z").unwrap();
        is_valid_nif("21178533h").unwrap();
        is_valid_nif("84731432F").unwrap();
        is_valid_nif("65553805X").unwrap();
        is_valid_nif("97294190g").unwrap();
    }

    #[test]
    fn test_invalid_nif() {
        is_valid_nif("").unwrap_err();
        is_valid_nif(" 123 ").unwrap_err();
        is_valid_nif(" 123 ").unwrap_err();
        is_valid_nif("5393710G ").unwrap_err();
        is_valid_nif("53493710G").unwrap_err();
        is_valid_nif("6´420582W").unwrap_err();
        is_valid_nif("X6725600C").unwrap_err();
        is_valid_nif("01675401Z").unwrap_err();
        is_valid_nif("946967460D").unwrap_err();

        // Spaces
        is_valid_nif("         ").unwrap_err();  // 9 spaces
        is_valid_nif(" 5393710G").unwrap_err();
        is_valid_nif("539 3710G").unwrap_err();

        // Too short
        is_valid_nif("1").unwrap_err();
        is_valid_nif("12345678").unwrap_err();
        is_valid_nif("1234567").unwrap_err();

        // Too long
        is_valid_nif("123456789T").unwrap_err();
        is_valid_nif("1234567890T").unwrap_err();

        // Wrong check digit
        is_valid_nif("24591177A").unwrap_err(); // Correct is Z
        is_valid_nif("21178533X").unwrap_err(); // Correct is H
        is_valid_nif("84731432Z").unwrap_err(); // Correct is F

        // Special characters
        is_valid_nif("1234.567T").unwrap_err();
        is_valid_nif("123-5678T").unwrap_err();
        is_valid_nif("1234567\nT").unwrap_err();
        is_valid_nif("1234567\tT").unwrap_err();

        // All letters
        is_valid_nif("ABCDEFGHT").unwrap_err();

        // Unicode
        is_valid_nif("1234567ñT").unwrap_err();
        is_valid_nif("ñ234567T").unwrap_err();
    }

    #[test]
    fn test_valid_nie() {
        is_valid_nie("Y5937943R").unwrap();
        is_valid_nie("Z4132550F").unwrap();
        is_valid_nie("X9675401Z").unwrap();
        is_valid_nie("x9675401Z").unwrap();
    }

    #[test]
    fn test_invalid_nie() {
        is_valid_nie("Y59-7943R").unwrap_err();
        is_valid_nie("096754014Z").unwrap_err();
        is_valid_nie("Z6843868E").unwrap_err();
    }
}
