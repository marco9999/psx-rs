pub mod instruction;
pub mod memory_controller;
pub mod instruction_impl;
pub mod debug;

use std::time::Duration;
use std::fmt;
use log::debug;
use crate::State;
use crate::constants::r3000::{CLOCK_SPEED, INSTRUCTION_SIZE};
use crate::controllers::Event;
use crate::controllers::r3000::memory_controller::translate_address;
use crate::controllers::r3000::instruction::lookup as instruction_lookup;
use crate::types::mips1::instruction::Instruction;
use crate::utilities::mips1::status_push_exception;
use crate::resources::r3000::cp0::{STATUS_BEV, STATUS_IM, CAUSE_IP, CAUSE_BD, STATUS_IEC, CAUSE_EXCCODE, CAUSE_EXCCODE_INT, CAUSE_EXCCODE_SYSCALL};

#[derive(PartialEq)]
pub enum Hazard {
    BusLockedMemoryRead(u32),
    BusLockedMemoryWrite(u32),
    MemoryRead(u32),
    MemoryWrite(u32),
}

impl fmt::Display for Hazard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Hazard::BusLockedMemoryRead(a) => write!(f, "BusLockedMemoryRead(0x{:08X})", a),
            Hazard::BusLockedMemoryWrite(a) => write!(f, "BusLockedMemoryWrite(0x{:08X})", a),
            Hazard::MemoryRead(a) => write!(f, "MemoryRead(0x{:08X})", a),
            Hazard::MemoryWrite(a) => write!(f, "MemoryWrite(0x{:08X})", a),
        }
    }
}

pub type InstResult = Result<(), Hazard>;

pub fn run(state: &State, event: Event) {
    match event {
        Event::Time(duration) => run_time(state, duration),
    }
}

fn run_time(state: &State, duration: Duration) {
    let mut ticks = (CLOCK_SPEED * duration.as_secs_f64()) as i64;
    while ticks > 0 {
        ticks -= unsafe { tick(state) };
    }
}

unsafe fn tick(state: &State) -> i64 {
    let resources = &mut *state.resources;

    handle_interrupts(state);

    if let Some(target) = resources.r3000.branch_delay.advance() {
        resources.r3000.pc.write_u32(target);
    }

    let pc_va = resources.r3000.pc.read_u32();
    let pc_pa = translate_address(pc_va);

    let inst_value = resources.r3000.memory_mapper.read_u32(pc_pa).unwrap();   
    let inst = Instruction::new(inst_value);                

    resources.r3000.pc.write_u32(pc_va + INSTRUCTION_SIZE);

    let (fn_ptr, cycles) = instruction_lookup(inst).unwrap();

    debug::trace_state(state);

    let result = fn_ptr(state, inst);

    if result.is_err() {
        // "Pipeline" hazard, go back to previous state, instruction was not performed.
        resources.r3000.branch_delay.back();
        resources.r3000.pc.write_u32(pc_va);

        debug::trace_hazard(result.unwrap_err());
    }
    
    cycles as i64
} 

unsafe fn set_exception(state: &State, exccode: usize) {
    let resources = &mut *state.resources;
    let pc = &mut resources.r3000.pc;
    let cause = &mut resources.r3000.cp0.cause;
    let status = &mut resources.r3000.cp0.status;
    let mut pc_value = pc.read_u32();

    let _cp0_lock = resources.r3000.cp0.mutex.lock();

    if resources.r3000.branch_delay.branching() {
        cause.write_bitfield(CAUSE_BD, 1);
        pc_value -= INSTRUCTION_SIZE;
    }

    // Push IEc & KUc (stack).
    let status_value = status_push_exception(status.read_u32());
    status.write_u32(status_value);

    // Set ExcCode cause.
    cause.write_bitfield(CAUSE_EXCCODE, exccode as u32);

    // Set EPC address.
    let epc = &mut resources.r3000.cp0.epc;
    epc.write_u32(pc_value);
    
    // Figure out base exception vector address.
    let bev = status.read_bitfield(STATUS_BEV) != 0;
    let mut vector_offset = if bev {
        0xBF80_0100
    } else {
        0x8000_0000
    };

    // Figure out exception vector offset.
    match exccode {
        CAUSE_EXCCODE_INT | CAUSE_EXCCODE_SYSCALL => {
            // General exception vector.
            vector_offset += 0x80;
        },
        _ => {
            unimplemented!("Unimplemented exception type encountered")
        },
    }

    // Set PC to exception vector.
    pc.write_u32(vector_offset);
}

unsafe fn handle_interrupts(state: &State) {
    let resources = &mut *state.resources;
    
    if resources.r3000.cp0.status.read_bitfield(STATUS_IEC) == 0 {
        return;
    }

    let im = resources.r3000.cp0.status.read_bitfield(STATUS_IM);
    let ip = resources.r3000.cp0.cause.read_bitfield(CAUSE_IP);
    if (im & ip) > 0 {
        debug::trace_interrupt(state);
        set_exception(state, CAUSE_EXCCODE_INT);
    }
}
