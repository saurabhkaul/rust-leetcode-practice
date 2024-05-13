//https://leetcode.com/problems/multiply-strings/



use crate::problems::Solution;

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let bytes_1 = num1.as_bytes();
        let bytes_2 = num2.as_bytes();

        let ascii_offset = 48;

        let nums_1: u32 = bytes_1.iter().enumerate().map(|(idx, b)| {
            let digit: u32 = (b - ascii_offset) as u32;
            let number = digit * 10_u32.pow((bytes_1.len() - idx) as u32);
            number
        }).sum();
        let nums_2: u32 = bytes_2.iter().enumerate().map(|(idx, b)| {
            let digit: u32 = (b - ascii_offset) as u32;
            let number = digit * 10_u32.pow((bytes_1.len() - idx) as u32);
            number
        }).sum();

        let mut mult = nums_1 * nums_2;
        let mut vec_of_digits: Vec<u8> = vec![];
        while mult > 0 {
            let digit: u8 = mult as u8 % 10;
            vec_of_digits.push(digit);
            mult = mult / 10;
        }
        let bytes: Vec<u8> = vec_of_digits.iter().rev().map(|digit| digit + 48_u8).collect();
        match std::str::from_utf8(&bytes) {
            Ok(string) => {
                string.to_string()
            }
            Err(e) => { format!("error in str::from_utf8 {}", e).to_string() }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::problems::Solution;

    #[test]
    fn test_multiply() {
        let result = Solution::multiply("2".to_string(),"3".to_string());
        assert_eq!(result,"6")
    }
}