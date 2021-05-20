use crate::logging;
use std::env;
use std::time::SystemTime;

#[derive(Copy, Clone)]
pub enum LogLevel {
    TRACE = 1,
    DEBUG,
    INFO,
    WARN,
    ERROR,
    FATAL,
}

static mut flag: bool = logToTerminal();

const fn logToTerminal() -> bool {
    match env::var("LOG_TERMINAL") {
        Ok(val) => {
            if val == "on" {
                true
            } else {
                false
            }
        }
        Err(_) => true,
    }
}

pub struct Logger<T: logging::LogImp> {
    // level: LogLevel,
    // file: String,
    // line: u32,
    // t: Time,
    // buf: buffer::Buffer,
    logging: T,
}

fn level2str(level: LogLevel) -> String {
    match level {
        LogLevel::TRACE => String::from("TRACE"),
        LogLevel::DEBUG => String::from("DEBUG"),
        LogLevel::INFO => String::from("INFO"),
        LogLevel::WARN => String::from("WARN"),
        LogLevel::ERROR => String::from("ERROR"),
        LogLevel::FATAL => String::from("FATAL"),
    }
}

impl<T: logging::LogImp> Logger<T> {
    // FIXME : use file:&String replace file:String for performance?
    fn new(imp: T) -> Logger<T> {
        Logger { logging: imp }
    }

    fn level() -> LogLevel {
        LogLevel::INFO
    }

    fn log(&mut self, l: LogLevel, time: Time, file: &String, line: u32, content: String) {
        let s = format!("{} {} {} {} {}\n", level2str(l), time, file, line, &content);
        self.logging.output(&s)
    }
}

macro_rules! log {
    ($level:expr, $($arg:tt)*) => {
        Logger::new().log($level, nowtime(), file!(), line!, format!($($arg)*))
    };
}

macro_rules! debug {
    ($($arg:tt)*) => {
        if Logger::level() as u8 <= LogLevel::DEBUG as u8 {
            log!(LogLevel::DEBUG, $($arg)*);
        }
    };
}

// #[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        if Logger::level() as u8 <= LogLevel::INFO as u8 {
            log!(LogLevel::INFO, $($arg)*);
        }
    };
}

type Time = u64;

fn nowtime() -> Time {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
