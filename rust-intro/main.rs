// comment and basic readable programs

fn main(){
println!("Hello from rust");
// ! rust macro ln put line in buffer 

// complile time errors
// print!(10);
println!("this value is {}", 10);

println!("my name is {} and my roll is {}", "abhi", "23");

// escape sequences 

// \n next line 



// Variales andd data types
// all the variables in rust by default are immutable 



//  shadowing constants 

// let (first_numeber:i32, seconed_number:f64) = (250 , 480.22);

let x:i32 = 255;
// snake case variables name 

let n1:i32 = 14;
let n2:f64 =  15.3;

let n3:i32 = n1 + n2 as i32;
println!("the value of n3 = {}", n3);
let n4 = n1 as f64 + n2;
println!("the value of n4 is = {}", n4);
 

//  shadowing in rust :

// compound data types
// N-STR immutable N references 
// string types
// fixed length sting 

let fixed_string: &str = "this is mine";

// dynamic string 
let mut dynamic_string :String = String::from("this is good ");
println!("dynamic string {}", dynamic_string);

dynamic_string.push('s');
println!("dynamic string  push {}", dynamic_string);

dynamic_string.pop();
println!("dynamic string pull {}", dynamic_string);
dynamic_string.push_str("this is not good");

println!("dynamic string pull {}", dynamic_string);


println!(
    "basic function on strings, 
    is_empty():{},
    length():{},
    Bytes():{},
    contains_use():{}",
    dynamic_string.is_empty(),
    dynamic_string.len(),
    dynamic_string.capacity(),
    dynamic_string.contains('s')

)


// Compund data types 
// tuples fixed length 





}