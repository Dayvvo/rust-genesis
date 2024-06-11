fn concatenate_string(str1: &String, str2: & String) ->String {

    let mut result = String::new();
    result.push_str(str1);
    result.push_str(str2);
    print!("result is {} ", result);

    result
}

fn main() {
    let st1 = String::from("David");
    let st2 = String::from(" and Goliath");
    concatenate_string(&st1, &st2);
}
