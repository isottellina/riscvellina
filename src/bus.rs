pub type BusSize = u64;

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
            _ => panic!("Bad memory access (Tried to load {:08x}", addr)
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

    pub fn load64(&mut self, addr: BusSize) -> u64 {
        (self.load8(addr) as u64) |
        (self.load8(addr + 1) as u64) << 8 |
        (self.load8(addr + 2) as u64) << 16 |
        (self.load8(addr + 3) as u64) << 24 |
        (self.load8(addr + 4) as u64) << 32 |
        (self.load8(addr + 5) as u64) << 40 |
        (self.load8(addr + 6) as u64) << 48 |
        (self.load8(addr + 7) as u64) << 56
    }

    pub fn store8(&mut self, addr: BusSize, value: u8) {
        match addr {
            0x80000000..=BusSize::MAX => { self.dram[(addr - 0x80000000) as usize] = value; },
            _ => panic!("Bad memory access")
        }
    }

    pub fn store16(&mut self, addr: BusSize, value: u16) {
        self.store8(addr, value as u8);
        self.store8(addr, (value >> 8) as u8);
    }

    pub fn store32(&mut self, addr: BusSize, value: u32) {
        self.store8(addr, value as u8);
        self.store8(addr + 1, (value >> 8) as u8);
        self.store8(addr + 2, (value >> 16) as u8);
        self.store8(addr + 3, (value >> 24) as u8);
    }

    pub fn store64(&mut self, addr: BusSize, value: u64) {
        self.store8(addr, value as u8);
        self.store8(addr + 1, (value >> 8) as u8);
        self.store8(addr + 2, (value >> 16) as u8);
        self.store8(addr + 3, (value >> 24) as u8);
        self.store8(addr + 4, (value >> 32) as u8);
        self.store8(addr + 5, (value >> 40) as u8);
        self.store8(addr + 6, (value >> 48) as u8);
        self.store8(addr + 7, (value >> 56) as u8);
    }
}

impl std::fmt::Debug for Bus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Bus {{ DRAM[{:x}o] }}", self.dram.len())
    }
}