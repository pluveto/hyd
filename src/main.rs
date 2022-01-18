use simple_logger::SimpleLogger;
mod hyd_arg;

fn main() {
    SimpleLogger::new().init().unwrap();

    let args: Vec<String> = std::env::args().collect();
    let app_arg = hyd_arg::parse_all(&args);
    log::info!(
        "app_arg {} {}",
        app_arg.source_location.unwrap(),
        app_arg.target_location.unwrap()
    );
}
