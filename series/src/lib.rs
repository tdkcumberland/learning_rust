pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut output: Vec<String> = vec![];
    if digits.len() < len {
        output
    } else {
        let mut i = 0;
        while i <= digits.len() - len {
            output.push(digits[i..i+len].to_string());
            i+=1;
        }
        output
    }
}
