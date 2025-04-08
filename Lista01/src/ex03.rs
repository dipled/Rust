pub fn highest(v: &Vec<i32>) -> Option<i32>{

    if v.is_empty() {
        return None
    }

    let mut h: i32 = i32::MIN;

    for i in v{
        if h < *i {
            h = *i;
        }
    }
    Some(h)
}