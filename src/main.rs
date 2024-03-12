mod app;
mod init;
mod api;
mod library;

use log::{info,error};
use log4rs;

fn main() {
    let _ = log4rs::init_file("log4rs.yaml", Default::default());
    info!("booting up...............");

    let r = init::init_app();

    if let Err(_) = r{
        error!("bootup failed...............");
    }
}
