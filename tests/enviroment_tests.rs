#[cfg(test)]
mod enviroment_tests {
    use visa_device_handler::find_first;

    ///Tests to find the first USB instrument.
    ///Work with the [`visa::Binary::Default`]
    ///
    ///All these tests are to find out what kind of binary works the best for you, these test will probebly be removed.
    #[test]
    fn it_works_first_default() {
        let result = find_first::test_ni_default_find_first();
        assert_eq!(result, 0);
    }
    ///Tests to find the first USB instrument.
    ///Work with the [`visa::Binary::NiVisa`]
    ///
    ///All these tests are to find out what kind of binary works the best for you, these test will probebly be removed.
    #[test]
    fn it_works_first_visa() {
        let result = find_first::test_ni_visa_find_first();
        assert_eq!(result, 0);
    }
    ///Tests to find the first USB instrument.
    ///Work with the [`visa::Binary::Keysight`]
    ///
    ///All these tests are to find out what kind of binary works the best for you, these test will probebly be removed.
    #[test]
    fn it_works_first_ks() {
        let result = find_first::test_ni_ks_find_first();
        assert_eq!(result, 0);
    }
    ///Tests to find the first USB instrument.
    ///Work with the [`visa::Binary::Custom`]
    ///
    ///All these tests are to find out what kind of binary works the best for you, these test will probebly be removed.
    #[test]
    fn it_works_first_so() {
        let result = find_first::test_ni_so_find_first();
        assert_eq!(result, 0);
    }
}