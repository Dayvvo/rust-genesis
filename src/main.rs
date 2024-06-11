fn concatenate_string(str1: & mut String, str2: & mut String) ->String {

    let mut result = String::new();
    result.push_str(str1);
    result.push_str(str2);
    print!("result is {} ", result);

    result
}

fn main() {
    let mut st1 = String::from("David");
    let mut st2 = String::from(" and Goliath");
    concatenate_string(&mut st1, &mut st2);
}
