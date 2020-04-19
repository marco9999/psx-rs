use crate::{
    system::types::State as SystemState,
    types::memory::b8_memory::B8Memory,
};

#[derive(Clone, Copy, Debug)]
pub enum ReadErrorKind {
    Empty,
}

pub type ReadResult<T> = Result<T, ReadErrorKind>;

#[derive(Clone, Copy, Debug)]
pub enum WriteErrorKind {
    Full,
}

pub type WriteResult = Result<(), WriteErrorKind>;

pub struct State {
    pub expansion_1_base_address: B32Register,
    pub expansion_2_base_address: B32Register,
    pub expansion_1_delay: B32Register,
    pub expansion_3_delay: B32Register,
    pub bios_rom_control: B32Register,
    pub spu_delay: B32Register,
    pub cdrom_delay: B32Register,
    pub expansion_2_delay: B32Register,
    pub common_delay_control: B32Register,
    pub ram_size_control: B32Register,
    pub cache_control: B8Memory,
}

impl State {
    pub fn new() -> State {
        State {
            expansion_1_base_address: B32Register::new(),
            expansion_2_base_address: B32Register::new(),
            expansion_1_delay: B32Register::new(),
            expansion_3_delay: B32Register::new(),
            bios_rom_control: B32Register::new(),
            spu_delay: B32Register::new(),
            cdrom_delay: B32Register::new(),
            expansion_2_delay: B32Register::new(),
            common_delay_control: B32Register::new(),
            ram_size_control: B32Register::new(),
            cache_control: B8Memory::new(0x2_0000),
        }
    }
}

pub fn initialize(state: &mut SystemState) {
}
