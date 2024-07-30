use serde::Serialize;

#[derive(Serialize,Clone)]
pub struct Device {
    pub name: String,
    pub address: String,
    pub session:u32
}