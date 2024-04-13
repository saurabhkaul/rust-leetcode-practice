//https://leetcode.com/problems/reverse-integer/
pub fn run(x: i32) -> i32 {
    use std::fmt::Error;

    let mut num = x;
    let mut should_flip = false;
    if num < 0 {
        num = num.abs();
        should_flip = true;
    }
    let mut num_length: i32 = (num as u32).to_string().len() as i32;
    // println!("num len is {}", num_length);
    let mut reverse: i32 = 0;
    let mut attempt_reverse = || -> Result<i32, Error> {
        while num_length > 0 {
            let last_digit = num % 10;
            // println!("last digit is {}", last_digit);
            num = num / 10;
            num_length -= 1;
            let subtract = (num_length as u32)
                .checked_div(1_u32)
                .ok_or_else(|| Error)?;
            let power = 10_i32.checked_pow(subtract).ok_or_else(|| Error)?;
            let multiply = last_digit.checked_mul(power).ok_or_else(|| Error)?;
            reverse = reverse.checked_add(multiply).ok_or_else(|| Error)?;

            // println!("reverse is {}", reverse);
        }
        if should_flip {
            reverse = -reverse
        }
        Ok(reverse)
    };
    match attempt_reverse() {
        Ok(reverse) => reverse,
        Err(_e) => 0,
    }
}
