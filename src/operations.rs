

///Perform mathematical operations with user's input and return the result
pub fn calculate(user_input: &String) -> Option<i16>
{
    Some(0)
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
