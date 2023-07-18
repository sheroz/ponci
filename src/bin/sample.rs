use log::{trace, debug, warn, info, error};

fn main() {
    println!("Poncu!");

    env_logger::init();
    trace!("log message in {} level", "TRACE");
    debug!("log message in {} level", "DEBUG");
    warn!("log message in {} level", "WARN");
    info!("log message in {} level", "INFO");
    error!("log message in {} level", "ERROR");
}
