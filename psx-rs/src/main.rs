mod backend;
mod config;

use libpsx_rs::{
    Config as CoreConfig,
    Core,
};
use sdl2::{
    video::GLProfile,
    EventPump,
    video::Window,
};
use std::{
    env::args,
    panic,
    path::{
        Path,
        PathBuf,
    },
    sync::atomic::{
        AtomicBool,
        Ordering,
    },
    time::Instant,
};

static EXIT: AtomicBool = AtomicBool::new(false);

fn main() {
    // Signal handlers
    setup_signal_handler();

    // Working directory / workspace
    let workspace_path = PathBuf::from(r"./workspace/");
    println!("Working directory: {}, workspace directory: {}", std::env::current_dir().unwrap().to_str().unwrap(), workspace_path.to_str().unwrap());

    // Read config
    let config = config::load(&workspace_path);

    // Setup logging
    let logs_path = workspace_path.join(r"logs/");
    let log_file_path = setup_log_file(&logs_path);
    setup_logger(&log_file_path);
    log::info!("Logging initialized");

    // Initialize SDL
    let sdl_context = sdl2::init().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(3, 3);
    gl_attr.set_double_buffer(false);
    gl_attr.set_context_flags().debug().set();
    let mut window = video_subsystem.window("psx-rs: Running", 1024, 512).position_centered().resizable().allow_highdpi().opengl().build().unwrap();
    log::info!("SDL initialized");

    // Initialize video.
    let video_backend = backend::initialize_video_backend(config.video_backend_kind, &window);

    // Initialize audio.
    let audio_backend = backend::initialize_audio_backend(config.audio_backend_kind);

    // Initialize CDROM.
    let cdrom_backend = backend::initialize_cdrom_backend(config.cdrom_backend_kind);

    // Initialize psx-rs core.
    let core_config = CoreConfig {
        workspace_path: PathBuf::from(r"./workspace/"),
        bios_filename: "scph5501.bin".to_owned(),
        video_backend,
        audio_backend,
        cdrom_backend,
        time_delta: config.time_delta_secs,
        worker_threads: config.worker_threads,
    };

    main_inner(&window, &mut event_pump, config, core_config);

    if config.pause_on_exit {
        window.set_title("psx-rs: Stopped").unwrap();

        log::info!("Pausing before exit, quit the application to exit gracefully");

        loop {
            match event_pump.wait_event() {
                sdl2::event::Event::Quit {
                    ..
                } => break,
                _ => {},
            }
        }
    }

    // CDROM teardown.
    backend::terminate_cdrom_backend(config.cdrom_backend_kind);

    // Audio teardown.
    backend::terminate_audio_backend(config.audio_backend_kind);

    // Video teardown.
    backend::terminate_video_backend(config.video_backend_kind);
}

fn setup_log_file(logs_path: &Path) -> PathBuf {
    std::fs::create_dir_all(&logs_path).unwrap();
    let file_name = format!("{}.log", chrono::Local::now().format("%Y%m%d_%H%M%S"));
    logs_path.join(file_name)
}

fn setup_logger(log_file_path: &Path) {
    let now = Instant::now();
    fern::Dispatch::new()
        .format(move |out, message, record| out.finish(format_args!("[{}ms][{}][{}] {}", now.elapsed().as_millis(), record.target(), record.level(), message)))
        .level(log::LevelFilter::Trace)
        .chain(std::io::stdout())
        .chain(fern::log_file(log_file_path).unwrap())
        .apply()
        .unwrap();
}

fn main_inner(window: &Window, event_pump: &mut EventPump, config: config::Config, core_config: CoreConfig) {
    let mut core = Core::new(core_config);
    log::info!("Core initialized");

    match args().nth(1) {
        Some(disc_path_raw) => {
            let disc_path = Path::new(&disc_path_raw);
            core.change_disc(disc_path);
            log::info!("Changed disc to {}", disc_path.display());
        },
        None => {},
    }

    // Do event loop
    let result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
        'event_loop: while !EXIT.load(Ordering::Relaxed) {
            for event in event_pump.poll_iter() {
                match event {
                    sdl2::event::Event::Quit {
                        ..
                    } => break 'event_loop,
                    sdl2::event::Event::KeyDown {
                        keycode,
                        ..
                    } => {
                        if let Some(key) = keycode {
                            handle_keycode(key, &mut core);
                        }
                    },
                    sdl2::event::Event::Window { win_event, .. } => {
                        if let sdl2::event::WindowEvent::Resized(_, _) = win_event {
                            let (width, height) = window.drawable_size();
                            let width = width as usize;
                            let height = height as usize;
                            log::info!("Resizing to {}x{}", width, height);
                            backend::on_resize_window(config.video_backend_kind, window, width, height);
                        }
                    }
                    _ => {},
                }
            }

            core.step();
        }
    }));

    if result.is_err() {
        log::error!("Panic occurred, exiting");
    }

    // Post mortem
    core.analyze();
}

fn setup_signal_handler() {
    let ctrl_c_handler = || {
        EXIT.store(true, Ordering::Relaxed);
    };

    ctrlc::set_handler(ctrl_c_handler).unwrap();
}

fn handle_keycode(keycode: sdl2::keyboard::Keycode, core: &mut Core) {
    use sdl2::keyboard::Keycode;

    match keycode {
        Keycode::F1 => {
            core.save_state(None).unwrap();
            log::info!("Saved state ok");
        },
        Keycode::F2 => {
            core.load_state(None).unwrap();
            log::info!("Loaded state ok");
        },
        Keycode::F3 => {
            // toggle_debug_option(&ENABLE_REGISTER_TRACING, "R3000 register output");
        },
        Keycode::F4 => {},
        Keycode::F5 => {},
        Keycode::F6 => {},
        Keycode::F7 => {},
        _ => {},
    }
}

#[allow(dead_code)]
fn toggle_debug_option(flag: &'static AtomicBool, identifier: &str) {
    let old_value = flag.fetch_xor(true, Ordering::Relaxed);
    log::debug!("Toggled {} from {} to {}", identifier, old_value, !old_value);
}
