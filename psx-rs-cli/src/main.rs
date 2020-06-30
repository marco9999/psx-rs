use libpsx_rs::{
    backends::{
        audio::AudioBackend,
        cdrom::CdromBackend,
        video::VideoBackend,
    },
    Config,
    Core,
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

    // Setup logging
    let logs_path = workspace_path.join(r"logs/");
    let log_file_path = setup_log_file(&logs_path);
    setup_logger(&log_file_path);
    log::info!("Logging initialized");

    // Initialize psx_rs core
    let time_delta_us = args().nth(1).map_or(25, |v| v.parse::<usize>().unwrap());
    let worker_threads = args().nth(2).map_or(2, |v| v.parse::<usize>().unwrap());
    let config = Config {
        workspace_path: PathBuf::from(r"./workspace/"),
        bios_filename: "scph5501.bin".to_owned(),
        video_backend: VideoBackend::None,
        audio_backend: AudioBackend::None,
        cdrom_backend: CdromBackend::None,
        time_delta: time_delta_us as f64 / 1e6,
        worker_threads,
    };

    main_inner(config);
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

fn setup_signal_handler() {
    ctrlc::set_handler(|| EXIT.store(true, Ordering::Release)).unwrap();
}

fn main_inner(config: Config) {
    let mut core = Core::new(config);
    log::info!("Core initialized");

    let result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
        while !EXIT.load(Ordering::Acquire) {
            core.step();
        }
    }));

    if result.is_err() {
        log::error!("Panic occurred, exiting");
    }
}
