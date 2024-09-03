#[cfg(test)]
mod core_tests{
    use visa_device_handler::visa_module::SafeDeviceMap;

    ///General test to see if the SafeDeviceMap works.
    fn test_mapper() {
        let mapper_res = SafeDeviceMap::init(None,None);
        match mapper_res {
            Ok(mapper) => {
                let stat = mapper.get_first_device(Some("?*INSTR"), true);
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
                            println!("res got {}", res.unwrap());
                        }
                        let dis = mapper.disconnect_device(dev.address.clone());
                        match dis {
                            Ok(_) => {
                                assert_eq!(1, 1);
                            }
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
    #[test]
    fn test_find_all() {
        let mapper_res = SafeDeviceMap::init(Some("C:\\Windows\\System32\\visa64.dll"),None);
        match mapper_res {
            Ok(mapper) => {
                let stat = mapper.find_all_devices(Some("?*USB"), true);
                match stat {
                    Ok(_) => assert_eq!(1, 1),
                    Err(_) => assert_eq!(1, -2),
                }
            }
            Err(e) => {
                println!("got error of {}", e);
                assert_eq!(1, -1);
            }
        }
    }
}