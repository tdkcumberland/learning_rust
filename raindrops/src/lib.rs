pub fn raindrops(n: u32) -> String {
    let mut output = "".to_string();
    if n % 3 == 0 {
        output.push_str("Pling");
    }
    if n % 5 == 0 {
        output.push_str("Plang")
    }
    if n % 7 == 0{
        output.push_str("Plong")
    }

    if output.starts_with("P") {
        output
    } else {
        n.to_string()
    }
    
}
