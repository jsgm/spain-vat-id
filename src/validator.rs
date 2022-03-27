fn nif_check_digit(val: u32) -> char {
    // Calculates the check digit based on the remainder.
    let letters: [char; 23] = ['T','R','W','A','G','M','Y','F','P','D','X','B','N','J','Z','S','Q','V','H','L','C','K','E'];
    let remainder = val % 23;
    return letters[remainder as usize];
}

fn is_valid_nie(v: &str) -> (bool, String){
    // Validate a NIE - Número de Identidad de Extranjero (NIE)
    // https://es.wikipedia.org/wiki/N%C3%BAmero_de_identidad_de_extranjero
    //
    // Note that this function only considers a NIE valid under the INT/2058/2008 regulation.
    if v.len() != 9{
        return (false, String::from("Not valid: The length should be 9, [X/Y/Z] + [7 numbers] + [Check digit]"));
    }

    let letter = &v[0..1];
    let mut value: i8 = -1;

    if letter == "X"{
        value = 0;
    }
    if letter == "Y"{
        value = 1;
    }
    if letter == "Z"{
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
    let check_digit = nif_check_digit(*n);
    if nif_check_digit(*n) == letter{
        return (true, String::from("Valid"));
    }
    return (false, String::from(format!("Not valid: The check digit should be '{}' instead of '{}'", check_digit, letter)));
}