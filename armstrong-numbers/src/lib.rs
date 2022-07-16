pub fn is_armstrong_number(num: u32) -> bool {  
    let digits: u32 = num.to_string().len().try_into().unwrap();
    num
    .to_string()
    .chars()
    .map(|c| c.to_digit(10).unwrap())    
    .fold(0, |acc, n| acc + n.pow(digits))
    == num
}
