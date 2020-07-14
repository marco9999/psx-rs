mod dac;
mod transfer;

use crate::types::{
    fifo::Fifo,
    memory::*,
    exclusive_state::ExclusiveState,
};
pub(crate) use dac::*;
#[cfg(feature = "serialization")]
use serde::{
    Deserialize,
    Serialize,
};
pub(crate) use transfer::*;

#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
#[derive(Clone)]
pub(crate) struct ControllerState {
    pub(crate) clock: f64,
    pub(crate) enabled: bool,
    pub(crate) muted: bool,
    pub(crate) transfer_state: TransferState,
    pub(crate) dac_state: DacState,
    pub(crate) memory: Vec<u8>,
}

impl ControllerState {
    pub(crate) fn new() -> ControllerState {
        ControllerState {
            clock: 0.0,
            enabled: false,
            muted: false,
            transfer_state: TransferState::new(),
            dac_state: DacState::new(),
            memory: vec![0; 0x8_0000],
        }
    }
}

#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
#[derive(Clone)]
pub(crate) struct State {
    pub(crate) main_volume_left: B16LevelRegister,
    pub(crate) main_volume_right: B16LevelRegister,
    pub(crate) reverb_volume: B32LevelRegister,

    pub(crate) voice_key_on: B32EdgeRegister,
    pub(crate) voice_key_off: B32EdgeRegister,
    pub(crate) voice_channel_fm: B32LevelRegister,
    pub(crate) voice_channel_noise: B32LevelRegister,
    pub(crate) voice_channel_reverb: B32LevelRegister,
    pub(crate) voice_channel_status: B32LevelRegister,

    pub(crate) unknown_0: B16LevelRegister,
    pub(crate) reverb_start_address: B16LevelRegister,
    pub(crate) irq_address: B16LevelRegister,
    pub(crate) control: B16EdgeRegister,
    pub(crate) data_transfer_address: B16EdgeRegister,
    pub(crate) data_fifo: Fifo<u16>,
    pub(crate) data_transfer_control: B16LevelRegister,
    pub(crate) stat: B16LevelRegister,
    pub(crate) cd_volume: B32LevelRegister,
    pub(crate) extern_volume: B32LevelRegister,
    pub(crate) unknown_1: B32LevelRegister,

    pub(crate) controller_state: ExclusiveState<ControllerState>,

    pub(crate) dapf1: B16LevelRegister,
    pub(crate) dapf2: B16LevelRegister,
    pub(crate) viir: B16LevelRegister,
    pub(crate) vcomb1: B16LevelRegister,
    pub(crate) vcomb2: B16LevelRegister,
    pub(crate) vcomb3: B16LevelRegister,
    pub(crate) vcomb4: B16LevelRegister,
    pub(crate) vwall: B16LevelRegister,
    pub(crate) vapf1: B16LevelRegister,
    pub(crate) vapf2: B16LevelRegister,
    pub(crate) msame: B32LevelRegister,
    pub(crate) mcomb1: B32LevelRegister,
    pub(crate) mcomb2: B32LevelRegister,
    pub(crate) dsame: B32LevelRegister,
    pub(crate) mdiff: B32LevelRegister,
    pub(crate) mcomb3: B32LevelRegister,
    pub(crate) mcomb4: B32LevelRegister,
    pub(crate) ddiff: B32LevelRegister,
    pub(crate) mapf1: B32LevelRegister,
    pub(crate) mapf2: B32LevelRegister,
    pub(crate) vin: B32LevelRegister,

    pub(crate) voice0_voll: B16LevelRegister,
    pub(crate) voice0_volr: B16LevelRegister,
    pub(crate) voice0_srate: B16LevelRegister,
    pub(crate) voice0_saddr: B16LevelRegister,
    pub(crate) voice0_adsr: B32LevelRegister,
    pub(crate) voice0_cvol: B16LevelRegister,
    pub(crate) voice0_raddr: B16LevelRegister,

    pub(crate) voice1_voll: B16LevelRegister,
    pub(crate) voice1_volr: B16LevelRegister,
    pub(crate) voice1_srate: B16LevelRegister,
    pub(crate) voice1_saddr: B16LevelRegister,
    pub(crate) voice1_adsr: B32LevelRegister,
    pub(crate) voice1_cvol: B16LevelRegister,
    pub(crate) voice1_raddr: B16LevelRegister,

