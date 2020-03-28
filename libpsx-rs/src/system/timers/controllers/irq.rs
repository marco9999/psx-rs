use log::warn;
use log::debug;
use crate::system::Resources;
use crate::system::timers::*;
use crate::controllers::timers::timer::*;

pub fn handle_irq_trigger(resources: &mut Resources, timer_id: usize, irq_type: IrqType) {
    let mode = get_mode(resources, timer_id);
    let state = get_state(resources, timer_id);

    // First check if we are in one-shot mode, don't raise an IRQ if we have already done so.
    let oneshot = mode.register.read_bitfield(MODE_IRQ_REPEAT) > 0;
    if oneshot {
        if state.irq_raised {
            return;
        }
    }

    match irq_type {
        IrqType::None => {},
        IrqType::Overflow => {
            let overflow_trigger = mode.register.read_bitfield(MODE_IRQ_OVERFLOW) > 0;
            
            if overflow_trigger {
                handle_irq_raise(resources, timer_id);
                state.irq_raised = true;
            }
        },
        IrqType::Target => {
            let target_trigger = mode.register.read_bitfield(MODE_IRQ_TARGET) > 0;
            
            if target_trigger {
                handle_irq_raise(resources, timer_id);
                state.irq_raised = true;
            }
        },
    }
}

pub fn handle_irq_raise(resources: &mut Resources, timer_id: usize) {
    let mode = get_mode(resources, timer_id);

    let mut raise_irq = false;

    match mode.register.read_bitfield(MODE_IRQ_PULSE) {
        0 => {
            // Pulse mode.
            // TODO: Do nothing? How long is a few clock cycles? Will the BIOS see this? Probably not...
            warn!("Pulse IRQ mode not implemented properly?");
            raise_irq = true;
        }, 
        1 => {
            // Toggle mode. IRQ's will effectively only be raised every 2nd time.
            let new_irq_status = mode.register.read_bitfield(MODE_IRQ_STATUS) ^ 1;
            mode.register.write_bitfield(MODE_IRQ_STATUS, new_irq_status);

            if new_irq_status == 0 {
                raise_irq = true;
            }
        },
        _ => unreachable!(),
    }

    if raise_irq {
        use crate::system::intc::register::Line;

        let irq_line = match timer_id {
            0 => Line::Tmr0,
            1 => Line::Tmr1,
            2 => Line::Tmr2,
            _ => unreachable!(),
        };
    
        let stat = &resources.intc.stat;
        stat.assert_line(irq_line);
    
        debug!("Raised INTC IRQ for timer {}", timer_id);
    }
}