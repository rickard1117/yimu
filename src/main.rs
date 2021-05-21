mod buffer;
mod file;
mod logger;

macro_rules! log {
    ($level:expr, $($arg:tt)*) => {
        logger::Logger::log($level, logger::nowtime(), file!(), line!(), format!($($arg)*))
    };
}

macro_rules! debug {
    ($($arg:tt)*) => {
        if logger::Logger::level() as u8 <= logger::LogLevel::DEBUG as u8 {
            log!(logger::LogLevel::DEBUG, $($arg)*);
        }
    };
}

// macro_rules! info {
//     ($($arg:tt)*) => {
//         if logger::Logger::level() as u8 <= logger::LogLevel::INFO as u8 {
//             log!(logger::LogLevel::INFO, $($arg)*);
//         }
//     };
// }

fn main() {
    let content = "abcd";
    let content2 = String::from("abcd");
    debug!("{}", "content2");
    debug!("{}", content);
    debug!("{}", content2);
}
