mod parsing;
mod operations;

fn main() {
    let mut exit_command = false;
    println!("Wellcome to Calc New Roman!\nType \"help\" to display instruction manual ");

    while exit_command == false{
        let user_input = parsing::get_user_input();
        let result = operations::calculate(&user_input);
        match result{
            Some(x) => println!("Result: {}",x),
            None => {
                match &user_input as &str {
                    "exit" => {
                        exit_command = true;
                        println!("Thank you for using Calc New Roman!");
                    },
                    "help" => {operations::print_documentation();},
                    _ => (println!("ERROR: invalid input!"))
                }
            }
        }
    }
}
