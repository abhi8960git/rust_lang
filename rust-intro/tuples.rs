fn main(){


    let my_information:(&str, i32) = ("salary", 1);

    println!("{} is equal to {}", my_information.0, my_information.1);
    println!("another way is this {:?}", my_information);
    // destructring 
    // let (salary: &str,salary_value: i32 ) = my_information;

    let nested_tuple:(i32,i32,(i32,&str),i32) = (2,3(4,"name"),3);
    println!("this is element {}", nested_tuple.2.1)
}