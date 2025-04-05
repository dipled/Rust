pub fn primefy (v : &Vec<u64>) -> Vec<u64> {
    let mut new: Vec<u64> = Vec::new();
    for i in v {
        if crate::ex04::isPrimeBruteforce(*i) {
            new.push(*i);
        }
    }
    new
}
