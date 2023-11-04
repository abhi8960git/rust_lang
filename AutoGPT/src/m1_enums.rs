use core::num;

#[derive(Debug)]
// tell car color should be able to use debug
enum carColor {
    Red,
    Green,
    Blue,
    Silver,
}

fn handle_car_color_blue() -> carColor {
    let my_color: carColor = carColor::Blue;
    my_color
}

// ---------------------------------------------------------------
#[derive(Debug)]

enum Error<T, E> {
    Ok(T),
    Err(E),
}

fn check_number_greater_than_five(num_check: u8) -> Error<u8, String> {
    if num_check > 5 {
        Error::Ok(num_check)
    } else {
        Error::Err("Number is not Greater than 5".to_string())
    }
}

fn check_number_greater_than_five_builtIn(num_check: u8) -> Result<u8, String> {
    if num_check > 5 {
        Ok(num_check)
    } else {
        Err("Number is not Greater than 5".to_string())
    }
}

// default enums in rust some and none

#[derive(Debug)]
enum result<T> {
    None,
    Some(T),
}

fn check_remainder_zero(num: f32) -> result<f32> {
    let is_remainder: f32 = num % 10.0;

    if is_remainder != 0.0 {
        result::None
    } else {
        result::Some(is_remainder)
    }
}

fn check_remainder_zero_builtIn(num: f32) -> Option<f32> {
    let is_remainder: f32 = num % 10.0;

    if is_remainder != 0.0 {
        None
    } else {
        Some(is_remainder)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]

    fn test_enums() {
        let car_color: carColor = handle_car_color_blue();
        dbg!(car_color);

        let is_under_five_res: Error<u8, String> = check_number_greater_than_five(3);
        dbg!(is_under_five_res);

        let is_under_five_res: Error<u8, String> = check_number_greater_than_five(6);
        dbg!(is_under_five_res);

        let is_under_five_res:Result<u8, String> = check_number_greater_than_five_builtIn(6);
        dbg!(is_under_five_res);

        let remainder: result<f32> = check_remainder_zero(20.0);
        dbg!(remainder);


        let remainder: Option<f32> = check_remainder_zero_builtIn(20.0);
        dbg!(remainder);

        let remainder: result<f32> = check_remainder_zero(3.0);
        dbg!(remainder);
    }
}
