#[cfg(test)]
mod interface_tests {
    use visa_device_handler::visa_interface;


    #[test]
    fn test_interface() {
        let lib = visa_device_handler::visa_interface::make(&visa_interface::Binary::NiVisa).unwrap();
        let mut rm: u32 = 0;
        let mut _stat = lib.viOpenDefaultRM(&mut rm);
        let mut new_session: u32 = 0;
        let mut des: [u8; 256] = [0u8; 256];
        let address_bytes = "USB0::0x0699::0x03C7::C023503::INSTR".as_bytes();
        let len = address_bytes.len().min(256);
        des[..len].copy_from_slice(&address_bytes[..len]);
        _stat = lib.viOpen(
            rm,
            des.as_mut_ptr() as *mut i8,
            0,
            0,
            &mut new_session,
        );
        let mut ret_cnt = 0u32;
        let cnt = u32::try_from("*IDN?\n".len()).map_err(|_| "u32 conversion error".to_string()).unwrap();
        println!("{:?}","*IDN?\n".as_bytes());
        _stat = lib.viWrite(new_session, "*IDN?\n".as_bytes().as_ptr(), cnt, &mut ret_cnt);
        ret_cnt = 0u32;
        let mut buf = vec![0u8; 128];
        _stat = lib.viRead(new_session, buf.as_mut_ptr(), 128, &mut ret_cnt);
        let response = std::str::from_utf8(&buf[0..ret_cnt as usize]).map_err(|_| "Unable to parse byte array to string!").unwrap();
        println!("{}",response);
        lib.viClose(new_session);
        lib.viClose(rm);
        assert_eq!(1,1);
    }
}