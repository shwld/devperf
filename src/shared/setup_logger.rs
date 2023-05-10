use env_logger;

pub fn init(verbose: bool) {
    let log_level = if verbose {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Info
    };
    env_logger::builder().filter_level(log_level).init();
}
