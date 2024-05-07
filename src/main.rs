use ::log::{error, LevelFilter};
use nexus::{log, Nexus, NexusArgs};

#[rocket::main]
async fn main() {
    let min_log_level = match nexus::is_debug() {
        true => LevelFilter::Debug,
        false => LevelFilter::Info,
    };

    let result = log::set_up(min_log_level);
    if let Err(err) = result {
        eprintln!("{}", err);
        return;
    }

    let nexus_args = NexusArgs::new("https://nexus-cdn.schweren.dev/registry.json".to_string());
    let nexus = Nexus::new(nexus_args);
    let result = nexus.start().await;
    if let Err(err) = result {
        error!("{}", err);
    }
}
