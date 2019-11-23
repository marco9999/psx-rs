use log::debug;
use crate::types::bitfield::Bitfield;
use crate::types::color::Color;
use crate::types::geometry::*;
use crate::resources::Resources;
use crate::resources::gpu::*;
use crate::backends::video::VideoBackend;
use crate::controllers::gpu::opengl;
use crate::controllers::gpu::data::*;

pub fn command_00_length(_data: &[u32]) -> Option<usize> {
    Some(1)
}

pub fn command_00_handler<'a>(_resources: &mut Resources, _video_backend: &VideoBackend<'a>, _data: &[u32]) {
}

pub fn command_01_length(_data: &[u32]) -> Option<usize> {
    Some(1)
}

pub fn command_01_handler<'a>(_resources: &mut Resources, _video_backend: &VideoBackend<'a>, _data: &[u32]) {
    // Flush cache (NOP)
}

pub fn command_05_length(_data: &[u32]) -> Option<usize> {
    Some(1)
}

pub fn command_05_handler<'a>(_resources: &mut Resources, _video_backend: &VideoBackend<'a>, _data: &[u32]) {
    // NOP
}

pub fn command_06_length(_data: &[u32]) -> Option<usize> {
    Some(1)
}

pub fn command_06_handler<'a>(_resources: &mut Resources, _video_backend: &VideoBackend<'a>, _data: &[u32]) {
    // NOP
}

pub fn command_0c_length(_data: &[u32]) -> Option<usize> {
    Some(1)
}

pub fn command_0c_handler<'a>(_resources: &mut Resources, _video_backend: &VideoBackend<'a>, _data: &[u32]) {
    // NOP
}

pub fn command_28_length(_data: &[u32]) -> Option<usize> {
    Some(5)
}

pub fn command_28_handler<'a>(_resources: &mut Resources, video_backend: &VideoBackend<'a>, data: &[u32]) {
    //debug!("Monochrome four-point polygon, opaque, c1 = 0x{:0X}, c2 = 0x{:0X}, c3 = 0x{:0X}, c4 = 0x{:0X}, c5 = 0x{:0X}", data[0], data[1], data[2], data[3], data[4]);
    
    let color = extract_color_rgb(data[0], std::u8::MAX);
    let vertices = extract_vertices_4_normalized([data[1], data[2], data[3], data[4]]);
    
    match video_backend {
        VideoBackend::None => { unimplemented!("") },
        VideoBackend::Opengl(ref backend_params) => {
            opengl::draw_polygon_4_solid(backend_params, vertices, color);
        },
    }
}

pub fn command_2c_length(_data: &[u32]) -> Option<usize> {
    Some(9)
}

pub fn command_2c_handler<'a>(_resources: &mut Resources, video_backend: &VideoBackend<'a>, data: &[u32]) {
    //debug!("Textured four-point polygon, opaque, texture-blending, c1 = 0x{:0X}, c2 = 0x{:0X}, c3 = 0x{:0X}, c4 = 0x{:0X}, c5 = 0x{:0X}, c6 = 0x{:0X}, c7 = 0x{:0X}, c8 = 0x{:0X}, c9 = 0x{:0X}", data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7], data[8]);

    // TODO: implement this properly - need to make a shader to do this I think...
    // CLUT not implemented at all, texcoords currently passed through scaled by the CLUT mode.

    let _color = extract_color_rgb(data[0], std::u8::MAX);
    let vertices = extract_vertices_4_normalized([data[1], data[3], data[5], data[7]]);
    let clut_mode = extract_texpage_clut_mode(data[4]);
    let _transparency_mode = extract_texpage_transparency_mode(data[4]);
    let texcoords = extract_texcoords_4_normalized(data[4], clut_mode, [data[2], data[4], data[6], data[8]]);
    let _clut = extract_clut_base_normalized(data[2]);

    match video_backend {
        VideoBackend::None => { unimplemented!("") },
        VideoBackend::Opengl(ref backend_params) => {
            opengl::draw_polygon_4_textured_framebuffer(backend_params, vertices, texcoords);
        },
    }
}

pub fn command_30_length(_data: &[u32]) -> Option<usize> {
    Some(6)
}

