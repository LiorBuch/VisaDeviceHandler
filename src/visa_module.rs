use std::{
    collections::HashMap,
    env,
    ffi::CString,
    sync::{Arc, Mutex},
};

use dlopen::wrapper::Container;
use visa::Wrapper;

use crate::types::Device;

pub struct SafeDeviceMap {
    lib: Arc<Mutex<Container<Wrapper>>>,
    rm: Arc<Mutex<u32>>,
    map: Arc<Mutex<HashMap<String, Device>>>,
}
impl Drop for SafeDeviceMap {
    fn drop(&mut self) {
        let lib = self.lib.lock().map_err(|e| e.to_string()).unwrap();
        let rm = self.rm.lock().map_err(|e| e.to_string()).unwrap();
        lib.viClose(rm.clone());
    }
}
impl SafeDeviceMap {
    pub fn init(file_path: Option<&str>) -> Result<SafeDeviceMap, String> {
        let os = env::consts::OS;
        let visa: Container<Wrapper>;
        match os {
            "windows" => {
                visa = visa::create(visa::Binary::NiVisa)
                    .map_err(|_| "error opening windows library file!".to_string())?;
            }
            "macos" => {
                visa = visa::create(visa::Binary::Custom(file_path.unwrap().to_string()))
                    .map_err(|_| "error opening macos library file!".to_string())?;
            }
            _ => {
                unimplemented!("the os: {} is not supported!", os)
            }
        }
        let mut rm_session = 0;
        let status = visa.viOpenDefaultRM(&mut rm_session);
        if status != 0 {
            return Err("unable to open RM!".to_string());
        }
        let safe = SafeDeviceMap {
            lib: Arc::new(Mutex::new(visa)),
            rm: Arc::new(Mutex::new(rm_session)),
            map: Arc::new(Mutex::new(HashMap::new())),
        };
        Ok(safe)
    }
    pub fn connect_device(&self, address: String) -> Result<(), String> {
        let lib = self.lib.lock().map_err(|e| e.to_string())?;
        let rm = self.rm.lock().map_err(|e| e.to_string())?;
        let mut map = self.map.lock().map_err(|e| e.to_string())?;

        let cmd = b"*IDN?\n";
        let mut ret_cnt = 0u32;
        let mut des = [0u8; 256];
        let mut device_session: u32 = 0;
        let c_address = CString::new(address).unwrap();
        let mut status = lib.viFindRsrc(
            rm.clone(),
            c_address.as_ptr(),
            &mut device_session,
            &mut ret_cnt,
            des.as_mut_ptr() as *mut i8,
        );
        status = lib.viOpen(
            rm.clone(),
            des.as_ptr() as *mut i8,
            0,
            0,
            &mut device_session,
        );
        status = lib.viWrite(
            device_session,
            cmd.as_ptr(),
            u32::try_from(cmd.len()).unwrap(),
            &mut ret_cnt,
        );
        let resp = vec![0u8; 50];
        status = lib.viRead(device_session, resp.as_ptr() as *mut _, 50, &mut ret_cnt);
        let response = std::str::from_utf8(&resp[0..ret_cnt as usize]).unwrap();

        let device = Device {
            address: String::from_utf8_lossy(&des).to_string(),
            name: response.to_string(),
            session: device_session,
        };
        map.insert(response.to_string(), device);
        Ok(())
    }
    pub fn disconnect_device(&self, name: String) -> Result<Device, String> {
        let lib = self.lib.lock().map_err(|e| e.to_string())?;
        let mut map = self.map.lock().map_err(|e| e.to_string())?;

        let remove_result = map.remove(&name);
        match remove_result {
            Some(device) => {
                lib.viClose(device.session);
                Ok(device)
            }
            None => Err("no device was found!".to_string()),
        }
    }
    pub fn write_to_device(&self, name: String, msg: &str) -> Result<(), String> {
        let lib = self.lib.lock().map_err(|e| e.to_string())?;
        let map = self.map.lock().map_err(|e| e.to_string())?;

        let device_session_option = map.get(&name);
        match device_session_option {
            Some(device) => {
                let mut device_session = device.session;
                let cmd: &[u8] = msg.as_bytes();
                let mut ret_cnt = 0u32;
                let status = lib.viWrite(
                    device_session,
                    cmd.as_ptr(),
                    u32::try_from(cmd.len()).unwrap(),
                    &mut ret_cnt,
                );
                Ok(())
            }
            None => Err("device not exist!".to_string()),
        }
    }
    pub fn read_from_device(&self, name: String) -> Result<String, String> {
        let lib = self.lib.lock().map_err(|e| e.to_string())?;
        let map = self.map.lock().map_err(|e| e.to_string())?;

        let device_session_option = map.get(&name);
        match device_session_option {
            Some(device) => {
                let device_session = device.session;
                let mut ret_cnt = 0u32;
                let resp = vec![0u8; 50];
                let status = lib.viRead(device_session, resp.as_ptr() as *mut _, 50, &mut ret_cnt);
                let response = std::str::from_utf8(&resp[0..ret_cnt as usize])
                    .map_err(|_| "error, cannot parse response")?;
                Ok(response.to_string())
            }
            None => Err("device not exist!".to_string()),
        }
    }
    pub fn query_from_device(&self, name: String, msg: &str) -> Result<String, String> {
        let lib = self.lib.lock().map_err(|e| e.to_string())?;
        let map = self.map.lock().map_err(|e| e.to_string())?;

        let device_session_option = map.get(&name);
        match device_session_option {
            Some(device) => {
                let device_session = device.session;
                let cmd: &[u8] = msg.as_bytes();
                let mut ret_cnt = 0u32;
                let mut status = lib.viWrite(
                    device_session,
                    cmd.as_ptr(),
                    u32::try_from(cmd.len()).unwrap(),
                    &mut ret_cnt,
                );
                let resp = vec![0u8; 50];
                status = lib.viRead(device_session, resp.as_ptr() as *mut _, 50, &mut ret_cnt);
                let response = std::str::from_utf8(&resp[0..ret_cnt as usize])
                    .map_err(|_| "error, cannot parse response")?;
                Ok(response.to_string())
            }
            None => Err("device not exist!".to_string()),
        }
    }
    pub fn get_first_device(&self) -> Result<(), String> {
        let lib = self.lib.lock().map_err(|e| e.to_string())?;
        let rm = self.rm.lock().map_err(|e| e.to_string())?;

        let cmd = b"*IDN?\n";
        let mut ret_cnt = 0u32;
        let mut des = [0u8; 256];
        let mut device_session: u32 = 0;
        let c_address = CString::new(format!("USB?*")).unwrap();
        let mut status = lib.viFindRsrc(
            rm.clone(),
            c_address.as_ptr(),
            &mut device_session,
            &mut ret_cnt,
            des.as_mut_ptr() as *mut i8,
        );
        status = lib.viOpen(
            rm.clone(),
            des.as_ptr() as *mut i8,
            0,
            0,
            &mut device_session,
        );
        status = lib.viWrite(
            device_session,
            cmd.as_ptr(),
            u32::try_from(cmd.len()).unwrap(),
            &mut ret_cnt,
        );
        let resp = vec![0u8; 50];
        status = lib.viRead(device_session, resp.as_ptr() as *mut _, 50, &mut ret_cnt);
        let response = std::str::from_utf8(&resp[0..ret_cnt as usize]).unwrap();

        let device = Device {
            address: String::from_utf8_lossy(&des).to_string(),
            name: response.to_string(),
            session: device_session,
        };
        println!("device name: {}", device.name);
        println!("device address: {}", device.address);
        println!("device session: {}", device.session);
        println!("rm session: {}", rm);
        Ok(())
    }
    pub fn get_all_devices(&self) -> Result<(), String> {
        let lib = self.lib.lock().map_err(|e| e.to_string())?;
        let rm = self.rm.lock().map_err(|e| e.to_string())?;

        let cmd = b"*IDN?\n";
        let mut ret_cnt = 0u32;
        let mut des = [0u8; 256];
        let mut device_session: u32 = 0;
        let c_address = CString::new(format!("USB?*")).unwrap();
        let mut status = lib.viFindRsrc(
            rm.clone(),
            c_address.as_ptr(),
            &mut device_session,
            &mut ret_cnt,
            des.as_mut_ptr() as *mut i8,
        );
        for i in 0..ret_cnt {
            lib.viFindNext(device_session, des.as_mut_ptr() as *mut i8);
            status = lib.viOpen(
                rm.clone(),
                des.as_ptr() as *mut i8,
                0,
                0,
                &mut device_session,
            );
            status = lib.viWrite(
                device_session,
                cmd.as_ptr(),
                u32::try_from(cmd.len()).unwrap(),
                &mut ret_cnt,
            );
            let resp = vec![0u8; 50];
            status = lib.viRead(device_session, resp.as_ptr() as *mut _, 50, &mut ret_cnt);
            let response = std::str::from_utf8(&resp[0..ret_cnt as usize]).unwrap();

            let device = Device {
                address: String::from_utf8_lossy(&des).to_string(),
                name: response.to_string(),
                session: device_session,
            };
            println!("<---- Device No:{i} ---->");
            println!("device name: {}", device.name);
            println!("device address: {}", device.address);
            println!("device session: {}", device.session);
            println!("rm session: {}", rm);
            println!("<-----------------------> \n");
        }
        Ok(())
    }
    pub fn clear_map() {
        unimplemented!()
    }
    pub fn get_all_mapped_devices(&self) -> Result<Vec<Device>, String> {
        let map = self.map.lock().map_err(|e| e.to_string())?;
        let mut devices: Vec<Device> = Vec::new();
        for value in map.values() {
            let dev = Device {
                address: value.address.to_string(),
                name: value.name.to_string(),
                session: value.session,
            };
            devices.push(dev);
        }
        Ok(devices)
    }
}