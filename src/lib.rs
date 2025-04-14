use log::{info, warn, error, debug};
use serde::Deserialize;
use clap::Parser;
use std::str::FromStr;

#[derive(Debug, Parser)]
pub struct Cli {
    #[clap(value_parser, default_value_t = String::from(""))]
    pub message: String,
    #[clap(short = 't', long, default_value_t = String::from("info"))]
    pub log_type: String,
}

#[derive(Debug, Deserialize)]
pub enum LogType {
    Info,
    Warn,
    Error,
    Debug,
}

impl LogType {
    pub const fn default() -> LogType {
        LogType::Info
    }
}

impl FromStr for LogType {
    type Err = ();

    fn from_str(s: &str) -> Result<LogType, ()> {
        match s.to_lowercase().as_str() {
            "info" => Ok(LogType::Info),
            "warn" => Ok(LogType::Warn),
            "error" => Ok(LogType::Error),
            "debug" => Ok(LogType::Debug),
            _ => Ok(LogType::Info),
        }
    }
}

pub fn run_from_args(args: Cli) {
    let log_type = LogType::from_str(&args.log_type).unwrap();
    log_message(&args.message, log_type);
}

pub fn log_message(msg: &str, log_type: LogType) {
    match log_type {
        LogType::Info =>  info!("{}", msg),
        LogType::Warn =>  warn!("{}", msg),
        LogType::Error => error!("{}", msg),
        LogType::Debug => debug!("{}", msg),
    }
}