    pub(crate) voice2_voll: B16LevelRegister,
    pub(crate) voice2_volr: B16LevelRegister,
    pub(crate) voice2_srate: B16LevelRegister,
    pub(crate) voice2_saddr: B16LevelRegister,
    pub(crate) voice2_adsr: B32LevelRegister,
    pub(crate) voice2_cvol: B16LevelRegister,
    pub(crate) voice2_raddr: B16LevelRegister,

    pub(crate) voice3_voll: B16LevelRegister,
    pub(crate) voice3_volr: B16LevelRegister,
    pub(crate) voice3_srate: B16LevelRegister,
    pub(crate) voice3_saddr: B16LevelRegister,
    pub(crate) voice3_adsr: B32LevelRegister,
    pub(crate) voice3_cvol: B16LevelRegister,
    pub(crate) voice3_raddr: B16LevelRegister,

    pub(crate) voice4_voll: B16LevelRegister,
    pub(crate) voice4_volr: B16LevelRegister,
    pub(crate) voice4_srate: B16LevelRegister,
    pub(crate) voice4_saddr: B16LevelRegister,
    pub(crate) voice4_adsr: B32LevelRegister,
    pub(crate) voice4_cvol: B16LevelRegister,
    pub(crate) voice4_raddr: B16LevelRegister,

    pub(crate) voice5_voll: B16LevelRegister,
    pub(crate) voice5_volr: B16LevelRegister,
    pub(crate) voice5_srate: B16LevelRegister,
    pub(crate) voice5_saddr: B16LevelRegister,
    pub(crate) voice5_adsr: B32LevelRegister,
    pub(crate) voice5_cvol: B16LevelRegister,
    pub(crate) voice5_raddr: B16LevelRegister,

    pub(crate) voice6_voll: B16LevelRegister,
    pub(crate) voice6_volr: B16LevelRegister,
    pub(crate) voice6_srate: B16LevelRegister,
    pub(crate) voice6_saddr: B16LevelRegister,
    pub(crate) voice6_adsr: B32LevelRegister,
    pub(crate) voice6_cvol: B16LevelRegister,
    pub(crate) voice6_raddr: B16LevelRegister,

    pub(crate) voice7_voll: B16LevelRegister,
    pub(crate) voice7_volr: B16LevelRegister,
    pub(crate) voice7_srate: B16LevelRegister,
    pub(crate) voice7_saddr: B16LevelRegister,
    pub(crate) voice7_adsr: B32LevelRegister,
    pub(crate) voice7_cvol: B16LevelRegister,
    pub(crate) voice7_raddr: B16LevelRegister,

    pub(crate) voice8_voll: B16LevelRegister,
    pub(crate) voice8_volr: B16LevelRegister,
    pub(crate) voice8_srate: B16LevelRegister,
    pub(crate) voice8_saddr: B16LevelRegister,
    pub(crate) voice8_adsr: B32LevelRegister,
    pub(crate) voice8_cvol: B16LevelRegister,
    pub(crate) voice8_raddr: B16LevelRegister,

    pub(crate) voice9_voll: B16LevelRegister,
    pub(crate) voice9_volr: B16LevelRegister,
    pub(crate) voice9_srate: B16LevelRegister,
    pub(crate) voice9_saddr: B16LevelRegister,
    pub(crate) voice9_adsr: B32LevelRegister,
    pub(crate) voice9_cvol: B16LevelRegister,
    pub(crate) voice9_raddr: B16LevelRegister,

    pub(crate) voice10_voll: B16LevelRegister,
    pub(crate) voice10_volr: B16LevelRegister,
    pub(crate) voice10_srate: B16LevelRegister,
    pub(crate) voice10_saddr: B16LevelRegister,
    pub(crate) voice10_adsr: B32LevelRegister,
    pub(crate) voice10_cvol: B16LevelRegister,
    pub(crate) voice10_raddr: B16LevelRegister,

