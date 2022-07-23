pub fn verse(n: u32) -> String {
    if n > 2 {
        format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n", n, n, n-1)
    }
    else if n == 2 {
        format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottle of beer on the wall.\n", n, n, n-1)
    }
    else if n <= 0 {
        "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string()
    }
    else if n == 1 {
        "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".to_string()
    } else {
        "nope".to_string()
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let mut lyric = String::from("");
    for i in (end..=start).rev() {
        lyric.push_str(&verse(i));
        if i != end {
            lyric.push_str("\n");
        }
    }
    lyric
}
