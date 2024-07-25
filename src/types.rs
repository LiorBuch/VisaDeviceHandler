use serde::Serialize;

#[derive(Serialize)]
pub struct Device {
    pub name: String,
    pub address: String,
    pub session:u32
}