    pub(crate) voice11_voll: B16LevelRegister,
    pub(crate) voice11_volr: B16LevelRegister,
    pub(crate) voice11_srate: B16LevelRegister,
    pub(crate) voice11_saddr: B16LevelRegister,
    pub(crate) voice11_adsr: B32LevelRegister,
    pub(crate) voice11_cvol: B16LevelRegister,
    pub(crate) voice11_raddr: B16LevelRegister,

    pub(crate) voice12_voll: B16LevelRegister,
    pub(crate) voice12_volr: B16LevelRegister,
    pub(crate) voice12_srate: B16LevelRegister,
    pub(crate) voice12_saddr: B16LevelRegister,
    pub(crate) voice12_adsr: B32LevelRegister,
    pub(crate) voice12_cvol: B16LevelRegister,
    pub(crate) voice12_raddr: B16LevelRegister,

    pub(crate) voice13_voll: B16LevelRegister,
    pub(crate) voice13_volr: B16LevelRegister,
    pub(crate) voice13_srate: B16LevelRegister,
    pub(crate) voice13_saddr: B16LevelRegister,
    pub(crate) voice13_adsr: B32LevelRegister,
    pub(crate) voice13_cvol: B16LevelRegister,
    pub(crate) voice13_raddr: B16LevelRegister,

    pub(crate) voice14_voll: B16LevelRegister,
    pub(crate) voice14_volr: B16LevelRegister,
    pub(crate) voice14_srate: B16LevelRegister,
    pub(crate) voice14_saddr: B16LevelRegister,
    pub(crate) voice14_adsr: B32LevelRegister,
    pub(crate) voice14_cvol: B16LevelRegister,
    pub(crate) voice14_raddr: B16LevelRegister,

    pub(crate) voice15_voll: B16LevelRegister,
    pub(crate) voice15_volr: B16LevelRegister,
    pub(crate) voice15_srate: B16LevelRegister,
    pub(crate) voice15_saddr: B16LevelRegister,
    pub(crate) voice15_adsr: B32LevelRegister,
    pub(crate) voice15_cvol: B16LevelRegister,
    pub(crate) voice15_raddr: B16LevelRegister,

    pub(crate) voice16_voll: B16LevelRegister,
    pub(crate) voice16_volr: B16LevelRegister,
    pub(crate) voice16_srate: B16LevelRegister,
    pub(crate) voice16_saddr: B16LevelRegister,
    pub(crate) voice16_adsr: B32LevelRegister,
    pub(crate) voice16_cvol: B16LevelRegister,
    pub(crate) voice16_raddr: B16LevelRegister,

    pub(crate) voice17_voll: B16LevelRegister,
    pub(crate) voice17_volr: B16LevelRegister,
    pub(crate) voice17_srate: B16LevelRegister,
    pub(crate) voice17_saddr: B16LevelRegister,
    pub(crate) voice17_adsr: B32LevelRegister,
    pub(crate) voice17_cvol: B16LevelRegister,
    pub(crate) voice17_raddr: B16LevelRegister,

    pub(crate) voice18_voll: B16LevelRegister,
    pub(crate) voice18_volr: B16LevelRegister,
    pub(crate) voice18_srate: B16LevelRegister,
    pub(crate) voice18_saddr: B16LevelRegister,
    pub(crate) voice18_adsr: B32LevelRegister,
    pub(crate) voice18_cvol: B16LevelRegister,
    pub(crate) voice18_raddr: B16LevelRegister,

    pub(crate) voice19_voll: B16LevelRegister,
    pub(crate) voice19_volr: B16LevelRegister,
    pub(crate) voice19_srate: B16LevelRegister,
    pub(crate) voice19_saddr: B16LevelRegister,
    pub(crate) voice19_adsr: B32LevelRegister,
    pub(crate) voice19_cvol: B16LevelRegister,
    pub(crate) voice19_raddr: B16LevelRegister,

    pub(crate) voice20_voll: B16LevelRegister,
    pub(crate) voice20_volr: B16LevelRegister,
    pub(crate) voice20_srate: B16LevelRegister,
    pub(crate) voice20_saddr: B16LevelRegister,
    pub(crate) voice20_adsr: B32LevelRegister,
    pub(crate) voice20_cvol: B16LevelRegister,
    pub(crate) voice20_raddr: B16LevelRegister,

