pub type BusSize = u32;

#[derive(Default)]
pub struct Bus {
    dram: Vec<u8>
}

impl Bus {
    pub fn new(dram_size: usize) -> Bus {
        Bus {
            dram: vec![0; dram_size]
        }
    }

    pub fn load_code(&mut self, value: Vec<u8>) {
        self.dram.splice(..value.len(), value.iter().cloned());
    }

    pub fn load8(&mut self, addr: BusSize) -> u8 {
        match addr {
            0x80000000..=0xFFFFFFFF => self.dram[(addr - 0x80000000) as usize],
            _ => panic!("Bad memory access")
        }
    }

    pub fn load16(&mut self, addr: BusSize) -> u16 {
        (self.load8(addr) as u16) |
        (self.load8(addr + 1) as u16) << 8
    }

    pub fn load32(&mut self, addr: BusSize) -> u32 {
        (self.load8(addr) as u32) |
        (self.load8(addr + 1) as u32) << 8 |
        (self.load8(addr + 2) as u32) << 16 |
        (self.load8(addr + 3) as u32) << 24
    }
}

impl std::fmt::Debug for Bus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Bus {{ DRAM[{:x}o] }}", self.dram.len())
    }
}