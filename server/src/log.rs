use log::{error, info, warn};
use log4rs;

pub fn logger() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

    info!("booting up");

    // ...
}
