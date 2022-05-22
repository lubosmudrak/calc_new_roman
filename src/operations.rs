use super::parsing;

///Perform mathematical operations with user's input and return the result
pub fn calculate(user_input: &String) -> Option<i16>
{
    let input_chars_raw: Vec<char> = user_input.to_uppercase().chars().collect();
    let mut input_chars= Vec::new();
    let mut result_collector = 0;
    let mut roman_numeral_collector = String::new();
    let mut operator = '0';
       
    for i in 0..input_chars_raw.len(){
        match input_chars_raw[i]{
            'I'|'V'|'X'| 'L'|'C'|'D'|'M'|'+'|'-'|'/'|'*'|'÷'|'=' => {input_chars.push(input_chars_raw[i]);},
            _ => {return None}
        }
    }

    if input_chars[input_chars.len()-1] != '='{
        match input_chars[input_chars.len()-1]{
            '+'|'-'|'/'|'*'|'÷' => {println!("Warning: incomplete mathematical operation!\nHint: You are missing a number on which you want to perform a mathematical operation.")}
            'I'|'V'|'X'| 'L'|'C'|'D'|'M' => {println!("Warning: incomplete mathematical operation!\nHint: You are missing an operation to perform on a roman number.")}
            _ => {println!("Warning: incomplete mathematical operation!\nHint: you may be missing \"=\"")}
        }
        return None
    }

    for i in 0..input_chars.len()
    {
        match input_chars[i]{
            '+'|'-'|'/'|'*'|'÷'|'=' => {
                if operator == '0'{
                    match parsing::parse_user_input(&roman_numeral_collector.to_string()){
                        Some(x) => {
                            result_collector = result_collector+x;
                            operator = input_chars[i];
                            roman_numeral_collector.clear();
                        }
                        None => {println!("input error!");}
                    }
                } else {
                    match operator{
                        '+' => {
                            match parsing::parse_user_input(&roman_numeral_collector.to_string()){
                                Some(x) => {result_collector = result_collector+x;}
                                None => {println!("input error!");}
                            }
                        }
                        '-' => {                            
                            match parsing::parse_user_input(&roman_numeral_collector.to_string()){
                                Some(x) => {result_collector = result_collector-x;}
                                None => {println!("input error!");}
                            }
                        }
                        '*' => {                            
                            match parsing::parse_user_input(&roman_numeral_collector.to_string()){
                                Some(x) => {result_collector = result_collector*x;}
                                None => {println!("input error!");}
                                }
                            }
                        '÷' | '/'=> {
                            match parsing::parse_user_input(&roman_numeral_collector.to_string()){
                                Some(x) => {result_collector = result_collector/x;}
                                None => {println!("input error!");}
                                }
                        }
                        '=' => { if i < input_chars.len()-1{
                            println!("Warning: \"=\" must be placed on the last place of your mathematical operation!");
                            return None
                            }
                            Some(result_collector);
                        }
                        _ => {Some(result_collector);}
                    }
                    operator = input_chars[i];
                    roman_numeral_collector.clear();
                }
            }

            _ => {roman_numeral_collector.push(input_chars[i])}
        }
    }

    Some(result_collector)
}

///Print a short user manual onto user's screen
pub fn print_documentation(){
    println!("USER MANUAL:

commands:
help - display this manual
exit - exit application

Application usage:
you can input roman numbers with the mathematical operations you want to perform. State that your mathematical operation is finished by putting  \"=\" in it's last place.
Note: support for correct ordering of mathematical operations is not yet implemented. Operations are executed in the order in which you put them in.
There are 4 basic principles for writing Roman numerals as listed below:

The letters I, X, C can be repeated thrice in succession. Additionally, L, V, D cannot be repeated or the number is considered to be invalid.
If a lower value digit is written to the left of a higher value digit, it is subtracted.
If a lower value digit is written to the right of a higher value digit, it is added.
Only I, X, and C can be used as subtractive numerals.
");
}

#[test]
fn calculate_i_plus_i_equals(){
    assert_eq!(calculate(&"I+I=".to_string()), Some(2));
}

#[test]
fn calculate_i_plus_i_equals_lowercase(){
    assert_eq!(calculate(&"i+i=".to_string()), Some(2));
}

#[test]
fn calculate_i_plus_i(){
    assert_eq!(calculate(&"I+I".to_string()), None);
}

#[test]
fn calculate_v_plus_x_equals(){
    assert_eq!(calculate(&"V+X=".to_string()), Some(15));
}

#[test]
fn calculate_v_minus_x_equals(){
    assert_eq!(calculate(&"V-X=".to_string()), None);
}

#[test]
fn calculate_x_minus_v_equals(){
    assert_eq!(calculate(&"X-V=".to_string()), Some(5));
}

#[test]
fn calculate_x_times_x_equals(){
    assert_eq!(calculate(&"X*X=".to_string()), Some(100));
}

#[test]
fn calculate_m_divided_x_equals(){
    assert_eq!(calculate(&"M/X=".to_string()), Some(100));
}

#[test]
fn calculate_alternate_m_divided_x_equals(){
    assert_eq!(calculate(&"M÷X=".to_string()), Some(100));
}

#[test]
fn calculate_mmm_times_mmm_equals(){
    assert_eq!(calculate(&"MMM*MMM=".to_string()), None);
}

#[test]
fn calculate_i_plus_i_plus_i_plus_i_equals(){
    assert_eq!(calculate(&"I+I+I+I=".to_string()), Some(4));
}

#[test]
fn calculate_x_plus_v_minus_iv_equals(){
    assert_eq!(calculate(&"X+V-IV=".to_string()), Some(11));
}

#[test]
fn calculate_x_plus_v_times_ii_equals(){
    assert_eq!(calculate(&"X+V*II=".to_string()), Some(20));
}

#[test]
fn calculate_x_plus_v_times_ii_plus_v_with_brackets_equals(){
    assert_eq!(calculate(&"X+V*(II+V)=".to_string()), Some(45));
}
