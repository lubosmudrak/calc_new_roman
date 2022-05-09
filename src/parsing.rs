/// Get a valid user input and trim all unneeded invisible characters from it
pub fn get_user_input() -> String 
{
    let mut user_input = String::new();
    std::io::stdin()
    .read_line(&mut user_input)
    .expect("FATAL ERROR: invalid user input!");
    let user_input_trimmed = user_input.trim().to_string();

    user_input_trimmed
}

/// Extract roman numbers from string and convert them into arabic
pub fn parse_user_input(user_input: String) -> Option<u16>
{
    //currently supporting only additive notation without enforcing proper rules
    let mut number_counter = 0;
    let input_chars: Vec<char> = user_input.chars().collect();
    for i in 0..input_chars.len() {
        match input_chars[i]{
            'I' | 'i' => (number_counter+=1),
            'V' | 'v' => (number_counter+=5),
            'X' | 'x' => (number_counter+=10),
            'L' | 'l' => (number_counter+=50),
            'C' | 'c' => (number_counter+=100),
            'D' | 'd' => (number_counter+=500),
            'M' | 'm' => (number_counter+=1000),
            _ => (number_counter+=0)
        }
    }

        Some(number_counter) //TODO: return None in case of invalid user input
}

/*
    There are 4 basic principles for writing Roman numerals as listed below:

    The letters I, X, C can be repeated thrice in succession. Additionally, L, V, D cannot be repeated or the number is considered to be invalid.
    If a lower value digit is written to the left of a higher value digit, it is subtracted.
    If a lower value digit is written to the right of a higher value digit, it is added.
    Only I, X, and C can be used as subtractive numerals.
*/

#[test]
fn parse_i(){
    assert_eq!(parse_user_input("I".to_string()), Some(1));
}

#[test]
fn parse_ii(){
    assert_eq!(parse_user_input("iI".to_string()), Some(2));
}

#[test]
fn parse_iiii(){
    assert_eq!(parse_user_input("iiii".to_string()), Option::None);
}

#[test]
fn parse_iv(){
    assert_eq!(parse_user_input("IV".to_string()),Some(4));
}

#[test]
fn parse_v(){
    assert_eq!(parse_user_input("V".to_string()), Some(5));
}

#[test]
fn parse_x(){
    assert_eq!(parse_user_input("X".to_string()), Some(10));
}

#[test]
fn parse_lxiii(){
    assert_eq!(parse_user_input("LXIII".to_string()), Some(63));
}

#[test]
fn parse_lxiv(){
    assert_eq!(parse_user_input("LXIV".to_string()), Some(64));
}

#[test]
fn parse_xxxx(){
    assert_eq!(parse_user_input("XXXX".to_string()), Option::None);
}

#[test]
fn parse_xxxxi(){
    assert_eq!(parse_user_input("XXXXI".to_string()), Option::None);
}

#[test]
fn parse_xxll(){
    assert_eq!(parse_user_input("XXLL".to_string()), Option::None);
}

#[test]
fn parse_llc(){
    assert_eq!(parse_user_input("LLC".to_string()), Option::None);
}

#[test]
fn parse_lc(){
    assert_eq!(parse_user_input("LC".to_string()), Option::None);
}

#[test]
fn parse_ilc(){
    assert_eq!(parse_user_input("ILC".to_string()), Option::None);
}

#[test]
fn parse_mmmx(){
    assert_eq!(parse_user_input("MMMX".to_string()), Some(3010));
}

#[test]
fn parse_mmmcmxcix(){
    assert_eq!(parse_user_input("MMMCMXCIX".to_string()), Some(3999));
}

#[test]
fn parse_lxxxix(){
    assert_eq!(parse_user_input("LXXXIX".to_string()), Some(89));
}

#[test]
fn parse_lxxxiv(){
    assert_eq!(parse_user_input("LXXXIV".to_string()), Some(84));
}