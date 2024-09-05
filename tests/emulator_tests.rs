#[cfg(test)]
mod emulator_tests{
    use visa_device_handler::visa_module::SafeDeviceMap;

    #[test]
    fn test_connect() {
        let mapper_res = SafeDeviceMap::default(None);
        match mapper_res {
            Ok(mapper) => {
                let mut stat = mapper.connect_device("TCPIP0::10.0.0.21::5025::SOCKET".to_string());
                match stat {
                    Ok(_) => assert_eq!(1, 1),
                    Err(_) => assert_eq!(1, -2),
                }
                stat = mapper.connect_device("TCPIP0::10.0.0.21::5026::SOCKET".to_string());
                match stat {
                    Ok(_) => assert_eq!(1, 1),
                    Err(_) => assert_eq!(1, -3),
                }
            }
            Err(e) => {
                println!("got error of {}", e);
                assert_eq!(1, -1);
            }
        }
    }
    #[test]
    fn test_map_call() {
        let mapper_res = SafeDeviceMap::default(None);
        match mapper_res {
            Ok(mapper) => {
                let stat = mapper.connect_device("TCPIP0::10.0.0.21::5026::SOCKET".to_string());
                match stat {
                    Ok(_) => assert_eq!(1, 1),
                    Err(_) => assert_eq!(1, -2),
                }
                let quer = mapper
                    .query_from_device("TCPIP0::10.0.0.21::5026::SOCKET".to_string(), "*IDN?");
                match quer {
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
                assert_eq!(1, -1);
            }
        }
    }
}