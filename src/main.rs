mod parsing;

fn main() {
    let mut exit_command = false;
    println!("Wellcome to Calc New Roman!\nType \"help\" to display instruction manual ");

    while exit_command == false{
        let user_input = parsing::get_user_input();
        let result = parsing::parse_user_input(&user_input);
        match result{
            Some(x) => println!("Result: {}",x),
            None => {
                match &user_input as &str {
                    "exit" => {
                        exit_command = true;
                        println!("Thank you for using Calc New Roman!");
                    },
                    "help" => (println!("TODO: documentation...\n")),
                    _ => (println!("ERROR: invalid input!"))
                }
            }
        }
    }
}
