Visa Device Handler
======

Based on the visa crate, this crate aims to provide a wrapper to allow multiple device connecton and handling,
and make the usage of NI-VISA easier to use.

The crate supports Windows, MacOS and Linux. but is being tested mainly on windows.

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