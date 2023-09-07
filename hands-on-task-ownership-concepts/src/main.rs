fn main() {
    let string1 = String::from("I'm string1");
    let string2 = String::from(" & I'm string2");
    let concatenated_string = concatenate_strings(&string1, &string2);
    println!("{}", concatenated_string);
}

fn concatenate_strings(s_1: &str, s_2: &str) -> String {
    let mut result = String::new();
    result.push_str(s_1);
    result.push_str(s_2);
    result
}
