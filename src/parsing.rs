pub fn get_user_input() -> String 
{
    let mut user_input = String::new();
    std::io::stdin()
    .read_line(&mut user_input)
    .expect("FATAL ERROR: invalid user input!");
    let user_input_trimmed = user_input.trim().to_string();

    user_input_trimmed
}
pub fn parse_user_input(){}