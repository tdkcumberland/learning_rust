pub fn reverse(input: &str) -> String {
    let input_array = input.chars();
    input_array.rev().collect::<String>()
}
