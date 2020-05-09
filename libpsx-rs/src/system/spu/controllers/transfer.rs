use crate::system::{
    spu::{
        types::*,
        constants::*,
    },
    types::State,
};

pub fn handle_transfer(state: &State, controller_state: &mut ControllerState) {
    match controller_state.transfer_state.current_transfer_mode {
        TransferMode::Stop => {},
        TransferMode::ManualWrite => handle_manual_write_transfer(state, controller_state),
        TransferMode::DmaWrite => unimplemented!("DmaWrite transfer mode not implemented"),
        TransferMode::DmaRead => unimplemented!("DmaRead transfer mode not implemented"),
    }
}

fn handle_manual_write_transfer(state: &State, controller_state: &mut ControllerState) {
    if state.spu.data_transfer_control.read_u16() != 0x4 {
        unimplemented!("Data transfer control not set to normal mode");
    }

    let fifo = &state.spu.data_fifo;
    let memory = &mut controller_state.memory;
    let current_transfer_mode = &mut controller_state.transfer_state.current_transfer_mode;
    let current_transfer_address = &mut controller_state.transfer_state.current_transfer_address;

    match fifo.read_one() {
        Ok(value) => {
            let bytes = u16::to_le_bytes(value);
            memory[*current_transfer_address as usize] = bytes[0];
            memory[*current_transfer_address as usize + 1] = bytes[1];
            *current_transfer_address += 2;
            *current_transfer_address &= 0x7FFFF;
        },
        Err(_) => {
            *current_transfer_mode = TransferMode::Stop;
            state.spu.stat.write_bitfield(STAT_DATA_BUSY_FLAG, 0);
        },
    }
}
