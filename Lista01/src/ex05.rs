pub fn is_palindrome(s: &String) -> bool {
    let size: usize = s.len();
    let mut iter = s.chars();
    let mut iter2 = s.chars().rev();
    for _ in 0..size / 2 {
        if iter.next() != iter2.next() {return false}
    }
    true
}
