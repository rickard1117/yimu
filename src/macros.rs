#[macro_export]
macro_rules! log {
    ($level:expr, $($arg:tt)*) => {
        logger::Logger::log($level, logger::nowtime(), file!(), line!(), format!($($arg)*))
    };
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        if logger::Logger::level() as u8 <= logger::LogLevel::DEBUG as u8 {
            log!(logger::LogLevel::DEBUG, $($arg)*);
        }
    };
}
