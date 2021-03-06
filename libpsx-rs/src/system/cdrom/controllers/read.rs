use crate::{
    backends::cdrom::CdromBackend,
    system::{
        cdrom::{
            constants::*,
            controllers::{
                backend_dispatch,
                interrupt::*,
                state::*,
            },
            types::ControllerState,
        },
        types::{
            ControllerResult,
            State,
        },
    },
    utilities::binary_to_ascii_escaped,
};

pub(crate) fn handle_read(state: &State, controller_state: &mut ControllerState, cdrom_backend: &CdromBackend) -> ControllerResult<()> {
    if controller_state.sector_buffer.len() > 0 {
        if controller_state.loading_data {
            fill_data_fifo(state, controller_state);

            if controller_state.sector_buffer.len() == 0 {
                controller_state.loading_data = false;
            }
        } else {
            if controller_state.load_data_flag {
                controller_state.loading_data = true;
                controller_state.load_data_flag = false;
            }
        }
    } else {
        if !controller_state.reading {
            return Ok(());
        }

        if controller_state.sector_delay_counter > 0 {
            controller_state.sector_delay_counter -= 1;
            return Ok(());
        }

        if !state.cdrom.data.is_empty() {
            log::warn!("Data FIFO was not empty before reading a sector... trying again later");
            return Ok(());
        }

        read_sector(controller_state, cdrom_backend)?;
        controller_state.sector_delay_counter = SECTOR_DELAY_CYCLES_SINGLE_SPEED;
        let stat_value = calculate_stat_value(controller_state);
        state.cdrom.response.write_one(stat_value).map_err(|_| "Couldn't write to the response FIFO".to_owned())?;
        handle_irq_raise(state, controller_state, 1)?;
    }

    Ok(())
}

fn fill_data_fifo(state: &State, controller_state: &mut ControllerState) {
    loop {
        if state.cdrom.data.is_full() {
            break;
        }

        match controller_state.sector_buffer.pop_front() {
            Some(v) => state.cdrom.data.write_one(v).unwrap(),
            None => break,
        }
    }
}

fn read_sector(controller_state: &mut ControllerState, cdrom_backend: &CdromBackend) -> ControllerResult<()> {
    assert_eq!(controller_state.sector_buffer.len(), 0);
    let msf_address_base = controller_state.msf_address_base;
    let msf_address_offset = controller_state.msf_address_offset;
    let data_block = backend_dispatch::read_sector(cdrom_backend, msf_address_base, msf_address_offset)?.map_err(|_| "No backend available for reading sector".to_owned())?;
    assert_eq!(data_block.len(), 2048);
    controller_state.msf_address_offset += 1;
    controller_state.sector_buffer.extend(&data_block);

    // log::debug!("Sector {:?} + offset {} read ok", msf_address_base, msf_address_offset);

    if false {
        log::debug!("{}", &binary_to_ascii_escaped(&data_block));
    }

    Ok(())
}
