// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut outcome = true;
    let mut magazine_map: HashMap<&str, u32> = HashMap::new();
    for word in magazine {
        magazine_map.entry(word).and_modify(|value| *value += 1).or_insert(1);
    };

    for word in note {
        if magazine_map.contains_key(word) && magazine_map.get(word) >= Some(&1) {
            magazine_map.entry(word).and_modify(|value| *value -= 1);
        } else {
            outcome = false;
            break;
        }
    }
    outcome
}