pub fn command_30_handler<'a>(_resources: &mut Resources, video_backend: &VideoBackend<'a>, data: &[u32]) {
    //debug!("Shaded three-point polygon, opaque, c1 = 0x{:0X}, c2 = 0x{:0X}, c3 = 0x{:0X}, c4 = 0x{:0X}, c5 = 0x{:0X}, c6 = 0x{:0X}", data[0], data[1], data[2], data[3], data[4], data[5]);

    let colors = extract_colors_3_rgb([data[0], data[2], data[4]], std::u8::MAX);
    let vertices = extract_vertices_3_normalized([data[1], data[3], data[5]]);

    match video_backend {
        VideoBackend::None => { unimplemented!("") },
        VideoBackend::Opengl(ref backend_params) => {
            opengl::draw_polygon_3_shaded(backend_params, vertices, colors);
        },
    }
}

pub fn command_38_length(_data: &[u32]) -> Option<usize> {
    Some(8)
}

pub fn command_38_handler<'a>(_resources: &mut Resources, video_backend: &VideoBackend<'a>, data: &[u32]) {
    //debug!("Shaded four-point polygon, opaque, c1 = 0x{:0X}, c2 = 0x{:0X}, c3 = 0x{:0X}, c4 = 0x{:0X}, c5 = 0x{:0X}, c6 = 0x{:0X}, c7 = 0x{:0X}, c8 = 0x{:0X}", data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7]);
    
    let colors = extract_colors_4_rgb([data[0], data[2], data[4], data[6]], std::u8::MAX);
    let vertices = extract_vertices_4_normalized([data[1], data[3], data[5], data[7]]);

    match video_backend {
        VideoBackend::None => { unimplemented!("") },
        VideoBackend::Opengl(ref backend_params) => {
            opengl::draw_polygon_4_shaded(backend_params, vertices, colors);
        },
    }
}

pub fn command_3c_length(_data: &[u32]) -> Option<usize> {
    Some(12)
}

pub fn command_3c_handler<'a>(_resources: &mut Resources, _video_backend: &VideoBackend<'a>, data: &[u32]) {
    debug!("Shaded Textured four-point polygon, opaque, texture-blending, c1 = 0x{:0X}, c2 = 0x{:0X}, c3 = 0x{:0X}, c4 = 0x{:0X}, c5 = 0x{:0X}, c6 = 0x{:0X}, c7 = 0x{:0X}, c8 = 0x{:0X}, c9 = 0x{:0X}, c10 = 0x{:0X}, c11 = 0x{:0X}, c12 = 0x{:0X}", data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7], data[8], data[9], data[10], data[11]);
}

pub fn command_50_length(_data: &[u32]) -> Option<usize> {
    Some(4)
}

pub fn command_50_handler<'a>(_resources: &mut Resources, _video_backend: &VideoBackend<'a>, data: &[u32]) {
    debug!("Shaded line, opaque, c1 = 0x{:0X}, c2 = 0x{:0X}, c3 = 0x{:0X}, c4 = 0x{:0X}", data[0], data[1], data[2], data[3]);
}

pub fn command_6f_length(_data: &[u32]) -> Option<usize> {
    Some(3)
}

pub fn command_6f_handler<'a>(_resources: &mut Resources, _video_backend: &VideoBackend<'a>, data: &[u32]) {
    debug!("Textured Rectangle, 1x1 (nonsense), semi-transp, raw-texture, c1 = 0x{:0X}, c2 = 0x{:0X}, c3 = 0x{:0X}", data[0], data[1], data[2]);
}

pub fn command_80_length(_data: &[u32]) -> Option<usize> {
    Some(4)
}

pub fn command_80_handler<'a>(_resources: &mut Resources, _video_backend: &VideoBackend<'a>, data: &[u32]) {
    debug!("Copy Rectangle (VRAM to VRAM), c1 = 0x{:0X}, c2 = 0x{:0X}, c3 = 0x{:0X}, c4 = 0x{:0X}", data[0], data[1], data[2], data[3]);
}

pub fn command_a0_length(data: &[u32]) -> Option<usize> {
    if data.len() < 3 { 
        return None; 
    }

    let width = Bitfield::new(0, 16).extract_from(data[2]) as usize;
    let height = Bitfield::new(16, 16).extract_from(data[2]) as usize;
    let count = width * height;
    let data_words = 3 + ((count + 1) / 2);

    return Some(data_words);
}

