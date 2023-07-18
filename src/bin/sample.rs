use log;

fn main() {
    println!("Poncu!");

    std::env::set_var("RUST_LOG", "debug");

    env_logger::init();

    // log::set_max_level(log::LevelFilter::Debug);

    println!("Log level: {}", log::max_level());

    if log::log_enabled!(log::Level::Trace) {
        log::trace!("log message in {} level", "TRACE");
    }

    if log::log_enabled!(log::Level::Debug) {
        log::debug!("log message in {} level", "DEBUG");
    }

    log::warn!("log message in {} level", "WARN");
    log::info!("log message in {} level", "INFO");
    log::error!("log message in {} level", "ERROR");
}
