fn concatenate_string(str1: & mut String, str2: & mut String) ->String {

    let mut result =  str1.push_str(str2);

    result
}

fn main() {
    let mut st1 = String::from("David");
    let mut st2 = String::from(" and Goliath");
    concatenate_string(&mut st1, &mut st2);
}
