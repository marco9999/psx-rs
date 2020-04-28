use crate::{
    backends::{
        audio::AudioBackend,
        cdrom::CdromBackend,
        video::VideoBackend,
    },
    system::{
        cdrom::types::State as CdromState,
        dmac::types::State as DmacState,
        gpu::types::State as GpuState,
        intc::types::State as IntcState,
        memory::types::State as MemoryState,
        padmc::types::State as PadmcState,
        r3000::types::{
            initialize as r3000_initialize,
            State as R3000State,
        },
        spu::types::State as SpuState,
        timers::types::State as TimersState,
    },
};
use log::info;
use std::{
    fs::File,
    io::Read,
    path::Path,
    sync::atomic::AtomicBool,
    time::Duration,
};

#[derive(Copy, Clone, Debug)]
pub enum Event {
    Time(Duration),
}

pub struct ControllerContext<'a: 'b, 'b: 'c, 'c> {
    pub state: &'c State,
    pub video_backend: &'c VideoBackend<'a, 'b>,
    pub audio_backend: &'c AudioBackend<'a, 'b>,
    pub cdrom_backend: &'c CdromBackend<'a, 'b>,
}

pub struct State {
    pub r3000: R3000State,
    pub intc: IntcState,
    pub dmac: DmacState,
    pub timers: TimersState,
    pub spu: SpuState,
    pub memory: MemoryState,
    pub gpu: GpuState,
    pub cdrom: CdromState,
    pub padmc: PadmcState,

    /// Bus lock status
    /// Needed in order to emulate the fact that the CPU is (almost) stopped when DMA transfers are happening.
    /// The CPU sometimes doesn't use interrupts to determine when to clear the ordering table etc, causing
    /// the DMA controller to read/write garbage if the CPU is allowed to continue to run.
    pub bus_locked: AtomicBool,
}

impl State {
    pub fn new() -> Box<State> {
        Box::new(State {
            r3000: R3000State::new(),
            intc: IntcState::new(),
            dmac: DmacState::new(),
            timers: TimersState::new(),
            spu: SpuState::new(),
            memory: MemoryState::new(),
            gpu: GpuState::new(),
            cdrom: CdromState::new(),
            padmc: PadmcState::new(),
            bus_locked: AtomicBool::new(false),
        })
    }

    pub fn initialize(state: &mut State) {
        r3000_initialize(state);
    }

    pub fn load_bios(state: &mut State, path: &Path) {
        info!("Loading BIOS from {}", path.to_str().unwrap());
        let mut file = File::open(path).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        state.memory.bios.write_raw(0, &buffer);
    }
}
