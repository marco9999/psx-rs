use crate::system::Resources;
use crate::types::register::b16_register::B16Register;
use crate::types::register::b32_register::B32Register;
use crate::system::spu::*;
use crate::system::spu::register::*;
use crate::system::spu::voice::*;

pub fn get_transfer_mode(control: &B16Register) -> TransferMode {
    match control.read_bitfield(CONTROL_TRANSFER_MODE) {
        0 => TransferMode::Stop,
        1 => TransferMode::ManualWrite,
        2 => TransferMode::DmaWrite,
        3 => TransferMode::DmaRead,
        _ => unreachable!("Invalid transfer mode"),
    }
}

pub fn get_voll(resources: &mut Resources, voice_id: usize) -> *mut B16Register {
    match voice_id {
        0 => &mut resources.spu.voice0_voll as *mut B16Register,
        1 => &mut resources.spu.voice1_voll as *mut B16Register,
        2 => &mut resources.spu.voice2_voll as *mut B16Register,
        3 => &mut resources.spu.voice3_voll as *mut B16Register,
        4 => &mut resources.spu.voice4_voll as *mut B16Register,
        5 => &mut resources.spu.voice5_voll as *mut B16Register,
        6 => &mut resources.spu.voice6_voll as *mut B16Register,
        7 => &mut resources.spu.voice7_voll as *mut B16Register,
        8 => &mut resources.spu.voice8_voll as *mut B16Register,
        9 => &mut resources.spu.voice9_voll as *mut B16Register,
        10 => &mut resources.spu.voice10_voll as *mut B16Register,
        11 => &mut resources.spu.voice11_voll as *mut B16Register,
        12 => &mut resources.spu.voice12_voll as *mut B16Register,
        13 => &mut resources.spu.voice13_voll as *mut B16Register,
        14 => &mut resources.spu.voice14_voll as *mut B16Register,
        15 => &mut resources.spu.voice15_voll as *mut B16Register,
        16 => &mut resources.spu.voice16_voll as *mut B16Register,
        17 => &mut resources.spu.voice17_voll as *mut B16Register,
        18 => &mut resources.spu.voice18_voll as *mut B16Register,
        19 => &mut resources.spu.voice19_voll as *mut B16Register,
        20 => &mut resources.spu.voice20_voll as *mut B16Register,
        21 => &mut resources.spu.voice21_voll as *mut B16Register,
        22 => &mut resources.spu.voice22_voll as *mut B16Register,
        23 => &mut resources.spu.voice23_voll as *mut B16Register,
        _ => unreachable!("Invalid voice id"),
    }
}

pub fn get_volr(resources: &mut Resources, voice_id: usize) -> *mut B16Register {
    match voice_id {
        0 => &mut resources.spu.voice0_volr as *mut B16Register,
        1 => &mut resources.spu.voice1_volr as *mut B16Register,
        2 => &mut resources.spu.voice2_volr as *mut B16Register,
        3 => &mut resources.spu.voice3_volr as *mut B16Register,
        4 => &mut resources.spu.voice4_volr as *mut B16Register,
        5 => &mut resources.spu.voice5_volr as *mut B16Register,
        6 => &mut resources.spu.voice6_volr as *mut B16Register,
        7 => &mut resources.spu.voice7_volr as *mut B16Register,
        8 => &mut resources.spu.voice8_volr as *mut B16Register,
        9 => &mut resources.spu.voice9_volr as *mut B16Register,
        10 => &mut resources.spu.voice10_volr as *mut B16Register,
        11 => &mut resources.spu.voice11_volr as *mut B16Register,
        12 => &mut resources.spu.voice12_volr as *mut B16Register,
        13 => &mut resources.spu.voice13_volr as *mut B16Register,
        14 => &mut resources.spu.voice14_volr as *mut B16Register,
        15 => &mut resources.spu.voice15_volr as *mut B16Register,
        16 => &mut resources.spu.voice16_volr as *mut B16Register,
        17 => &mut resources.spu.voice17_volr as *mut B16Register,
        18 => &mut resources.spu.voice18_volr as *mut B16Register,
        19 => &mut resources.spu.voice19_volr as *mut B16Register,
        20 => &mut resources.spu.voice20_volr as *mut B16Register,
        21 => &mut resources.spu.voice21_volr as *mut B16Register,
        22 => &mut resources.spu.voice22_volr as *mut B16Register,
        23 => &mut resources.spu.voice23_volr as *mut B16Register,
        _ => unreachable!("Invalid voice id"),
    }
}

