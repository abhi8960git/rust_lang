

fn main(){
    //     let mut name:String = String::from("Jhon");
    //     let name_t:&mut String = &name;
         
    //      name_t = String::from("Abhi");
    //      println!("{}", name_t);
         
    // // error[E0308]: mismatched type bez name is the reference type not a String type we are assigning a string type to reference type
    
    let mut name:String = String::from("Abhi");
    let name_t: &mut String = &mut name;
    println!("{:p}", name_t);
    // 0x7ffe2030fe78 refernce address stored in the stack not the value 
    
    *name_t = String::from("Jhon");
    println!("{}", name_t);
    println!("{}", name);
    // println!("{:p}", name_t);
    // println!("{}", name);
    
    }