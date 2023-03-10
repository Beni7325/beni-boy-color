enum PpuState {
    Glitched,

}


pub struct Ppu {
    vram: Box<[u8; 0x2000]>,
    oam: Box<[u8; 0x00A0]>,

    wx: u8,
    wy: u8,
    ly: u8,
    lyc: u8,
    scx: u8,
    scy: u8,

    bgp: u8,
    obp0: u8,
    obp1: u8,

    lcdc: u8,
    stat: u8
}

impl Ppu {

    pub fn new() -> Ppu {
        Ppu {
            vram: vec![0; 0x2000].into_boxed_slice().try_into().expect("Array size mismatch!"),
            oam: vec![0; 0x00A0].into_boxed_slice().try_into().expect("Array size mismatch!"),
            wx: 0x00,
            wy: 0x00,
            ly: 0x00,
            lyc: 0x00,
            scx: 0x00,
            scy: 0x00,
            bgp: 0xFC,
            obp0: 0xFF, // Revise
            obp1: 0xFF, // Revise
            lcdc: 0x91,
            stat: 0x85
        }
    }

    pub fn read_vram(&self, addr: u16) -> u8 {
        self.vram[addr as usize]
    }

    pub fn write_vram(&mut self, addr: u16, data: u8) {
        self.vram[addr as usize] = data;
    }

    pub fn read_oam(&self, addr: u16) -> u8 {
        self.oam[addr as usize]
    }

    pub fn write_oam(&mut self, addr: u16, data: u8) {
        self.oam[addr as usize] = data;
    }

    pub fn tick(&mut self, m_cycles: u64, interrupt_flag: &mut u8) {

        for _ in 0..m_cycles {



        }

    }

}
