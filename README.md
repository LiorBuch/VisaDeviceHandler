Visa Device Handler
======

Based on the visa crate, this crate aims to provide a wrapper to allow multiple device connections and handling,
and make the usage of NI-VISA easier to use.

The crate supports Windows, and sould support Linux and MacOS. but is being tested mainly on Windows.

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

## Emulators

The next project creates an emulator of a visa device to be used with a TCP connection.
As the project is not mine, i cant help much with its usage.
One issue i spotted is that read operations always timeout, probably a missing termination character.
Link to the project: https://github.com/bluehands/Open-SCPI-Protocol-Emulator/tree/main

## Change Log 0.3.0

- `get_first_device` now returns the device instead of printing.
- `get_first_device` and `find_all_devices` now won't print unless `debug` boolean is true.
- Fixed the error where the CString parses null terminators.
- Added key error map in some functions.

    ### Change Log 0.3.1

    - Opened project as open source.
    - Added Tauri example project -> https://github.com/LiorBuch/showcaser
    - Added filter keywords for `get_first_device` and `find_all_devices`

## Change Log 0.4.0

- Fixed the disconnect issue.
- Cleaned structure.

## Change Log 0.5.0

- All Device operations now use its address, more uniform approch since the address is always presented.
- Linux is now enabled, yet not fully tested.
- Added logger to the Map to save all the warnings and suppress them. (set to Debug by default).
- status codes are now fully tested to give more accurate information.

## Change Log 0.6.0
- Fixed the problem where unable to use devices mapped since key include [`\0`] chars and address not.
- Validated usage of TCP deviecs and emulators.
- Added the option to define if a program should terminate by the `Verbosity` of the error, as some error codes can be warnings. defaults to Error level.

    ### Change Log 0.6.1

    - Updated the logger to V1.0.0, adds better log format.
    - Fixed the find all issue where it cant find more than one device without crashing.