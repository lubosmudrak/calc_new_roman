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
pub fn parse_user_input(){}