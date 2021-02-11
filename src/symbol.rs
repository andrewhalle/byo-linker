use byteorder::{LittleEndian, ReadBytesExt};

#[derive(Debug)]
pub struct Symbol {
    pub name: u32,
    pub info: u8,
    pub other: u8,
    pub shndx: u16,
    pub value: usize,
    pub size: usize,
}

#[derive(Debug)]
pub struct SymbolIterator<'a> {
    pub data: &'a [u8],
}

impl<'a> Iterator for SymbolIterator<'a> {
    type Item = Symbol;

    fn next(&mut self) -> Option<Self::Item> {
        if self.data.len() == 0 {
            None
        } else {
            let (mut head, tail) = self.data.split_at(std::mem::size_of::<Symbol>());
            self.data = tail;

            // XXX address endianness
            let name = head
                .read_u32::<LittleEndian>()
                .expect("could not read name");
            let info = head.read_u8().expect("could not read info");
            let other = head.read_u8().expect("could not read other");
            let shndx = head
                .read_u16::<LittleEndian>()
                .expect("could not read shndx");
            let value = head
                .read_u64::<LittleEndian>()
                .expect("could not read value") as usize;
            let size = head
                .read_u64::<LittleEndian>()
                .expect("could not read size") as usize;

            let symbol = Symbol {
                name,
                info,
                other,
                shndx,
                value,
                size,
            };

            Some(symbol)
        }
    }
}
