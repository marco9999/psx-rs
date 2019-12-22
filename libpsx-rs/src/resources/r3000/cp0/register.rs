use std::sync::atomic::{AtomicBool, Ordering};
use crate::utilities::bool_to_flag;
use crate::types::register::b32_register::B32Register;
use crate::resources::r3000::cp0::*;

#[derive(Copy, Clone, Debug)]
pub enum IrqLine {
    Intc,
}

pub struct Cause {
    pub register: B32Register,
    intc_pending: AtomicBool,
}

impl Cause {
    pub fn new() -> Cause {
        Cause {
            register: B32Register::new(),
            intc_pending: AtomicBool::new(false),
        }
    }

    pub fn assert_line(&self, irq_line: IrqLine) {
        match irq_line {
            IrqLine::Intc => self.intc_pending.store(true, Ordering::Release),
        }
    }

    pub fn deassert_line(&self, irq_line: IrqLine) {
        match irq_line {
            IrqLine::Intc => self.intc_pending.store(false, Ordering::Release),
        }
    }

    pub fn update_ip_field(&mut self) {
        self.register.write_bitfield(CAUSE_IP_INTC, bool_to_flag(self.intc_pending.load(Ordering::Acquire)));
    }

    pub fn clear_ip_field(&mut self) {
        self.intc_pending.store(false, Ordering::Release);
        self.register.write_bitfield(CAUSE_IP, 0);
    }
}
