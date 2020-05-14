use crate::system::types::State;
use crate::system::dmac::types::*;
use crate::system::dmac::controllers::channel::*;
use crate::system::dmac::controllers::transfer::*;
use crate::types::memory::*;
use crate::system::dmac::constants::*;
use crate::utilities::bool_to_flag;

pub fn handle_chcr(state: &State, controller_state: &mut ControllerState, channel_id: usize) {
    let mut write_fn = |mut value| {
        if channel_id == 6 {
            let mut otc_value = 0;
            otc_value = CHCR_TRANSFER_DIRECTION.insert_into(otc_value, 0);
            otc_value = CHCR_MADR_STEP_DIRECTION.insert_into(otc_value, 1);
            otc_value = CHCR_STARTBUSY.copy(otc_value, value);
            otc_value = CHCR_STARTTRIGGER.copy(otc_value, value);
            otc_value = CHCR_BIT30.copy(otc_value, value);
            value = otc_value;
        }

        if CHCR_STARTTRIGGER.extract_from(value) > 0 {
            //log::warn!("Unhandled start/trigger bit set");
        }

        let transfer_state = get_transfer_state(controller_state, channel_id);

        if CHCR_STARTBUSY.extract_from(value) > 0 {
            assert!(!transfer_state.started, format!("DMA transfer already started, channel_id = {}", channel_id));
            transfer_state.started = true;
        } else {
            transfer_state.started = false;
        }

        transfer_state.direction = if CHCR_TRANSFER_DIRECTION.extract_from(value) > 0 {
            TransferDirection::ToChannel
        } else {
            TransferDirection::FromChannel
        };

        transfer_state.step_direction = if CHCR_MADR_STEP_DIRECTION.extract_from(value) > 0 {
            StepDirection::Backwards
        } else {
            StepDirection::Forwards
        };

        transfer_state.sync_mode = match CHCR_SYNCMODE.extract_from(value) {
            0 => SyncMode::Continuous(ContinuousState::new()),
            1 => SyncMode::Blocks(BlocksState::new()),
            2 => SyncMode::LinkedList(LinkedListState::new()),
            3 => panic!("Reserved sync mode"),
            _ => unreachable!("Invalid sync mode"),
        };

        if transfer_state.started {
            handle_transfer_initialization(state, transfer_state, channel_id);
        }

        value
    };

    get_chcr(state, channel_id).acknowledge(|value, latch_kind| {
        match latch_kind {
            LatchKind::Read => value,
            LatchKind::Write => write_fn(value),
        }
    });
}

pub fn handle_dicr(state: &State, controller_state: &mut ControllerState) {
    let mut write_fn = |value| {
        if DICR_IRQ_FORCE.extract_from(value) > 0 {
            unimplemented!("IRQ force bit set");
        }

        controller_state.master_interrupt_enabled = DICR_IRQ_MASTER_ENABLE.extract_from(value) > 0;

        for channel_id in 0..7 {
            let transfer_state = get_transfer_state(controller_state, channel_id);

            transfer_state.interrupt_enabled = DICR_IRQ_ENABLE_BITFIELDS[channel_id].extract_from(value) > 0;

            if DICR_IRQ_FLAG_BITFIELDS[channel_id].extract_from(value) > 0 {
                get_transfer_state(controller_state, channel_id).interrupted = false;
            }
        }

        calculate_dicr_value(controller_state)
    };

    state.dmac.dicr.acknowledge(|value, latch_kind| {
        match latch_kind {
            LatchKind::Read => value,
            LatchKind::Write => write_fn(value),
        }
    });
}

pub fn calculate_dicr_value(controller_state: &mut ControllerState) -> u32 {
    let mut value = 0;

    value = DICR_IRQ_MASTER_ENABLE.insert_into(value, bool_to_flag(controller_state.master_interrupt_enabled));

    for channel_id in 0..7 {
        let transfer_state = get_transfer_state(controller_state, channel_id);
        value = DICR_IRQ_ENABLE_BITFIELDS[channel_id].insert_into(value, bool_to_flag(transfer_state.interrupt_enabled));
        value = DICR_IRQ_FLAG_BITFIELDS[channel_id].insert_into(value, bool_to_flag(transfer_state.interrupted));
    }

    value = DICR_IRQ_MASTER_FLAG.insert_into(value, bool_to_flag(controller_state.master_interrupted));

    value
}