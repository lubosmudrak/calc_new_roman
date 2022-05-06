mod parsing;

fn main() {
    let mut exit_command = false;
    println!("Wellcome to Calc New Roman!\nType exit to quit this application.");

    while exit_command == false{
        let user_input = parsing::get_user_input();
        match &user_input as &str {
            "exit" => (exit_command = true),
            _ => parsing::parse_user_input()
        }
    }
    println!("Thank you for using Calc New Roman!");
}