pub fn get_adpcm_sr(resources: &mut Resources, voice_id: usize) -> *mut B16Register {
    match voice_id {
        0 => &mut resources.spu.voice0_srate as *mut B16Register,
        1 => &mut resources.spu.voice1_srate as *mut B16Register,
        2 => &mut resources.spu.voice2_srate as *mut B16Register,
        3 => &mut resources.spu.voice3_srate as *mut B16Register,
        4 => &mut resources.spu.voice4_srate as *mut B16Register,
        5 => &mut resources.spu.voice5_srate as *mut B16Register,
        6 => &mut resources.spu.voice6_srate as *mut B16Register,
        7 => &mut resources.spu.voice7_srate as *mut B16Register,
        8 => &mut resources.spu.voice8_srate as *mut B16Register,
        9 => &mut resources.spu.voice9_srate as *mut B16Register,
        10 => &mut resources.spu.voice10_srate as *mut B16Register,
        11 => &mut resources.spu.voice11_srate as *mut B16Register,
        12 => &mut resources.spu.voice12_srate as *mut B16Register,
        13 => &mut resources.spu.voice13_srate as *mut B16Register,
        14 => &mut resources.spu.voice14_srate as *mut B16Register,
        15 => &mut resources.spu.voice15_srate as *mut B16Register,
        16 => &mut resources.spu.voice16_srate as *mut B16Register,
        17 => &mut resources.spu.voice17_srate as *mut B16Register,
        18 => &mut resources.spu.voice18_srate as *mut B16Register,
        19 => &mut resources.spu.voice19_srate as *mut B16Register,
        20 => &mut resources.spu.voice20_srate as *mut B16Register,
        21 => &mut resources.spu.voice21_srate as *mut B16Register,
        22 => &mut resources.spu.voice22_srate as *mut B16Register,
        23 => &mut resources.spu.voice23_srate as *mut B16Register,
        _ => unreachable!("Invalid voice id"),
    }
}

pub fn get_adpcm_sa(resources: &mut Resources, voice_id: usize) -> *mut B16Register {
    match voice_id {
        0 => &mut resources.spu.voice0_saddr as *mut B16Register,
        1 => &mut resources.spu.voice1_saddr as *mut B16Register,
        2 => &mut resources.spu.voice2_saddr as *mut B16Register,
        3 => &mut resources.spu.voice3_saddr as *mut B16Register,
        4 => &mut resources.spu.voice4_saddr as *mut B16Register,
        5 => &mut resources.spu.voice5_saddr as *mut B16Register,
        6 => &mut resources.spu.voice6_saddr as *mut B16Register,
        7 => &mut resources.spu.voice7_saddr as *mut B16Register,
        8 => &mut resources.spu.voice8_saddr as *mut B16Register,
        9 => &mut resources.spu.voice9_saddr as *mut B16Register,
        10 => &mut resources.spu.voice10_saddr as *mut B16Register,
        11 => &mut resources.spu.voice11_saddr as *mut B16Register,
        12 => &mut resources.spu.voice12_saddr as *mut B16Register,
        13 => &mut resources.spu.voice13_saddr as *mut B16Register,
        14 => &mut resources.spu.voice14_saddr as *mut B16Register,
        15 => &mut resources.spu.voice15_saddr as *mut B16Register,
        16 => &mut resources.spu.voice16_saddr as *mut B16Register,
        17 => &mut resources.spu.voice17_saddr as *mut B16Register,
        18 => &mut resources.spu.voice18_saddr as *mut B16Register,
        19 => &mut resources.spu.voice19_saddr as *mut B16Register,
        20 => &mut resources.spu.voice20_saddr as *mut B16Register,
        21 => &mut resources.spu.voice21_saddr as *mut B16Register,
        22 => &mut resources.spu.voice22_saddr as *mut B16Register,
        23 => &mut resources.spu.voice23_saddr as *mut B16Register,
        _ => unreachable!("Invalid voice id"),
    }
}

