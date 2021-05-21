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

type LogOutput = fn(&String);

fn DefaultOutput(content: &String) {
    print!("{}", content)
}

static mut g_output: LogOutput = DefaultOutput;

pub struct Logger {}

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

impl Logger {
    pub fn level() -> LogLevel {
        LogLevel::DEBUG
    }

    fn setOutput(f: LogOutput) {
        // don't call this function in different threads.
        // we should only call this function once when initing the whole library, like in the main function.
        unsafe {
            g_output = f;
        }
    }

    pub fn log(l: LogLevel, time: Time, file: &str, line: u32, content: String) {
        let s = format!("{} {} {} {} {}\n", level2str(l), time, file, line, content);
        unsafe { g_output(&s) }
    }

    // pub fn log<S: AsRef<str> + std::fmt::Display>(
    //     l: LogLevel,
    //     time: Time,
    //     file: &String,
    //     line: u32,
    //     content: S,
    // ) {
    //     let s = format!("{} {} {} {} {}\n", level2str(l), time, file, line, content);
    //     unsafe { g_output(&s) }
    // }
}

type Time = u64;

pub fn nowtime() -> Time {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
