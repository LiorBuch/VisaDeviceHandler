mod find_first;
pub mod types;
pub mod visa_module;
mod status_testers;
#[cfg(test)]
mod tests {
    use visa_module::SafeDeviceMap;
    use super::*;

    ///General test to see if the SafeDeviceMap works.
    #[test]
    fn test_mapper() {
        let mapper_res = SafeDeviceMap::init(None);
        match mapper_res {
            Ok(mapper) => {
                let stat = mapper.get_first_device(Some("?*INSTR"),true);
                match stat {
                    Ok(dev) => {
                        assert_eq!(1, 1);
                        let con = mapper.connect_device(dev.address.clone());
                        if con.is_err() {
                            println!("got error: con error");
                            assert_eq!(1, -11);
                        }
                        let res = mapper.query_from_device(dev.address.clone(), "*IDN?\n");
                        if res.is_ok() {
                            println!("res got {}",res.unwrap());
                        }
                        let dis = mapper.disconnect_device(dev.address.clone());
                        match dis{
                            Ok(_)=>{assert_eq!(1, 1);},
                            Err(e) => {
                                println!("got error: {}", e);
                                assert_eq!(1, -10);
                            }
                        }
                    }
                    Err(e) => {
                        println!("got error: {}", e);
                        assert_eq!(1, -1);
                    }
                }
            }
            Err(e) => {
                println!("got error of {}", e);
                assert_eq!(1, -1);
            }
        }
    }
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
