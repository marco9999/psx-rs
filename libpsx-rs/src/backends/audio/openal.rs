pub mod rendering;

use openal_sys::*;
use crate::backends::context::*;
use crate::backends::audio::openal::rendering::*;
use crate::constants::spu::dac::VOICES_COUNT;

static mut INITIALIZED: bool = false;

pub struct BackendParams<'a: 'b, 'b> {
    pub context: BackendContext<'a, 'b, ()>,
}

pub fn setup(backend_params: &BackendParams) {
    let (_context_guard, _context) = backend_params.context.guard();

    unsafe {
        assert_eq!(INITIALIZED, false);
        
        alGenSources(rendering::SOURCES.len() as ALsizei, rendering::SOURCES.as_mut_ptr());
        alGenBuffers(rendering::BUFFERS.len() as ALsizei, rendering::BUFFERS.as_mut_ptr());

        if alGetError() != AL_NO_ERROR as ALenum {
            panic!("Error initializing OpenAL audio backend");
        }

        INITIALIZED = true;
    }
}

pub fn teardown(backend_params: &BackendParams) {
    let (_context_guard, _context) = backend_params.context.guard();

    unsafe {
        if INITIALIZED {
            for i in 0..VOICES_COUNT {
                alSourcei(SOURCES[i], AL_BUFFER as ALenum, 0);
            }

            alDeleteBuffers(rendering::BUFFERS.len() as ALsizei, rendering::BUFFERS.as_mut_ptr());
            alDeleteSources(rendering::SOURCES.len() as ALsizei, rendering::SOURCES.as_mut_ptr());
    
            if alGetError() != AL_NO_ERROR as ALenum {
                panic!("Error tearing down OpenAL audio backend");
            }
            
            INITIALIZED = false;
        }
    }
}
