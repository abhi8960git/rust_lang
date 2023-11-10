
#[derive(Debug)]

struct User{
    username:String,
    email:String,
    active:bool,
    sign_in_count:u8
}

impl User{

    fn incrime_signin_acocunt(&mut self){
        self.sign_in_count +=1;
    }

    fn change_email(&mut self, new_email: &str){
        self.email = String::from(new_email);
    }
}





#[cfg(test)]

mod test{
    use super::*;

    #[test]
    fn test_structs(){
     let user_1:User = User{
        username:String::from("someusesrname1"),
        email:String::from("some@email.com"),
        active:true,
        sign_in_count:1
     };

     let mut user_2:User = User{
        username:String::from("someusesrnamde1"),
        email:String::from("some@email.dcom"),
        active:false,
        sign_in_count:0
     };


     user_2.incrime_signin_acocunt();
     user_2.change_email("email.com");

     dbg!(user_2);


    }
}

