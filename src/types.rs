use serde::Serialize;

#[derive(Serialize,Clone)]
pub struct Device {
    pub name: String,
    pub address: String,
    pub session:u32
}

pub enum MapError{
    ReadError,
    OpenError,
    WriteError,
}