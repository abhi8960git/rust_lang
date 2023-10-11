

fn change_string(text: &mut String){
    text.push('!');
}


fn main(){
    let mut s:String = String::from("does work");
    let t:&mut String = &mut s;
    change_string(t);
    println!("{}", s);
    // println!("{}", t);
    // error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
}


