pub mod channel;
pub mod debug;
pub mod linked_list;
pub mod transfer;

use std::time::Duration;
use std::sync::atomic::Ordering;
use std::cmp::min;
use log::debug;
use crate::controllers::ControllerState;
use crate::system::Resources;
use crate::constants::dmac::*;
use crate::controllers::Event;
use crate::controllers::dmac::channel::*;
use crate::controllers::dmac::transfer::*;
use crate::system::dmac::*;

pub fn run(state: &mut ControllerState, event: Event) {
    match event {
        Event::Time(time) => run_time(state.resources, time),
    }
}

fn run_time(resources: &mut Resources, duration: Duration) {
    // TODO: Properly obey priorities of channels - usually its DMA6 -> DMA0, so just do that for now.

    // Don't run if the CPU needs to use the bus.
    if resources.dmac.cooloff_runs > 0 {
        resources.bus_locked.store(false, Ordering::Release);
        resources.dmac.cooloff_runs -= 1;
        return;
    }

    handle_bus_lock(resources);

    let mut ticks = (CLOCK_SPEED * duration.as_secs_f64()) as i64;
    let mut channel_id: usize = 6;
    let mut cooloff = false;
    while ticks > 0 {
        match tick(resources, channel_id, ticks as usize) {
            Ok(channel_ticks) => {
                if channel_ticks == 0 {
                    ticks -= 16;
        
                    if channel_id == 0 {
                        channel_id = 6;
                    } else {
                        channel_id -= 1;
                    }
                } else {
                    ticks -= channel_ticks as i64;
                }
            },
            Err(channel_ticks) => {
                if channel_ticks == 0 {
                    cooloff = true;
                    break;
                } else {
                    ticks -= channel_ticks as i64;
                }
            },
        }
    }

    if cooloff {
        resources.dmac.cooloff_runs = 4;
    }
    
    handle_bus_unlock(resources);

    handle_irq_check(resources);
}

fn tick(resources: &mut Resources, channel_id: usize, ticks_remaining: usize) -> Result<usize, usize> {
    // Number of ticks per word transfer.
    const TICK_WORD_RATIO: usize = 2;

    let dpcr = &resources.dmac.dpcr;
    let enable = DPCR_CHANNEL_ENABLE_BITFIELDS[channel_id];

    // Round up to nearset alignment for no remainder.
    let mut word_transfers_allowed = (ticks_remaining + (TICK_WORD_RATIO - 1)) / TICK_WORD_RATIO;

    // Cap it to a maximum.
    word_transfers_allowed = min(word_transfers_allowed, 16);

    let word_transfers_actual = if dpcr.read_bitfield(enable) != 0 {
        handle_transfer(resources, channel_id, word_transfers_allowed)
    } else {
        Ok(0)
    };

    word_transfers_actual.map(|v| v * TICK_WORD_RATIO).map_err(|v| v * TICK_WORD_RATIO)
}

/// Check if any channels are in progress, and acquires the bus lock if true.
fn handle_bus_lock(resources: &mut Resources) {
    for channel_id in 0..6 {
        let transfer_state = get_transfer_state(resources, channel_id);
        
        if transfer_state.started {
            resources.bus_locked.store(true, Ordering::Release);
            return;
        }
    }
}

/// Check if all channels are finished, and release the bus lock if true.
fn handle_bus_unlock(resources: &mut Resources) {
    for channel_id in 0..6 {
        let transfer_state = get_transfer_state(resources, channel_id);
        
        if transfer_state.started {
            return;
        }
    }

    resources.bus_locked.store(false, Ordering::Release);
}

/// Performs interrupt check for raising an IRQ on the INTC.
fn handle_irq_check(resources: &mut Resources) {
    let dicr = &mut resources.dmac.dicr;
    let _icr_lock = dicr.mutex.lock();

    let force_irq = dicr.register.read_bitfield(DICR_IRQ_FORCE) != 0;
    
    let mut channel_irq = false;
    let irq_channel_enable = dicr.register.read_bitfield(DICR_IRQ_MASTER_ENABLE) != 0;
    if irq_channel_enable {
        for (&enable, &flag) in DICR_IRQ_ENABLE_BITFIELDS.iter().zip(DICR_IRQ_FLAG_BITFIELDS.iter()) {
            let enable_value = dicr.register.read_bitfield(enable) != 0;
            let flag_value = dicr.register.read_bitfield(flag) != 0;
            if enable_value && flag_value {
                channel_irq = true;
            }
        }
    }

    if force_irq || channel_irq {
        if dicr.register.read_bitfield(DICR_IRQ_MASTER_FLAG) == 0 {
            dicr.register.write_bitfield(DICR_IRQ_MASTER_FLAG, 1);

            use crate::system::intc::register::Line;
            let stat = &resources.intc.stat;
            stat.assert_line(Line::Dma);
        }
    } else {
        dicr.register.write_bitfield(DICR_IRQ_MASTER_FLAG, 0);
    }
}