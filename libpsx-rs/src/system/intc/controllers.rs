pub mod debug;

use std::time::Duration;
use crate::system::types::ControllerContext;
use crate::system::types::State;
use crate::system::intc::constants::CLOCK_SPEED;
use crate::system::types::Event;
use crate::system::r3000::cp0::types::IrqLine;

pub fn run(context: &mut ControllerContext, event: Event) {
    match event {
        Event::Time(time) => run_time(context.state, time),
    }
}

fn run_time(state: &mut State, duration: Duration) {
    let ticks = (CLOCK_SPEED * duration.as_secs_f64()) as i64;

    for _ in 0..ticks {
        tick(state);
    }
}

fn tick(state: &mut State) {
    handle_interrupt_check(state);
}

fn handle_interrupt_check(state: &mut State) {
    let stat = &mut state.intc.stat;
    let mask = &mut state.intc.mask;

    let stat_value = stat.value();
    let mask_value = mask.read_u32();
    let masked_value = stat_value & mask_value;

    if masked_value == 0 {
        state.r3000.cp0.cause.deassert_line(IrqLine::Intc);
    } else {
        state.r3000.cp0.cause.assert_line(IrqLine::Intc);
    }
}
