pub fn build_proverb(list: &[&str]) -> String {
    let length = list.len();
    let mut output: Vec<String> = vec![];
    if length > 0 {
        if length > 1 {
            for i in 0..length-1 {
                output.push(format!("For want of a {} the {} was lost.", list.get(i).unwrap(), list.get(i+1).unwrap()));
            }
        }
        let final_sentence = format!("And all for the want of a {}.", list.get(0).unwrap());
        output.push(final_sentence);
    }
    output.join("\n")
}
