#[derive(Debug)]

enum CarColor{
    Red,
    Green,
    Blue,
    Silver
}






// Genrics 
enum GivenResult<T,E>{
    Ok(T),
    Err(E)
}

fn create_car_color_blue() ->CarColor{
    let my_car_color:CarColor=  CarColor::Blue;
    my_car_color
}


fn check_under_five(num_check : u8) -> GivenResult<u8, String>{
    if num_check < 5 {
        GivenResult::Ok(num_check)
    }else{
        GivenResult::Err("Not under 5!".to_string())
    }
}

fn check_under_five_builtin(num_check : u8) -> Result<u8, String>{
    if num_check < 5 {
        Ok(num_check)
    }else{
       Err("Not under 5!".to_string())
    }
}

#[cfg(test)]

mod test1{
    use super::*;
    #[test]
    fn test_enums(){
        let car_color:CarColor = create_car_color_blue();
        dbg!(car_color);
        let is_under_five_res = check_under_five(4);
        let is_under_five_res = check_under_five_builtin(6);
    }
}