use sublist::{sublist, Comparison};


#[test]
fn superlist_at_start() {
    assert_eq!(Comparison::Superlist, sublist(&[1, 2, 3, 4, 5], &[1, 2, 3]));
}

#[test]
// #[ignore]
fn superlist_in_middle() {
    assert_eq!(Comparison::Superlist, sublist(&[5, 4, 3, 2, 1], &[4, 3, 2]));
}

#[test]
// #[ignore]
fn superlist_early_in_huge_list() {
    let huge: Vec<u32> = (1..1_000_000).collect();

    assert_eq!(Comparison::Superlist, sublist(&huge, &[3, 4, 5]));
}