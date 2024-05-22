use std::ffi::CString;

use visa::{self};

mod find_first;
mod find_scope;
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works_ni() {
        let result = find_scope::test_ni_visa();
        assert_eq!(result, 0);
    }
    #[test]
    fn it_works_ks() {
        let result = find_scope::test_ni_ks();
        assert_eq!(result, 0);
    }
    #[test]
    fn it_works_default() {
        let result = find_scope::test_ni_default();
        assert_eq!(result, 0);
    }
    #[test]
    fn it_works_so() {
        let result = find_scope::test_ni_so();
        assert_eq!(result, 0);
    }
    #[test]
    fn it_works_first_default() {
        let result = find_first::test_ni_default_find_first();
        assert_eq!(result, 0);
    }
    #[test]
    fn it_works_first_visa() {
        let result = find_first::test_ni_visa_find_first();
        assert_eq!(result, 0);
    }
    #[test]
    fn it_works_first_ks() {
        let result = find_first::test_ni_ks_find_first();
        assert_eq!(result, 0);
    }
    #[test]
    fn it_works_first_so() {
        let result = find_first::test_ni_so_find_first();
        assert_eq!(result, 0);
    }
}
