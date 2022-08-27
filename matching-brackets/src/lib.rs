pub fn brackets_are_balanced(string: &str) -> bool {
    let mut queue: Vec<char>= vec![];
    let closer = vec!['}','{', '[', ']', '(', ')'];
    if string.is_empty() {
        return true
    }
    for c in string.chars() {
        if closer.contains(&c) {
            if c == '}' {
                if queue.pop() != Some('{') {
                    return false
                }
            } else if c == ')' {
                if queue.pop() != Some('(') {
                    return false
                }
            } else if c == ']' {
                if queue.pop() != Some('[') {
                    return false
                }
            } else {
                queue.push(c)
            }
        }
    }
    queue.is_empty()
    
}
