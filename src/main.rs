use std;

mod parsing;

fn main() {
    let mut exit_command = false;
    let mut result:std::option::Option<i16> = None;
    println!("Wellcome to Calc New Roman!\nType exit to quit this application.");

    while exit_command == false{
        let user_input = parsing::get_user_input();
        match &user_input as &str {
            "exit" => (exit_command = true),
            _ => (result = parsing::parse_user_input(user_input)
                
            )
        }
        match result{
            Some(x) => println!("Result: {}",x),
            None => println!("ERROR: invalid input!")
        }
    }
    println!("Thank you for using Calc New Roman!");
}
