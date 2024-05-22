use std::ffi::CString;

pub fn test_ni_visa() -> i32 {
    let visa_result = visa::create(visa::Binary::NiVisa);
    match visa_result {
        Ok(visa) => {
            let mut _session = 0;
            let mut status = visa.viOpenDefaultRM(&mut _session);
            if status != 0 {
                return status;
            }
            let address = CString::new(format!("USB0::0x0699::0x03C7::C023503::INSTR")).unwrap();
            let mut vi = 0;
            status = visa.viOpen(_session, address.as_ptr(), 0, 0, &mut vi);
            let cmd = b"*IDN?\n";
            let mut ret_cnt = 0u32;
            status = visa.viWrite(
                vi,
                cmd.as_ptr(),
                u32::try_from(cmd.len()).unwrap(),
                &mut ret_cnt,
            );
            let resp = vec![0u8; 50];
            status = visa.viRead(vi, resp.as_ptr() as *mut _, 50, &mut ret_cnt);
            let response = std::str::from_utf8(&resp[0..ret_cnt as usize]).unwrap();
            println!("Response : {}", response);
            return 0;
        }
        Err(e) => {
            println!("got the next error: {}", e);
            return -1;
        }
    }
}
pub fn test_ni_so() -> i32 {
    let visa_result = visa::create(visa::Binary::Custom("visa.so".into()));
    match visa_result {
        Ok(visa) => {
            let mut _session = 0;
            let mut status = visa.viOpenDefaultRM(&mut _session);
            if status != 0 {
                return status;
            }
            let address = CString::new(format!("USB0::0x0699::0x03C7::C023503::INSTR")).unwrap();
            let mut vi = 0;
            status = visa.viOpen(_session, address.as_ptr(), 0, 0, &mut vi);
            let cmd = b"*IDN?\n";
            let mut ret_cnt = 0u32;
            status = visa.viWrite(
                vi,
                cmd.as_ptr(),
                u32::try_from(cmd.len()).unwrap(),
                &mut ret_cnt,
            );
            let resp = vec![0u8; 50];
            let status = visa.viRead(vi, resp.as_ptr() as *mut _, 50, &mut ret_cnt);
            let response = std::str::from_utf8(&resp[0..ret_cnt as usize]).unwrap();
            println!("Response : {}", response);
            return 0;
        }
        Err(e) => {
            println!("got the next error: {}", e);
            return -1;
        }
    }
}
pub fn test_ni_default() -> i32 {
    let visa_result = visa::create(visa::Binary::Default);
    match visa_result {
        Ok(visa) => {
            let mut _session = 0;
            let mut status = visa.viOpenDefaultRM(&mut _session);
            if status != 0 {
                return status;
            }
            let address = CString::new(format!("USB0::0x0699::0x03C7::C023503::INSTR")).unwrap();
            let mut vi = 0;
            status = visa.viOpen(_session, address.as_ptr(), 0, 0, &mut vi);
            let cmd = b"*IDN?\n";
            let mut ret_cnt = 0u32;
            status = visa.viWrite(
                vi,
                cmd.as_ptr(),
                u32::try_from(cmd.len()).unwrap(),
                &mut ret_cnt,
            );
            let resp = vec![0u8; 50];
            let status = visa.viRead(vi, resp.as_ptr() as *mut _, 50, &mut ret_cnt);
            let response = std::str::from_utf8(&resp[0..ret_cnt as usize]).unwrap();
            println!("Response : {}", response);
            return 0;
        }
        Err(e) => {
            println!("got the next error: {}", e);
            return -1;
        }
    }
}
pub fn test_ni_ks() -> i32 {
    let visa_result = visa::create(visa::Binary::Keysight);
    match visa_result {
        Ok(visa) => {
            let mut _session = 0;
            let mut status = visa.viOpenDefaultRM(&mut _session);
            if status != 0 {
                return status;
            }
            let address = CString::new(format!("USB0::0x0699::0x03C7::C023503::INSTR")).unwrap();
            let mut vi = 0;
            status = visa.viOpen(_session, address.as_ptr(), 0, 0, &mut vi);
            let cmd = b"*IDN?\n";
            let mut ret_cnt = 0u32;
            status = visa.viWrite(
                vi,
                cmd.as_ptr(),
                u32::try_from(cmd.len()).unwrap(),
                &mut ret_cnt,
            );
            let resp = vec![0u8; 50];
            status = visa.viRead(vi, resp.as_ptr() as *mut _, 50, &mut ret_cnt);
            let response = std::str::from_utf8(&resp[0..ret_cnt as usize]).unwrap();
            println!("Response : {}", response);
            return 0;
        }
        Err(e) => {
            println!("got the next error: {}", e);
            return -1;
        }
    }
}