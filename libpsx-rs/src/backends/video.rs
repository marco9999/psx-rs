pub mod opengl;

use crate::backends::video::opengl::*;

pub enum VideoBackend<'a> {
    None,
    Opengl(BackendParams<'a>),
}

pub fn setup(video_backend: &VideoBackend) {
    match video_backend {
        VideoBackend::None => unimplemented!(),
        VideoBackend::Opengl(ref params) => opengl::setup(params),
    }
}

pub fn teardown(video_backend: &VideoBackend) {
    match video_backend {
        VideoBackend::None => unimplemented!(),
        VideoBackend::Opengl(ref params) => opengl::teardown(params),
    }
}
