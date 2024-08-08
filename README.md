Visa Device Handler
======

Based on the visa crate, this crate aims to provide a wrapper to allow multiple device connections and handling,
and make the usage of NI-VISA easier to use.

The crate supports Windows, Linux and MacOS. but is being tested mainly on Windows and Linux so I cannot guarantee its behavior on Mac.

## Example

```rust

let sdm_result:SafeDeviceMap = SafeDeviceMap::init(None);
match sdm_result {
    Ok(mapper) => {
        mapper.connect_device("address_01".to_string());
        let data = mapper.query_from_device("name_01".to_string(),"cool funcation with args").unwrap();
        println!("got {} from the device",data);
        mapper.disconnect_device("name_01".to_string());
    }
    Err(e) => {/*print codes or anything */}
}
```
## Tauri

The next Tauri project utilizes the crate as a peripheral for NI-VISA.
https://github.com/LiorBuch/showcaser.
this project uses React and Mantine as the UI, it's not failproof as it's just a POC.


## Change Log 0.2.0 -> 0.3.0

- `get_first_device` now returns the device instead of printing.
- `get_first_device` and `find_all_devices` now won't print unless `debug` boolean is true.
- Fixed the error where the CString parses null terminators.
- Added key error map in some functions.

## Change Log 0.3.0 -> 0.3.1

- Opened project as open source.
- Added Tauri example project -> https://github.com/LiorBuch/showcaser
- added filter keywords for `get_first_device` and `find_all_devices`
