use crate::backends::cdrom::CdromBackend;
use crate::resources::Resources;
use crate::constants::cdrom::*;
use crate::controllers::cdrom::libmirage;
use crate::controllers::cdrom::interrupt::*;

pub fn command_01(resources: &mut Resources, _cdrom_backend: &CdromBackend<'_>, command_iteration: usize) -> bool {
    // GetStat
    
    assert_eq!(command_iteration, 0);

    let response = &mut resources.cdrom.response;

    // No CD inserted.
    // match command_iteration {
    //     0 => {
    //         response.write_one(0x1).unwrap();
    //         int_flag.set_interrupt(3);
    //         false
    //     },
    //     1 => {
    //         response.write_one(0x80).unwrap();
    //         int_flag.set_interrupt(5);
    //         true
    //     },
    //     _ => panic!(),
    // }

    response.write_one(0b0000_0010).unwrap(); // Motor on
    raise_irq(resources, 3);
    true
}

pub fn command_02(resources: &mut Resources, cdrom_backend: &CdromBackend<'_>, command_iteration: usize) -> bool {
    // Setloc

    // TODO: Assumed to be absolute addressing?
    
    assert_eq!(command_iteration, 0);
    let parameter = &resources.cdrom.parameter;
    let response = &mut resources.cdrom.response;

    let minute = parameter.read_one().unwrap();
    let second = parameter.read_one().unwrap();
    let frame = parameter.read_one().unwrap();

    let lba_address = match cdrom_backend {
        CdromBackend::None => panic!(),
        CdromBackend::Libmirage(ref params) => libmirage::msf_to_lba_address(params, minute, second, frame),
    };

    resources.cdrom.lba_address = lba_address;

    response.write_one(0b0000_0010).unwrap(); // Motor on
    raise_irq(resources, 3);
    true
}

pub fn command_06(resources: &mut Resources, cdrom_backend: &CdromBackend<'_>, command_iteration: usize) -> bool {
    // ReadN

    match command_iteration {
        0 => {
            let response = &mut resources.cdrom.response;
            response.write_one(0b0010_0010).unwrap(); // Motor on | Reading
            raise_irq(resources, 3);
            false
        },
        _ => {
            let response = &mut resources.cdrom.response;
            response.write_one(0b0010_0010).unwrap(); // Motor on | Reading

            resources.cdrom.lba_address;

            let data_block = match cdrom_backend {
                CdromBackend::None => panic!(),
                CdromBackend::Libmirage(ref params) => libmirage::read_sector(params, resources.cdrom.lba_address),
            };

            for data in data_block.iter() {
                // TODO: completely wrong! :D
                match response.write_one(*data) {
                    Ok(()) => {},
                    Err(()) => { break },
                }
            }  

            resources.cdrom.lba_address += 1;

            raise_irq(resources, 1);

            false
        },
    }
}

pub fn command_0e(resources: &mut Resources, _cdrom_backend: &CdromBackend<'_>, command_iteration: usize) -> bool {
    // Setmode
    
    assert_eq!(command_iteration, 0);
    let parameter = &resources.cdrom.parameter;
    let response = &mut resources.cdrom.response;

    let _mode = parameter.read_one().unwrap();

    response.write_one(0b0000_0010).unwrap(); // Motor on
    raise_irq(resources, 3);
    true
}

pub fn command_15(resources: &mut Resources, _cdrom_backend: &CdromBackend<'_>, command_iteration: usize) -> bool {
    // SeekL
    
    match command_iteration {
        0 => {
            let response = &mut resources.cdrom.response;
            response.write_one(0b0100_0010).unwrap(); // Motor on | Seek
            raise_irq(resources, 3);
            false
        },
        1 => {
            let response = &mut resources.cdrom.response;
            response.write_one(0b0000_0010).unwrap(); // Motor on
            raise_irq(resources, 2);
            true
        },
        _ => panic!(),
    }
}

pub fn command_19(resources: &mut Resources, _cdrom_backend: &CdromBackend<'_>, command_iteration: usize) -> bool {
    // Test

    assert_eq!(command_iteration, 0);

    let parameter = &resources.cdrom.parameter;
    let response = &resources.cdrom.response;

    let sub_function = parameter.read_one().unwrap();

    match sub_function {
        0x20 => {
            for &i in VERSION.iter() {
                response.write_one(i).unwrap();
            }
        },
        _ => unimplemented!(),
    }

    raise_irq(resources, 3);

    true
}

pub fn command_1a(resources: &mut Resources, cdrom_backend: &CdromBackend<'_>, command_iteration: usize) -> bool {
    // GetID

    match command_iteration {
        0 => {
            let response = &resources.cdrom.response;
            response.write_one(0b0000_0010).unwrap(); // Motor on
            raise_irq(resources, 3);
            false
        },
        1 => {
            let response = &resources.cdrom.response;

            // Determine disc mode type.
            let mode = match cdrom_backend {
                CdromBackend::None => panic!(),
                CdromBackend::Libmirage(ref params) => libmirage::disc_mode(params),
            };

            match mode {
                2 => {
                    response.write_one(0x02).unwrap(); 
                    response.write_one(0x00).unwrap(); 
                    response.write_one(0x20).unwrap(); 
                    response.write_one(0x00).unwrap(); 
                    response.write_one(0x53).unwrap(); 
                    response.write_one(0x43).unwrap(); 
                    response.write_one(0x45).unwrap(); 
                    response.write_one(0x41).unwrap(); // SCEx: ASCII A = 0x41, E = 0x45, I = 0x49 
                },
                _ => unimplemented!("Disc mode {} not handled", mode),
            }
            
            raise_irq(resources, 2);
            true
        },
        _ => panic!(),
    }
}
