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