pub fn command_a0_handler<'a>(_resources: &mut Resources, video_backend: &VideoBackend<'a>, data: &[u32]) {
    //debug!("Copy Rectangle (CPU to VRAM), c1 = 0x{:0X}, c2 = 0x{:0X}, c3 = 0x{:0X}, data len = {}", data[0], data[1], data[2], data.len() - 3);
    
    let base_point = extract_point_normalized(data[1]);
    let size = extract_size_normalized(data[2]);
    let texture_width = Bitfield::new(0, 16).extract_from(data[2]) as usize;
    let texture_height = Bitfield::new(16, 16).extract_from(data[2]) as usize;

    let positions = [
        Point2D::new(base_point.x, base_point.y),
        Point2D::new(base_point.x + size.width, base_point.y),
        Point2D::new(base_point.x, base_point.y - size.height),
        Point2D::new(base_point.x + size.width, base_point.y - size.height),
    ];

    let texcoords = [
        Point2D::new(0.0, 0.0),
        Point2D::new(1.0, 0.0),
        Point2D::new(0.0, 1.0),
        Point2D::new(1.0, 1.0),
    ];

    // TODO: This is not a proper way to implement this command - the halfwords do not strictly represent pixels (16-bit colors / 5-5-5-1 colors).
    // However, the command addresses the VRAM (and incoming data) as 16-bit units through the coordinates given.
    // Ie: they could be 24-bit pixel data where 8-bits overflows into next pixel and so on... gives correct result for now. Hope this makes sense.... :/

    let mut texture_colors = Vec::with_capacity((data.len() - 3) * 2);
    let color_bitfields = [Bitfield::new(0, 16), Bitfield::new(16, 16)];
    for i in 3..data.len() {
        for field in color_bitfields.iter() {
            let packed_16 = field.extract_from(data[i]);
            let r = ((Bitfield::new(0, 5).extract_from(packed_16) * 255) / 31) as u8;
            let g = ((Bitfield::new(5, 5).extract_from(packed_16) * 255) / 31) as u8;
            let b = ((Bitfield::new(10, 5).extract_from(packed_16) * 255) / 31) as u8;
            //let mask = if Bitfield::new(15, 1).extract_from(packed_16) != 0 { std::u8::MAX } else { 0 };
            let mask = std::u8::MAX;
            texture_colors.push(Color::new(r, g, b, mask));
        }
    }

    match video_backend {
        VideoBackend::None => { unimplemented!("") },
        VideoBackend::Opengl(ref backend_params) => {
            opengl::draw_polygon_4_textured(backend_params, positions, texcoords, texture_width, texture_height, &texture_colors);
        },
    }
}

pub fn command_c0_length(_data: &[u32]) -> Option<usize> {
    Some(3)
}

pub fn command_c0_handler<'a>(resources: &mut Resources, video_backend: &VideoBackend<'a>, data: &[u32]) {
    //debug!("Copy Rectangle (VRAM to CPU), c1 = 0x{:0X}, c2 = 0x{:0X}, c3 = 0x{:0X}", data[0], data[1], data[2]);

    let origin = Point2D::new(
        Bitfield::new(0, 16).extract_from(data[1]) as usize,
        Bitfield::new(16, 16).extract_from(data[1]) as usize,
    );

    let size = Size2D::new(
        Bitfield::new(0, 16).extract_from(data[2]) as usize,
        Bitfield::new(16, 16).extract_from(data[2]) as usize,
    );

    let count = size.width * size.height;

    if count == 0 {
        panic!("Empty size - what happens? ({:?})", size);
    }

    let fifo_words = (count + 1) / 2;

    let mut data = match video_backend {
        VideoBackend::None => { unimplemented!("") },
        VideoBackend::Opengl(ref backend_params) => {
            opengl::read_framebuffer_5551(backend_params, origin, size)
        },
    };

    // Data is to be packed from 2 x u16 into u32.
    // Pad the last u16 if its an odd amount.
    if data.len() % 2 != 0 {
        data.push(0);
    }

    let read_buffer = &mut resources.gpu.gp0_read_buffer;
    for i in 0..fifo_words {
        let word: u32 = 0;
        Bitfield::new(0, 16).insert_into(word, data[i * 2] as u32);
        Bitfield::new(16, 16).insert_into(word, data[i * 2 + 1] as u32);
        read_buffer.push_back(word)
    }

    if size.height > 1 {
        unimplemented!("Verify function works properly for multiple lines");
    }
}

