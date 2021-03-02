#[derive(Debug)]
pub struct Section64 {
    pub name: String,
    pub r#type: u32,
    pub flags: u64,
    pub addr: u64,
    pub link: u32,
    pub info: u32,
    pub addralign: u64,
    pub data: Vec<u8>,
}
