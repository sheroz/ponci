use log::{trace, debug, info, warn, error, log_enabled, Level};
use log4rs;

fn main() {
    println!("Poncu!");
    
    log4rs::init_file("log.yaml", Default::default()).unwrap();

    if log_enabled!(Level::Trace) {
        trace!("log message in {} level", "TRACE");
    }

    if log_enabled!(Level::Debug) {
        debug!("log message in {} level", "DEBUG");
    }

    info!("log message in {} level", "INFO");
    warn!("log message in {} level", "WARN");
    error!("log message in {} level", "ERROR");
}
