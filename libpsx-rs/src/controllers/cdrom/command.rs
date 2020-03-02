use std::sync::atomic::Ordering;
use crate::backends::cdrom::CdromBackend;
use crate::resources::Resources;
use crate::resources::cdrom::*;
use crate::controllers::cdrom::command_impl;
use crate::controllers::cdrom::libmirage;
use crate::controllers::cdrom::interrupt::*;

type LengthFn = fn(usize) -> usize;

type HandlerFn = fn(&mut Resources, &CdromBackend, usize) -> bool;

pub fn handle_command(resources: &mut Resources, cdrom_backend: &CdromBackend<'_>) {
    // Don't run anything until all previous interrupts have been acknowledged, otherwise new ones could be missed.
    {
        let int_flag = &resources.cdrom.int_flag;
        if int_flag.register.read_bitfield(INTERRUPT_FLAGS) != 0 {
            return;
        }
    }

    let irq_raised = handle_reading(resources, cdrom_backend);
    if irq_raised {
        return;
    }

    if resources.cdrom.command_index.is_none() {
        // Read a new command if available.
        {
            let status = &mut resources.cdrom.status;
            let command = &mut resources.cdrom.command;
            
            status.write_bitfield(STATUS_BUSYSTS, 0);

            if !command.write_latch.load(Ordering::Acquire) {
                return;
            }
    
            status.write_bitfield(STATUS_BUSYSTS, 1);
        }

        let command_value = {
            let command = &mut resources.cdrom.command;
            let value = command.register.read_u8();
            command.write_latch.store(false, Ordering::Release);
            value
        };

        resources.cdrom.command_index = Some(command_value);
        resources.cdrom.command_iteration = 0;
    }

    if resources.cdrom.command_index.is_some() {
        let command_index = resources.cdrom.command_index.unwrap();
        let command_iteration = resources.cdrom.command_iteration;

        let handler = get_handler_fn(command_index);

        let parameter_count = resources.cdrom.parameter.read_available();
        if parameter_count < (handler.0)(command_iteration) {
            return;
        }

        assert!(resources.cdrom.response.read_available() == 0, "CDROM response FIFO still had bytes when a new command was run!");

        let finished = (handler.1)(resources, cdrom_backend, command_iteration);

        if !finished {
            resources.cdrom.command_iteration += 1;
        } else {
            resources.cdrom.command_index = None;
        }

        assert!(resources.cdrom.parameter.read_available() == 0, "CDROM parameter FIFO still had bytes when a command was just run!");
    }
}

fn handle_reading(resources: &mut Resources, cdrom_backend: &CdromBackend<'_>) -> bool {
    let reading = resources.cdrom.reading;
    if !reading {
        return false;
    }

    let response = &mut resources.cdrom.response;

    // Let the BIOS read a bit of data before filling the FIFO up again.
    if response.write_available() < 12 {
        return false;
    }

    response.write_one(0b0010_0010).unwrap(); // Motor on | Reading

    let read_buffer = &mut resources.cdrom.read_buffer;

    if read_buffer.is_empty() {
        let data_block = match cdrom_backend {
            CdromBackend::None => panic!(),
            CdromBackend::Libmirage(ref params) => libmirage::read_sector(params, resources.cdrom.lba_address),
        };

        resources.cdrom.lba_address += 1;
        read_buffer.extend(&data_block);
    }

    loop {
        if response.is_full() {
            break;
        }

        match read_buffer.pop_front() {
            Some(v) => response.write_one(v).unwrap(),
            None => break,
        }
    }

    raise_irq(resources, 1);
    true
}

fn get_handler_fn(command_index: u8) -> (LengthFn, HandlerFn) {
    match command_index {
        0x01 => (command_impl::command_01_length, command_impl::command_01_handler),
        0x02 => (command_impl::command_02_length, command_impl::command_02_handler),
        0x06 => (command_impl::command_06_length, command_impl::command_06_handler),
        0x0E => (command_impl::command_0e_length, command_impl::command_0e_handler),
        0x15 => (command_impl::command_15_length, command_impl::command_15_handler),
        0x19 => (command_impl::command_19_length, command_impl::command_19_handler),
        0x1A => (command_impl::command_1a_length, command_impl::command_1a_handler),
        _ => unimplemented!("Command not implemented: 0x{:0X}", command_index),
    }
}
