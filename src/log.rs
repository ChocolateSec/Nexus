use fern::colors::{Color, ColoredLevelConfig};
use log::{LevelFilter, SetLoggerError};
use std::{
    io,
    sync::atomic::{AtomicBool, Ordering},
    time::SystemTime,
};

static IS_LOGGER_SET_UP: AtomicBool = AtomicBool::new(false);

pub fn is_set_up() -> bool {
    IS_LOGGER_SET_UP.load(Ordering::Relaxed)
}

pub fn set_up(min_log_level: LevelFilter) -> Result<(), SetLoggerError> {
    let colors = ColoredLevelConfig::new()
        .info(Color::Green)
        .debug(Color::Magenta)
        .warn(Color::Yellow)
        .error(Color::Red)
        .trace(Color::Cyan);

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{} {: <30} {: <5}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.target(),
                colors.color(record.level()),
                message
            ))
        })
        .level(min_log_level)
        .chain(io::stdout())
        .apply()?;

    IS_LOGGER_SET_UP.store(true, Ordering::Relaxed);

    Ok(())
}
