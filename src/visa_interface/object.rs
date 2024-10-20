#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]


use super::visa_ffi::*;
use super::err::VisaWrapperError;
use dlopen2::wrapper::Container;
use std::borrow::Cow;
// use visa::Visa;
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum Binary {
    ///Keysight specific Visa binary which only exists if Keysight IO is installed. Source: <https://www.keysight.com/de/de/lib/software-detail/computer-software/io-libraries-suite-downloads-2175637/keysight-io-libraries-suite-2022-for-windows.html>
    Keysight,
    ///National Instruments specific Visa binary which only exists if Ni-Visa is installed. Source: <https://www.ni.com/en-us/support/downloads/drivers/download.ni-visa.html>
    NiVisa,
    #[default]
    ///Primary visa binary. This could be any vendor implementation. If visa from any vendor is installed, this option typically works. The primary binary is typically named visa32.dll in windows.
    PlatformDefault,
    ///Custom path to a binary
    Custom(String),
}

impl Binary {
    fn binary_name(&self) -> Result<Cow<str>, VisaWrapperError> {
        Ok(match self {
            Binary::Keysight => {
                if cfg!(target_family = "windows") {
                    "ktvisa32.dll".into()
                } else if cfg!(target_family = "unix") && cfg!(target_pointer_width = "64") {
                    "libiovisa.so".into()
                } else {
                    return Err(VisaWrapperError::UnsupportedPlatform);
                }
            } //Keysight doesn't have official support for unix 32bit however it might have a .so file for 32bit
            Binary::NiVisa => {
                if cfg!(target_family = "windows") && cfg!(target_pointer_width = "64") {
                    "nivisa64.dll".into()
                } else if cfg!(target_family = "windows") && cfg!(target_pointer_width = "32") {
                    "visa32.dll".into()
                } else if cfg!(target_family = "unix") && cfg!(target_pointer_width = "64") {
                    "libvisa.so".into()
                } else {
                    return Err(VisaWrapperError::UnsupportedPlatform);
                }
            } //NiVisa doesn't have official support for unix 32bit however it might have a .so file for 32bit
            Binary::PlatformDefault => {
                if cfg!(target_family = "windows") {
                    "visa32".into()
                } else if cfg!(target_family = "unix") && cfg!(target_pointer_width = "64") {
                    "libvisa.so".into()
                } else if cfg!(target_family = "unix") && cfg!(target_pointer_width = "32") {
                    "libvisa32.so".into()
                } else {
                    return Err(VisaWrapperError::UnsupportedPlatform);
                }
            }
            Binary::Custom(path) => path.into(),
        })
    }
}

impl ToString for Binary {
    fn to_string(&self) -> String {
        self.binary_name()
            .unwrap_or_else(|e| e.to_string().into())
            .into()
    }
}
///This factory method loads a visa dynamically linked library .dll or .so etc.
///```rust
/// let visa =   visa::create(&visa::Binary::Keysight)
/// .or_else(|_| visa::create(&visa::Binary::NiVisa))
/// .or_else(|_| visa::create(&visa::Binary::Primary))
/// .or_else(|_| visa::create(&visa::Binary::Custom("visa.so".into())));
///```
pub fn make(bin: &Binary) -> Result<Container<VisaFFI>, VisaWrapperError> {
    unsafe { Container::load(bin.binary_name()?.as_ref()).map_err(|e| VisaWrapperError::from(e)) }
}