pub fn get_adpcm_envelope(resources: &mut Resources, voice_id: usize) -> *mut B32Register {
    match voice_id {
        0 => &mut resources.spu.voice0_adsr as *mut B32Register,
        1 => &mut resources.spu.voice1_adsr as *mut B32Register,
        2 => &mut resources.spu.voice2_adsr as *mut B32Register,
        3 => &mut resources.spu.voice3_adsr as *mut B32Register,
        4 => &mut resources.spu.voice4_adsr as *mut B32Register,
        5 => &mut resources.spu.voice5_adsr as *mut B32Register,
        6 => &mut resources.spu.voice6_adsr as *mut B32Register,
        7 => &mut resources.spu.voice7_adsr as *mut B32Register,
        8 => &mut resources.spu.voice8_adsr as *mut B32Register,
        9 => &mut resources.spu.voice9_adsr as *mut B32Register,
        10 => &mut resources.spu.voice10_adsr as *mut B32Register,
        11 => &mut resources.spu.voice11_adsr as *mut B32Register,
        12 => &mut resources.spu.voice12_adsr as *mut B32Register,
        13 => &mut resources.spu.voice13_adsr as *mut B32Register,
        14 => &mut resources.spu.voice14_adsr as *mut B32Register,
        15 => &mut resources.spu.voice15_adsr as *mut B32Register,
        16 => &mut resources.spu.voice16_adsr as *mut B32Register,
        17 => &mut resources.spu.voice17_adsr as *mut B32Register,
        18 => &mut resources.spu.voice18_adsr as *mut B32Register,
        19 => &mut resources.spu.voice19_adsr as *mut B32Register,
        20 => &mut resources.spu.voice20_adsr as *mut B32Register,
        21 => &mut resources.spu.voice21_adsr as *mut B32Register,
        22 => &mut resources.spu.voice22_adsr as *mut B32Register,
        23 => &mut resources.spu.voice23_adsr as *mut B32Register,
        _ => unreachable!("Invalid voice id"),
    }
}

pub fn get_adsr_cvol(resources: &mut Resources, voice_id: usize) -> *mut B16Register {
    match voice_id {
        0 => &mut resources.spu.voice0_cvol as *mut B16Register,
        1 => &mut resources.spu.voice1_cvol as *mut B16Register,
        2 => &mut resources.spu.voice2_cvol as *mut B16Register,
        3 => &mut resources.spu.voice3_cvol as *mut B16Register,
        4 => &mut resources.spu.voice4_cvol as *mut B16Register,
        5 => &mut resources.spu.voice5_cvol as *mut B16Register,
        6 => &mut resources.spu.voice6_cvol as *mut B16Register,
        7 => &mut resources.spu.voice7_cvol as *mut B16Register,
        8 => &mut resources.spu.voice8_cvol as *mut B16Register,
        9 => &mut resources.spu.voice9_cvol as *mut B16Register,
        10 => &mut resources.spu.voice10_cvol as *mut B16Register,
        11 => &mut resources.spu.voice11_cvol as *mut B16Register,
        12 => &mut resources.spu.voice12_cvol as *mut B16Register,
        13 => &mut resources.spu.voice13_cvol as *mut B16Register,
        14 => &mut resources.spu.voice14_cvol as *mut B16Register,
        15 => &mut resources.spu.voice15_cvol as *mut B16Register,
        16 => &mut resources.spu.voice16_cvol as *mut B16Register,
        17 => &mut resources.spu.voice17_cvol as *mut B16Register,
        18 => &mut resources.spu.voice18_cvol as *mut B16Register,
        19 => &mut resources.spu.voice19_cvol as *mut B16Register,
        20 => &mut resources.spu.voice20_cvol as *mut B16Register,
        21 => &mut resources.spu.voice21_cvol as *mut B16Register,
        22 => &mut resources.spu.voice22_cvol as *mut B16Register,
        23 => &mut resources.spu.voice23_cvol as *mut B16Register,
        _ => unreachable!("Invalid voice id"),
    }
}

