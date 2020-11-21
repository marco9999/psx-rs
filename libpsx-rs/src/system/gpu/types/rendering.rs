use crate::{
    system::gpu::{
        types::ClutMode,
    },
    types::geometry::*,
};

#[derive(Copy, Clone, Debug)]
pub(crate) enum ClutKind {
    Bits4 {
        base: Point2D<f32, TexcoordNormalized>,
    },
    Bits8 {
        base: Point2D<f32, TexcoordNormalized>,
    },
    Direct,
}

impl ClutKind {
    pub(crate) fn from_data(mode: ClutMode, base: Point2D<f32, TexcoordNormalized>) -> ClutKind {
        match mode {
            ClutMode::Bits4 => {
                ClutKind::Bits4 { base }
            },
            ClutMode::Bits8 => {
                ClutKind::Bits8 { base }
            },
            ClutMode::Bits15 => ClutKind::Direct,
            ClutMode::Reserved => unreachable!("Reserved CLUT mode used!"),
        }
    }
}
