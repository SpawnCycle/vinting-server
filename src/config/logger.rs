use anyhow::Context;
use log::LevelFilter;
use log4rs::{
    append::{
        console::ConsoleAppender,
        rolling_file::{
            RollingFileAppender,
            policy::compound::{
                CompoundPolicy, roll::fixed_window::FixedWindowRoller, trigger::size::SizeTrigger,
            },
        },
    },
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
};

const TRIGGER_FILE_SIZE: u64 = 10 * 1024 * 1024;
const FILE_PATH: &str = "./logs/vinting.log";
const ROLLER_PATTERN: &str = "./logs/archive/vinting.{}.log";

pub fn logger() -> anyhow::Result<log4rs::Handle> {
    let file_level = if cfg!(test) {
        LevelFilter::Warn
    } else if cfg!(debug_assertions) {
        LevelFilter::Trace
    } else {
        LevelFilter::Info
    };
    let stdout_level = file_level.decrement_severity();

    let encoder = PatternEncoder::new("{h({l})} {d(%Y-%m-%d %H:%M:%S)} - {m}{n}");

    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(encoder.clone()))
        .build();

    let trigger = SizeTrigger::new(TRIGGER_FILE_SIZE);
    let roller = FixedWindowRoller::builder().build(ROLLER_PATTERN, 5)?;
    let policy = CompoundPolicy::new(Box::new(trigger), Box::new(roller));

    let logfile = RollingFileAppender::builder()
        .encoder(Box::new(encoder.clone()))
        .build(FILE_PATH, Box::new(policy))?;

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(stdout_level)))
                .build("stdout", Box::new(stdout)),
        )
        .build(
            Root::builder()
                .appender("stdout")
                .appender("logfile")
                .build(file_level),
        )?;

    let handle = log4rs::init_config(config).context("Couldn't configure the logger")?;

    Ok(handle)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn logger_set_up() -> anyhow::Result<()> {
        let _ = logger()?;

        Ok(())
    }
}
