pub fn all_equal(vec: &Vec<impl PartialEq>) -> bool {
    let prev = &vec[0];
    for e in vec {
        if e != prev {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod all_equal {
    use super::all_equal as vec_equality_checker;

    #[test]
    fn all_equal() {
        assert!(vec_equality_checker(&vec![1, 1, 1]));
    }

    #[test]
    fn not_all_equal() {
        assert!(!vec_equality_checker(&vec![1, 2, 3]));
    }
}