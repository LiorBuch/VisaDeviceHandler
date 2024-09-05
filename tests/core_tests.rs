#[cfg(test)]
mod core_tests{
    use visa_device_handler::visa_module::SafeDeviceMap;

    ///General test to see if the SafeDeviceMap works.
    fn test_mapper() {
        let mapper_res = SafeDeviceMap::default(None);
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
        let mapper_res = SafeDeviceMap::default(None);
        match mapper_res {
            Ok(mapper) => {
                let stat = mapper.find_all_devices(Some("USB?*"), true);
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
    #[test]
    fn test_map_multi_device() {
        let mapper_res = SafeDeviceMap::default(None);
        match mapper_res {
            Ok(mapper) => {
                let dev1 =
                    mapper.connect_device("USB0::0x0957::0x5407::MY59002371::INSTR".to_string());
                match dev1 {
                    Ok(_) => assert_eq!(1, 1),
                    Err(_) => assert_eq!(1, -1),
                }
                let dev2 =
                    mapper.connect_device("USB0::0x0699::0x03C7::C023503::INSTR".to_string());
                match dev2 {
                    Ok(_) => assert_eq!(1, 1),
                    Err(_) => assert_eq!(1, -2),
                }
                let quer_dev1 = mapper.query_from_device(
                    "USB0::0x0957::0x5407::MY59002371::INSTR".to_string(),
                    "*IDN?",
                );
                match quer_dev1 {
                    Ok(res) => {
                        println!("{}", res);
                        assert_eq!(1, 1)
                    }
                    Err(e) => {
                        println!("{}", e);
                        assert_eq!(1, -3)
                    }
                }
                let quer_dev2 = mapper.query_from_device(
                    "USB0::0x0699::0x03C7::C023503::INSTR".to_string(),
                    "*IDN?",
                );
                match quer_dev2 {
                    Ok(res) => {
                        println!("{}", res);
                        assert_eq!(1, 1)
                    }
                    Err(e) => {
                        println!("{}", e);
                        assert_eq!(1, -4)
                    }
                }
            }
            Err(e) => {
                println!("got error of {}", e);
                assert_eq!(1, -11);
            }
        }
    }
}