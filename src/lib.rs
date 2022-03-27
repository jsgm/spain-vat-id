#[allow(dead_code)]
fn check_digit_nif(val: u32) -> char {
    // Calculates the check digit based on the remainder.
    let letters: [char; 23] = ['T','R','W','A','G','M','Y','F','P','D','X','B','N','J','Z','S','Q','V','H','L','C','K','E'];
    let remainder = val % 23;
    return letters[remainder as usize];
}

#[allow(dead_code)]
fn is_valid_nie(v: &str) -> (bool, String){
    // Validate a NIE - Número de Identidad de Extranjero (NIE)
    // https://es.wikipedia.org/wiki/N%C3%BAmero_de_identidad_de_extranjero
    //
    // Note that this function only considers a NIE valid under the INT/2058/2008 regulation.
    if v.len() != 9{
        return (false, String::from("Not valid: The length should be 9, [X/Y/Z] + [7 numbers] + [Check digit]"));
    }

    let letter = &v[0..1].to_lowercase();
    let mut value: i8 = -1;

    if letter == "x"{
        value = 0;
    }
    if letter == "y"{
        value = 1;
    }
    if letter == "z"{
        value = 2;
    }
    drop(letter);

    if value == -1{
        return (false, String::from("Not valid: The first character should be a 'X', 'Y' or 'Z'."));
    }

    let numbers = &v[1..v.len()];
    let (valid, explain) = is_valid_nif(&String::from(format!("{}{}", value, numbers)));
    return (valid, explain)
}

#[allow(dead_code)]
fn is_valid_nif(v: &str) -> (bool, String){
    // Validate a NIF - Número de Identificación Fiscal (NIF)
    // https://es.wikipedia.org/wiki/N%C3%BAmero_de_identificaci%C3%B3n_fiscal
    if v.len() != 9{
        return (false, String::from("Not valid: The length should be 9, [8 numbers] + [Check digit]"));
    }
    let letter = v.chars().last().unwrap();
    let numbers = &v[..v.len()-1];

    let mut index = 0;
    for c in numbers.chars(){
        index = index+1;
        if !c.is_digit(10){
            return (false, String::from(format!("Not valid: Char '{}' at position {} is not a number", c, index)))
        }
    }
    drop(index);

    let n = &numbers.parse::<u32>().unwrap();
    let check_digit = check_digit_nif(*n);
    if check_digit == letter{
        return (true, String::from("Valid"));
    }
    return (false, String::from(format!("Not valid: The check digit should be '{}' instead of '{}'", check_digit, letter)));
}

#[cfg(test)]
mod tests {
    use crate::is_valid_nie;
    use crate::is_valid_nif;
    use crate::check_digit_nif;

    #[test]
    fn test_valid_check_digit_nif(){
        assert_eq!(check_digit_nif(54878787), "M".chars().next().unwrap());
        assert_eq!(check_digit_nif(98457021), "R".chars().next().unwrap());
        assert_eq!(check_digit_nif(47894211), "T".chars().next().unwrap());
        assert_eq!(check_digit_nif(00000000), "T".chars().next().unwrap());
    }

    #[test]
    fn test_invalid_check_digit_nif(){
        assert_ne!(check_digit_nif(54878787), "X".chars().next().unwrap());
        assert_ne!(check_digit_nif(98457021), "T".chars().next().unwrap());
        assert_ne!(check_digit_nif(47894211), "V".chars().next().unwrap());
        assert_ne!(check_digit_nif(00000000), "D".chars().next().unwrap());
    }

    #[test]
    fn test_invalid_nif(){
        assert_eq!(is_valid_nif("53493710G").0, false);
        assert_eq!(is_valid_nif("6´420582W").0, false);
        assert_eq!(is_valid_nif("X6725600C").0, false);
        assert_eq!(is_valid_nif("946967460D").0, false);
        assert_eq!(is_valid_nif("01675401Z").0, false);
    }

    #[test]
    fn test_valid_nie(){
        assert_eq!(is_valid_nie("Y5937943R").0, true);
        assert_eq!(is_valid_nie("Z4132550F").0, true);
        assert_eq!(is_valid_nie("X9675401Z").0, true);
        assert_eq!(is_valid_nie("x9675401Z").0, true);
    }

    #[test]
    fn test_invalid_nie(){
        assert_eq!(is_valid_nie("Y59-7943R").0, false);
        assert_eq!(is_valid_nie("7943R").0, false);
        assert_eq!(is_valid_nie("096754014Z").0, false);
        assert_eq!(is_valid_nie("Z6843868E").0, false);
    }
}