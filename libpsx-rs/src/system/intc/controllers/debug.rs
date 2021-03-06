use crate::system::{
    intc::constants::*,
    types::State,
};
use log::trace;

pub(crate) fn trace_intc(state: &State, only_enabled: bool, enable_assert: bool) {
    let stat = state.intc.stat.read_u32();
    let mask = state.intc.mask.read_u32();
    let mut pending_sticky = false;
    for (name, bitfield) in IRQ_NAMES.iter().zip(IRQ_BITFIELDS.iter()) {
        let stat_value = bitfield.extract_from(stat) != 0;
        let mask_value = bitfield.extract_from(mask) != 0;
        let pending = stat_value && mask_value;
        pending_sticky |= pending;

        if only_enabled && !mask_value {
            continue;
        }

        trace!("INTC [{}]: stat = {}, mask = {} (pending = {})", name, stat_value, mask_value, pending);
    }

    if enable_assert {
        assert!(pending_sticky, "No pending interrupts");
    }
}
