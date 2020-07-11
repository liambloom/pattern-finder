pub fn all_equal<>(vec: &Vec<impl PartialEq>) -> bool {
    let prev = &vec[0];
    for e in vec {
        if e != prev {
            return false;
        }
    }
    true
}