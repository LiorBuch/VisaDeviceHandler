use std::{
    collections::HashMap,
    env,
    ffi::CString,
    sync::{Arc, Mutex},
};

use dlopen::wrapper::Container;
use visa::Wrapper;

use crate::types::Device;

/// The `SafeDeviceMap` provides a lock safe way to store The resource manager and all the sessions in one place.  
/// SafeDeviceMap uses Arc and Mutex wrapped around rm and map to provide a safe way to interact with them.
///
/// # Params
/// @lib -> The main visa library, its just to create all kind of calls.  
/// @rm -> [`u32`] session number, will hold the sessions and open new ones.  
/// @map -> [`HashMap<String,Device>`] instance, stores all the instruments by thier device name (Human readable string).
///
/// #Panic
/// As the program panics, the rm will be droped so the connections should be freed as well.  
/// But just to be safe allways call [`SafeDeviceMap::disconnect_device`] or [`SafeDeviceMap::clear_map`] when finished.  
/// I will try too look more into it and update it in the future.
///
/// # Examples
/// ```
/// let sdm_result:SafeDeviceMap = SafeDeviceMap::init(None);
/// match sdm_result {
///     Ok(mapper) => {
///         mapper.connect_device("address_01".to_string());
///         let data = mapper.query_from_device("name_01".to_string(),"cool funcation with args").unwrap();
///         println!("got {} from the device",data);
///         mapper.disconnect_device("name_01".to_string());
///     }
///     Err(e) => {/*print codes or anything */}
/// }
/// ```
///
/// To get a SafeDeviceMap call [`SafeDeviceMap::init()`].
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
    ///Call this to get the SafeDeviceMap.  
    ///It will generate a DefaultRM ,create a new HasMap and save the library instance.
    ///
    ///This Method **MUST** be called.
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
    ///Call this to insert a new device to the HashMap.
    ///
    ///@address -> type [`String`], the address of the device you wish to add to the map.  
    ///*Before a device is entered to the map it will open a connection too.*
    ///
    ///Returns -> error [`String`] if failed to insert or void if success.
    pub fn connect_device(&self, address: String) -> Result<(), String> {
        let lib = self.lib.lock().map_err(|e| e.to_string())?;
        let rm = self.rm.lock().map_err(|e| e.to_string())?;
        let mut map = self.map.lock().map_err(|e| e.to_string())?;

        let cmd = b"*IDN?\n";
        let mut ret_cnt = 0u32;
        let mut des = [0u8; 256];
        let mut device_session: u32 = 0;
        let c_address = CString::new(address.trim_end_matches('\0').as_bytes()).map_err(|_| "CString Addrees conversion error".to_string())?;
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
            u32::try_from(cmd.len()).map_err(|_| "u32 conversion error".to_string())?,
            &mut ret_cnt,
        );
        let resp = vec![0u8; 50];
        status = lib.viRead(device_session, resp.as_ptr() as *mut _, 50, &mut ret_cnt);
        let response = std::str::from_utf8(&resp[0..ret_cnt as usize]).map_err(|_| "response parse error".to_string())?;

        let device = Device {
            address: String::from_utf8_lossy(&des).to_string(),
            name: response.to_string(),
            session: device_session,
        };
        map.insert(response.to_string(), device);
        Ok(())
    }
    ///Call this to remove a device from the map.
    ///Note, the removed device data will be returned.
    ///
    ///@name -> Device name to get removed.
    ///
    ///Return -> The removed [`Device`] on success or error string on fail.
    pub fn disconnect_device(&self, name: String) -> Result<Device, String> {
        let lib = self.lib.lock().map_err(|e| e.to_string())?;
        let mut map = self.map.lock().map_err(|e| e.to_string())?;

        let remove_result = map.remove(&name);
        match remove_result {
            Some(mut device) => {
                lib.viClose(device.session);
                device.session = 0;
                Ok(device)
            }
            None => Err("no device was found!".to_string()),
        }
    }
    ///Call this to write message to the device.
    ///
    ///@name -> Device name send the message to.  
    ///@msg -> The message to send, a concated [`str`] of function name and its args.
    ///
    ///Returns -> Void on success string if error.
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
                    u32::try_from(cmd.len()).map_err(|_| "u32 conversion error".to_string())?,
                    &mut ret_cnt,
                );
                Ok(())
            }
            None => Err("device not exist!".to_string()),
        }
    }
    ///Call this to read message from the device buffer.  
    ///Note, the buffer will be cleared after a read!
    ///
    ///@name -> Device name you wish to read its buffer.
    ///
    ///Returns -> The read string if success error string if failed.
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
    ///Call this to write and read message from the device buffer.  
    ///Note! the buffer will be cleared after a read!  
    ///*This calls behaves like write and then read.*
    ///
    ///@name -> Device name to interact with.  
    ///@msg -> Message to send to the device (the write message).
    ///
    ///Returns -> The read string if success error string if failed.
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
                    u32::try_from(cmd.len()).map_err(|_| "u32 conversion error".to_string())?,
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
    ///This function will find the first device the rm can finds.
    /// 
    ///@filter -> &str, choose the filter keyword for NI-VISA to look for the devices,None defaults to "USB?*".  
    ///@debug -> bool, if you wish to print the device info.
    /// 
    ///Returns -> The first [`Device`].
    pub fn get_first_device(&self,filter:Option<&str>,debug:bool) -> Result<Device, String> {
        let lib = self.lib.lock().map_err(|e| e.to_string())?;
        let rm = self.rm.lock().map_err(|e| e.to_string())?;

        let cmd = b"*IDN?\n";
        let mut ret_cnt = 0u32;
        let mut des = [0u8; 256];
        let mut device_session: u32 = 0;
        let c_address = CString::new(filter.unwrap_or("USB?*")).map_err(|_| "CString Addrees conversion error".to_string())?;
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
            u32::try_from(cmd.len()).map_err(|_| "u32 conversion error".to_string())?,
            &mut ret_cnt,
        );
        let resp = vec![0u8; 50];
        status = lib.viRead(device_session, resp.as_ptr() as *mut _, 50, &mut ret_cnt);
        let response = std::str::from_utf8(&resp[0..ret_cnt as usize]).map_err(|_| "response parse error".to_string())?;

        let device = Device {
            address: String::from_utf8_lossy(&des).to_string(),
            name: response.to_string(),
            session: device_session,
        };
        if debug{
            println!("device name: {}", device.name);
            println!("device address: {}", device.address);
            println!("device session: {}", device.session);
            println!("rm session: {}", rm);
        }
        Ok(device)
    }
    ///This function will find all the devices connected with USB to the PC.
    ///
    ///@filter -> &str, choose the filter keyword for NI-VISA to look for the devices,None defaults to "USB?*".  
    ///@debug -> bool, if you wish to print the device info.
    ///
    ///Returns -> A [`Vec`] of [`Device`] with USB connections.
    pub fn find_all_devices(&self,filter:Option<&str>,debug:bool) -> Result<Vec<Device>, String> {
        let lib = self.lib.lock().map_err(|e| e.to_string())?;
        let rm = self.rm.lock().map_err(|e| e.to_string())?;

        let cmd = b"*IDN?\n";
        let mut ret_cnt = 0u32;
        let mut des = [0u8; 256];
        let mut device_session: u32 = 0;
        let c_address = CString::new(filter.unwrap_or("USB?*")).map_err(|_| "CString Adrees conversion error".to_string())?;
        let mut status = lib.viFindRsrc(
            rm.clone(),
            c_address.as_ptr(),
            &mut device_session,
            &mut ret_cnt,
            des.as_mut_ptr() as *mut i8,
        );
        let mut devices: Vec<Device> = Vec::new();
        for i in 0..ret_cnt {
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
                u32::try_from(cmd.len()).map_err(|_| "u32 conversion error".to_string())?,
                &mut ret_cnt,
            );
            let resp = vec![0u8; 50];
            status = lib.viRead(device_session, resp.as_ptr() as *mut _, 50, &mut ret_cnt);
            let response = std::str::from_utf8(&resp[0..ret_cnt as usize]).map_err(|_| "response parse error".to_string())?;

            let device = Device {
                address: String::from_utf8_lossy(&des).to_string(),
                name: response.to_string(),
                session: device_session,
            };
            if debug{
                println!("<---- Device No:{i} ---->");
                println!("device name: {}", device.name);
                println!("device address: {}", device.address);
                println!("device session: {}", device.session);
                println!("rm session: {}", rm);
                println!("<-----------------------> \n");
            }
            devices.push(device);
            lib.viFindNext(device_session, des.as_mut_ptr() as *mut i8);
        }
        Ok(devices)
    }
    ///This function clears the mapping of the devices.  
    ///It will close **all** the connections and **drop** all the Devices.
    pub fn clear_map(&self) -> Result<(), String> {
        let lib = self.lib.lock().map_err(|e| e.to_string())?;
        let mut map = self.map.lock().map_err(|e| e.to_string())?;
        for value in map.values() {
            lib.viClear(value.session);
            lib.viClose(value.session);
        }
        map.clear();
        Ok(())
    }
    ///This function returns to the user all the active [`Device`] that the [`SafeDeviceMap`] have access to.
    ///
    ///Returns -> A [`Vec`] of [`Device`] currently in use.
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
