#[macro_use] extern crate log;
extern crate android_log;

fn main() {
    android_log::init("MyApp").unwrap();

    trace!("Initialized Rust");
    debug!("Address is {:p}", main as *const ());
    info!("Did you know? {} = {}", "1 + 1", 2);
    warn!("Don't log sensitive information!");
    error!("Nothing more to say");
}