pub fn command_e1_length(_data: &[u32]) -> Option<usize> {
    Some(1)
}

pub fn command_e1_handler<'a>(resources: &mut Resources, _video_backend: &VideoBackend<'a>, data: &[u32]) {
    let stat = &mut resources.gpu.gpu1814.stat;
    stat.write_bitfield(STAT_TEXPAGEX, Bitfield::new(0, 4).extract_from(data[0]));
    stat.write_bitfield(STAT_TEXPAGEY, Bitfield::new(4, 1).extract_from(data[0]));
    stat.write_bitfield(STAT_TRANSPARENCY, Bitfield::new(5, 2).extract_from(data[0]));
    stat.write_bitfield(STAT_TEXPAGE_COLORS, Bitfield::new(7, 2).extract_from(data[0]));
    stat.write_bitfield(STAT_DITHER, Bitfield::new(9, 1).extract_from(data[0]));
    stat.write_bitfield(STAT_DRAW_DISPLAY, Bitfield::new(10, 1).extract_from(data[0]));
    stat.write_bitfield(STAT_TEXTURE_DISABLE, Bitfield::new(11, 1).extract_from(data[0]));
    resources.gpu.textured_rect_x_flip = Bitfield::new(12, 1).extract_from(data[0]) != 0;
    resources.gpu.textured_rect_y_flip = Bitfield::new(13, 1).extract_from(data[0]) != 0;
}

pub fn command_e2_length(_data: &[u32]) -> Option<usize> {
    Some(1)
}

pub fn command_e2_handler<'a>(resources: &mut Resources, _video_backend: &VideoBackend<'a>, data: &[u32]) {
    resources.gpu.texture_window_mask_x = Bitfield::new(0, 5).extract_from(data[0]) as usize;
    resources.gpu.texture_window_mask_y = Bitfield::new(5, 5).extract_from(data[0]) as usize;
    resources.gpu.texture_window_offset_x = Bitfield::new(10, 5).extract_from(data[0]) as usize;
    resources.gpu.texture_window_offset_y = Bitfield::new(15, 5).extract_from(data[0]) as usize;
}

pub fn command_e3_length(_data: &[u32]) -> Option<usize> {
    Some(1)
}

pub fn command_e3_handler<'a>(resources: &mut Resources, _video_backend: &VideoBackend<'a>, data: &[u32]) {
    resources.gpu.drawing_area_x1 = Bitfield::new(0, 10).extract_from(data[0]) as usize;
    resources.gpu.drawing_area_y1 = Bitfield::new(10, 9).extract_from(data[0]) as usize;
}

pub fn command_e4_length(_data: &[u32]) -> Option<usize> {
    Some(1)
}

pub fn command_e4_handler<'a>(resources: &mut Resources, _video_backend: &VideoBackend<'a>, data: &[u32]) {
    resources.gpu.drawing_area_x2 = Bitfield::new(0, 10).extract_from(data[0]) as usize;
    resources.gpu.drawing_area_y2 = Bitfield::new(10, 9).extract_from(data[0]) as usize;
}

pub fn command_e5_length(_data: &[u32]) -> Option<usize> {
    Some(1)
}

pub fn command_e5_handler<'a>(resources: &mut Resources, _video_backend: &VideoBackend<'a>, data: &[u32]) {
    resources.gpu.drawing_offset_x = Bitfield::new(0, 11).extract_from(data[0]) as usize;
    resources.gpu.drawing_offset_y = Bitfield::new(11, 11).extract_from(data[0]) as usize;
}

pub fn command_e6_length(_data: &[u32]) -> Option<usize> {
    Some(1)
}

pub fn command_e6_handler<'a>(resources: &mut Resources, _video_backend: &VideoBackend<'a>, data: &[u32]) {
    let stat = &mut resources.gpu.gpu1814.stat;
    stat.write_bitfield(STAT_DRAW_MASK, Bitfield::new(0, 1).extract_from(data[0]));
    stat.write_bitfield(STAT_DRAW_PIXELS, Bitfield::new(1, 1).extract_from(data[0]));
}