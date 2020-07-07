use crate::{
    system::{
        gpu::types::*,
        types::State,
    },
    types::memory::*,
};

pub(crate) fn handle_gp1(state: &State, controller_state: &mut ControllerState) {
    state.gpu.gp1.acknowledge(|value, latch_kind| {
        match latch_kind {
            LatchKind::Read => unreachable!(),
            LatchKind::Write => {
                assert!(controller_state.gp1_command.is_none());
                controller_state.gp1_command = Some(value);
                value
            },
        }
    });
}