use crate::backends::video::VideoBackend;
use crate::resources::Resources;
use crate::types::bitfield::Bitfield;
use crate::resources::gpu::*;
use crate::controllers::gpu::command_gp0_impl;

pub fn command_00(resources: &mut Resources, video_backend: &VideoBackend, _command: u32) {
    command_01(resources, video_backend, 0);
    command_02(resources, video_backend, 0);
    command_03(resources, video_backend, 1);
    command_04(resources, video_backend, 0);
    command_05(resources, video_backend, 0);
    command_06(resources, video_backend, 0);
    command_07(resources, video_backend, 0);
    command_08(resources, video_backend, 0);
    command_gp0_impl::command_e1_handler(resources, video_backend, &[0]);
    command_gp0_impl::command_e2_handler(resources, video_backend, &[0]);
    command_gp0_impl::command_e3_handler(resources, video_backend, &[0]);
    command_gp0_impl::command_e4_handler(resources, video_backend, &[0]);
    command_gp0_impl::command_e5_handler(resources, video_backend, &[0]);
    command_gp0_impl::command_e6_handler(resources, video_backend, &[0]);
}

pub fn command_01(resources: &mut Resources, _video_backend: &VideoBackend, _command: u32) {
    resources.gpu.gpu1810.gp0.clear();
}

pub fn command_02(resources: &mut Resources, _video_backend: &VideoBackend, _command: u32) {
    resources.gpu.gpu1814.stat.write_bitfield(STAT_IRQ_REQUEST, 0);
}

pub fn command_03(resources: &mut Resources, _video_backend: &VideoBackend, command: u32) {
    resources.gpu.gpu1814.stat.write_bitfield(STAT_DISPLAY_ENABLE, Bitfield::new(0, 1).extract_from(command));
}

pub fn command_04(resources: &mut Resources, _video_backend: &VideoBackend, command: u32) {
    let dma_direction = Bitfield::new(0, 2).extract_from(command);
    
    let mut read_clear_required = false;

    match dma_direction {
        0 => {
            //debug!("DMA direction set to 0 (off)");
            read_clear_required = true;
        }
        1 => {
            //debug!("DMA direction set to 1 (FIFO) - what does this mean???");
            read_clear_required = true;
        },
        2 => {
            //debug!("DMA direction set to 2 (CPUtoGP0)");
            read_clear_required = true;
        },
        3 => {
            //debug!("DMA direction set to 3 (GPUREADtoCPU)");
        },
        _ => unreachable!(),
    }

    if read_clear_required {
        resources.gpu.gp0_read_buffer.clear();
        while let Ok(_) = resources.gpu.gpu1810.read.read_one() {}
    }

    resources.gpu.gpu1814.stat.write_bitfield(STAT_DMA_DIRECTION, dma_direction);
}

pub fn command_05(resources: &mut Resources, _video_backend: &VideoBackend, command: u32) {
    resources.gpu.display_area_start_x = Bitfield::new(0, 10).extract_from(command) as usize;
    resources.gpu.display_area_start_y = Bitfield::new(10, 9).extract_from(command) as usize;
}

pub fn command_06(resources: &mut Resources, _video_backend: &VideoBackend, command: u32) {
    resources.gpu.horizontal_display_range_x1 = Bitfield::new(0, 12).extract_from(command) as usize;
    resources.gpu.horizontal_display_range_x2 = Bitfield::new(12, 12).extract_from(command) as usize;
}

pub fn command_07(resources: &mut Resources, _video_backend: &VideoBackend, command: u32) {
    resources.gpu.vertical_display_range_y1 = Bitfield::new(0, 10).extract_from(command) as usize;
    resources.gpu.vertical_display_range_y2 = Bitfield::new(10, 10).extract_from(command) as usize;
}

pub fn command_08(resources: &mut Resources, _video_backend: &VideoBackend, command: u32) {
    let stat = &mut resources.gpu.gpu1814.stat;
    stat.write_bitfield(STAT_HORIZONTAL_RES_1, Bitfield::new(0, 2).extract_from(command));
    stat.write_bitfield(STAT_VERTICAL_RES, Bitfield::new(2, 1).extract_from(command));
    stat.write_bitfield(STAT_VIDEO_MODE, Bitfield::new(3, 1).extract_from(command));
    stat.write_bitfield(STAT_DISPLAY_COLOR_DEPTH, Bitfield::new(4, 1).extract_from(command));
    stat.write_bitfield(STAT_INTERLACE_VERTICAL, Bitfield::new(5, 1).extract_from(command));
    stat.write_bitfield(STAT_HORIZONTAL_RES_2, Bitfield::new(6, 1).extract_from(command));
    stat.write_bitfield(STAT_REVERSE, Bitfield::new(7, 1).extract_from(command));
}
