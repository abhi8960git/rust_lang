const OUR_COURSE: &str = "RUST with AUTOGPT";

fn main() {
    let name: &str = "Shaun";
    println!("name is {:?}", name);

    let dynamic_name: String = String::from("Happy life");
    println!("dynamic name is {:?}", dynamic_name);

    let mut chars: Vec<char> = Vec::new();
    // chars.insert(index:0, element:'h');
    chars.push('l');

    dbg!(&chars);

    let removed_char: char = chars.pop().unwrap();
    // In the code snippet let removed_char: char = chars.pop().unwrap();, the unwrap() method is used to extract the value from the Option type returned by the pop() method.
    // The pop() method is used to remove and return the last element from a collection. In this case, it seems that chars is a collection (possibly a Vec<char> or a String) and pop() is being called on it to remove and return the last character.
    // However, the pop() method returns an Option type because the collection might be empty, and there would be no element to return. The Option type represents the possibility of either having a value (Some) or not having a value (None).
    // By using unwrap(), you are asserting that there will always be a value present and you want to extract it. If the Option is Some, unwrap() will return the value inside. But if the Option is None, it will panic and cause the program to crash.
    println!("removed char is {:?}", removed_char);


    //  use of collect in rust a
    let chars_again: Vec<char> = vec!['a', 'e', 'i', 'o', 'u'];
    dbg!(&chars_again);
    let collected: String = chars_again.iter().collect();
    dbg!(collected);
    for &c in &chars_again {
        print!("{}", c);
        if c == 'o' {
            println!(", world");
        }
    }


    // Closures 
    let num:i32 = 5;
    let add_num = |x:i32| x + num;
    let new_num:i32 = add_num(7);
    dbg!(new_num);
}