    pub(crate) voice21_voll: B16LevelRegister,
    pub(crate) voice21_volr: B16LevelRegister,
    pub(crate) voice21_srate: B16LevelRegister,
    pub(crate) voice21_saddr: B16LevelRegister,
    pub(crate) voice21_adsr: B32LevelRegister,
    pub(crate) voice21_cvol: B16LevelRegister,
    pub(crate) voice21_raddr: B16LevelRegister,

    pub(crate) voice22_voll: B16LevelRegister,
    pub(crate) voice22_volr: B16LevelRegister,
    pub(crate) voice22_srate: B16LevelRegister,
    pub(crate) voice22_saddr: B16LevelRegister,
    pub(crate) voice22_adsr: B32LevelRegister,
    pub(crate) voice22_cvol: B16LevelRegister,
    pub(crate) voice22_raddr: B16LevelRegister,

    pub(crate) voice23_voll: B16LevelRegister,
    pub(crate) voice23_volr: B16LevelRegister,
    pub(crate) voice23_srate: B16LevelRegister,
    pub(crate) voice23_saddr: B16LevelRegister,
    pub(crate) voice23_adsr: B32LevelRegister,
    pub(crate) voice23_cvol: B16LevelRegister,
    pub(crate) voice23_raddr: B16LevelRegister,
}

