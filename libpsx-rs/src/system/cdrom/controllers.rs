pub mod backend_dispatch;
pub mod command;
pub mod command_impl;
pub mod debug;
pub mod interrupt;
pub mod read;
pub mod state;

use crate::{
    backends::cdrom::CdromBackend,
    system::{
        cdrom::{
            constants::*,
            controllers::{
                command::*,
                read::*,
            },
            types::ControllerState,
        },
        types::State,
    },
    utilities::bool_to_flag,
};
use log::warn;
use std::sync::atomic::Ordering;

pub fn handle_tick(state: &State, cdrom_state: &mut ControllerState, cdrom_backend: &CdromBackend) {
    handle_interrupt_enable(state);
    handle_interrupt_flags(state);
    handle_request(state);

    handle_state(state, cdrom_state, cdrom_backend);

    handle_parameter_fifo(state);
    handle_response_fifo(state);
    handle_data_fifo(state);
}

fn handle_state(state: &State, cdrom_state: &mut ControllerState, cdrom_backend: &CdromBackend) {
    // Don't run anything until all previous interrupts have been acknowledged, otherwise new ones could be missed.
    {
        let int_flag = &state.cdrom.int_flag;
        if int_flag.register.read_bitfield(INTERRUPT_FLAGS) != 0 {
            return;
        }
    }

    // Can only do one action per cycle.
    // Commands get priority over anything else.
    let mut handled = false;

    if !handled {
        handled = handle_command(state, cdrom_state, cdrom_backend);
    }

    if !handled {
        handled = handle_read(state, cdrom_state, cdrom_backend);
    }

    if !handled {
        // ...
    }
}

fn handle_request(state: &State) {
    let request = &state.cdrom.request;

    if request.write_latch.load(Ordering::Acquire) {
        assert!(request.register.read_bitfield(REQUEST_SMEN) == 0);
        assert!(request.register.read_bitfield(REQUEST_BFWR) == 0);

        let reset_data_fifo = request.register.read_bitfield(REQUEST_BFRD) == 0;
        if reset_data_fifo {
            let count = state.cdrom.data.read_available();
            // state.cdrom.data.clear();
            warn!("Ignored Reset CDROM data FIFO (has {} bytes)", count);
        }

        request.write_latch.store(false, Ordering::Release);
    }
}

fn handle_interrupt_enable(state: &State) {
    let int_enable = &state.cdrom.int_enable;

    if int_enable.write_latch.load(Ordering::Acquire) {
        int_enable.write_latch.store(false, Ordering::Release);
    }
}

fn handle_interrupt_flags(state: &State) {
    let int_flag = &state.cdrom.int_flag;

    if int_flag.write_latch.load(Ordering::Acquire) {
        state.cdrom.response.clear();
        int_flag.write_latch.store(false, Ordering::Release);
    }

    if int_flag.parameter_reset.load(Ordering::Acquire) {
        // TODO: actually performing a reset causes problems, where the BIOS is writing the clear bit and the parameters
        // at the same time, before the controller gets a chance to run - this is an emulator level issue. There
        // are asserts in the command handler that check if the parameter is empty after a command has been run
        // (which it should be). state.cdrom.parameter.clear();
        int_flag.parameter_reset.store(false, Ordering::Release);
    }
}

fn handle_parameter_fifo(state: &State) {
    let status = &state.cdrom.status;
    let fifo = &state.cdrom.parameter;

    let empty_bit = bool_to_flag(fifo.is_empty()) as u8;
    status.write_bitfield(STATUS_PRMEMPT, empty_bit);

    let ready_bit = bool_to_flag(!fifo.is_full()) as u8;
    status.write_bitfield(STATUS_PRMWRDY, ready_bit);
}

fn handle_response_fifo(state: &State) {
    let status = &state.cdrom.status;
    let fifo = &state.cdrom.response;

    let ready_bit = bool_to_flag(!fifo.is_empty()) as u8;
    status.write_bitfield(STATUS_RSLRRDY, ready_bit);
}

fn handle_data_fifo(state: &State) {
    let status = &state.cdrom.status;
    let fifo = &state.cdrom.response;

    let empty_bit = bool_to_flag(!fifo.is_empty()) as u8;
    status.write_bitfield(STATUS_DRQSTS, empty_bit);
}
