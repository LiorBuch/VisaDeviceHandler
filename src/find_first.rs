use std::ffi::CString;

pub fn test_ni_visa_find_first() -> i32 {
    let visa_result = visa::create(visa::Binary::NiVisa);
    match visa_result {
        Ok(visa) => {
            let mut _session = 0;
            let mut status = visa.viOpenDefaultRM(&mut _session);
            if status != 0 {
                return status;
            }
            let address = CString::new(format!("USB?*")).unwrap();
            let mut vi = 0;
            let cmd = b"*IDN?\n";
            let mut ret_cnt = 0u32;
            let mut des= [0u8;256];
            status = visa.viFindRsrc(_session, address.as_ptr(), &mut vi, &mut ret_cnt,des.as_mut_ptr() as *mut i8);
            println!("vi value was changed to: {}",vi);
            println!("and count: {}",ret_cnt);
            println!("with the address: {}",String::from_utf8_lossy(&des));
            status = visa.viOpen(_session, des.as_ptr() as *mut i8, 0, 0, &mut vi);
            status = visa.viWrite(
                vi,
                cmd.as_ptr(),
                u32::try_from(cmd.len()).unwrap(),
                &mut ret_cnt,
            );
            let resp = vec![0u8; 50];
            status = visa.viRead(vi, resp.as_ptr() as *mut _, 50, &mut ret_cnt);
            let response = std::str::from_utf8(&resp[0..ret_cnt as usize]).unwrap();
            println!("and the name: {}", response);
            return 0;
        }
        Err(e) => {
            println!("got the next error: {}", e);
            return -1;
        }
    }
}
pub fn test_ni_default_find_first() -> i32 {
    let visa_result = visa::create(visa::Binary::Default);
    match visa_result {
        Ok(visa) => {
            let mut _session = 0;
            let mut status = visa.viOpenDefaultRM(&mut _session);
            if status != 0 {
                return status;
            }
            let address = CString::new(format!("USB?*")).unwrap();
            let mut vi = 0;
            let cmd = b"*IDN?\n";
            let mut ret_cnt = 0u32;
            let mut des= [0u8;256];
            status = visa.viFindRsrc(_session, address.as_ptr(), &mut vi, &mut ret_cnt,des.as_mut_ptr() as *mut i8);
            println!("vi value was changed to: {}",vi);
            println!("and count: {}",ret_cnt);
            println!("with the address: {}",String::from_utf8_lossy(&des));
            status = visa.viOpen(_session, des.as_ptr() as *mut i8, 0, 0, &mut vi);
            status = visa.viWrite(
                vi,
                cmd.as_ptr(),
                u32::try_from(cmd.len()).unwrap(),
                &mut ret_cnt,
            );
            let resp = vec![0u8; 50];
            status = visa.viRead(vi, resp.as_ptr() as *mut _, 50, &mut ret_cnt);
            let response = std::str::from_utf8(&resp[0..ret_cnt as usize]).unwrap();
            println!("and the name: {}", response);
            return 0;
        }
        Err(e) => {
            println!("got the next error: {}", e);
            return -1;
        }
    }
}
pub fn test_ni_ks_find_first() -> i32 {
    let visa_result = visa::create(visa::Binary::Keysight);
    match visa_result {
        Ok(visa) => {
            let mut _session = 0;
            let mut status = visa.viOpenDefaultRM(&mut _session);
            if status != 0 {
                return status;
            }
            let address = CString::new(format!("USB?*")).unwrap();
            let mut vi = 0;
            let cmd = b"*IDN?\n";
            let mut ret_cnt = 0u32;
            let mut des= [0u8;256];
            status = visa.viFindRsrc(_session, address.as_ptr(), &mut vi, &mut ret_cnt,des.as_mut_ptr() as *mut i8);
            println!("vi value was changed to: {}",vi);
            println!("and count: {}",ret_cnt);
            println!("with the address: {}",String::from_utf8_lossy(&des));
            status = visa.viOpen(_session, des.as_ptr() as *mut i8, 0, 0, &mut vi);
            status = visa.viWrite(
                vi,
                cmd.as_ptr(),
                u32::try_from(cmd.len()).unwrap(),
                &mut ret_cnt,
            );
            let resp = vec![0u8; 50];
            status = visa.viRead(vi, resp.as_ptr() as *mut _, 50, &mut ret_cnt);
            let response = std::str::from_utf8(&resp[0..ret_cnt as usize]).unwrap();
            println!("and the name: {}", response);
            return 0;
        }
        Err(e) => {
            println!("got the next error: {}", e);
            return -1;
        }
    }
}
pub fn test_ni_so_find_first() -> i32 {
    let visa_result = visa::create(visa::Binary::Custom("visa.so".into()));
    match visa_result {
        Ok(visa) => {
            let mut _session = 0;
            let mut status = visa.viOpenDefaultRM(&mut _session);
            if status != 0 {
                return status;
            }
            let address = CString::new(format!("USB?*")).unwrap();
            let mut vi = 0;
            let cmd = b"*IDN?\n";
            let mut ret_cnt = 0u32;
            let mut des= [0u8;256];
            status = visa.viFindRsrc(_session, address.as_ptr(), &mut vi, &mut ret_cnt,des.as_mut_ptr() as *mut i8);
            println!("vi value was changed to: {}",vi);
            println!("and count: {}",ret_cnt);
            println!("with the address: {}",String::from_utf8_lossy(&des));
            status = visa.viOpen(_session, des.as_ptr() as *mut i8, 0, 0, &mut vi);
            status = visa.viWrite(
                vi,
                cmd.as_ptr(),
                u32::try_from(cmd.len()).unwrap(),
                &mut ret_cnt,
            );
            let resp = vec![0u8; 50];
            status = visa.viRead(vi, resp.as_ptr() as *mut _, 50, &mut ret_cnt);
            let response = std::str::from_utf8(&resp[0..ret_cnt as usize]).unwrap();
            println!("and the name: {}", response);
            return 0;
        }
        Err(e) => {
            println!("got the next error: {}", e);
            return -1;
        }
    }
}
