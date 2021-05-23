use std::time::SystemTime;

fn DefaultOutput(content: &String) {
    println!("{}", content);
}

static mut g_log_output: fn(&String) = DefaultOutput;
static mut g_log_level: LogLevel = LogLevel::DEBUG;

pub struct Logger {}

impl Logger {
    pub fn setOutput(f: fn(&String)) {
        unsafe {
            g_log_output = f;
        }
    }

    pub fn setLevel(level: LogLevel) {
        unsafe {
            g_log_level = level;
        }
    }

    pub fn level() -> LogLevel {
        unsafe { g_log_level }
    }

    pub fn log(l: LogLevel, time: Time, file: &str, line: u32, content: String) {
        let s = format!("{} {} {} {} {}\n", level2str(l), time, file, line, content);
        unsafe {
            g_log_output(&s);
        }
    }
}

type Time = u64;

pub fn nowtime() -> Time {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn level2str(level: LogLevel) -> &'static str {
    match level {
        LogLevel::TRACE => "TRACE",
        LogLevel::DEBUG => "DEBUG",
        LogLevel::INFO => "INFO",
        LogLevel::WARN => "WARN",
        LogLevel::ERROR => "ERROR",
        LogLevel::FATAL => "FATAL",
    }
}

#[derive(Copy, Clone)]
pub enum LogLevel {
    TRACE = 1,
    DEBUG,
    INFO,
    WARN,
    ERROR,
    FATAL,
}
