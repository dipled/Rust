pub fn isPalindrome(s: &String) -> bool {
    let size: usize = s.len();
    let mut iter = s.chars();
    let mut iter2 = s.chars().rev();

    for _ in 0..size / 2 {
        if let Some(c1) = iter.next() {
            if let Some(c2) = iter2.next() {
                if c1 != c2 {
                    return false;
                }
            }
        }
    }
    true
}