pub fn get_adpcm_ra(resources: &mut Resources, voice_id: usize) -> *mut B16Register {
    match voice_id {
        0 => &mut resources.spu.voice0_raddr as *mut B16Register,
        1 => &mut resources.spu.voice1_raddr as *mut B16Register,
        2 => &mut resources.spu.voice2_raddr as *mut B16Register,
        3 => &mut resources.spu.voice3_raddr as *mut B16Register,
        4 => &mut resources.spu.voice4_raddr as *mut B16Register,
        5 => &mut resources.spu.voice5_raddr as *mut B16Register,
        6 => &mut resources.spu.voice6_raddr as *mut B16Register,
        7 => &mut resources.spu.voice7_raddr as *mut B16Register,
        8 => &mut resources.spu.voice8_raddr as *mut B16Register,
        9 => &mut resources.spu.voice9_raddr as *mut B16Register,
        10 => &mut resources.spu.voice10_raddr as *mut B16Register,
        11 => &mut resources.spu.voice11_raddr as *mut B16Register,
        12 => &mut resources.spu.voice12_raddr as *mut B16Register,
        13 => &mut resources.spu.voice13_raddr as *mut B16Register,
        14 => &mut resources.spu.voice14_raddr as *mut B16Register,
        15 => &mut resources.spu.voice15_raddr as *mut B16Register,
        16 => &mut resources.spu.voice16_raddr as *mut B16Register,
        17 => &mut resources.spu.voice17_raddr as *mut B16Register,
        18 => &mut resources.spu.voice18_raddr as *mut B16Register,
        19 => &mut resources.spu.voice19_raddr as *mut B16Register,
        20 => &mut resources.spu.voice20_raddr as *mut B16Register,
        21 => &mut resources.spu.voice21_raddr as *mut B16Register,
        22 => &mut resources.spu.voice22_raddr as *mut B16Register,
        23 => &mut resources.spu.voice23_raddr as *mut B16Register,
        _ => unreachable!("Invalid voice id"),
    }
}

pub fn get_play_state(resources: &mut Resources, voice_id: usize) -> *mut PlayState {
    match voice_id {
        0 => &mut resources.spu.dac.voice0_state as *mut PlayState,
        1 => &mut resources.spu.dac.voice1_state as *mut PlayState,
        2 => &mut resources.spu.dac.voice2_state as *mut PlayState,
        3 => &mut resources.spu.dac.voice3_state as *mut PlayState,
        4 => &mut resources.spu.dac.voice4_state as *mut PlayState,
        5 => &mut resources.spu.dac.voice5_state as *mut PlayState,
        6 => &mut resources.spu.dac.voice6_state as *mut PlayState,
        7 => &mut resources.spu.dac.voice7_state as *mut PlayState,
        8 => &mut resources.spu.dac.voice8_state as *mut PlayState,
        9 => &mut resources.spu.dac.voice9_state as *mut PlayState,
        10 => &mut resources.spu.dac.voice10_state as *mut PlayState,
        11 => &mut resources.spu.dac.voice11_state as *mut PlayState,
        12 => &mut resources.spu.dac.voice12_state as *mut PlayState,
        13 => &mut resources.spu.dac.voice13_state as *mut PlayState,
        14 => &mut resources.spu.dac.voice14_state as *mut PlayState,
        15 => &mut resources.spu.dac.voice15_state as *mut PlayState,
        16 => &mut resources.spu.dac.voice16_state as *mut PlayState,
        17 => &mut resources.spu.dac.voice17_state as *mut PlayState,
        18 => &mut resources.spu.dac.voice18_state as *mut PlayState,
        19 => &mut resources.spu.dac.voice19_state as *mut PlayState,
        20 => &mut resources.spu.dac.voice20_state as *mut PlayState,
        21 => &mut resources.spu.dac.voice21_state as *mut PlayState,
        22 => &mut resources.spu.dac.voice22_state as *mut PlayState,
        23 => &mut resources.spu.dac.voice23_state as *mut PlayState,
        _ => unreachable!("Invalid voice id"),
    }
}