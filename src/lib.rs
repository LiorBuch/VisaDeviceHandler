mod find_first;
mod find_scope;
pub mod visa_module;
pub mod types;
#[cfg(test)]
mod tests {

    use visa_module::SafeDeviceMap;

    use super::*;

    #[test]
    fn test_mapper(){
        let mapper_res = SafeDeviceMap::init(None);
        match mapper_res {
            Ok(mapper) => {
                let stat = mapper.get_all_devices();
                match stat {
                    Ok(_) => {assert_eq!(1,1)},
                    Err(e) => {
                        println!("got error: {}",e);
                        assert_eq!(1,-1);
                    }
                    
                }
            },
            Err(e) => {
                println!("got error of {}",e);
                assert_eq!(1,-1);
            }
        }
    }

    //tests to find the scope using the scope address.
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
    //tests to find the first USB instrument.
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