impl State {
    pub(crate) fn new() -> State {
        State {
            main_volume_left: B16LevelRegister::new(),
            main_volume_right: B16LevelRegister::new(),
            reverb_volume: B32LevelRegister::new(),
            voice_key_on: B32EdgeRegister::new(),
            voice_key_off: B32EdgeRegister::new(),
            voice_channel_fm: B32LevelRegister::new(),
            voice_channel_noise: B32LevelRegister::new(),
            voice_channel_reverb: B32LevelRegister::new(),
            voice_channel_status: B32LevelRegister::new(),
            unknown_0: B16LevelRegister::new(),
            reverb_start_address: B16LevelRegister::new(),
            irq_address: B16LevelRegister::new(),
            data_transfer_address: B16EdgeRegister::new(),
            control: B16EdgeRegister::new(),
            data_transfer_control: B16LevelRegister::new(),
            stat: B16LevelRegister::new(),
            cd_volume: B32LevelRegister::new(),
            extern_volume: B32LevelRegister::new(),
            unknown_1: B32LevelRegister::new(),
            dapf1: B16LevelRegister::new(),
            dapf2: B16LevelRegister::new(),
            viir: B16LevelRegister::new(),
            vcomb1: B16LevelRegister::new(),
            vcomb2: B16LevelRegister::new(),
            vcomb3: B16LevelRegister::new(),
            vcomb4: B16LevelRegister::new(),
            vwall: B16LevelRegister::new(),
            vapf1: B16LevelRegister::new(),
            vapf2: B16LevelRegister::new(),
            msame: B32LevelRegister::new(),
            mcomb1: B32LevelRegister::new(),
            mcomb2: B32LevelRegister::new(),
            dsame: B32LevelRegister::new(),
            mdiff: B32LevelRegister::new(),
            mcomb3: B32LevelRegister::new(),
            mcomb4: B32LevelRegister::new(),
            ddiff: B32LevelRegister::new(),
            mapf1: B32LevelRegister::new(),
            mapf2: B32LevelRegister::new(),
            vin: B32LevelRegister::new(),
            voice0_voll: B16LevelRegister::new(),
            voice0_volr: B16LevelRegister::new(),
            voice0_srate: B16LevelRegister::new(),
            voice0_saddr: B16LevelRegister::new(),
            voice0_adsr: B32LevelRegister::new(),
            voice0_cvol: B16LevelRegister::new(),
            voice0_raddr: B16LevelRegister::new(),
            voice1_voll: B16LevelRegister::new(),
            voice1_volr: B16LevelRegister::new(),
            voice1_srate: B16LevelRegister::new(),
            voice1_saddr: B16LevelRegister::new(),
            voice1_adsr: B32LevelRegister::new(),
            voice1_cvol: B16LevelRegister::new(),
            voice1_raddr: B16LevelRegister::new(),
            voice2_voll: B16LevelRegister::new(),
            voice2_volr: B16LevelRegister::new(),
            voice2_srate: B16LevelRegister::new(),
            voice2_saddr: B16LevelRegister::new(),
            voice2_adsr: B32LevelRegister::new(),
            voice2_cvol: B16LevelRegister::new(),
            voice2_raddr: B16LevelRegister::new(),
            voice3_voll: B16LevelRegister::new(),
            voice3_volr: B16LevelRegister::new(),
            voice3_srate: B16LevelRegister::new(),
            voice3_saddr: B16LevelRegister::new(),
            voice3_adsr: B32LevelRegister::new(),
            voice3_cvol: B16LevelRegister::new(),
            voice3_raddr: B16LevelRegister::new(),
            voice4_voll: B16LevelRegister::new(),
            voice4_volr: B16LevelRegister::new(),
            voice4_srate: B16LevelRegister::new(),
            voice4_saddr: B16LevelRegister::new(),
            voice4_adsr: B32LevelRegister::new(),
            voice4_cvol: B16LevelRegister::new(),
            voice4_raddr: B16LevelRegister::new(),
            voice5_voll: B16LevelRegister::new(),
            voice5_volr: B16LevelRegister::new(),
            voice5_srate: B16LevelRegister::new(),
            voice5_saddr: B16LevelRegister::new(),
            voice5_adsr: B32LevelRegister::new(),
            voice5_cvol: B16LevelRegister::new(),
            voice5_raddr: B16LevelRegister::new(),
            voice6_voll: B16LevelRegister::new(),
            voice6_volr: B16LevelRegister::new(),
            voice6_srate: B16LevelRegister::new(),
            voice6_saddr: B16LevelRegister::new(),
            voice6_adsr: B32LevelRegister::new(),
            voice6_cvol: B16LevelRegister::new(),
            voice6_raddr: B16LevelRegister::new(),
            voice7_voll: B16LevelRegister::new(),
            voice7_volr: B16LevelRegister::new(),
            voice7_srate: B16LevelRegister::new(),
            voice7_saddr: B16LevelRegister::new(),
            voice7_adsr: B32LevelRegister::new(),
            voice7_cvol: B16LevelRegister::new(),
            voice7_raddr: B16LevelRegister::new(),
            voice8_voll: B16LevelRegister::new(),
            voice8_volr: B16LevelRegister::new(),
            voice8_srate: B16LevelRegister::new(),
            voice8_saddr: B16LevelRegister::new(),
            voice8_adsr: B32LevelRegister::new(),
            voice8_cvol: B16LevelRegister::new(),
            voice8_raddr: B16LevelRegister::new(),
            voice9_voll: B16LevelRegister::new(),
            voice9_volr: B16LevelRegister::new(),
            voice9_srate: B16LevelRegister::new(),
            voice9_saddr: B16LevelRegister::new(),
            voice9_adsr: B32LevelRegister::new(),
            voice9_cvol: B16LevelRegister::new(),
            voice9_raddr: B16LevelRegister::new(),
            voice10_voll: B16LevelRegister::new(),
            voice10_volr: B16LevelRegister::new(),
            voice10_srate: B16LevelRegister::new(),
            voice10_saddr: B16LevelRegister::new(),
            voice10_adsr: B32LevelRegister::new(),
            voice10_cvol: B16LevelRegister::new(),
            voice10_raddr: B16LevelRegister::new(),
            voice11_voll: B16LevelRegister::new(),
            voice11_volr: B16LevelRegister::new(),
            voice11_srate: B16LevelRegister::new(),
            voice11_saddr: B16LevelRegister::new(),
            voice11_adsr: B32LevelRegister::new(),
            voice11_cvol: B16LevelRegister::new(),
            voice11_raddr: B16LevelRegister::new(),
            voice12_voll: B16LevelRegister::new(),
            voice12_volr: B16LevelRegister::new(),
            voice12_srate: B16LevelRegister::new(),
            voice12_saddr: B16LevelRegister::new(),
            voice12_adsr: B32LevelRegister::new(),
            voice12_cvol: B16LevelRegister::new(),
            voice12_raddr: B16LevelRegister::new(),
            voice13_voll: B16LevelRegister::new(),
            voice13_volr: B16LevelRegister::new(),
            voice13_srate: B16LevelRegister::new(),
            voice13_saddr: B16LevelRegister::new(),
            voice13_adsr: B32LevelRegister::new(),
            voice13_cvol: B16LevelRegister::new(),
            voice13_raddr: B16LevelRegister::new(),
            voice14_voll: B16LevelRegister::new(),
            voice14_volr: B16LevelRegister::new(),
            voice14_srate: B16LevelRegister::new(),
            voice14_saddr: B16LevelRegister::new(),
            voice14_adsr: B32LevelRegister::new(),
            voice14_cvol: B16LevelRegister::new(),
            voice14_raddr: B16LevelRegister::new(),
            voice15_voll: B16LevelRegister::new(),
            voice15_volr: B16LevelRegister::new(),
            voice15_srate: B16LevelRegister::new(),
            voice15_saddr: B16LevelRegister::new(),
            voice15_adsr: B32LevelRegister::new(),
            voice15_cvol: B16LevelRegister::new(),
            voice15_raddr: B16LevelRegister::new(),
            voice16_voll: B16LevelRegister::new(),
            voice16_volr: B16LevelRegister::new(),
            voice16_srate: B16LevelRegister::new(),
            voice16_saddr: B16LevelRegister::new(),
            voice16_adsr: B32LevelRegister::new(),
            voice16_cvol: B16LevelRegister::new(),
            voice16_raddr: B16LevelRegister::new(),
            voice17_voll: B16LevelRegister::new(),
            voice17_volr: B16LevelRegister::new(),
            voice17_srate: B16LevelRegister::new(),
            voice17_saddr: B16LevelRegister::new(),
            voice17_adsr: B32LevelRegister::new(),
            voice17_cvol: B16LevelRegister::new(),
            voice17_raddr: B16LevelRegister::new(),
            voice18_voll: B16LevelRegister::new(),
            voice18_volr: B16LevelRegister::new(),
            voice18_srate: B16LevelRegister::new(),
            voice18_saddr: B16LevelRegister::new(),
            voice18_adsr: B32LevelRegister::new(),
            voice18_cvol: B16LevelRegister::new(),
            voice18_raddr: B16LevelRegister::new(),
            voice19_voll: B16LevelRegister::new(),
            voice19_volr: B16LevelRegister::new(),
            voice19_srate: B16LevelRegister::new(),
            voice19_saddr: B16LevelRegister::new(),
            voice19_adsr: B32LevelRegister::new(),
            voice19_cvol: B16LevelRegister::new(),
            voice19_raddr: B16LevelRegister::new(),
            voice20_voll: B16LevelRegister::new(),
            voice20_volr: B16LevelRegister::new(),
            voice20_srate: B16LevelRegister::new(),
            voice20_saddr: B16LevelRegister::new(),
            voice20_adsr: B32LevelRegister::new(),
            voice20_cvol: B16LevelRegister::new(),
            voice20_raddr: B16LevelRegister::new(),
            voice21_voll: B16LevelRegister::new(),
            voice21_volr: B16LevelRegister::new(),
            voice21_srate: B16LevelRegister::new(),
            voice21_saddr: B16LevelRegister::new(),
            voice21_adsr: B32LevelRegister::new(),
            voice21_cvol: B16LevelRegister::new(),
            voice21_raddr: B16LevelRegister::new(),
            voice22_voll: B16LevelRegister::new(),
            voice22_volr: B16LevelRegister::new(),
            voice22_srate: B16LevelRegister::new(),
            voice22_saddr: B16LevelRegister::new(),
            voice22_adsr: B32LevelRegister::new(),
            voice22_cvol: B16LevelRegister::new(),
            voice22_raddr: B16LevelRegister::new(),
            voice23_voll: B16LevelRegister::new(),
            voice23_volr: B16LevelRegister::new(),
            voice23_srate: B16LevelRegister::new(),
            voice23_saddr: B16LevelRegister::new(),
            voice23_adsr: B32LevelRegister::new(),
            voice23_cvol: B16LevelRegister::new(),
            voice23_raddr: B16LevelRegister::new(),
            data_fifo: Fifo::new(2048),
            controller_state: ExclusiveState::new(ControllerState::new()),
        }
    }
}
