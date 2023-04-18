pub fn main() {
    rust_logger_sample::init();

    println!("============================================================");
    println!("log::log_enabled!() being called...");
    println!("");
    let b = log::log_enabled!(log::Level::Debug);
    println!("called: {}", b);
    println!("");

    println!("============================================================");
    println!("log::error!() being called...");
    println!("");
    log::error!("Error");

    println!("============================================================");
    println!("log::warn!() being called...");
    println!("");
    log::warn!("warn");

    println!("============================================================");
    println!("log::info!() being called...");
    println!("");
    log::info!("Info");

    println!("============================================================");
    println!("log::debug!() being called...");
    println!("");
    log::debug!("Debug");

    println!("============================================================");
    println!("log::trace!() being called...");
    println!("");
    log::trace!("Trace");

    println!("============================================================");
}
