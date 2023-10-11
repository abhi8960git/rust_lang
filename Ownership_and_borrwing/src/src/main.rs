
// fn make_string_dangle()->&String{
// let s:String =String::from("dangle");
// let r:String&String = &s;
// }


fn make_string_not_dangle()->String{
    String::from("dangle")
    }
    
    fn main(){
    let x:i32 = 50;
    let y:i32 = x;
    println!("{}",x);
    
    // will not work
    // here exists shallow copy which is opposing rule of ownership in rust 
    // let s:String = String::from("Hello");
    // let t:String = s;
    // println!("{}",s);
    
    
    let s:String = String::from("Hwllo");
    let t:String = s.clone();
    println!("{}", s);
    // Deep copy clone does't recommended because it doubles memory 
    
    
    
    let s:String = String::from("Hello");
    let t:&String = &s;
    // we borrow the value instead of cloning
    println!("{}", s);
    
    let r:String = make_string_not_dangle();
    println!("{}", r);
    
    
    
    
    
    }