pub(crate) mod debug;

use crate::system::{
    intc::constants::*,
    r3000::cp0::types::IrqLine,
    types::{
        ControllerContext,
        Event,
        State,
    },
};

pub(crate) fn run(context: &ControllerContext, event: Event) {
    match event {
        Event::Time(time) => run_time(context.state, time),
    }
}

fn run_time(state: &State, duration: f64) {
    let controller_state = &mut state.intc.controller_state.lock();
    controller_state.clock += duration;

    while controller_state.clock > 0.0 {
        tick(state);
        controller_state.clock -= CLOCK_SPEED_PERIOD;
    }
}

fn tick(state: &State) {
    handle_interrupt_check(state);
}

fn handle_interrupt_check(state: &State) {
    let stat = &state.intc.stat;
    let mask = &state.intc.mask;

    let stat_value = stat.value();
    let mask_value = mask.read_u32();
    let masked_value = stat_value & mask_value;

    if masked_value == 0 {
        state.r3000.cp0.interrupt.deassert_line(IrqLine::Intc);
    } else {
        state.r3000.cp0.interrupt.assert_line(IrqLine::Intc);
    }
}
