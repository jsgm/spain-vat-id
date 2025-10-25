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
pub fn check_digit_nif(val: u32) -> char {
    // Calculates the check digit based on the remainder.
    let remainder = val % 23;
    NIF_LETTERS[remainder as usize]
}

pub fn is_valid_nie(v: &str) -> Result<(), Error> {
    // Validate a NIE - Número de Identidad de Extranjero (NIE)
    // https://es.wikipedia.org/wiki/N%C3%BAmero_de_identidad_de_extranjero
    // Only considers validity under INT/2058/2008 regulation.

    if v.len() != 9 {
        Err(Error::BadLength(v.len()))?
    }

    let first_letter = v.chars().next().unwrap().to_ascii_lowercase();

    let value = match first_letter {
        'x' => 0,
        'y' => 1,
        'z' => 2,
        other => return Err(Error::NieBadPrefix(other)),
    };

    let numbers = &v[1..v.len()];
    is_valid_nif(&format!("{}{}", value, numbers))
}

pub fn is_valid_nif(v: &str) -> Result<(), Error> {
    // Validate a NIF - Número de Identificación Fiscal (NIF)
    // https://es.wikipedia.org/wiki/N%C3%BAmero_de_identificaci%C3%B3n_fiscal
    if v.len() != 9 {
        Err(Error::BadLength(v.len()))?
    }

    let letter = v.chars().last().unwrap().to_ascii_uppercase();
    let numbers = &v[..v.len() - 1];

    for (index, c) in numbers.chars().enumerate() {
        if !c.is_ascii_digit() {
            Err(Error::ExpectedNumeric(c, index + 1))?
        }
    }

    let n = numbers.parse::<u32>().unwrap();
    let check_digit = check_digit_nif(n);
    if check_digit == letter {
        return Ok(());
    }
    Err(Error::InvalidCheckNumber(check_digit, letter))
}

#[cfg(test)]
mod tests {
    use crate::check_digit_nif;
    use crate::is_valid_nie;
    use crate::is_valid_nif;

    #[test]
    fn test_valid_check_digit_nif() {
        assert_eq!(check_digit_nif(54878787), 'M');
        assert_eq!(check_digit_nif(98457021), 'R');
        assert_eq!(check_digit_nif(47894211), 'T');
        assert_eq!(check_digit_nif(00000000), 'T');
    }

    #[test]
    fn test_invalid_check_digit_nif() {
        assert_ne!(check_digit_nif(54878787), 'X');
        assert_ne!(check_digit_nif(98457021), 'T');
        assert_ne!(check_digit_nif(47894211), 'V');
        assert_ne!(check_digit_nif(00000000), 'D');
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
        is_valid_nif("5393710G ").unwrap_err();
        is_valid_nif("53493710G").unwrap_err();
        is_valid_nif("6´420582W").unwrap_err();
        is_valid_nif("X6725600C").unwrap_err();
        is_valid_nif("01675401Z").unwrap_err();
        is_valid_nif("946967460D").unwrap_err();
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
