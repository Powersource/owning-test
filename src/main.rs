fn main() {
    let mut text = "Testing".to_string();
    println!("1: {}", text);
    text = do_stuff(&text);
    println!("3: {}", text);
}

fn do_stuff(my_string: &String) -> String {
    println!("2: {}", my_string);
    let mut new_string = "".to_string();
    new_string.push_str(my_string);
    new_string.push_str(my_string);
    new_string
}